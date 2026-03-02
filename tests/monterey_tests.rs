// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use std::{fs, io::Read, path::PathBuf};

use macos_unifiedlogs::{
    filesystem::LogarchiveProvider,
    log_data_iterator::{LogDataIterator, iterate_all_logs},
    noalloc_iterator::NoAllocLogStream,
    parser::{collect_timesync, parse_log},
    traits::FileProvider,
    unified_log::{EventType, LogData, LogType},
};
use regex::Regex;
use uuid::Uuid;

#[test]
fn test_parse_log_monterey() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_monterey.logarchive");

    test_path.push("Persist/000000000000000a.tracev3");
    let handle = fs::File::open(test_path.as_path()).unwrap();

    let log_data = parse_log(handle).unwrap();

    assert_eq!(log_data.catalog_data[0].firehose.len(), 17);
    assert_eq!(log_data.catalog_data[0].simpledump.len(), 383);
    assert_eq!(log_data.header.len(), 1);
    assert_eq!(
        log_data.catalog_data[0]
            .catalog
            .catalog_process_info_entries
            .len(),
        17
    );
    assert_eq!(log_data.catalog_data[0].statedump.len(), 0);
}

#[test]
fn test_build_log_monterey() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_monterey.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let buf = fs::read(test_path.join("Persist/000000000000000a.tracev3")).unwrap();

    let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);
    let results: Vec<LogData> = iter.collect();
    assert_eq!(results.len(), 322812);

    // Find the specific kernel entry (streaming path may have error entries before it)
    let entry = results
        .iter()
        .find(|r| r.process.as_str() == "/kernel" && r.time == 1651345928766719209.0)
        .expect("Should find kernel sandbox entry");
    assert_eq!(entry.subsystem.as_str(), "");
    assert_eq!(entry.activity_id, 0);
    assert_eq!(
        entry.library.as_str(),
        "/System/Library/Extensions/Sandbox.kext/Contents/MacOS/Sandbox"
    );
    assert_eq!(
        entry.message.as_str(),
        "2 duplicate reports for Sandbox: MTLCompilerServi(187) deny(1) file-read-metadata /private"
    );
    assert_eq!(entry.pid, 0);
    assert_eq!(entry.thread_id, 2241);
    assert_eq!(entry.category.as_str(), "");
    assert_eq!(entry.log_type, LogType::Error);
    assert_eq!(entry.event_type, EventType::Log);
    assert_eq!(entry.euid, 0);
    assert_eq!(
        entry.boot_uuid,
        Uuid::parse_str("17AB576950394796B7F3CD2C157F4A2F").unwrap()
    );
    assert_eq!(entry.timezone_name.as_str(), "New_York");
    assert_eq!(
        entry.library_uuid,
        Uuid::parse_str("7EFAFB8B6CA63090957FC68A6230BC38").unwrap()
    );
    assert_eq!(
        entry.process_uuid,
        Uuid::parse_str("C342869FFFB93CCEA5A3EA711C1E87F6").unwrap()
    );
    assert_eq!(entry.raw_message.as_str(), "%s");
}

#[test]
fn test_parse_all_logs_monterey() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_monterey.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let exclude_missing = false;
    let log_data_vec = iterate_all_logs(&mut provider, &timesync_data, exclude_missing);

    let message_re = Regex::new(r"^[\s]*%s\s*$").unwrap();

    assert_eq!(log_data_vec.len(), 2396970);

    let mut unknown_strings = 0;
    let mut invalid_offsets = 0;
    let mut invalid_shared_string_offsets = 0;
    let mut statedump_custom_objects = 0;
    let mut statedump_protocol_buffer = 0;
    let mut string_count = 0;

    let mut mutilities_worldclock = 0;
    let mut mutililties_return = 0;
    let mut location_tracker = 0;
    let mut pauses_tracker = 0;
    let mut dns_counts = 0;

    for logs in &log_data_vec {
        if logs.message.contains("Failed to get string message from ")
            || logs.message.contains("Unknown shared string message")
        {
            unknown_strings += 1;
        }

        if logs.message.contains("Error: Invalid offset ") {
            invalid_offsets += 1;
        }

        if logs.message.contains("Error: Invalid shared string offset") {
            invalid_shared_string_offsets += 1;
        }

        if logs.message.contains("Unsupported Statedump object") {
            statedump_custom_objects += 1;
        }
        if logs.message.contains("Failed to parse StateDump protobuf")
            || logs
                .message
                .contains("Failed to serialize Protobuf HashMap")
        {
            statedump_protocol_buffer += 1;
        }

        if message_re.is_match(&logs.raw_message) {
            string_count += 1;
        }

        if logs.message.contains("MTUtilities: WorldClockWidget:")
            && logs.log_type == LogType::Default
        {
            mutilities_worldclock += 1;
        }
        if logs.message.contains("MTUtilities: Returning widget") {
            mutililties_return += 1;
        }
        if logs.message.contains("allowsMapCorrection") {
            location_tracker += 1;
        }
        if logs
            .message
            .contains("\"pausesLocationUpdatesAutomatically\":1,")
        {
            pauses_tracker += 1;
        }
        if logs.message.contains("Question Count: 1, Answer Record Count: 0, Authority Record Count: 0, Additional Record Count: 0") {
            dns_counts += 1;
        }
    }
    assert_eq!(unknown_strings, 531);
    assert_eq!(invalid_offsets, 60);
    assert_eq!(invalid_shared_string_offsets, 309);
    assert_eq!(statedump_custom_objects, 0);
    assert_eq!(statedump_protocol_buffer, 0);
    assert_eq!(string_count, 28196); // Accurate count based on log raw-dump -a <monterey.logarchive> | grep "format:\s*%s$" | sort | uniq -c | sort -n
    assert_eq!(mutilities_worldclock, 59);
    assert_eq!(mutililties_return, 71);
    assert_eq!(dns_counts, 3196);

    assert_eq!(location_tracker, 297);
    assert_eq!(pauses_tracker, 179);
}

#[test]
fn test_parse_all_logs_monterey_noalloc() {
    let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_path.push("tests/test_data/system_logs_monterey.logarchive");

    let mut provider = LogarchiveProvider::new(test_path.as_path());

    let timesync_data = collect_timesync(&provider).unwrap();

    let message_re = Regex::new(r"^[\s]*%s\s*$").unwrap();

    let mut total_count = 0u64;
    let mut unknown_strings = 0;
    let mut invalid_offsets = 0;
    let mut invalid_shared_string_offsets = 0;
    let mut statedump_custom_objects = 0;
    let mut statedump_protocol_buffer = 0;
    let mut string_count = 0;
    let mut mutilities_worldclock = 0;
    let mut mutililties_return = 0;
    let mut location_tracker = 0;
    let mut pauses_tracker = 0;
    let mut dns_counts = 0;

    let mut oversize_cache = Vec::new();
    for mut file in provider.tracev3_files() {
        let mut buf = Vec::new();
        file.reader().read_to_end(&mut buf).unwrap();

        let mut stream =
            NoAllocLogStream::with_oversize_cache(&buf, &timesync_data, oversize_cache);
        while let Some(entry) = stream.next_entry() {
            // continue;
            if let Ok(logs) = stream.resolve(&entry, &mut provider) {
                total_count += 1;

                if logs.message.contains("Failed to get string message from ")
                    || logs.message.contains("Unknown shared string message")
                {
                    unknown_strings += 1;
                }

                if logs.message.contains("Error: Invalid offset ") {
                    invalid_offsets += 1;
                }

                if logs.message.contains("Error: Invalid shared string offset") {
                    invalid_shared_string_offsets += 1;
                }

                if logs.message.contains("Unsupported Statedump object") {
                    statedump_custom_objects += 1;
                }
                if logs.message.contains("Failed to parse StateDump protobuf")
                    || logs
                        .message
                        .contains("Failed to serialize Protobuf HashMap")
                {
                    statedump_protocol_buffer += 1;
                }

                if message_re.is_match(&logs.raw_message) {
                    string_count += 1;
                }

                if logs.message.contains("MTUtilities: WorldClockWidget:")
                    && logs.log_type == LogType::Default
                {
                    mutilities_worldclock += 1;
                }
                if logs.message.contains("MTUtilities: Returning widget") {
                    mutililties_return += 1;
                }
                if logs.message.contains("allowsMapCorrection") {
                    location_tracker += 1;
                }
                if logs
                    .message
                    .contains("\"pausesLocationUpdatesAutomatically\":1,")
                {
                    pauses_tracker += 1;
                }
                if logs.message.contains("Question Count: 1, Answer Record Count: 0, Authority Record Count: 0, Additional Record Count: 0") {
                    dns_counts += 1;
                }
            }
        }
        oversize_cache = stream.into_oversize_cache();
    }

    assert_eq!(total_count, 2396970);
    assert_eq!(unknown_strings, 531);
    assert_eq!(invalid_offsets, 60);
    assert_eq!(invalid_shared_string_offsets, 309);
    assert_eq!(statedump_custom_objects, 0);
    assert_eq!(statedump_protocol_buffer, 0);
    assert_eq!(string_count, 28196);
    assert_eq!(mutilities_worldclock, 59);
    assert_eq!(mutililties_return, 71);
    assert_eq!(dns_counts, 3196);
    assert_eq!(location_tracker, 297);
    assert_eq!(pauses_tracker, 179);
}
