// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use log::{error, info, warn};
use lz4_flex::decompress;
use nom::bytes::complete::{take, take_while};
use nom::number::complete::le_u32;
use uuid::Uuid;

use crate::catalog::CatalogChunk;
use crate::chunks::firehose::firehose_log::FirehosePreamble;
use crate::chunks::oversize::Oversize;
use crate::chunks::simpledump::SimpleDump;
use crate::chunks::statedump::Statedump;
use crate::constants::*;
use crate::dsc::{SharedCacheStrings, SharedCacheStringsOwned};
use crate::error::ParserError;
use crate::header::HeaderChunkStr;
use crate::preamble::LogPreamble;
use crate::timesync::TimesyncBoot;
use crate::traits::FileProvider;
use crate::unified_log::{UnifiedLogCatalogData, UnifiedLogData};
use crate::util::{padding_size_8, parse_uuid_from_str, u64_to_usize};
use crate::uuidtext::UUIDText;
use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;

// Re-export iterate_all_logs from log_data_iterator for convenience
pub use crate::log_data_iterator::{iterate_all_logs, iterate_all_logs_callback};

/// Parse a tracev3 file and return the deconstructed log data
pub fn parse_log(mut reader: impl Read) -> Result<UnifiedLogData, ParserError> {
    let mut buf = Vec::new();
    if let Err(err) = reader.read_to_end(&mut buf) {
        error!("[macos-unifiedlogs] Failed to read the tracev3 file: {err:?}");
        return Err(ParserError::Read);
    }

    info!("Read {} bytes from tracev3 file", buf.len());

    let mut unified_log_data = UnifiedLogData {
        header: Vec::new(),
        catalog_data: Vec::new(),
        oversize: Vec::new(),
    };
    let mut catalog_data = UnifiedLogCatalogData::default();
    let mut input = buf.as_slice();

    // Walk top-level preambles
    while input.len() >= CHUNK_PREAMBLE_SIZE {
        let preamble = match LogPreamble::detect_preamble(input) {
            Ok((_, p)) => p,
            Err(err) => {
                error!("[macos-unifiedlogs] Failed to detect preamble: {err:?}");
                break;
            }
        };

        let chunk_size = match u64_to_usize(preamble.chunk_data_size) {
            Some(c) => c,
            None => {
                error!("[macos-unifiedlogs] Chunk data size exceeds system usize");
                return Err(ParserError::Tracev3Parse);
            }
        };

        let total_chunk_size = chunk_size + CHUNK_PREAMBLE_SIZE;
        if total_chunk_size > input.len() {
            warn!(
                "[macos-unifiedlogs] Chunk extends beyond buffer ({total_chunk_size} > {})",
                input.len()
            );
            break;
        }

        let chunk_data = &input[..total_chunk_size];

        match preamble.chunk_tag {
            HEADER_CHUNK => match HeaderChunkStr::parse_header(chunk_data) {
                Ok((_, header)) => unified_log_data.header.push(header.into_owned()),
                Err(err) => error!("[macos-unifiedlogs] Failed to parse header data: {err:?}"),
            },
            CATALOG_CHUNK => {
                // Push any pending catalog before starting a new one
                if catalog_data.catalog.chunk_tag != 0 {
                    unified_log_data.catalog_data.push(catalog_data);
                }
                catalog_data = UnifiedLogCatalogData::default();

                match CatalogChunk::parse_catalog(chunk_data) {
                    Ok((_, catalog)) => catalog_data.catalog = catalog,
                    Err(err) => {
                        error!("[macos-unifiedlogs] Failed to parse catalog data: {err:?}")
                    }
                }
            }
            CHUNKSET_CHUNK => {
                let decompressed = decompress_chunkset(&chunk_data[CHUNK_PREAMBLE_SIZE..]);
                if let Some(data) = decompressed {
                    parse_chunkset_entries(&data, &mut catalog_data);
                    unified_log_data.oversize.append(&mut catalog_data.oversize);
                }
            }
            other => {
                error!("[macos-unifiedlogs] Unknown chunk type: 0x{other:04x}");
            }
        }

        // Advance past chunk + padding
        let padding = padding_size_8(preamble.chunk_data_size) as usize;
        let advance = total_chunk_size + padding;
        if advance > input.len() {
            break;
        }
        input = &input[advance..];
    }

    // Push the final pending catalog
    if catalog_data.catalog.chunk_tag != 0 {
        unified_log_data.catalog_data.push(catalog_data);
    }

    Ok(unified_log_data)
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

/// Walk inner chunks of a decompressed chunkset and populate catalog data.
fn parse_chunkset_entries(data: &[u8], catalog_data: &mut UnifiedLogCatalogData) {
    let mut input = data;

    while input.len() >= CHUNK_PREAMBLE_SIZE {
        let preamble = match LogPreamble::detect_preamble(input) {
            Ok((_, p)) => p,
            Err(_) => break,
        };

        let chunk_size = match u64_to_usize(preamble.chunk_data_size) {
            Some(c) => c,
            None => break,
        };

        let total = chunk_size + CHUNK_PREAMBLE_SIZE;
        if total > input.len() {
            break;
        }

        let chunk_data = &input[..total];

        match preamble.chunk_tag {
            FIREHOSE_CHUNK => match FirehosePreamble::parse_firehose_preamble(chunk_data) {
                Ok((_, firehose)) => catalog_data.firehose.push(firehose),
                Err(err) => error!(
                    "[macos-unifiedlogs] Failed to parse firehose log entry (chunk): {err:?}"
                ),
            },
            OVERSIZE_CHUNK => match Oversize::parse_oversize(chunk_data) {
                Ok((_, oversize)) => catalog_data.oversize.push(oversize),
                Err(err) => error!(
                    "[macos-unifiedlogs] Failed to parse oversize log entry (chunk): {err:?}"
                ),
            },
            STATEDUMP_CHUNK => match Statedump::parse_statedump(chunk_data) {
                Ok((_, statedump)) => catalog_data.statedump.push(statedump.into_owned()),
                Err(err) => error!(
                    "[macos-unifiedlogs] Failed to parse statedump log entry (chunk): {err:?}"
                ),
            },
            SIMPLEDUMP_CHUNK => match SimpleDump::parse_simpledump(chunk_data) {
                Ok((_, simpledump)) => catalog_data.simpledump.push(simpledump.into_owned()),
                Err(err) => error!(
                    "[macos-unifiedlogs] Failed to parse simpledump log entry (chunk): {err:?}"
                ),
            },
            other => {
                error!("[macos-unifiedlogs] Unknown chunkset type: 0x{other:04x}");
            }
        }

        // Skip past chunk + zero padding
        let remaining = &input[total..];
        let trimmed = match take_while::<_, _, nom::error::Error<&[u8]>>(|b: u8| b == 0)(remaining)
        {
            Ok((rest, _)) => rest,
            Err(_) => remaining,
        };
        if trimmed.is_empty() {
            break;
        }
        input = trimmed;
    }
}

/// Parse all UUID files in provided directory. The directory should follow the same layout as the live system (ex: path/to/files/\<two character UUID\>/\<remaining UUID name\>)
pub fn collect_strings(provider: &dyn FileProvider) -> Result<Vec<UUIDText>, ParserError> {
    let mut uuidtext_vec: Vec<UUIDText> = Vec::new();
    // Start process to read a directory containing subdirectories that contain the uuidtext files
    for mut source in provider.uuidtext_files() {
        let mut buf = Vec::new();
        let path = source.source_path().to_owned();
        if let Err(e) = source.reader().read_to_end(&mut buf) {
            error!("[macos-unifiedlogs] Failed to read uuidfile {path}: {e:?}");
            continue;
        };

        info!("Read {} bytes for file {path}", buf.len());

        let uuid_results = UUIDText::parse_uuidtext(&buf);
        let mut uuidtext_data = match uuid_results {
            Ok((_, results)) => results,
            Err(err) => {
                error!("[macos-unifiedlogs] Failed to parse UUID file {path}: {err:?}");
                continue;
            }
        };

        uuidtext_data.uuid = PathBuf::from(path)
            .file_name()
            .map(|fname| fname.to_str())
            .flatten()
            .map(|f| parse_uuid_from_str(f).ok())
            .flatten()
            .unwrap_or_default();

        uuidtext_vec.push(uuidtext_data)
    }
    Ok(uuidtext_vec)
}

/// Parse all dsc uuid files in provided directory
pub fn collect_shared_strings(
    provider: &dyn FileProvider,
) -> Result<Vec<SharedCacheStringsOwned>, ParserError> {
    let mut shared_strings_vec: Vec<SharedCacheStringsOwned> = Vec::new();
    // Start process to read and parse uuid files related to dsc
    for mut source in provider.dsc_files() {
        let mut buf = Vec::new();
        if let Err(err) = source.reader().read_to_end(&mut buf) {
            error!("[macos-unifiedlogs] Failed to read dsc file: {err:?}");
            continue;
        }

        match SharedCacheStrings::parse_dsc(&buf) {
            Ok((_, mut results)) => {
                let dsc_uuid = PathBuf::from(source.source_path())
                    .file_name()
                    .map(|fname| fname.to_str())
                    .flatten()
                    .map(|f| Uuid::parse_str(f).ok())
                    .flatten()
                    .unwrap_or_default();
                results.dsc_uuid = dsc_uuid;
                shared_strings_vec.push(results.into_owned());
            }
            Err(err) => {
                error!("[macos-unifiedlogs] Failed to parse dsc file: {err:?}");
            }
        };
    }
    Ok(shared_strings_vec)
}

/// Parse all timesync files in provided directory
/// # Example
/// ```rust
///    use macos_unifiedlogs::filesystem::LogarchiveProvider;
///    use macos_unifiedlogs::parser::collect_timesync;
///    use std::path::PathBuf;
///
///    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
///    test_path.push("tests/test_data/system_logs_big_sur.logarchive");
///    let provider = LogarchiveProvider::new(test_path.as_path());
///    let timesync_data = collect_timesync(&provider).unwrap();
/// ```
pub fn collect_timesync(
    provider: &dyn FileProvider,
) -> Result<HashMap<Uuid, TimesyncBoot>, ParserError> {
    let mut timesync_data: HashMap<Uuid, TimesyncBoot> = HashMap::new();
    // Start process to read and parse all timesync files
    for mut source in provider.timesync_files() {
        let mut buffer = Vec::new();
        if let Err(err) = source.reader().read_to_end(&mut buffer) {
            error!("[macos-unifiedlogs] Failed to read timesync file: {err:?}");
            continue;
        }

        let timesync_map = match TimesyncBoot::parse_timesync_data(&buffer) {
            Ok((_, result)) => result,
            Err(err) => {
                error!("[macos-unifiedlogs] Failed to parse timesync file: {err:?}");
                continue;
            }
        };

        /*
         * If a macOS system has been online for a long time. macOS will create a new timesync file with the same boot UUID
         * So we check if we already have an existing UUID and if we do, we just add the data to the existing data we have
         */
        for (key, mut value) in timesync_map {
            if let Some(exiting_boot) = timesync_data.get_mut(&key) {
                exiting_boot.timesync.append(&mut value.timesync);
                continue;
            }
            timesync_data.insert(key, value);
        }
    }
    Ok(timesync_data)
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::filesystem::LogarchiveProvider;
    use crate::log_data_iterator::LogDataIterator;
    use crate::parser::{collect_shared_strings, collect_strings, collect_timesync, parse_log};
    use crate::unified_log::{EventType, LogType};
    use std::path::PathBuf;

    #[test]
    #[cfg(target_os = "macos")]
    fn test_collect_strings_system() {
        use crate::filesystem::LiveSystemProvider;
        let system_provider = LiveSystemProvider::default();
        let uuidtext_results = collect_strings(&system_provider).unwrap();
        assert!(uuidtext_results.len() > 100);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_collect_timesync_system() {
        use crate::filesystem::LiveSystemProvider;
        let system_provider = LiveSystemProvider::default();
        let timesync_results = collect_timesync(&system_provider).unwrap();
        assert!(timesync_results.len() > 1);
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_collect_timesync_archive() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        test_path.push("tests/test_data/system_logs_big_sur.logarchive");

        let provider = LogarchiveProvider::new(test_path.as_path());

        let timesync_data = collect_timesync(&provider).unwrap();
        assert_eq!(timesync_data.len(), 5);
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .signature,
            48048
        );
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .unknown,
            0
        );
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .boot_uuid,
            "9A6A3124274A44B29ABF2BC9E4599B3B"
        );
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .timesync
                .len(),
            5
        );
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .daylight_savings,
            0
        );
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .boot_time,
            1642302206000000000
        );
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .header_size,
            48
        );
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .timebase_denominator,
            1
        );
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .timebase_numerator,
            1
        );
        assert_eq!(
            timesync_data
                .get("9A6A3124274A44B29ABF2BC9E4599B3B")
                .unwrap()
                .timezone_offset_mins,
            0
        );
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_collect_shared_strings_system() {
        use crate::filesystem::LiveSystemProvider;
        let system_provider = LiveSystemProvider::default();
        let shared_strings_results = collect_shared_strings(&system_provider).unwrap();
        assert!(shared_strings_results[0].ranges.len() > 1);
        assert!(shared_strings_results[0].uuids.len() > 1);
        assert!(shared_strings_results[0].number_ranges > 1);
        assert!(shared_strings_results[0].number_uuids > 1);
    }

    #[test]
    fn test_shared_strings_archive() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_big_sur.logarchive");
        let provider = LogarchiveProvider::new(test_path.as_path());
        let shared_strings_results = collect_shared_strings(&provider).unwrap();
        assert_eq!(shared_strings_results.len(), 2);
        assert_eq!(shared_strings_results[0].number_uuids, 1976);
        assert_eq!(shared_strings_results[0].number_ranges, 2993);
        assert_eq!(
            shared_strings_results[0].dsc_uuid,
            Uuid::parse_str("80896B329EB13A10A7C5449B15305DE2").unwrap()
        );
        assert_eq!(shared_strings_results[0].minor_version, 0);
        assert_eq!(shared_strings_results[0].major_version, 1);
        assert_eq!(shared_strings_results[0].ranges.len(), 2993);
        assert_eq!(shared_strings_results[0].uuids.len(), 1976);
    }

    #[test]
    fn test_collect_strings_archive() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_big_sur.logarchive");
        let provider = LogarchiveProvider::new(test_path.as_path());

        let mut strings_results = collect_strings(&provider).unwrap();
        assert_eq!(strings_results.len(), 536);

        strings_results.sort_by(|a, b| a.uuid.cmp(&b.uuid));

        assert_eq!(strings_results[0].signature, 1719109785);
        assert_eq!(
            strings_results[0].uuid,
            Uuid::parse_str("00004EAF1C2B310DA0383BE3D60B80E8").unwrap()
        );
        assert_eq!(strings_results[0].entry_descriptors.len(), 1);
        assert_eq!(strings_results[0].footer_data.len(), 2847);
        assert_eq!(strings_results[0].number_entries, 1);
        assert_eq!(strings_results[0].unknown_minor_version, 1);
        assert_eq!(strings_results[0].unknown_major_version, 2);

        assert_eq!(
            strings_results[1].uuid,
            Uuid::parse_str("0000B3D870FB3AE8BDC1BA3A60D0B9A0").unwrap()
        );
        assert_eq!(strings_results[1].footer_data.len(), 2164);

        assert_eq!(
            strings_results[2].uuid,
            Uuid::parse_str("00014C44534A3A748476ABD88D376918").unwrap()
        );
        assert_eq!(strings_results[2].footer_data.len(), 19011);
    }

    #[test]
    fn test_parse_log() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_big_sur.logarchive");

        test_path.push("Persist/0000000000000002.tracev3");
        let handle = std::fs::File::open(test_path).unwrap();
        let log_data = parse_log(handle).unwrap();

        assert_eq!(log_data.catalog_data[0].firehose.len(), 99);
        assert_eq!(log_data.catalog_data[0].simpledump.len(), 0);
        assert_eq!(log_data.header.len(), 1);
        assert_eq!(
            log_data.catalog_data[0]
                .catalog
                .catalog_process_info_entries
                .len(),
            46
        );
        assert_eq!(log_data.catalog_data[0].statedump.len(), 0);
    }

    #[test]
    fn test_build_log() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_big_sur.logarchive");
        let mut provider = LogarchiveProvider::new(test_path.as_path());
        let timesync_data = collect_timesync(&provider).unwrap();

        let buf = std::fs::read(test_path.join("Persist/0000000000000002.tracev3")).unwrap();

        let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);
        let results: Vec<_> = iter.collect();
        assert_eq!(results.len(), 207366);
        assert_eq!(
            results[10].process.as_str(),
            "/usr/libexec/lightsoutmanagementd"
        );
        assert_eq!(results[10].subsystem.as_str(), "com.apple.lom");
        assert_eq!(results[10].time, 1642302327364384800.0);
        assert_eq!(results[10].activity_id, 0);
        assert_eq!(
            results[10].library.as_str(),
            "/System/Library/PrivateFrameworks/AppleLOM.framework/Versions/A/AppleLOM"
        );
        assert_eq!(
            results[10].message.as_str(),
            "<private> LOM isSupported : No"
        );
        assert_eq!(results[10].pid, 45);
        assert_eq!(results[10].thread_id, 588);
        assert_eq!(results[10].category.as_str(), "device");
        assert_eq!(results[10].log_type, LogType::Default);
        assert_eq!(results[10].event_type, EventType::Log);
        assert_eq!(results[10].euid, 0);
        assert_eq!(
            results[10].boot_uuid,
            Uuid::parse_str("80D194AF56A34C54867449D2130D41BB").unwrap()
        );
        assert_eq!(results[10].timezone_name.as_str(), "Pacific");
        assert_eq!(
            results[10].library_uuid,
            Uuid::parse_str("D8E5AF1CAF4F3CEB8731E6F240E8EA7D").unwrap()
        );
        assert_eq!(
            results[10].process_uuid,
            Uuid::parse_str("6C3ADF991F033C1C96C4ADFAA12D8CED").unwrap()
        );
        assert_eq!(results[10].raw_message.as_str(), "%@ LOM isSupported : %s");
    }
}
