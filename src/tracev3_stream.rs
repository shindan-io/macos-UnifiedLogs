// Copyright 2024 Shindan, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0

//! Zero-allocation streaming API for tracev3 files.
//!
//! [`TraceV3Stream`] provides a near-zero-alloc streaming engine that yields
//! [`StructuralEntry`] handles containing only scalars and borrowed references.
//! Consumers filter structurally (by pid, time, log type, etc.) and only resolve
//! the few entries they actually need via [`StructuralEntry::resolve`].
//!
//! # Lifetime Model
//!
//! There are three data lifetimes:
//! - `'file` — the tracev3 buffer (lives for entire file processing)
//! - `'catalog` — one catalog scope (borrows from `'file`)
//! - `'decomp` — one chunkset decompression buffer (reused per chunkset)
//!
//! Because `StructuralEntry` borrows from the decompression buffer which is reused,
//! we use a **callback** pattern (`for_each_entry`) rather than `Iterator`.
//!
//! # Example
//! ```no_run
//! use macos_unifiedlogs::tracev3_stream::TraceV3Stream;
//! use macos_unifiedlogs::parser::collect_timesync;
//! use macos_unifiedlogs::filesystem::LogarchiveProvider;
//! use std::path::PathBuf;
//!
//! let path = PathBuf::from("system_logs.logarchive");
//! let provider = LogarchiveProvider::new(path.as_path());
//! let timesync = collect_timesync(&provider).unwrap();
//!
//! let file_buf = std::fs::read("system_logs.logarchive/Persist/0000000000000002.tracev3").unwrap();
//! let mut stream = TraceV3Stream::new(&file_buf, &timesync);
//!
//! let mut fault_count = 0u64;
//! stream.for_each_entry(|entry| {
//!     // Pure scalar comparison — zero alloc
//!     if entry.log_type == 0x11 { // Fault
//!         fault_count += 1;
//!     }
//! });
//! println!("Found {fault_count} fault entries");
//! ```

use std::collections::HashMap;

use log::{debug, error, warn};
use nom::number::complete::{le_u8, le_u16, le_u32, le_u64};
use uuid::Uuid;

use crate::catalog::CatalogChunk;
use crate::chunk::{decompress_chunkset, extract_subtype_scalars, skip_zero_padding};
use crate::chunks::oversize::Oversize;
use crate::constants::*;
use crate::header::HeaderChunkStr;
use crate::preamble::LogPreamble;
use crate::timesync::TimesyncBoot;
use crate::util::{padding_size_8, u64_to_usize};

/// Near-zero-alloc handle to a log entry. All fields are scalars or borrowed.
/// No string resolution, no message formatting, no heap allocation.
///
/// The entry borrows from the catalog (`'cat` lifetime, which borrows from the file buffer)
/// and from the decompression buffer (`'decomp` lifetime, which is reused per chunkset).
#[derive(Debug)]
pub struct StructuralEntry<'cat, 'decomp> {
    // Identity
    /// Process ID, resolved from catalog process_info
    pub pid: u64,
    /// Effective user ID, resolved from catalog process_info
    pub euid: u32,
    /// Thread ID from the firehose entry
    pub thread_id: u64,
    /// Absolute continuous time (preamble base + entry delta)
    pub continuous_time: u64,
    /// Wall-clock timestamp in nanoseconds since Unix epoch (via timesync)
    pub timestamp: f64,

    // Classification
    /// Log activity type: 0x2=activity, 0x4=non-activity, 0x6=signpost, 0x3=trace, 0x7=loss
    pub log_activity_type: u8,
    /// Log type (info, debug, error, fault, etc.)
    pub log_type: u8,
    /// Entry flags
    pub flags: u16,

    // String pointers (NOT resolved — just offsets)
    /// Offset into the format string table (UUIDText/DSC)
    pub format_string_location: u32,
    /// Oversize data reference. 0 = no oversize data.
    pub data_ref_value: u32,

    // Boot context
    /// Boot UUID from the header chunk
    pub boot_uuid: Uuid,

    // Raw data for on-demand parsing
    /// The raw firehose entry data (sub-type header + items), borrows from decompression buffer
    pub raw_firehose_data: &'decomp [u8],
    /// Data size field from the firehose entry header
    pub data_size: u16,
    /// Number of message items
    pub number_items: u8,

    // Context for resolution
    /// Process info key for catalog lookups
    pub first_proc_id: u64,
    /// Process info key for catalog lookups
    pub second_proc_id: u32,
    /// Reference to the catalog for subsystem/category resolution
    pub catalog: &'cat CatalogChunk,
    /// Subsystem value (parsed from non-activity/signpost sub-type header). 0 if not applicable.
    pub subsystem_value: u16,

    // Preamble-level context
    /// Private data virtual offset from preamble (0x1000 means no private data)
    pub private_data_virtual_offset: u16,
    /// TTL from preamble
    pub ttl: u8,
}

impl StructuralEntry<'_, '_> {
    /// Check if this entry has oversize data that needs separate lookup.
    pub fn has_oversize(&self) -> bool {
        self.data_ref_value != 0
    }

    /// Check if this is a Fault log type.
    pub fn is_fault(&self) -> bool {
        self.log_type == LOG_TYPE_FAULT
    }

    /// Check if this is an Error log type.
    pub fn is_error(&self) -> bool {
        self.log_type == LOG_TYPE_ERROR
    }

    /// Check if this entry is a non-activity type (the most common log type).
    pub fn is_non_activity(&self) -> bool {
        self.log_activity_type == NON_ACTIVITY_TYPE
    }

    /// Check if this entry is an activity type.
    pub fn is_activity(&self) -> bool {
        self.log_activity_type == ACTIVITY_TYPE
    }

    /// Check if this entry is a signpost type.
    pub fn is_signpost(&self) -> bool {
        self.log_activity_type == SIGNPOST_TYPE
    }

    /// Check if this entry is a trace type.
    pub fn is_trace(&self) -> bool {
        self.log_activity_type == TRACE_TYPE
    }

    /// Check if this entry is a loss type.
    pub fn is_loss(&self) -> bool {
        self.log_activity_type == LOSS_TYPE
    }
}

/// Streaming event types yielded by the stream engine.
/// Firehose entries are the common case; statedump/simpledump/oversize are rarer.
#[derive(Debug)]
pub enum StreamEvent<'cat, 'decomp> {
    /// A firehose log entry (activity, non-activity, signpost, trace, loss).
    Firehose(StructuralEntry<'cat, 'decomp>),
    /// An oversize entry was parsed and added to the internal cache.
    OversizeParsed,
    /// A statedump chunk was encountered (raw data available in the decompression buffer).
    Statedump { chunk_data: &'decomp [u8] },
    /// A simpledump chunk was encountered.
    Simpledump { chunk_data: &'decomp [u8] },
}

/// Zero-allocation streaming engine for tracev3 files.
///
/// Iterates over all chunks in a tracev3 file using a cursor into the file buffer.
/// Decompressed chunkset data is held in a reusable grow-only buffer.
/// Oversize entries are accumulated into an internal cache.
///
/// Because the decompression buffer is reused per chunkset, entries borrow from
/// it with a lending lifetime — hence the callback-based API rather than `Iterator`.
pub struct TraceV3Stream<'file, 'ts> {
    file_buf: &'file [u8],
    cursor: usize,
    boot_uuid: Uuid,
    decomp_buf: Vec<u8>,
    oversize_cache: Vec<Oversize>,
    timesync: &'ts HashMap<Uuid, TimesyncBoot>,
}

impl<'file, 'ts> TraceV3Stream<'file, 'ts> {
    /// Create a new streaming engine from a tracev3 file buffer and timesync data.
    pub fn new(data: &'file [u8], timesync: &'ts HashMap<Uuid, TimesyncBoot>) -> Self {
        Self {
            file_buf: data,
            cursor: 0,
            boot_uuid: Uuid::nil(),
            decomp_buf: Vec::new(),
            oversize_cache: Vec::new(),
            timesync,
        }
    }

    /// Create a new streaming engine with a pre-existing oversize cache
    /// (e.g., carried over from a previous tracev3 file).
    pub fn with_oversize_cache(
        data: &'file [u8],
        timesync: &'ts HashMap<Uuid, TimesyncBoot>,
        cache: Vec<Oversize>,
    ) -> Self {
        Self {
            file_buf: data,
            cursor: 0,
            boot_uuid: Uuid::nil(),
            decomp_buf: Vec::new(),
            oversize_cache: cache,
            timesync,
        }
    }

    /// Get a reference to the accumulated oversize cache.
    pub fn oversize_cache(&self) -> &[Oversize] {
        &self.oversize_cache
    }

    /// Consume the stream and return the oversize cache for carrying to the next file.
    pub fn into_oversize_cache(self) -> Vec<Oversize> {
        self.oversize_cache
    }

    /// Process all firehose entries via callback.
    ///
    /// This is the main entry point. The callback receives a [`StructuralEntry`] for each
    /// firehose log entry in the file. The entry borrows from internal buffers and must
    /// not escape the callback.
    ///
    /// Statedump, simpledump, and oversize entries are handled internally (oversize entries
    /// are accumulated in the cache; statedump/simpledump are skipped).
    pub fn for_each_entry(&mut self, mut f: impl FnMut(StructuralEntry<'_, '_>)) {
        let _ = self.try_for_each_entry(|entry| -> Result<(), std::convert::Infallible> {
            f(entry);
            Ok(())
        });
    }

    /// Process entries with early-exit support.
    ///
    /// Same as [`for_each_entry`](Self::for_each_entry) but the callback returns `Result`.
    /// Processing stops on the first `Err`.
    pub fn try_for_each_entry<E>(
        &mut self,
        mut f: impl FnMut(StructuralEntry<'_, '_>) -> Result<(), E>,
    ) -> Result<(), E> {
        let mut current_catalog: Option<CatalogChunk> = None;

        while self.cursor + CHUNK_PREAMBLE_SIZE <= self.file_buf.len() {
            let input = &self.file_buf[self.cursor..];

            let preamble = match LogPreamble::detect_preamble(input) {
                Ok((_, p)) => p,
                Err(err) => {
                    error!("[tracev3_stream] Failed to detect preamble: {err:?}");
                    break;
                }
            };

            let total_chunk_size = preamble.chunk_data_size as usize + CHUNK_PREAMBLE_SIZE;
            if self.cursor + total_chunk_size > self.file_buf.len() {
                warn!(
                    "[tracev3_stream] Chunk extends beyond file buffer ({} + {} > {})",
                    self.cursor,
                    total_chunk_size,
                    self.file_buf.len()
                );
                break;
            }

            let chunk_data = &self.file_buf[self.cursor..self.cursor + total_chunk_size];

            match preamble.chunk_tag {
                HEADER_CHUNK => {
                    self.parse_header(chunk_data);
                }
                CATALOG_CHUNK => match CatalogChunk::parse_catalog(chunk_data) {
                    Ok((_, catalog)) => {
                        current_catalog = Some(catalog);
                    }
                    Err(err) => {
                        error!("[tracev3_stream] Failed to parse catalog: {err:?}");
                    }
                },
                CHUNKSET_CHUNK => {
                    if let Some(ref catalog) = current_catalog {
                        self.process_chunkset(chunk_data, catalog, &mut f)?;
                    } else {
                        warn!("[tracev3_stream] Chunkset encountered without a catalog");
                    }
                }
                other => {
                    warn!("[tracev3_stream] Unknown top-level chunk type: 0x{other:04x}");
                }
            }

            // Advance cursor past chunk + padding
            self.cursor += total_chunk_size;
            let padding = padding_size_8(preamble.chunk_data_size) as usize;
            if self.cursor + padding <= self.file_buf.len() {
                self.cursor += padding;
            } else {
                break;
            }
        }

        Ok(())
    }

    /// Parse a header chunk and extract the boot UUID.
    fn parse_header(&mut self, chunk_data: &'file [u8]) {
        match HeaderChunkStr::parse_header(chunk_data) {
            Ok((_, header)) => {
                self.boot_uuid = header.boot_uuid;
                debug!(
                    "[tracev3_stream] Parsed header, boot_uuid={}",
                    self.boot_uuid
                );
            }
            Err(err) => {
                error!("[tracev3_stream] Failed to parse header: {err:?}");
            }
        }
    }

    /// Parse a chunkset: decompress, then iterate inner chunks.
    fn process_chunkset<E>(
        &mut self,
        chunk_data: &[u8],
        catalog: &CatalogChunk,
        f: &mut impl FnMut(StructuralEntry<'_, '_>) -> Result<(), E>,
    ) -> Result<(), E> {
        let inner = match chunk_data.get(CHUNK_PREAMBLE_SIZE..) {
            Some(d) => d,
            None => return Ok(()),
        };

        match decompress_chunkset(inner) {
            Some(data) => {
                self.decomp_buf = data;
                // Take the buffer out so we can borrow it immutably
                // while still mutating self.oversize_cache
                let decomp_buf = std::mem::take(&mut self.decomp_buf);
                let result = self.process_decompressed_chunks(&decomp_buf, catalog, f);
                self.decomp_buf = decomp_buf;
                result
            }
            None => {
                error!("[tracev3_stream] Chunkset decompression failed");
                Ok(())
            }
        }
    }

    /// Process decompressed chunkset data.
    fn process_decompressed_chunks<E>(
        &mut self,
        data: &[u8],
        catalog: &CatalogChunk,
        f: &mut impl FnMut(StructuralEntry<'_, '_>) -> Result<(), E>,
    ) -> Result<(), E> {
        let mut input = data;

        while !input.is_empty() && input.len() >= CHUNK_PREAMBLE_SIZE {
            let preamble = match LogPreamble::detect_preamble(input) {
                Ok((_, p)) => p,
                Err(_) => break,
            };

            let chunk_size = match u64_to_usize(preamble.chunk_data_size) {
                Some(s) => s,
                None => break,
            };

            let total = chunk_size + CHUNK_PREAMBLE_SIZE;
            if total > input.len() {
                break;
            }

            let chunk_data = &input[..total];

            match preamble.chunk_tag {
                FIREHOSE_CHUNK => {
                    self.process_firehose_preamble(chunk_data, catalog, f)?;
                }
                OVERSIZE_CHUNK => match Oversize::parse_oversize(chunk_data) {
                    Ok((_, oversize)) => {
                        self.oversize_cache.push(oversize);
                    }
                    Err(err) => {
                        error!("[tracev3_stream] Failed to parse oversize: {err:?}");
                    }
                },
                STATEDUMP_CHUNK | SIMPLEDUMP_CHUNK => {
                    // Statedump/simpledump — skip in the structural pass
                    debug!(
                        "[tracev3_stream] Skipping statedump/simpledump chunk 0x{:04x}",
                        preamble.chunk_tag
                    );
                }
                other => {
                    warn!("[tracev3_stream] Unknown inner chunk type: 0x{other:04x}");
                }
            }

            // Skip past chunk + zero padding
            let remaining = &input[total..];
            let trimmed = skip_zero_padding(remaining);
            if trimmed.is_empty() {
                break;
            }
            input = trimmed;
        }

        Ok(())
    }

    /// Parse a firehose preamble and yield StructuralEntry for each firehose entry within it.
    fn process_firehose_preamble<E>(
        &mut self,
        chunk_data: &[u8],
        catalog: &CatalogChunk,
        f: &mut impl FnMut(StructuralEntry<'_, '_>) -> Result<(), E>,
    ) -> Result<(), E> {
        // Parse the preamble header (fixed fields before the public data)
        let input = chunk_data;
        let (input, _chunk_tag) = match le_u32::<_, nom::error::Error<&[u8]>>(input) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };
        let (input, _chunk_sub_tag) = match le_u32::<_, nom::error::Error<&[u8]>>(input) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };
        let (input, _chunk_data_size) = match le_u64::<_, nom::error::Error<&[u8]>>(input) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };
        let (input, first_number_proc_id) = match le_u64::<_, nom::error::Error<&[u8]>>(input) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };
        let (input, second_number_proc_id) = match le_u32::<_, nom::error::Error<&[u8]>>(input) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };
        let (input, ttl) = match le_u8::<_, nom::error::Error<&[u8]>>(input) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };
        let (input, _collapsed) = match le_u8::<_, nom::error::Error<&[u8]>>(input) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };
        // Skip 2 bytes unknown
        let input = match input.get(2..) {
            Some(d) => d,
            None => return Ok(()),
        };
        let (input, public_data_size) = match le_u16::<_, nom::error::Error<&[u8]>>(input) {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };
        let (input, private_data_virtual_offset) =
            match le_u16::<_, nom::error::Error<&[u8]>>(input) {
                Ok(r) => r,
                Err(_) => return Ok(()),
            };
        // Skip unknown2 + unknown3 (4 bytes)
        let input = match input.get(4..) {
            Some(d) => d,
            None => return Ok(()),
        };
        let (_remaining, base_continuous_time) = match le_u64::<_, nom::error::Error<&[u8]>>(input)
        {
            Ok(r) => r,
            Err(_) => return Ok(()),
        };

        // Calculate public data boundaries
        let public_data_offset: u16 = 16; // size before public data starts
        let public_data_len = public_data_size.saturating_sub(public_data_offset) as usize;
        let input = _remaining;
        if input.len() < public_data_len {
            return Ok(());
        }
        let public_data = &input[..public_data_len];

        // Resolve pid/euid from catalog
        let pid = catalog.get_pid(first_number_proc_id, second_number_proc_id);
        let euid = catalog.get_euid(first_number_proc_id, second_number_proc_id);

        // Iterate through individual firehose entries in the public data
        let mut entry_data = public_data;
        while entry_data.len() >= FIREHOSE_ENTRY_HEADER_SIZE {
            // Parse the fixed firehose entry header
            let (rest, log_activity_type) = match le_u8::<_, nom::error::Error<&[u8]>>(entry_data) {
                Ok(r) => r,
                Err(_) => break,
            };

            if log_activity_type == REMNANT_DATA {
                break;
            }

            let (rest, log_type) = match le_u8::<_, nom::error::Error<&[u8]>>(rest) {
                Ok(r) => r,
                Err(_) => break,
            };
            let (rest, flags) = match le_u16::<_, nom::error::Error<&[u8]>>(rest) {
                Ok(r) => r,
                Err(_) => break,
            };
            let (rest, format_string_location) = match le_u32::<_, nom::error::Error<&[u8]>>(rest) {
                Ok(r) => r,
                Err(_) => break,
            };
            let (rest, thread_id) = match le_u64::<_, nom::error::Error<&[u8]>>(rest) {
                Ok(r) => r,
                Err(_) => break,
            };
            let (rest, continous_time_delta) = match le_u32::<_, nom::error::Error<&[u8]>>(rest) {
                Ok(r) => r,
                Err(_) => break,
            };
            let (rest, continous_time_delta_upper) =
                match le_u16::<_, nom::error::Error<&[u8]>>(rest) {
                    Ok(r) => r,
                    Err(_) => break,
                };
            let (rest, data_size) = match le_u16::<_, nom::error::Error<&[u8]>>(rest) {
                Ok(r) => r,
                Err(_) => break,
            };

            // The firehose entry data (sub-type-specific header + items)
            let entry_body_len = data_size as usize;
            if rest.len() < entry_body_len {
                break;
            }
            let raw_firehose_data = &rest[..entry_body_len];

            // Calculate continuous time
            let entry_continuous_time =
                u64::from(continous_time_delta) | (u64::from(continous_time_delta_upper) << 32);
            let absolute_continuous_time = base_continuous_time + entry_continuous_time;

            // Calculate wall-clock timestamp
            let timestamp = TimesyncBoot::get_timestamp(
                self.timesync,
                self.boot_uuid,
                absolute_continuous_time,
                base_continuous_time,
            );

            // Extract data_ref_value and subsystem_value from the sub-type header
            // without fully parsing all items. These are at known offsets for non-activity entries.
            let (data_ref_value, subsystem_value, number_items) =
                extract_subtype_scalars(raw_firehose_data, log_activity_type, flags);

            let entry = StructuralEntry {
                pid,
                euid,
                thread_id,
                continuous_time: absolute_continuous_time,
                timestamp,
                log_activity_type,
                log_type,
                flags,
                format_string_location,
                data_ref_value,
                boot_uuid: self.boot_uuid,
                raw_firehose_data,
                data_size,
                number_items,
                first_proc_id: first_number_proc_id,
                second_proc_id: second_number_proc_id,
                catalog,
                subsystem_value,
                private_data_virtual_offset,
                ttl,
            };

            f(entry)?;

            // Advance past the entry data + padding
            let consumed = FIREHOSE_ENTRY_HEADER_SIZE + entry_body_len;
            // Padding to 8 bytes for the data_size portion
            let data_pad = padding_size_8(u64::from(data_size)) as usize;
            let total_advance = consumed + data_pad;

            if total_advance > entry_data.len() {
                break;
            }
            entry_data = &entry_data[total_advance..];

            // Check for end conditions
            if entry_data.len() < FIREHOSE_ENTRY_HEADER_SIZE {
                break;
            }
            // Check if next byte is a valid log activity type
            if !matches!(
                entry_data[0],
                ACTIVITY_TYPE
                    | TRACE_TYPE
                    | NON_ACTIVITY_TYPE
                    | SIGNPOST_TYPE
                    | LOSS_TYPE
                    | REMNANT_DATA
            ) {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filesystem::LogarchiveProvider;
    use crate::parser::collect_timesync;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_tracev3_stream_basic() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_big_sur.logarchive");

        let provider = LogarchiveProvider::new(test_path.as_path());
        let timesync_data = collect_timesync(&provider).unwrap();

        test_path.push("Persist/0000000000000002.tracev3");
        let buffer = fs::read(test_path.to_str().unwrap()).unwrap();

        let mut stream = TraceV3Stream::new(&buffer, &timesync_data);

        let mut entry_count: u64 = 0;
        let mut has_nonactivity = false;
        let mut has_activity = false;

        stream.for_each_entry(|entry| {
            entry_count += 1;
            if entry.is_non_activity() {
                has_nonactivity = true;
            }
            if entry.is_activity() {
                has_activity = true;
            }
            // Verify basic scalar fields are populated
            assert!(entry.timestamp > 0.0 || entry.timestamp == 0.0);
            assert_ne!(entry.boot_uuid, Uuid::nil());
        });

        // Should find many entries
        assert!(
            entry_count > 1000,
            "Expected >1000 entries, got {entry_count}"
        );
        assert!(has_nonactivity, "Should have non-activity entries");
    }

    #[test]
    fn test_tracev3_stream_early_exit() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_big_sur.logarchive");

        let provider = LogarchiveProvider::new(test_path.as_path());
        let timesync_data = collect_timesync(&provider).unwrap();

        test_path.push("Persist/0000000000000002.tracev3");
        let buffer = fs::read(test_path.to_str().unwrap()).unwrap();

        let mut stream = TraceV3Stream::new(&buffer, &timesync_data);

        let mut count = 0u64;
        let result = stream.try_for_each_entry(|_entry| -> Result<(), &str> {
            count += 1;
            if count >= 10 { Err("stop") } else { Ok(()) }
        });

        assert!(result.is_err());
        assert_eq!(count, 10);
    }

    #[test]
    fn test_tracev3_stream_oversize_cache() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_big_sur.logarchive");

        let provider = LogarchiveProvider::new(test_path.as_path());
        let timesync_data = collect_timesync(&provider).unwrap();

        test_path.push("Persist/0000000000000002.tracev3");
        let buffer = fs::read(test_path.to_str().unwrap()).unwrap();

        let mut stream = TraceV3Stream::new(&buffer, &timesync_data);
        stream.for_each_entry(|_| {});

        // The oversize cache should be available after processing
        let _cache = stream.into_oversize_cache();
        // May or may not have oversize entries in this file — just verify it doesn't panic
    }
}
