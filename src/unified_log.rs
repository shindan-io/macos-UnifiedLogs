// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

//! Parse macOS Unified Log data
//!
//! Provides a simple library to parse the macOS Unified Log format.

use crate::RcString;
use crate::catalog::CatalogChunk;
use crate::chunks::firehose::firehose_log::{FirehoseItemInfo, FirehosePreamble};
use crate::chunks::oversize::Oversize;
use crate::chunks::simpledump::SimpleDumpOwned;
use crate::chunks::statedump::StatedumpOwned;
use crate::chunkset::ChunksetChunk;
use crate::constants::*;
use crate::header::{HeaderChunk, HeaderChunkOwned};
use crate::preamble::LogPreamble;
use crate::util::{padding_size_8, u64_to_usize};
use chrono::{DateTime, Utc};
use log::{error, warn};
use nom::bytes::complete::take;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LogType {
    Debug,
    Info,
    Default,
    Error,
    Fault,
    Create,
    Useraction,
    ProcessSignpostEvent,
    ProcessSignpostStart,
    ProcessSignpostEnd,
    SystemSignpostEvent,
    SystemSignpostStart,
    SystemSignpostEnd,
    ThreadSignpostEvent,
    ThreadSignpostStart,
    ThreadSignpostEnd,
    Simpledump,
    Statedump,
    Loss,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum EventType {
    Unknown,
    Log,
    Activity,
    Trace,
    Signpost,
    Simpledump,
    Statedump,
    Loss,
}

#[derive(Debug, Clone, Default)]
pub struct UnifiedLogData {
    pub header: Vec<HeaderChunkOwned>,
    pub catalog_data: Vec<UnifiedLogCatalogData>,
    /// Keep a global cache of oversize string
    pub oversize: Vec<Oversize>,
}

#[derive(Debug, Clone, Default)]
pub struct UnifiedLogCatalogData {
    pub catalog: CatalogChunk,
    pub firehose: Vec<FirehosePreamble>,
    pub simpledump: Vec<SimpleDumpOwned>,
    pub statedump: Vec<StatedumpOwned>,
    pub oversize: Vec<Oversize>,
}

#[derive(Debug, Serialize)]
pub struct LogData {
    pub subsystem: RcString,
    pub thread_id: u64,
    pub pid: u64,
    pub euid: u32,
    pub library: RcString,
    pub library_uuid: Uuid,
    pub activity_id: u64,
    pub time: f64,
    pub category: RcString,
    pub event_type: EventType,
    pub log_type: LogType,
    pub process: RcString,
    pub process_uuid: Uuid,
    pub message: RcString,
    pub raw_message: RcString,
    pub boot_uuid: Uuid,
    pub timezone_name: RcString,
    pub message_entries: Vec<FirehoseItemInfo>,
    pub timestamp: DateTime<Utc>,
}

impl LogData {
    /// Parse the Unified log data read from a tracev3 file
    pub fn parse_unified_log(data: &[u8]) -> nom::IResult<&[u8], UnifiedLogData> {
        let mut unified_log_data_true = UnifiedLogData {
            header: Vec::new(),
            catalog_data: Vec::new(),
            oversize: Vec::new(),
        };

        let mut catalog_data = UnifiedLogCatalogData::default();

        let mut input = data;
        // Loop through traceV3 file until all file contents are read
        while !input.is_empty() {
            let (_, preamble) = LogPreamble::detect_preamble(input)?;
            let chunk_size = preamble.chunk_data_size;

            // Grab all data associated with Unified Log entry (chunk)
            let chunk_size = match u64_to_usize(chunk_size) {
                Some(c) => c,
                None => {
                    error!("[macos-unifiedlogs] u64 is bigger than system usize");
                    return Err(nom::Err::Error(nom::error::Error::new(
                        data,
                        nom::error::ErrorKind::TooLarge,
                    )));
                }
            };
            let (data, chunk_data) = take(chunk_size + CHUNK_PREAMBLE_SIZE)(input)?;

            if preamble.chunk_tag == HEADER_CHUNK {
                LogData::get_header_data(chunk_data, &mut unified_log_data_true);
            } else if preamble.chunk_tag == CATALOG_CHUNK {
                if catalog_data.catalog.chunk_tag != 0 {
                    unified_log_data_true.catalog_data.push(catalog_data);
                }
                catalog_data = UnifiedLogCatalogData::default();

                LogData::get_catalog_data(chunk_data, &mut catalog_data);
            } else if preamble.chunk_tag == CHUNKSET_CHUNK {
                LogData::get_chunkset_data(
                    chunk_data,
                    &mut catalog_data,
                    &mut unified_log_data_true,
                );
            } else {
                error!(
                    "[macos-unifiedlogs] Unknown chunk type: {:?}",
                    preamble.chunk_tag
                );
            }

            let padding_size = padding_size_8(preamble.chunk_data_size);
            if data.len() < padding_size as usize {
                break;
            }
            let padding_size = match u64_to_usize(padding_size) {
                Some(p) => p,
                None => {
                    error!("[macos-unifiedlogs] u64 is bigger than system usize");
                    return Err(nom::Err::Error(nom::error::Error::new(
                        data,
                        nom::error::ErrorKind::TooLarge,
                    )));
                }
            };

            let (data, _) = take(padding_size)(data)?;
            if data.is_empty() {
                break;
            }
            input = data;
            if input.len() < CHUNK_PREAMBLE_SIZE {
                warn!(
                    "Not enough data for preamble header, needed 16 bytes. Got: {:?}",
                    input.len()
                );
                break;
            }
        }
        // Make sure to get the last catalog
        if catalog_data.catalog.chunk_tag != 0 {
            unified_log_data_true.catalog_data.push(catalog_data);
        }
        Ok((input, unified_log_data_true))
    }

    /// Return log type based on parsed log data
    fn get_log_type(log_type: u8, activity_type: u8) -> LogType {
        match log_type {
            LOG_TYPE_INFO => {
                if activity_type == ACTIVITY_TYPE {
                    LogType::Create
                } else {
                    LogType::Info
                }
            }
            LOG_TYPE_DEBUG => LogType::Debug,
            LOG_TYPE_USERACTION => LogType::Useraction,
            LOG_TYPE_ERROR => LogType::Error,
            LOG_TYPE_FAULT => LogType::Fault,
            LOG_TYPE_PROCESS_SIGNPOST_EVENT => LogType::ProcessSignpostEvent,
            LOG_TYPE_PROCESS_SIGNPOST_START => LogType::ProcessSignpostStart,
            LOG_TYPE_PROCESS_SIGNPOST_END => LogType::ProcessSignpostEnd,
            LOG_TYPE_SYSTEM_SIGNPOST_EVENT => LogType::SystemSignpostEvent, // Not seen but may exist?
            LOG_TYPE_SYSTEM_SIGNPOST_START => LogType::SystemSignpostStart,
            LOG_TYPE_SYSTEM_SIGNPOST_END => LogType::SystemSignpostEnd,
            LOG_TYPE_THREAD_SIGNPOST_EVENT => LogType::ThreadSignpostEvent, // Not seen but may exist?
            LOG_TYPE_THREAD_SIGNPOST_START => LogType::ThreadSignpostStart,
            LOG_TYPE_THREAD_SIGNPOST_END => LogType::ThreadSignpostEnd,
            _ => LogType::Default,
        }
    }

    /// Return the log event type based on parsed log data
    fn get_event_type(event_type: u8) -> EventType {
        match event_type {
            NON_ACTIVITY_TYPE => EventType::Log,
            ACTIVITY_TYPE => EventType::Activity,
            TRACE_TYPE => EventType::Trace,
            SIGNPOST_TYPE => EventType::Signpost,
            LOSS_TYPE => EventType::Loss,
            _ => EventType::Unknown,
        }
    }

    /// Get the header of the Unified Log data (tracev3 file)
    pub(crate) fn get_header_data(data: &[u8], unified_log_data: &mut UnifiedLogData) {
        let header_results = HeaderChunk::parse_header(data);
        match header_results {
            Ok((_, header_data)) => unified_log_data.header.push(header_data.into_owned()),
            Err(err) => error!("[macos-unifiedlogs] Failed to parse header data: {err:?}"),
        }
    }

    /// Get the Catalog of the Unified Log data (tracev3 file)
    pub(crate) fn get_catalog_data(data: &[u8], unified_log_data: &mut UnifiedLogCatalogData) {
        let catalog_results = CatalogChunk::parse_catalog(data);
        match catalog_results {
            Ok((_, catalog_data)) => unified_log_data.catalog = catalog_data,
            Err(err) => error!("[macos-unifiedlogs] Failed to parse catalog data: {err:?}"),
        }
    }

    /// Get the Chunkset of the Unified Log data (tracev3)
    pub(crate) fn get_chunkset_data(
        data: &[u8],
        catalog_data: &mut UnifiedLogCatalogData,
        unified_log_data: &mut UnifiedLogData,
    ) {
        // Parse and decompress the chunkset entries
        let chunkset_data_results = ChunksetChunk::parse_chunkset(data);
        match chunkset_data_results {
            Ok((_, chunkset_data)) => {
                // Parse the decompressed data which contains the log data
                let _result = ChunksetChunk::parse_chunkset_data(
                    &chunkset_data.decompressed_data,
                    catalog_data,
                );
                unified_log_data.oversize.append(&mut catalog_data.oversize);
            }
            Err(err) => error!("[macos-unifiedlogs] Failed to parse chunkset data: {err:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use uuid::Uuid;

    use super::{LogData, UnifiedLogData};
    use crate::{
        catalog::CatalogProcessInfoKey,
        filesystem::LogarchiveProvider,
        log_data_iterator::LogDataIterator,
        parser::collect_timesync,
        unified_log::{EventType, LogType, UnifiedLogCatalogData},
    };
    use std::{fs, path::PathBuf};

    #[test]
    fn test_parse_unified_log() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push(
            "tests/test_data/system_logs_big_sur.logarchive/Persist/0000000000000002.tracev3",
        );

        let buffer = fs::read(test_path).unwrap();

        let (_, results) = LogData::parse_unified_log(&buffer).unwrap();
        assert_eq!(results.catalog_data.len(), 56);
        assert_eq!(results.header.len(), 1);
        assert_eq!(results.oversize.len(), 12);
    }

    #[test]
    fn test_bad_log_header() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/Bad Data/TraceV3/Bad_header_0000000000000005.tracev3");

        let buffer = fs::read(test_path).unwrap();
        let (_, results) = LogData::parse_unified_log(&buffer).unwrap();
        assert_eq!(results.catalog_data.len(), 36);
        assert_eq!(results.header.len(), 0);
        assert_eq!(results.oversize.len(), 28);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    #[should_panic(expected = "Eof")]
    fn test_bad_log_content_64() {
        test_bad_log_content();
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    #[should_panic(expected = "TooLarge")]
    fn test_bad_log_content_32() {
        test_bad_log_content();
    }

    fn test_bad_log_content() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/Bad Data/TraceV3/Bad_content_0000000000000005.tracev3");

        let buffer = fs::read(test_path).unwrap();
        let (_, _) = LogData::parse_unified_log(&buffer).unwrap();
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    #[should_panic(expected = "Eof")]
    fn test_bad_log_file_64() {
        test_bad_log_file();
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    #[should_panic(expected = "TooLarge")]
    fn test_bad_log_file_32() {
        test_bad_log_file();
    }

    fn test_bad_log_file() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/Bad Data/TraceV3/00.tracev3");

        let buffer = fs::read(test_path).unwrap();
        let (_, _) = LogData::parse_unified_log(&buffer).unwrap();
    }

    #[test]
    fn test_build_log() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_big_sur.logarchive");

        let mut provider = LogarchiveProvider::new(test_path.as_path());
        let timesync_data = collect_timesync(&provider).unwrap();

        let buf = fs::read(test_path.join("Persist/0000000000000002.tracev3")).unwrap();

        let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);
        let results: Vec<_> = iter.collect();

        assert_eq!(results.len(), 207366);
        assert_eq!(
            results[0].process.as_str(),
            "/usr/libexec/lightsoutmanagementd"
        );
        assert_eq!(results[0].subsystem.as_str(), "com.apple.lom");
        assert_eq!(results[0].time, 1_642_302_326_434_850_800.0);
        assert_eq!(results[0].activity_id, 0);
        assert_eq!(
            results[0].library.as_str(),
            "/usr/libexec/lightsoutmanagementd"
        );
        assert_eq!(
            results[0].library_uuid,
            Uuid::parse_str("6C3ADF991F033C1C96C4ADFAA12D8CED").unwrap()
        );
        assert_eq!(
            results[0].process_uuid,
            Uuid::parse_str("6C3ADF991F033C1C96C4ADFAA12D8CED").unwrap()
        );
        assert_eq!(results[0].message.as_str(), "LOMD Start");
        assert_eq!(results[0].pid, 45);
        assert_eq!(results[0].thread_id, 588);
        assert_eq!(results[0].category.as_str(), "device");
        assert_eq!(results[0].log_type, LogType::Default);
        assert_eq!(results[0].event_type, EventType::Log);
        assert_eq!(results[0].euid, 0);
        assert_eq!(
            results[0].boot_uuid,
            Uuid::parse_str("80D194AF56A34C54867449D2130D41BB").unwrap()
        );
        assert_eq!(results[0].timezone_name.as_str(), "Pacific");
        assert_eq!(results[0].raw_message.as_str(), "LOMD Start");
        assert_eq!(
            results[0].timestamp,
            DateTime::parse_from_rfc3339("2022-01-16T03:05:26.434850816Z").unwrap()
        );
    }

    #[test]
    fn test_get_log_type() {
        let mut log_type = 0x2;
        let activity_type = 0x2;

        let mut log_string = LogData::get_log_type(log_type, activity_type);
        assert_eq!(log_string, LogType::Debug);
        log_type = 0x1;
        log_string = LogData::get_log_type(log_type, activity_type);
        assert_eq!(log_string, LogType::Create);
    }

    #[test]
    fn test_get_event_type() {
        let event_type = 0x2;
        let event_string = LogData::get_event_type(event_type);
        assert_eq!(event_string, EventType::Activity);
    }

    #[test]
    fn test_get_header_data() {
        let test_chunk_header = [
            0, 16, 0, 0, 17, 0, 0, 0, 208, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 15, 105,
            217, 162, 204, 126, 0, 0, 48, 215, 18, 98, 0, 0, 0, 0, 203, 138, 9, 0, 44, 1, 0, 0, 0,
            0, 0, 0, 1, 0, 0, 0, 0, 97, 0, 0, 8, 0, 0, 0, 6, 112, 124, 198, 169, 153, 1, 0, 1, 97,
            0, 0, 56, 0, 0, 0, 7, 0, 0, 0, 8, 0, 0, 0, 50, 49, 65, 53, 53, 57, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 77, 97, 99, 66, 111, 111, 107, 80, 114, 111, 49, 54, 44, 49, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 97, 0, 0, 24, 0, 0, 0, 195, 32, 184, 206, 151,
            250, 77, 165, 159, 49, 125, 57, 46, 56, 156, 234, 85, 0, 0, 0, 0, 0, 0, 0, 3, 97, 0, 0,
            48, 0, 0, 0, 47, 118, 97, 114, 47, 100, 98, 47, 116, 105, 109, 101, 122, 111, 110, 101,
            47, 122, 111, 110, 101, 105, 110, 102, 111, 47, 65, 109, 101, 114, 105, 99, 97, 47, 78,
            101, 119, 95, 89, 111, 114, 107, 0, 0, 0, 0, 0, 0,
        ];
        let mut data = UnifiedLogData {
            header: Vec::new(),
            catalog_data: Vec::new(),
            oversize: Vec::new(),
        };

        LogData::get_header_data(&test_chunk_header, &mut data);
        assert_eq!(data.header.len(), 1);
    }

    #[test]
    fn test_get_catalog_data() {
        let test_chunk_catalog = [
            11, 96, 0, 0, 17, 0, 0, 0, 208, 1, 0, 0, 0, 0, 0, 0, 32, 0, 96, 0, 1, 0, 160, 0, 7, 0,
            0, 0, 0, 0, 0, 0, 20, 165, 44, 35, 253, 233, 2, 0, 43, 239, 210, 12, 24, 236, 56, 56,
            129, 79, 43, 78, 90, 243, 188, 236, 61, 5, 132, 95, 63, 101, 53, 143, 158, 191, 34, 54,
            231, 114, 172, 1, 99, 111, 109, 46, 97, 112, 112, 108, 101, 46, 83, 107, 121, 76, 105,
            103, 104, 116, 0, 112, 101, 114, 102, 111, 114, 109, 97, 110, 99, 101, 95, 105, 110,
            115, 116, 114, 117, 109, 101, 110, 116, 97, 116, 105, 111, 110, 0, 116, 114, 97, 99,
            105, 110, 103, 46, 115, 116, 97, 108, 108, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 158,
            0, 0, 0, 0, 0, 0, 0, 55, 1, 0, 0, 158, 0, 0, 0, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 87, 0, 0, 0, 19, 0, 78, 0, 0, 0, 47, 0, 0, 0, 0, 0,
            246, 113, 118, 43, 250, 233, 2, 0, 62, 195, 90, 26, 9, 234, 2, 0, 120, 255, 0, 0, 0, 1,
            0, 0, 1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 19, 0, 47, 0, 48, 89, 60, 28, 9, 234, 2, 0,
            99, 50, 207, 40, 18, 234, 2, 0, 112, 240, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 3, 0, 0,
            0, 0, 0, 19, 0, 47, 0, 153, 6, 208, 41, 18, 234, 2, 0, 0, 214, 108, 78, 32, 234, 2, 0,
            0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 19, 0, 47, 0, 128, 0, 87,
            79, 32, 234, 2, 0, 137, 5, 2, 205, 41, 234, 2, 0, 88, 255, 0, 0, 0, 1, 0, 0, 1, 0, 0,
            0, 0, 0, 3, 0, 0, 0, 0, 0, 19, 0, 47, 0, 185, 11, 2, 205, 41, 234, 2, 0, 172, 57, 107,
            20, 56, 234, 2, 0, 152, 255, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 19,
            0, 47, 0, 53, 172, 105, 21, 56, 234, 2, 0, 170, 167, 194, 43, 68, 234, 2, 0, 144, 255,
            0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 19, 0, 47, 0, 220, 202, 171, 57,
            68, 234, 2, 0, 119, 171, 170, 119, 76, 234, 2, 0, 240, 254, 0, 0, 0, 1, 0, 0, 1, 0, 0,
            0, 0, 0, 3, 0, 0, 0, 0, 0, 19, 0, 47, 0,
        ];
        let mut data = UnifiedLogCatalogData::default();

        LogData::get_catalog_data(&test_chunk_catalog, &mut data);
        assert_eq!(data.catalog.chunk_tag, 0x600b);
        assert_eq!(data.catalog.chunk_sub_tag, 17);
        assert_eq!(data.catalog.chunk_data_size, 464);
        assert_eq!(data.catalog.catalog_subsystem_strings_offset, 32);
        assert_eq!(data.catalog.catalog_process_info_entries_offset, 96);
        assert_eq!(data.catalog.number_process_information_entries, 1);
        assert_eq!(data.catalog.catalog_offset_sub_chunks, 160);
        assert_eq!(data.catalog.number_sub_chunks, 7);
        assert_eq!(data.catalog.unknown, [0, 0, 0, 0, 0, 0]);
        assert_eq!(data.catalog.earliest_firehose_timestamp, 820223379547412);
        assert_eq!(
            data.catalog.catalog_uuids,
            [
                Uuid::parse_str("2BEFD20C18EC3838814F2B4E5AF3BCEC").unwrap(),
                Uuid::parse_str("3D05845F3F65358F9EBF2236E772AC01").unwrap()
            ]
        );
        assert_eq!(
            data.catalog.catalog_subsystem_strings,
            [
                99, 111, 109, 46, 97, 112, 112, 108, 101, 46, 83, 107, 121, 76, 105, 103, 104, 116,
                0, 112, 101, 114, 102, 111, 114, 109, 97, 110, 99, 101, 95, 105, 110, 115, 116,
                114, 117, 109, 101, 110, 116, 97, 116, 105, 111, 110, 0, 116, 114, 97, 99, 105,
                110, 103, 46, 115, 116, 97, 108, 108, 115, 0, 0, 0
            ]
        );
        assert_eq!(data.catalog.catalog_process_info_entries.len(), 1);
        assert_eq!(
            data.catalog
                .catalog_process_info_entries
                .get(&CatalogProcessInfoKey(158, 311))
                .unwrap()
                .main_uuid,
            Uuid::parse_str("2BEFD20C18EC3838814F2B4E5AF3BCEC").unwrap()
        );
        assert_eq!(
            data.catalog
                .catalog_process_info_entries
                .get(&CatalogProcessInfoKey(158, 311))
                .unwrap()
                .dsc_uuid,
            Some(Uuid::parse_str("3D05845F3F65358F9EBF2236E772AC01").unwrap())
        );

        assert_eq!(data.catalog.catalog_subchunks.len(), 7)
    }

    #[test]
    fn test_get_chunkset_data() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/Chunkset Tests/high_sierra_compressed_chunkset.raw");

        let buffer = fs::read(test_path).unwrap();

        let mut unified_log = UnifiedLogCatalogData::default();

        let mut log_data = UnifiedLogData::default();

        LogData::get_chunkset_data(&buffer, &mut unified_log, &mut log_data);
        assert_eq!(unified_log.catalog.chunk_tag, 0);
        assert_eq!(unified_log.firehose.len(), 21);
        assert_eq!(unified_log.statedump.len(), 0);
        assert_eq!(unified_log.simpledump.len(), 0);
        assert_eq!(unified_log.oversize.len(), 0);

        assert_eq!(
            unified_log.firehose[0].public_data[0].message.item_info[0]
                .message_strings
                .as_cow(),
            "483.700"
        );
        assert_eq!(unified_log.firehose[0].base_continous_time, 0);
        assert_eq!(unified_log.firehose[0].first_number_proc_id, 70);
        assert_eq!(unified_log.firehose[0].second_number_proc_id, 71);
        assert_eq!(unified_log.firehose[0].public_data_size, 4040);
        assert_eq!(unified_log.firehose[0].private_data_virtual_offset, 4096);
    }
}
