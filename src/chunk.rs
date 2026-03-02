// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use nom::bytes::complete::take_while;
use nom::number::complete::{le_u8, le_u16, le_u32, le_u64};

use crate::catalog::CatalogChunk;
use crate::constants::*;
use crate::preamble::LogPreamble;
use crate::util::{padding_size_8, u64_to_usize};

// ── Shared chunk cursor helpers ─────────────────────────────────────────

/// A reference to a raw chunk within a buffer (preamble parsed, body not yet parsed).
pub(crate) struct RawChunkRef {
    pub tag: u32,
    /// Absolute byte offset of the chunk start (including the 16-byte preamble).
    pub start: usize,
    /// Absolute byte offset of the chunk end (start + preamble + data_size).
    pub end: usize,
}

/// Advance past one top-level chunk in a tracev3 file.
///
/// Reads the preamble at `pos`, validates bounds, and returns the chunk reference
/// together with the position after the chunk's 8-byte-aligned padding.
pub(crate) fn next_top_level_chunk(data: &[u8], pos: usize) -> Option<(RawChunkRef, usize)> {
    let input = data.get(pos..)?;
    if input.len() < CHUNK_PREAMBLE_SIZE {
        return None;
    }

    let preamble = LogPreamble::detect_preamble(input).ok()?.1;
    let chunk_size = u64_to_usize(preamble.chunk_data_size)?;
    let total = chunk_size + CHUNK_PREAMBLE_SIZE;

    if total > input.len() {
        return None;
    }

    let padding = padding_size_8(preamble.chunk_data_size) as usize;
    let next_pos = (pos + total + padding).min(data.len());

    Some((
        RawChunkRef {
            tag: preamble.chunk_tag,
            start: pos,
            end: pos + total,
        },
        next_pos,
    ))
}

/// Advance past one inner chunk within decompressed chunkset data.
///
/// Same as [`next_top_level_chunk`] but skips trailing zero-byte padding
/// instead of using 8-byte alignment.
pub(crate) fn next_inner_chunk(data: &[u8], pos: usize) -> Option<(RawChunkRef, usize)> {
    let input = data.get(pos..)?;
    if input.len() < CHUNK_PREAMBLE_SIZE {
        return None;
    }

    let preamble = LogPreamble::detect_preamble(input).ok()?.1;
    let chunk_size = u64_to_usize(preamble.chunk_data_size)?;
    let total = chunk_size + CHUNK_PREAMBLE_SIZE;

    if total > input.len() {
        return None;
    }

    let remaining = &input[total..];
    let trimmed = skip_zero_padding(remaining);
    let zeros = remaining.len() - trimmed.len();
    let next_pos = (pos + total + zeros).min(data.len());

    Some((
        RawChunkRef {
            tag: preamble.chunk_tag,
            start: pos,
            end: pos + total,
        },
        next_pos,
    ))
}

// ── Catalog helpers ─────────────────────────────────────────────────────

/// Walk top-level chunks and return the first catalog found.
pub(crate) fn parse_first_catalog(data: &[u8]) -> Option<CatalogChunk> {
    let mut pos = 0;
    while let Some((chunk, next_pos)) = next_top_level_chunk(data, pos) {
        pos = next_pos;
        if chunk.tag == CATALOG_CHUNK {
            if let Ok((_, catalog)) = CatalogChunk::parse_catalog(&data[chunk.start..chunk.end]) {
                return Some(catalog);
            }
        }
    }
    None
}

/// Walk top-level chunks and collect all catalogs in order.
pub(crate) fn parse_catalogs(data: &[u8]) -> Vec<CatalogChunk> {
    let mut catalogs = Vec::new();
    let mut pos = 0;
    while let Some((chunk, next_pos)) = next_top_level_chunk(data, pos) {
        pos = next_pos;
        if chunk.tag == CATALOG_CHUNK {
            if let Ok((_, catalog)) = CatalogChunk::parse_catalog(&data[chunk.start..chunk.end]) {
                catalogs.push(catalog);
            }
        }
    }
    catalogs
}

// ── Firehose subtype scalar extraction ──────────────────────────────────

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
