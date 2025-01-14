// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use log::{error, warn};
use nom::combinator::map;
use nom::multi::many_m_n;
use nom::number::complete::{be_u16, be_u32, be_u64, be_u8, le_u32, le_u8};

use crate::catalog::CatalogChunk;
use crate::chunks::firehose::firehose_log::{FirehoseItemData, FirehoseItemInfo};
use crate::chunks::firehose::message::MessageData;
use crate::uuidtext::UUIDText;

#[derive(Debug, Clone, Default)]
pub struct FirehoseTrace {
    /// Appears to be used to calculate string offset for firehose events with Absolute flag
    pub unknown_pc_id: u32,
    pub message_data: FirehoseItemData,
}

impl FirehoseTrace {
    /// Parse Trace Firehose log entry.
    //  Ex: tp 504 + 34: trace default (main_exe)
    pub fn parse_firehose_trace(input: &[u8]) -> nom::IResult<&[u8], FirehoseTrace> {
        let mut firehose_trace = FirehoseTrace::default();

        let (input, firehose_unknown_pc_id) = le_u32(input)?;

        firehose_trace.unknown_pc_id = firehose_unknown_pc_id;

        // Trace logs only have message values if more than 4 bytes remaining in log entry
        const MINIMUM_MESSAGE_SIZE: usize = 4;
        if input.len() < MINIMUM_MESSAGE_SIZE {
            return Ok((&[], firehose_trace));
        }

        let message_data = FirehoseTrace::get_message(&input);
        firehose_trace.message_data = message_data;

        Ok((&[], firehose_trace))
    }

    /// Get the Trace message
    fn get_message(data: &[u8]) -> FirehoseItemData {
        let mut data = data.to_vec();
        // The rest of the trace log entry appears to be related to log message values
        // But the data is stored differently from other log entries
        // The data appears to be stored backwards? Ex: Data value, Data size, number of data entries, instead normal: number of data entries, data size, data value
        data.reverse();

        let (_, item_data) = FirehoseTrace::parse_trace_message(&data).unwrap_or_else(|err| {
            error!("[macos-unifiedlogs] Could not get Trace message data: {err:?}");
            Default::default()
        });

        FirehoseItemData {
            item_info: item_data,
            ..Default::default()
        }
    }

    /// Parse the data associated with the trace message
    fn parse_trace_message(input: &[u8]) -> nom::IResult<&[u8], Vec<FirehoseItemInfo>> {
        const MINIMUM_MESSAGE_SIZE: usize = 4;
        if input.len() < MINIMUM_MESSAGE_SIZE {
            return Ok((input, Default::default()));
        }

        let (input, entries_count) = map(le_u8, |e| e as usize)(input)?;
        let (mut input, sizes) =
            many_m_n(entries_count as usize, entries_count as usize, le_u8)(input)?;

        let mut infos: Vec<FirehoseItemInfo> = Vec::with_capacity(entries_count);

        for entry_size in sizes {
            let mut item_info = FirehoseItemInfo::default();

            // So far all entries appears to be numbers. Using Big Endian because we reversed the data above
            let (entry_input, value) = match entry_size {
                1 => map(be_u8, |x| x as u64)(input)?,
                2 => map(be_u16, |x| x as _)(input)?,
                4 => map(be_u32, |x| x as _)(input)?,
                8 => map(be_u64, |x| x as _)(input)?,
                _ => {
                    warn!("[macos-unifiedlogs] Unhandled size of trace data: {entry_size}. Defaulting to size of one");
                    map(le_u8, |x| x as _)(input)?
                }
            };
            item_info.message_strings = value.to_string();
            infos.push(item_info);
            input = entry_input;
        }

        // Reverse the data back to expected format
        infos.reverse();

        Ok((input, infos))
    }

    /// Get base log message string formatter from shared cache strings (dsc) or UUID text file for firehose trace log entries (chunks)
    pub fn get_firehose_trace_strings<'a>(
        strings_data: &'a [UUIDText],
        string_offset: u64,
        first_proc_id: u64,
        second_proc_id: u32,
        catalogs: &CatalogChunk,
    ) -> nom::IResult<&'a [u8], MessageData> {
        // Only main_exe flag has been seen for format strings
        MessageData::extract_format_strings(
            strings_data,
            string_offset,
            first_proc_id,
            second_proc_id,
            catalogs,
            0,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        chunks::firehose::trace::FirehoseTrace,
        filesystem::LogarchiveProvider,
        parser::{collect_strings, parse_log},
    };

    #[test]
    fn test_parse_firehose_trace() {
        let test_data = [106, 139, 3, 0, 0];
        let (_, results) = FirehoseTrace::parse_firehose_trace(&test_data).unwrap();
        assert_eq!(results.unknown_pc_id, 232298);

        let test_data = [248, 145, 3, 0, 200, 0, 0, 0, 0, 0, 0, 0, 8, 1];
        let (_, results) = FirehoseTrace::parse_firehose_trace(&test_data).unwrap();
        assert_eq!(results.unknown_pc_id, 233976);
        assert_eq!(results.message_data.item_info.len(), 1);
    }

    #[test]
    fn test_parse_trace_message() {
        let mut test_message = vec![200, 0, 0, 0, 0, 0, 0, 0, 8, 1];
        test_message.reverse();
        let (_, results) = FirehoseTrace::parse_trace_message(&test_message).unwrap();
        assert_eq!(results[0].message_strings, "200");
    }

    #[test]
    fn test_parse_trace_message_multiple() {
        let test_message = &[
            2, 8, 8, 0, 0, 0, 0, 0, 0, 0, 200, 0, 0, 127, 251, 75, 225, 96, 176,
        ];
        let (_, results) = FirehoseTrace::parse_trace_message(test_message).unwrap();

        assert_eq!(results[0].message_strings, "140717286580400");
        assert_eq!(results[1].message_strings, "200");
    }

    #[test]
    fn test_get_message() {
        let test_message = &[200, 0, 0, 0, 0, 0, 0, 0, 8, 1];
        let results = FirehoseTrace::get_message(test_message);
        assert_eq!(results.item_info[0].message_strings, "200");
    }

    #[test]
    fn test_get_firehose_trace_strings() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_high_sierra.logarchive");
        let provider = LogarchiveProvider::new(test_path.as_path());

        let string_results = collect_strings(&provider).unwrap();

        test_path.push("logdata.LiveData.tracev3");
        let handle = std::fs::File::open(test_path).unwrap();

        let log_data = parse_log(handle).unwrap();

        let activity_type = 0x3;

        for catalog_data in log_data.catalog_data {
            for preamble in catalog_data.firehose {
                for firehose in preamble.public_data {
                    if firehose.unknown_log_activity_type == activity_type {
                        let (_, message_data) = FirehoseTrace::get_firehose_trace_strings(
                            &string_results,
                            u64::from(firehose.format_string_location),
                            preamble.first_number_proc_id,
                            preamble.second_number_proc_id,
                            &catalog_data.catalog,
                        )
                        .unwrap();
                        assert_eq!(message_data.format_string, "starting metadata download");
                        assert_eq!(message_data.library, "/usr/libexec/mobileassetd");
                        assert_eq!(message_data.process, "/usr/libexec/mobileassetd");
                        assert_eq!(
                            message_data.process_uuid,
                            "CC6C867B44D63D0ABAA7598659629484"
                        );
                        assert_eq!(
                            message_data.library_uuid,
                            "CC6C867B44D63D0ABAA7598659629484"
                        );
                        return;
                    }
                }
            }
        }
    }
}
