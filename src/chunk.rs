// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use std::fmt;

use log::{error, warn};
use lz4_flex::decompress;
use nom::bytes::complete::take;
use nom::number::complete::le_u32;

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
fn decompress_chunkset(data: &[u8]) -> Option<Vec<u8>> {
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
