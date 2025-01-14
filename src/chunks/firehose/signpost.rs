// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use crate::catalog::CatalogChunk;
use crate::chunks::firehose::flags::FirehoseFormatters;
use crate::chunks::firehose::message::MessageData;
use crate::dsc::SharedCacheStrings;
use crate::uuidtext::UUIDText;
use log::{debug, error};
use nom::bytes::complete::take;
use nom::number::complete::{le_u16, le_u32, le_u64, le_u8};
use nom::Needed;
use std::mem::size_of;

#[derive(Debug, Clone, Default)]
pub struct FirehoseSignpost {
    pub unknown_pc_id: u32, // Appears to be used to calculate string offset for firehose events with Absolute flag
    pub unknown_activity_id: u32,
    pub unknown_sentinel: u32,
    pub subsystem: u16,
    pub signpost_id: u64,
    pub signpost_name: u32,
    pub private_strings_offset: u16, // if flag 0x0100
    pub private_strings_size: u16,   // if flag 0x0100
    pub ttl_value: u8,
    pub data_ref_value: u32, // if flag 0x0800, has_oversize
    pub firehose_formatters: FirehoseFormatters,
}

#[derive(Clone, Copy)]
pub struct FirehoseFlags(u16);


impl FirehoseFlags {
    pub fn from_u16(value: u16) -> Self {
        FirehoseFlags(value)
    }

    // has_current_aid flag
    const ACTIVITY_ID_CURRENT: u16 = 0x1;
    // has_private_data flag
    const PRIVATE_STRING_RANGE: u16 = 0x100;
    // has_subsystem flag. In Signpost log entries this is the subsystem flag

    /// message strings UUID flag
    const MESSAGE_STRINGS_UUID: u16 = 0x2;

    const SUBSYSTEM: u16 = 0x200;
    /// has_rules flag
    const HAS_RULES: u16 = 0x400;
    /// has_oversize flag
    const DATA_REF: u16 = 0x800;
    const HAS_NAME: u16 = 0x8000;

    pub fn has_current_aid(&self) -> bool {
        self.has_flag(Self::ACTIVITY_ID_CURRENT)
    }
    pub fn has_private_string(&self) -> bool {
        self.has_flag(Self::PRIVATE_STRING_RANGE)
    }
    pub fn has_message_strings_uuid(&self) -> bool {
        self.has_flag(Self::MESSAGE_STRINGS_UUID)
    }
    pub fn has_subsystem(&self) -> bool {
        self.has_flag(Self::SUBSYSTEM)
    }
    pub fn has_rules(&self) -> bool {
        self.has_flag(Self::HAS_RULES)
    }
    pub fn has_data_ref(&self) -> bool {
        self.has_flag(Self::DATA_REF)
    }
    pub fn has_name(&self) -> bool {
        self.has_flag(Self::HAS_NAME)
    }


    pub fn has_flag(&self, flag_mask: u16) -> bool {
        (self.0 & flag_mask) != 0
    }


    /// Get only sub flags
    const FLAGS_CHECK: u16 = 0xe;

    /// large_shared_cache flag - Offset to format string is larger than normal
    const LARGE_SHARED_CACHE: u16 = 0xc;
    /// has_large_offset flag - Offset to format string is larger than normal
    const LARGE_OFFSET: u16 = 0x20;
    ///  absolute flag - The log uses an alterantive index number that points to the UUID file name in the Catalog which contains the format string
    const ABSOLUTE: u16 = 0x8;
    /// main_exe flag. A UUID file contains the format string
    const MAIN_EXE: u16 = 0x2;
    /// shared_cache flag. DSC file contains the format string
    const SHARED_CACHE: u16 = 0x4;
    /// uuid_relative flag. The UUID file name is in the log data (instead of the Catalog)
    const UUID_RELATIVE: u16 = 0xa;

    pub fn has_large_offset(&self) -> bool {
        self.flags() == Self::LARGE_OFFSET
    }
    pub fn has_large_shared_cache(&self) -> bool {
        self.flags() == Self::LARGE_SHARED_CACHE
    }
    pub fn has_absolute(&self) -> bool {
        self.flags() == Self::ABSOLUTE
    }
    pub fn has_main_exe(&self) -> bool {
        self.flags() == Self::MAIN_EXE
    }
    pub fn has_shared_cache(&self) -> bool {
        self.flags() == Self::SHARED_CACHE
    }
    pub fn has_uuid_relative(&self) -> bool {
        self.flags() == Self::UUID_RELATIVE
    }


    pub fn flags(&self) -> u16 {
        self.0 & Self::FLAGS_CHECK
    }
}

impl FirehoseSignpost {
    /// Parse Signpost Firehose log entry.
    /// Ex: tp 2368 + 92: process signpost event (shared_cache, has_name, has_subsystem)
    pub fn parse_signpost<'a>(
        data: &'a [u8],
        firehose_flags: u16,
    ) -> nom::IResult<&'a [u8], FirehoseSignpost> {
        let flags = FirehoseFlags(firehose_flags);

        let mut firehose_signpost = FirehoseSignpost::default();

        let mut input = data;

        if flags.has_current_aid() {
            debug!("[macos-unifiedlogs] Signpost Firehose has has_current_aid flag");
            let (firehose_input, unknown_activity_id) = le_u32(input)?;
            let (firehose_input, unknown_sentinel) = le_u32(firehose_input)?;
            firehose_signpost.unknown_activity_id = unknown_activity_id;
            firehose_signpost.unknown_sentinel = unknown_sentinel;
            input = firehose_input;
        }

        // Entry has private string data. The private data is found after parsing all the public data first
        if flags.has_private_string() {
            debug!("[macos-unifiedlogs] Signpost Firehose has has_private_data flag");
            let (firehose_input, private_strings_offset) = le_u16(input)?;
            let (firehose_input, private_strings_size) = le_u16(firehose_input)?;
            // Offset points to private string values found after parsing the public data. Size is the data size
            firehose_signpost.private_strings_offset = private_strings_offset;
            firehose_signpost.private_strings_size = private_strings_size;
            input = firehose_input;
        }

        let (input, unknown_pc_id) = le_u32(input)?;
        firehose_signpost.unknown_pc_id = unknown_pc_id;

        // Check for flags related to base string format location (shared string file (dsc) or UUID file)
        let (mut input, firehose_formatters) =
            FirehoseFormatters::firehose_formatter_flags(input, firehose_flags, flags)?;
        firehose_signpost.firehose_formatters = firehose_formatters;

        if flags.has_subsystem() {
            debug!("[macos-unifiedlogs] Signpost Firehose log chunk has has_subsystem flag");
            let (firehose_input, subsystem) = le_u16(input)?;
            firehose_signpost.subsystem = subsystem;
            input = firehose_input;
        }

        let (mut input, signpost_id) = le_u64(input)?;
        firehose_signpost.signpost_id = signpost_id;

        if flags.has_rules() {
            debug!("[macos-unifiedlogs] Signpost Firehose log chunk has has_rules flag");
            let (firehose_input, ttl_value) = le_u8(input)?;
            firehose_signpost.ttl_value = ttl_value;
            input = firehose_input;
        }

        if flags.has_data_ref() {
            debug!("[macos-unifiedlogs] Signpost Firehose log chunk has has_oversize flag");
            let (firehose_input, data_ref_value) = le_u32(input)?;
            firehose_signpost.data_ref_value = data_ref_value;
            input = firehose_input;
        }

        if flags.has_name() {
            debug!("[macos-unifiedlogs] Signpost Firehose log chunk has has_name flag");
            let (firehose_input, signpost_name) = le_u32(input)?;
            firehose_signpost.signpost_name = signpost_name;
            input = firehose_input;

            // If the signpost log has large_shared_cache flag
            // Then the signpost name has the same value after as the large_shared_cache
            if firehose_signpost.firehose_formatters.large_shared_cache != 0 {
                let (firehose_input, _) = take(size_of::<u16>())(input)?;
                input = firehose_input;
            }
        }

        Ok((input, firehose_signpost))
    }

    /// Get base log message string formatter from shared cache strings (dsc) or UUID text file for firehose signpost log entries (chunks)
    pub fn get_firehose_signpost<'a>(
        firehose: &FirehoseSignpost,
        strings_data: &'a [UUIDText],
        shared_strings: &'a [SharedCacheStrings],
        string_offset: u64,
        first_proc_id: u64,
        second_proc_id: u32,
        catalogs: &CatalogChunk,
    ) -> nom::IResult<&'a [u8], MessageData> {
        if firehose.firehose_formatters.shared_cache
            || (firehose.firehose_formatters.large_shared_cache != 0
                && firehose.firehose_formatters.has_large_offset != 0)
        {
            if firehose.firehose_formatters.has_large_offset != 0 {
                let mut large_offset = firehose.firehose_formatters.has_large_offset;
                let extra_offset_value;
                // large_shared_cache should be double the value of has_large_offset
                // Ex: has_large_offset = 1, large_shared_cache = 2
                // If the value do not match then there is an issue with shared string offset
                // Can recover by using large_shared_cache
                // Apple records this as an error: "error: ~~> <Invalid shared cache code pointer offset>"
                //   But is still able to get string formatter
                if large_offset != firehose.firehose_formatters.large_shared_cache / 2
                    && !firehose.firehose_formatters.shared_cache
                {
                    large_offset = firehose.firehose_formatters.large_shared_cache / 2;
                    // Combine large offset value with current string offset to get the true offset
                    extra_offset_value = format!("{:X}{:08X}", large_offset, string_offset);
                } else if firehose.firehose_formatters.shared_cache {
                    // Large offset is 8 if shared_cache flag is set
                    large_offset = 8;
                    extra_offset_value = format!("{:X}{:07X}", large_offset, string_offset);
                } else {
                    extra_offset_value = format!("{:X}{:08X}", large_offset, string_offset);
                }

                // Combine large offset value with current string offset to get the true offset
                //let extra_offset_value = format!("{:X}{:07X}", large_offset, string_offset);
                let extra_offset_value_result = u64::from_str_radix(&extra_offset_value, 16);
                match extra_offset_value_result {
                    Ok(offset) => {
                        return MessageData::extract_shared_strings(
                            shared_strings,
                            strings_data,
                            offset,
                            first_proc_id,
                            second_proc_id,
                            catalogs,
                            string_offset,
                        );
                    }
                    Err(err) => {
                        // We should not get errors since we are combining two numbers to create the offset
                        error!(
                            "Failed to get shared string offset to format string for signpost firehose entry: {:?}",
                            err
                        );
                        return Err(nom::Err::Incomplete(Needed::Unknown));
                    }
                }
            }
            MessageData::extract_shared_strings(
                shared_strings,
                strings_data,
                string_offset,
                first_proc_id,
                second_proc_id,
                catalogs,
                string_offset,
            )
        } else {
            if firehose.firehose_formatters.absolute {
                let extra_offset_value = format!(
                    "{:X}{:08X}",
                    firehose.firehose_formatters.main_exe_alt_index, firehose.unknown_pc_id,
                );

                let offset_result = u64::from_str_radix(&extra_offset_value, 16);
                match offset_result {
                    Ok(offset) => {
                        return MessageData::extract_absolute_strings(
                            strings_data,
                            offset,
                            string_offset,
                            first_proc_id,
                            second_proc_id,
                            catalogs,
                            string_offset,
                        );
                    }
                    Err(err) => {
                        // We should not get errors since we are combining two numbers to create the offset
                        error!("Failed to get absolute offset to format string for signpost firehose entry: {:?}", err);
                        return Err(nom::Err::Incomplete(Needed::Unknown));
                    }
                }
            }
            if !firehose.firehose_formatters.uuid_relative.is_empty() {
                return MessageData::extract_alt_uuid_strings(
                    strings_data,
                    string_offset,
                    &firehose.firehose_formatters.uuid_relative,
                    first_proc_id,
                    second_proc_id,
                    catalogs,
                    string_offset,
                );
            }
            MessageData::extract_format_strings(
                strings_data,
                string_offset,
                first_proc_id,
                second_proc_id,
                catalogs,
                string_offset,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::chunks::firehose::firehose_log::FirehoseItem;
    use crate::chunks::firehose::signpost::FirehoseSignpost;
    use crate::filesystem::LogarchiveProvider;
    use crate::parser::{collect_shared_strings, collect_strings, parse_log};
    use std::path::PathBuf;

    #[test]
    fn test_parse_signpost() {
        let test_data = [
            225, 244, 2, 0, 1, 0, 238, 238, 178, 178, 181, 176, 238, 238, 176, 63, 27, 0, 0, 0,
        ];
        let test_flags = 33282;
        let (_, results) = FirehoseSignpost::parse_signpost(&test_data, test_flags).unwrap();
        assert_eq!(results.unknown_pc_id, 193761);
        assert_eq!(results.unknown_activity_id, 0);
        assert_eq!(results.unknown_sentinel, 0);
        assert_eq!(results.subsystem, 1);
        assert_eq!(results.signpost_id, 17216892719917625070);
        assert_eq!(results.signpost_name, 1785776);
        assert_eq!(results.ttl_value, 0);
        assert_eq!(results.data_ref_value, 0);

        assert!(results.firehose_formatters.main_exe);
        assert!(!results.firehose_formatters.shared_cache);
        assert_eq!(results.firehose_formatters.has_large_offset, 0);
        assert_eq!(results.firehose_formatters.large_shared_cache, 0);
        assert!(!results.firehose_formatters.absolute);
        assert_eq!(results.firehose_formatters.uuid_relative, String::new());
        assert!(!results.firehose_formatters.main_plugin);
        assert!(!results.firehose_formatters.pc_style);
        assert_eq!(results.firehose_formatters.main_exe_alt_index, 0);
    }

    #[test]
    fn test_get_firehose_signpost_big_sur() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/system_logs_big_sur.logarchive");
        let provider = LogarchiveProvider::new(test_path.as_path());

        let string_results = collect_strings(&provider).unwrap();
        let shared_strings_results = collect_shared_strings(&provider).unwrap();

        test_path.push("Signpost/0000000000000001.tracev3");

        let handle = std::fs::File::open(&test_path).unwrap();
        let log_data = parse_log(handle).unwrap();

        for catalog_data in log_data.catalog_data {
            for preamble in catalog_data.firehose {
                for firehose in preamble.public_data {
                    if let FirehoseItem::Signpost(signpost) = firehose.item {
                        let (_, message_data) = FirehoseSignpost::get_firehose_signpost(
                            &signpost,
                            &string_results,
                            &shared_strings_results,
                            u64::from(firehose.format_string_location),
                            preamble.first_number_proc_id,
                            preamble.second_number_proc_id,
                            &catalog_data.catalog,
                        )
                        .unwrap();
                        assert_eq!(message_data.format_string, "");
                        assert_eq!(message_data.library, "/usr/libexec/kernelmanagerd");
                        assert_eq!(message_data.process, "/usr/libexec/kernelmanagerd");
                        assert_eq!(
                            message_data.process_uuid,
                            "CCCF30257483376883C824222233386D"
                        );
                        assert_eq!(
                            message_data.library_uuid,
                            "CCCF30257483376883C824222233386D"
                        );

                        return;
                    }
                }
            }
        }
    }
}
