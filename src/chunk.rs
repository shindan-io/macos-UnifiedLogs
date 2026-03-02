// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use std::fmt;

use log::{error, warn};
use lz4_flex::decompress;
use nom::bytes::complete::{take, take_while};
use nom::number::complete::{le_u8, le_u16, le_u32, le_u64};

use crate::catalog::CatalogChunk;
use crate::chunks::firehose::firehose_log::FirehosePreamble;
use crate::chunks::oversize::Oversize;
use crate::chunks::simpledump::{SimpleDump, SimpleDumpOwned};
use crate::chunks::statedump::{Statedump, StatedumpOwned};
use crate::constants::*;
use crate::header::{HeaderChunkOwned, HeaderChunkStr};
use crate::preamble::LogPreamble;
use crate::util::{padding_size_8, u64_to_usize};

/// A parsed chunk from a tracev3 file.
pub enum Chunk {
    Header(HeaderChunkOwned),
    Catalog(CatalogChunk),
    Firehose(FirehosePreamble),
    Oversize(Oversize),
    Simpledump(SimpleDumpOwned),
    Statedump(StatedumpOwned),
}

/// Errors that can occur during chunk iteration.
pub enum ChunkError {
    /// Preamble couldn't be parsed (truncated data at top level)
    InvalidPreamble,
    /// Chunk data size exceeds available buffer
    TruncatedChunk { chunk_tag: u32 },
    /// LZ4 decompression failed
    DecompressionFailed,
    /// A known chunk type failed to parse (logged, chunk skipped)
    ParseFailed { chunk_tag: u32 },
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPreamble => write!(f, "Failed to detect chunk preamble"),
            Self::TruncatedChunk { chunk_tag } => {
                write!(f, "Chunk 0x{chunk_tag:04x} extends beyond buffer")
            }
            Self::DecompressionFailed => write!(f, "Failed to decompress chunkset"),
            Self::ParseFailed { chunk_tag } => {
                write!(f, "Failed to parse chunk 0x{chunk_tag:04x}")
            }
        }
    }
}

impl fmt::Debug for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for ChunkError {}

/// Iterator that walks top-level and inner (decompressed chunkset) chunks of a
/// tracev3 file, yielding parsed, owned [`Chunk`] variants.
pub struct ChunksIterator<'a> {
    data: &'a [u8],
    cursor: usize,
    decomp_buf: Vec<u8>,
    inner_cursor: usize,
    in_chunkset: bool,
}

impl<'a> ChunksIterator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            cursor: 0,
            decomp_buf: Vec::new(),
            inner_cursor: 0,
            in_chunkset: false,
        }
    }
}

impl Iterator for ChunksIterator<'_> {
    type Item = Result<Chunk, ChunkError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Drain inner (decompressed chunkset) chunks first
            if self.in_chunkset {
                if let Some(result) = self.next_inner() {
                    return Some(result);
                }
                self.in_chunkset = false;
                continue;
            }

            // Top-level mode
            let input = &self.data[self.cursor..];
            if input.len() < CHUNK_PREAMBLE_SIZE {
                return None;
            }

            let preamble = match LogPreamble::detect_preamble(input) {
                Ok((_, p)) => p,
                Err(_) => return Some(Err(ChunkError::InvalidPreamble)),
            };

            let chunk_size = match u64_to_usize(preamble.chunk_data_size) {
                Some(c) => c,
                None => {
                    return Some(Err(ChunkError::TruncatedChunk {
                        chunk_tag: preamble.chunk_tag,
                    }));
                }
            };

            let total_chunk_size = chunk_size + CHUNK_PREAMBLE_SIZE;
            if total_chunk_size > input.len() {
                warn!(
                    "[macos-unifiedlogs] Chunk extends beyond buffer ({total_chunk_size} > {})",
                    input.len()
                );
                return None;
            }

            let chunk_data = &input[..total_chunk_size];

            // Advance cursor past chunk + alignment padding
            let padding = padding_size_8(preamble.chunk_data_size) as usize;
            let advance = total_chunk_size + padding;
            self.cursor = (self.cursor + advance).min(self.data.len());

            match preamble.chunk_tag {
                HEADER_CHUNK => match HeaderChunkStr::parse_header(chunk_data) {
                    Ok((_, header)) => return Some(Ok(Chunk::Header(header.into_owned()))),
                    Err(err) => {
                        error!("[macos-unifiedlogs] Failed to parse header data: {err:?}");
                        return Some(Err(ChunkError::ParseFailed {
                            chunk_tag: HEADER_CHUNK,
                        }));
                    }
                },
                CATALOG_CHUNK => match CatalogChunk::parse_catalog(chunk_data) {
                    Ok((_, catalog)) => return Some(Ok(Chunk::Catalog(catalog))),
                    Err(err) => {
                        error!("[macos-unifiedlogs] Failed to parse catalog data: {err:?}");
                        return Some(Err(ChunkError::ParseFailed {
                            chunk_tag: CATALOG_CHUNK,
                        }));
                    }
                },
                CHUNKSET_CHUNK => {
                    match decompress_chunkset(&chunk_data[CHUNK_PREAMBLE_SIZE..]) {
                        Some(data) => {
                            self.decomp_buf = data;
                            self.inner_cursor = 0;
                            self.in_chunkset = true;
                            continue; // loop back to yield inner chunks
                        }
                        None => return Some(Err(ChunkError::DecompressionFailed)),
                    }
                }
                other => {
                    warn!("[macos-unifiedlogs] Unknown chunk type: 0x{other:04x}");
                    continue; // skip unknown chunks
                }
            }
        }
    }
}

impl ChunksIterator<'_> {
    fn next_inner(&mut self) -> Option<Result<Chunk, ChunkError>> {
        loop {
            let input = &self.decomp_buf[self.inner_cursor..];
            if input.len() < CHUNK_PREAMBLE_SIZE {
                return None;
            }

            let preamble = match LogPreamble::detect_preamble(input) {
                Ok((_, p)) => p,
                Err(_) => return None,
            };

            let chunk_size = match u64_to_usize(preamble.chunk_data_size) {
                Some(c) => c,
                None => return None,
            };

            let total = chunk_size + CHUNK_PREAMBLE_SIZE;
            if total > input.len() {
                return None;
            }

            let chunk_data = &input[..total];

            // Advance past chunk + skip zero-byte padding (matching original behavior)
            let remaining = &input[total..];
            let zeros = remaining.iter().take_while(|&&b| b == 0).count();
            let new_pos = self.inner_cursor + total + zeros;
            self.inner_cursor = new_pos.min(self.decomp_buf.len());

            match preamble.chunk_tag {
                FIREHOSE_CHUNK => match FirehosePreamble::parse_firehose_preamble(chunk_data) {
                    Ok((_, firehose)) => return Some(Ok(Chunk::Firehose(firehose))),
                    Err(err) => {
                        error!("[macos-unifiedlogs] Failed to parse firehose log entry: {err:?}");
                        return Some(Err(ChunkError::ParseFailed {
                            chunk_tag: FIREHOSE_CHUNK,
                        }));
                    }
                },
                OVERSIZE_CHUNK => match Oversize::parse_oversize(chunk_data) {
                    Ok((_, oversize)) => return Some(Ok(Chunk::Oversize(oversize))),
                    Err(err) => {
                        error!("[macos-unifiedlogs] Failed to parse oversize log entry: {err:?}");
                        return Some(Err(ChunkError::ParseFailed {
                            chunk_tag: OVERSIZE_CHUNK,
                        }));
                    }
                },
                STATEDUMP_CHUNK => match Statedump::parse_statedump(chunk_data) {
                    Ok((_, statedump)) => {
                        return Some(Ok(Chunk::Statedump(statedump.into_owned())));
                    }
                    Err(err) => {
                        error!("[macos-unifiedlogs] Failed to parse statedump log entry: {err:?}");
                        return Some(Err(ChunkError::ParseFailed {
                            chunk_tag: STATEDUMP_CHUNK,
                        }));
                    }
                },
                SIMPLEDUMP_CHUNK => match SimpleDump::parse_simpledump(chunk_data) {
                    Ok((_, simpledump)) => {
                        return Some(Ok(Chunk::Simpledump(simpledump.into_owned())));
                    }
                    Err(err) => {
                        error!("[macos-unifiedlogs] Failed to parse simpledump log entry: {err:?}");
                        return Some(Err(ChunkError::ParseFailed {
                            chunk_tag: SIMPLEDUMP_CHUNK,
                        }));
                    }
                },
                other => {
                    warn!("[macos-unifiedlogs] Unknown chunkset type: 0x{other:04x}");
                    continue; // skip unknown inner chunks
                }
            }
        }
    }
}

/// Decompress a chunkset's inner data (BV41 compressed or uncompressed).
pub(crate) fn decompress_chunkset(data: &[u8]) -> Option<Vec<u8>> {
    let (input, signature) = le_u32::<_, nom::error::Error<&[u8]>>(data).ok()?;
    let (input, uncompress_size) = le_u32::<_, nom::error::Error<&[u8]>>(input).ok()?;

    if signature == BV41_UNCOMPRESSED {
        let (_, raw) =
            take::<_, _, nom::error::Error<&[u8]>>(uncompress_size as usize)(input).ok()?;
        return Some(raw.to_vec());
    }

    if signature != BV41_COMPRESSED {
        error!(
            "[macos-unifiedlogs] Incorrect compression signature expected bv41, got: {signature:?}"
        );
        return None;
    }

    let (input, block_size) = le_u32::<_, nom::error::Error<&[u8]>>(input).ok()?;
    let compressed = input.get(..block_size as usize)?;

    match decompress(compressed, uncompress_size as usize) {
        Ok(decompressed) => Some(decompressed),
        Err(err) => {
            error!("[macos-unifiedlogs] Failed to decompress log data: {err:?}");
            None
        }
    }
}

/// Extract key scalar values from a firehose sub-type header without full parsing.
/// Returns (data_ref_value, subsystem_value, number_items).
pub(crate) fn extract_subtype_scalars(
    raw_data: &[u8],
    log_activity_type: u8,
    flags: u16,
) -> (u32, u16, u8) {
    let mut data_ref_value: u32 = 0;
    let mut subsystem_value: u16 = 0;
    let mut number_items: u8 = 0;

    if raw_data.is_empty() {
        return (data_ref_value, subsystem_value, number_items);
    }

    match log_activity_type {
        NON_ACTIVITY_TYPE => {
            if let Ok((_, vals)) = parse_nonactivity_scalars(raw_data, flags) {
                data_ref_value = vals.0;
                subsystem_value = vals.1;
                number_items = vals.2;
            }
        }
        ACTIVITY_TYPE => {
            if let Ok((_, n)) = parse_activity_item_count(raw_data, flags) {
                number_items = n;
            }
        }
        SIGNPOST_TYPE => {
            if let Ok((_, vals)) = parse_signpost_scalars(raw_data, flags) {
                subsystem_value = vals.0;
                number_items = vals.1;
            }
        }
        TRACE_TYPE => {
            if let Ok((_, n)) = parse_trace_item_count(raw_data) {
                number_items = n;
            }
        }
        LOSS_TYPE => {
            // Loss entries don't have items
        }
        _ => {}
    }

    (data_ref_value, subsystem_value, number_items)
}

/// Parse non-activity sub-type header to extract data_ref, subsystem, and item count.
fn parse_nonactivity_scalars(data: &[u8], flags: u16) -> nom::IResult<&[u8], (u32, u16, u8)> {
    let mut input = data;
    let mut data_ref_value: u32 = 0;
    let mut subsystem_value: u16 = 0;

    if (flags & FLAG_HAS_CURRENT_AID) != 0 {
        let (i, _) = le_u32(input)?;
        let (i, _) = le_u32(i)?;
        input = i;
    }

    if (flags & FLAG_HAS_PRIVATE_DATA) != 0 {
        let (i, _) = le_u16(input)?;
        let (i, _) = le_u16(i)?;
        input = i;
    }

    if (flags & FLAG_HAS_UNKNOWN_REF) != 0 {
        let (i, _) = le_u32(input)?;
        input = i;
    }

    if (flags & FLAG_HAS_SUBSYSTEM) != 0 {
        let (i, val) = le_u16(input)?;
        subsystem_value = val;
        input = i;
    }

    if (flags & FLAG_HAS_RULES) != 0 {
        let (i, _) = le_u8(input)?;
        input = i;
    }

    if (flags & FLAG_HAS_OVERSIZE) != 0 {
        let (i, val) = le_u32(input)?;
        data_ref_value = val;
        input = i;
    }

    let number_items = extract_number_items_after_formatters(input, flags);

    Ok((&[], (data_ref_value, subsystem_value, number_items)))
}

/// Parse activity sub-type header to extract item count.
fn parse_activity_item_count(data: &[u8], flags: u16) -> nom::IResult<&[u8], u8> {
    let mut input = data;

    // Activity always has current_aid
    let (i, _) = le_u64(input)?;
    let (i, _) = le_u32(i)?;
    input = i;

    if (flags & FLAG_HAS_CURRENT_AID) != 0 {
        let (i, _) = le_u32(input)?;
        let (i, _) = le_u32(i)?;
        input = i;
    }

    let number_items = extract_number_items_after_formatters(input, flags);
    Ok((&[], number_items))
}

/// Parse signpost sub-type header to extract subsystem and item count.
fn parse_signpost_scalars(data: &[u8], flags: u16) -> nom::IResult<&[u8], (u16, u8)> {
    let mut input = data;
    let mut subsystem_value: u16 = 0;

    // Signpost always has these:
    let (i, _) = le_u64(input)?;
    let (i, _) = le_u32(i)?;
    input = i;

    if (flags & FLAG_HAS_CURRENT_AID) != 0 {
        let (i, _) = le_u32(input)?;
        let (i, _) = le_u32(i)?;
        input = i;
    }

    if (flags & FLAG_HAS_PRIVATE_DATA) != 0 {
        let (i, _) = le_u16(input)?;
        let (i, _) = le_u16(i)?;
        input = i;
    }

    if (flags & FLAG_HAS_SUBSYSTEM) != 0 {
        let (i, val) = le_u16(input)?;
        subsystem_value = val;
        input = i;
    }

    if (flags & FLAG_HAS_RULES) != 0 {
        let (i, _) = le_u8(input)?;
        input = i;
    }

    // Signpost name
    if (flags & FLAG_HAS_NAME) != 0 {
        if input.len() >= 8 {
            input = &input[8..];
        }
    } else if input.len() >= 4 {
        input = &input[4..];
    }

    let number_items = extract_number_items_after_formatters(input, flags);
    Ok((&[], (subsystem_value, number_items)))
}

/// Parse trace sub-type header to extract item count.
fn parse_trace_item_count(data: &[u8]) -> nom::IResult<&[u8], u8> {
    // Trace has: unknown_pc_id (u32) then formatters then items
    if data.len() < 4 {
        return Ok((&[], 0));
    }
    let input = &data[4..]; // skip unknown_pc_id

    // For trace, we don't have flags-dependent formatters
    // The item count is found after skipping formatter data
    // Trace uses a simpler structure
    if input.len() >= 2 {
        Ok((&[], input[1])) // unknown_item at [0], number_items at [1]
    } else {
        Ok((&[], 0))
    }
}

/// Skip past FirehoseFormatters fields to extract the number_items byte.
///
/// The formatters section encodes where to find the format string (main exe, shared cache, uuid, etc).
/// After formatters come: unknown_item (u8) + number_items (u8).
fn extract_number_items_after_formatters(data: &[u8], flags: u16) -> u8 {
    let mut offset: usize = 0;

    // FirehoseFormatters flags:
    // main_exe (0x0002): no extra data
    // absolute (0x0004): has unknown_pc_id u32
    // uuid (0x0010): has uuidtext_ref u16
    // has_large_offset (0x8000): u32 instead of u16 for large shared cache
    // shared_cache (0x0020): has shared_cache_ref u16 (or u32 with 0x8000)

    // absolute flag
    if (flags & 0x0004) != 0 {
        offset += 4; // unknown_pc_id
    }

    // uuid flag
    if (flags & 0x0010) != 0 {
        offset += 2; // uuidtext_ref
    }

    // shared_cache flag
    if (flags & 0x0020) != 0 {
        if (flags & 0x8000) != 0 {
            offset += 4; // large shared cache ref
        } else {
            offset += 2; // shared cache ref
        }
    }

    // After formatters: unknown_item (u8) + number_items (u8)
    if data.len() > offset + 1 {
        data[offset + 1]
    } else {
        0
    }
}

/// Skip zero-padding bytes at the start of a slice.
pub(crate) fn skip_zero_padding(data: &[u8]) -> &[u8] {
    match take_while::<_, _, nom::error::Error<&[u8]>>(|b: u8| b == 0)(data) {
        Ok((remaining, _)) => remaining,
        Err(_) => data,
    }
}
