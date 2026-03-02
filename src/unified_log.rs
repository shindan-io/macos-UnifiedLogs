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
use crate::header::HeaderChunkOwned;
use chrono::{DateTime, Utc};
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

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use uuid::Uuid;
    use crate::{
        filesystem::LogarchiveProvider,
        log_data_iterator::LogDataIterator,
        parser::collect_timesync,
        unified_log::{EventType, LogType},
    };
    use std::{fs, path::PathBuf};

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
}
