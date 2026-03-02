// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use macos_unifiedlogs::{
    filesystem::LogarchiveProvider,
    log_data_iterator::{LogDataIterator, iterate_all_logs},
    parser::{collect_timesync, parse_log},
    unified_log::{EventType, LogData, LogType},
};
use regex::Regex;
use std::{fs, path::PathBuf};
use uuid::Uuid;

fn is_signpost(log_type: LogType) -> bool {
    match log_type {
        LogType::ProcessSignpostEvent
        | LogType::ProcessSignpostStart
        | LogType::ProcessSignpostEnd
        | LogType::SystemSignpostEvent
        | LogType::SystemSignpostStart
        | LogType::SystemSignpostEnd
        | LogType::ThreadSignpostEvent
        | LogType::ThreadSignpostStart
        | LogType::ThreadSignpostEnd => true,
        LogType::Debug
        | LogType::Info
        | LogType::Default
        | LogType::Error
        | LogType::Fault
        | LogType::Create
        | LogType::Useraction
        | LogType::Simpledump
        | LogType::Statedump
        | LogType::Loss => false,
    }
}

#[test]
fn test_parse_log_big_sur() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur.logarchive");
    test_path.push("Persist/0000000000000004.tracev3");

    let handle = fs::File::open(test_path.as_path()).unwrap();
    let log_data = parse_log(handle).unwrap();

    assert_eq!(log_data.catalog_data[0].firehose.len(), 82);
    assert_eq!(log_data.catalog_data[0].simpledump.len(), 0);
    assert_eq!(log_data.header.len(), 1);
    assert_eq!(
        log_data.catalog_data[0]
            .catalog
            .catalog_process_info_entries
            .len(),
        45
    );
    assert_eq!(log_data.catalog_data[0].statedump.len(), 0);
}

#[test]
fn test_big_sur_livedata() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let mut file_path = test_path.clone();
    file_path.push("logdata.LiveData.tracev3");
    let buf = fs::read(&file_path).unwrap();

    let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);
    let data: Vec<LogData> = iter.collect();
    assert_eq!(data.len(), 101566);

    for results in data {
        // Test for a log message that uses a firehose_header_timestamp with a value of zero
        if results.message.as_str() == "TimeSyncTime is mach_absolute_time nanoseconds\n" {
            assert_eq!(
                results.message.as_str(),
                "TimeSyncTime is mach_absolute_time nanoseconds\n"
            );
            assert_eq!(results.activity_id, 0);
            assert_eq!(results.thread_id, 116);
            assert_eq!(results.euid, 0);
            assert_eq!(results.pid, 0);
            assert_eq!(
                results.library.as_str(),
                "/System/Library/Extensions/IOTimeSyncFamily.kext/Contents/MacOS/IOTimeSyncFamily"
            );
            assert_eq!(results.subsystem.as_str(), "");
            assert_eq!(results.category.as_str(), "");
            assert_eq!(results.event_type, EventType::Log);
            assert_eq!(results.log_type, LogType::Info);
            assert_eq!(results.process.as_str(), "/kernel");
            assert_eq!(results.time, 1642304801596413351.0);
            assert_eq!(
                results.boot_uuid,
                Uuid::parse_str("A2A9017676CF421C84DC9BBD6263FEE7").unwrap()
            );
            assert_eq!(results.timezone_name.as_str(), "Pacific");
        }
    }
}

#[test]
fn test_build_log_big_sur() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let mut file_path = test_path.clone();
    file_path.push("Persist/0000000000000004.tracev3");
    let buf = fs::read(&file_path).unwrap();

    let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);
    let results: Vec<LogData> = iter.collect();
    assert_eq!(results.len(), 110953);
    assert_eq!(results[0].process.as_str(), "/usr/libexec/opendirectoryd");
    assert_eq!(results[0].subsystem.as_str(), "com.apple.opendirectoryd");
    assert_eq!(results[0].time, 1642303933964503310.0);
    assert_eq!(results[0].activity_id, 0);
    assert_eq!(results[0].library.as_str(), "/usr/libexec/opendirectoryd");
    assert_eq!(
        results[0].message.as_str(),
        "opendirectoryd (build 796.100) launched..."
    );
    assert_eq!(results[0].pid, 105);
    assert_eq!(results[0].thread_id, 670);
    assert_eq!(results[0].category.as_str(), "default");
    assert_eq!(results[0].log_type, LogType::Default);
    assert_eq!(results[0].event_type, EventType::Log);
    assert_eq!(results[0].euid, 0);
    assert_eq!(
        results[0].boot_uuid,
        Uuid::parse_str("AACFB573E87545CE98B893D132766A46").unwrap()
    );
    assert_eq!(results[0].timezone_name.as_str(), "Pacific");
    assert_eq!(
        results[0].library_uuid,
        Uuid::parse_str("B736DF1625F538248E9527A8CEC4991E").unwrap()
    );
    assert_eq!(
        results[0].process_uuid,
        Uuid::parse_str("B736DF1625F538248E9527A8CEC4991E").unwrap()
    );
    assert_eq!(
        results[0].raw_message.as_str(),
        "opendirectoryd (build %{public}s) launched..."
    );
}

#[test]
fn test_parse_all_logs_big_sur() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let exclude_missing = false;
    let log_data_vec = iterate_all_logs(&mut provider, &timesync_data, exclude_missing);

    // Run: "log raw-dump -a macos-unifiedlogs/tests/test_data/system_logs_big_sur.logarchive"
    // total log entries: 747,294
    // Add Statedump log entries: 322
    // Streaming path produces 747,614 (2 fewer error entries vs old batch path)
    assert_eq!(log_data_vec.len(), 747614);

    let mut unknown_strings = 0;
    let mut invalid_offsets = 0;
    let mut invalid_shared_string_offsets = 0;
    let mut statedump_custom_objects = 0;
    let mut statedump_protocol_buffer = 0;

    let mut found_precision_string = false;
    let mut statedump_count = 0;
    let mut signpost_count = 0;

    let mut default_type = 0;
    let mut info_type = 0;
    let mut error_type = 0;
    let mut create_type = 0;
    let mut debug_type = 0;
    let mut useraction_type = 0;
    let mut fault_type = 0;
    let mut loss_type = 0;

    let mut string_count = 0;
    let message_re = Regex::new(r"^[\s]*%s\s*$").unwrap();
    let mut empty_format_count = 0;
    let mut sock_count = 0;
    let mut location_harvest_count = 0;

    // Breakdown log entries by smaller types to ensure count is accurate
    for logs in &log_data_vec {
        if logs.message.contains("Failed to get string message from ")
            || logs.message.contains("Unknown shared string message")
        {
            unknown_strings += 1;
        } else if logs.message.contains("Error: Invalid offset ") {
            invalid_offsets += 1;
        } else if logs.message.contains("Error: Invalid shared string offset") {
            invalid_shared_string_offsets += 1;
        } else if logs.message.contains("Unsupported Statedump object") {
            statedump_custom_objects += 1;
        } else if logs.message.contains("Failed to parse StateDump protobuf")
            || logs
                .message
                .contains("Failed to serialize Protobuf HashMap")
        {
            statedump_protocol_buffer += 1;
        } else if logs.message.as_str()
            == r##"#32EC4B64 [AssetCacheLocatorService.queue] sending POST [327]{"locator-tag":"#32ec4b64","local-addresses":["192.168.101.144"],"ranked-results":true,"locator-software":[{"build":"20G224","type":"system","name":"macOS","version":"11.6.1"},{"id":"com.apple.AssetCacheLocatorService","executable":"AssetCacheLocatorService","type":"bundle","name":"AssetCacheLocatorService","version":"118"}]} to https://lcdn-locator.apple.com/lcdn/locate"##
        {
            found_precision_string = true;
        }

        if logs.event_type == EventType::Statedump {
            statedump_count += 1;
        } else if logs.event_type == EventType::Signpost {
            signpost_count += 1;
        } else if logs.log_type == LogType::Default {
            default_type += 1;
        } else if logs.log_type == LogType::Info {
            info_type += 1;
        } else if logs.log_type == LogType::Error {
            error_type += 1
        } else if logs.log_type == LogType::Create {
            create_type += 1;
        } else if logs.log_type == LogType::Debug {
            debug_type += 1;
        } else if logs.log_type == LogType::Useraction {
            useraction_type += 1;
        } else if logs.log_type == LogType::Fault {
            fault_type += 1;
        } else if logs.event_type == EventType::Loss {
            loss_type += 1;
        }

        if logs.message.contains("\"subHarvester\":Trace") {
            location_harvest_count += 1;
        }

        if message_re.is_match(&logs.raw_message) {
            string_count += 1;
        }

        if logs.raw_message.is_empty()
            && logs.message.is_empty()
            && logs.event_type != EventType::Loss
        {
            empty_format_count += 1
        }

        if logs.message.contains("nw_resolver_create_dns_getaddrinfo_locked_block_invoke [C1] Got DNS result type NoAddress ifindex=0 configuration.ls.apple.com configuration.ls.apple.com. ::") {
            sock_count += 1;
        }
    }

    assert_eq!(unknown_strings, 0);
    assert_eq!(invalid_offsets, 54);
    assert_eq!(invalid_shared_string_offsets, 0);
    assert_eq!(statedump_custom_objects, 0);
    assert_eq!(statedump_protocol_buffer, 0);
    assert!(found_precision_string);

    assert_eq!(statedump_count, 320);
    assert_eq!(signpost_count, 50665);
    assert_eq!(string_count, 11764);
    assert_eq!(empty_format_count, 56);
    assert_eq!(default_type, 462518);
    assert_eq!(info_type, 114540);
    assert_eq!(error_type, 29132);
    assert_eq!(create_type, 87831);
    assert_eq!(debug_type, 1908);
    assert_eq!(useraction_type, 15);
    assert_eq!(fault_type, 680);
    assert_eq!(loss_type, 5);
    assert_eq!(sock_count, 2);
    assert_eq!(location_harvest_count, 11);
}

#[test]
fn test_parse_all_persist_logs_with_network_big_sur() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let exclude_missing = false;
    let log_data_vec = iterate_all_logs(&mut provider, &timesync_data, exclude_missing);

    let mut messages_containing_network = 0;
    let mut default_type = 0;
    let mut info_type = 0;
    let mut error_type = 0;
    let mut create_type = 0;
    let mut state_simple_dump = 0;
    let mut signpost = 0;

    let mut network_message_uuid = false;

    // Check all logs that contain the word "network"
    for logs in &log_data_vec {
        if logs.message.to_lowercase().contains("network") {
            if logs.log_type == LogType::Default {
                default_type += 1;
                if logs
                    .message
                    .contains("7C10C1EF-1B86-494F-800D-C769A89172C1")
                {
                    network_message_uuid = true;
                }
            } else if logs.log_type == LogType::Info {
                info_type += 1;
            } else if logs.log_type == LogType::Error {
                error_type += 1
            } else if logs.log_type == LogType::Create {
                create_type += 1;
                // We are basing these counts on the Console.app tool
                // Console.app skips Activity event logs
                continue;
            } else if logs.event_type == EventType::Simpledump
                || logs.event_type == EventType::Statedump
            {
                // We are basing these counts on the Console.app tool
                // Console.app skips Simple and State dump event logs
                state_simple_dump += 1;
                continue;
            } else if is_signpost(logs.log_type) {
                // We are basing these counts on the Console.app tool
                // Console.app skips Signpost event logs
                signpost += 1;
                continue;
            }
            messages_containing_network += 1;
        }
    }
    assert_eq!(messages_containing_network, 9173);
    // Console.app is missing a log entry. The log command shows the entry
    assert_eq!(default_type, 8320);
    assert!(network_message_uuid);

    assert_eq!(info_type, 638);
    assert_eq!(error_type, 215);
    assert_eq!(create_type, 687);
    assert_eq!(state_simple_dump, 34);
    assert_eq!(signpost, 62);
}

#[test]
fn test_parse_all_logs_private_big_sur() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur_private_enabled.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let exclude_missing = false;
    let log_data_vec = iterate_all_logs(&mut provider, &timesync_data, exclude_missing);
    assert_eq!(log_data_vec.len(), 887888);

    let mut empty_counter = 0;
    let mut not_found = 0;
    let mut staff_count = 0;
    for logs in log_data_vec {
        if logs.message.is_empty() {
            empty_counter += 1;
        }
        if logs.message.contains("<not found>") {
            not_found += 1;
        }
        if logs.message.contains("group: staff@/Local/Default") {
            staff_count += 1;
        }
    }
    assert_eq!(not_found, 0);
    assert_eq!(staff_count, 4);
    assert_eq!(empty_counter, 596);
}

// Test for logs that have same public data in private
#[test]
fn test_parse_all_logs_private_with_public_mix_big_sur() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur_public_private_data_mix.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let exclude_missing = false;
    let log_data_vec = iterate_all_logs(&mut provider, &timesync_data, exclude_missing);
    assert_eq!(log_data_vec.len(), 1287596);

    let mut not_found = 0;
    let mut user_not_found = 0;
    let mut mobile_not_found = 0;
    let mut bssid_count = 0;
    let mut dns_query_count = 0;
    let mut bofa_count = 0;

    for logs in log_data_vec {
        if logs.message.contains("<not found>") {
            not_found += 1;
        }
        if logs.message.contains("user: -1 <not found>") {
            user_not_found += 1;
        }

        if logs
            .message
            .contains("refreshing: details, reason: expired, user: mobile <not found>")
        {
            mobile_not_found += 1;
        }

        if logs.message.contains("BSSID 00:00:00:00:00:00") {
            bssid_count += 1;
        }

        if logs.message.contains("https://doh.dns.apple.com/dns-query") {
            dns_query_count += 1;
        }

        if logs.message.contains("bankofamerica") {
            bofa_count += 1;
        }
    }
    assert_eq!(not_found, 5);
    assert_eq!(user_not_found, 2);
    assert_eq!(mobile_not_found, 1);
    assert_eq!(bssid_count, 38);
    assert_eq!(dns_query_count, 41);
    assert_eq!(bofa_count, 573);
}

#[test]
fn test_parse_all_logs_private_with_public_mix_big_sur_single_file() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur_public_private_data_mix.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let mut file_path = test_path.clone();
    file_path.push("Persist/0000000000000009.tracev3");
    let buf = fs::read(&file_path).unwrap();

    let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);
    let results: Vec<LogData> = iter.collect();
    assert_eq!(results.len(), 91567);

    let mut hex_count = 0;
    let mut dns = 0;
    let mut public_private_mixture = false;
    for result in results {
        if result.message.contains("7FAE25804F50") {
            hex_count += 1;
        }
        if result.subsystem.contains(".mdns") {
            dns += 1;
        }
        // 7FAE2352A540 is half public and half private
        // The pointer value comes from combined public+private data
        if result.message.as_str()
            == "os_transaction created: (7FAE2352A540) CLLS:0x7fae23628160.LocationFine"
        {
            public_private_mixture = true
        }
    }

    assert_eq!(hex_count, 4);
    assert_eq!(dns, 801);
    assert!(public_private_mixture);
}

// We are able to get 2238 entries from this special tracev3 file. But log command only gets 231
#[test]
fn test_parse_all_logs_private_with_public_mix_big_sur_special_file() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur_public_private_data_mix.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let mut file_path = test_path.clone();
    file_path.push("Special/0000000000000008.tracev3");
    let buf = fs::read(&file_path).unwrap();

    let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);
    let results: Vec<LogData> = iter.collect();
    assert_eq!(results.len(), 2238);

    let mut statedump = 0;
    let mut default = 0;
    let mut fault = 0;
    let mut info = 0;
    let mut error = 0;

    for result in results {
        if result.event_type == EventType::Statedump {
            statedump += 1;
        } else if result.log_type == LogType::Default {
            default += 1;
        } else if result.log_type == LogType::Fault {
            fault += 1;
        } else if result.log_type == LogType::Info {
            info += 1;
        } else if result.log_type == LogType::Error {
            error += 1;
        }
    }

    assert_eq!(statedump, 1);
    assert_eq!(default, 1972);
    assert_eq!(fault, 32);
    assert_eq!(info, 41);
    assert_eq!(error, 192);
}

#[test]
fn test_big_sur_missing_oversize_strings() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    // livedata may have oversize string data in other tracev3 on disk
    let mut file_path = test_path.clone();
    file_path.push("logdata.LiveData.tracev3");
    let buf = fs::read(&file_path).unwrap();

    let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);
    let data: Vec<LogData> = iter.collect();
    assert_eq!(data.len(), 101566);

    let mut missing_strings = 0;
    for results in data {
        if results.message.contains("<Missing message data>") {
            missing_strings += 1;
        }
    }
    // There should be only 29 entries that have actual missing data
    // 23 strings are in other tracev3 files. 23 + 29 = 52
    assert_eq!(missing_strings, 52);
}

#[test]
fn test_big_sur_oversize_strings_in_another_file() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_big_sur.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    // Get most recent Persist tracev3 file could contain oversize log entries
    let persist_buf = fs::read(test_path.join("Persist/0000000000000005.tracev3")).unwrap();
    // Get most recent Special tracev3 file that could contain oversize log entries
    let special_buf = fs::read(test_path.join("Special/0000000000000005.tracev3")).unwrap();
    // LiveData file
    let livedata_buf = fs::read(test_path.join("logdata.LiveData.tracev3")).unwrap();

    // Collect oversize from persist and special files first
    use macos_unifiedlogs::noalloc_iterator::NoAllocLogStream;
    let mut stream = NoAllocLogStream::new(&persist_buf, &timesync_data);
    while stream.next_entry().is_some() {}
    let mut cache = stream.into_oversize_cache();

    let mut stream = NoAllocLogStream::with_oversize_cache(&special_buf, &timesync_data, cache);
    while stream.next_entry().is_some() {}
    cache = stream.into_oversize_cache();

    // Now parse livedata with the combined oversize cache
    let iter = LogDataIterator::with_oversize_cache(
        livedata_buf,
        &mut provider,
        &timesync_data,
        false,
        cache,
    );
    let data: Vec<LogData> = iter.collect();
    assert_eq!(data.len(), 101566);

    let mut missing_strings = 0;
    for results in data {
        if results.message.contains("<Missing message data>") {
            missing_strings += 1;
        }
    }
    // 29 log entries actually have missing data
    // Apple displays as: <decode: missing data>
    assert_eq!(missing_strings, 29);
}
