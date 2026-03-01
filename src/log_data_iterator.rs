// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use std::collections::HashMap;

use uuid::Uuid;

use crate::chunks::oversize::Oversize;
use crate::noalloc_iterator::{NoAllocEntry, NoAllocLogStream, ResolveError};
use crate::timesync::TimesyncBoot;
use crate::traits::FileProvider;
use crate::unified_log::{LogData, UnifiedLogData};
use crate::util::unixepoch_to_datetime;
use crate::{empty_rc_string, rc_string};

/// An item yielded by [`LogDataIterator`].
#[derive(Debug)]
pub enum LogIteratorItem {
    /// A fully resolved log entry.
    Log(LogData),
    /// A chunk of log data that could not be fully resolved (missing strings).
    MissingData(UnifiedLogData),
}

/// Streaming iterator that yields individual [`LogIteratorItem`] entries from a tracev3 file.
///
/// Uses [`NoAllocLogStream`] internally for efficient single-pass iteration. Each entry
/// is resolved on demand via [`NoAllocLogStream::resolve()`].
///
/// # Example
/// ```no_run
/// use macos_unifiedlogs::filesystem::LogarchiveProvider;
/// use macos_unifiedlogs::parser::collect_timesync;
/// use macos_unifiedlogs::log_data_iterator::{LogDataIterator, LogIteratorItem};
/// use std::path::PathBuf;
///
/// let path = PathBuf::from("system_logs.logarchive");
/// let mut provider = LogarchiveProvider::new(path.as_path());
/// let timesync = collect_timesync(&provider).unwrap();
///
/// let buf = std::fs::read("system_logs.logarchive/Persist/0000000000000002.tracev3").unwrap();
/// let iter = LogDataIterator::new(buf, &mut provider, &timesync, true);
/// for item in iter {
///     match item {
///         LogIteratorItem::Log(entry) => println!("{}: {}", entry.pid, entry.message),
///         LogIteratorItem::MissingData(_) => {}
///     }
/// }
/// ```
pub struct LogDataIterator<'a> {
    stream: NoAllocLogStream<'static, 'a>,
    provider: &'a mut dyn FileProvider,
    exclude_missing: bool,
}

impl<'a> LogDataIterator<'a> {
    /// Create a new iterator over a tracev3 file buffer.
    pub fn new(
        data: Vec<u8>,
        provider: &'a mut dyn FileProvider,
        timesync_data: &'a HashMap<Uuid, TimesyncBoot>,
        exclude_missing: bool,
    ) -> Self {
        Self::with_oversize_cache(data, provider, timesync_data, exclude_missing, Vec::new())
    }

    /// Create a new iterator, carrying over an oversize cache from a previous tracev3 file.
    ///
    /// When processing multiple tracev3 files in sequence, pass the oversize cache from
    /// the previous file via [`into_oversize_cache`](Self::into_oversize_cache) to allow
    /// resolution of oversize strings that span files.
    pub fn with_oversize_cache(
        data: Vec<u8>,
        provider: &'a mut dyn FileProvider,
        timesync_data: &'a HashMap<Uuid, TimesyncBoot>,
        exclude_missing: bool,
        oversize_cache: Vec<Oversize>,
    ) -> Self {
        Self {
            stream: NoAllocLogStream::from_owned_with_oversize_cache(
                data,
                timesync_data,
                oversize_cache,
            ),
            provider,
            exclude_missing,
        }
    }

    /// Peek at the current oversize cache.
    pub fn oversize_cache(&self) -> &[Oversize] {
        self.stream.oversize_cache()
    }

    /// Consume the iterator and return the oversize cache for use with the next tracev3 file.
    pub fn into_oversize_cache(self) -> Vec<Oversize> {
        self.stream.into_oversize_cache()
    }
}

/// Create a minimal error [`LogData`] from a [`NoAllocEntry`] and error description.
///
/// This matches the old `build_log` behavior where unresolvable entries still produce
/// a `LogData` with the error as the `message` field.
fn error_to_log_data(entry: &NoAllocEntry, error: &ResolveError) -> LogData {
    LogData {
        subsystem: empty_rc_string(),
        thread_id: entry.thread_id,
        pid: entry.pid,
        euid: entry.euid,
        library: empty_rc_string(),
        library_uuid: Uuid::nil(),
        activity_id: 0,
        time: entry.timestamp,
        timestamp: unixepoch_to_datetime(entry.timestamp as i64),
        category: empty_rc_string(),
        log_type: entry.log_type_enum(),
        event_type: entry.event_type_enum(),
        process: empty_rc_string(),
        process_uuid: Uuid::nil(),
        message: rc_string!(format!("{error}")),
        raw_message: empty_rc_string(),
        boot_uuid: entry.boot_uuid,
        timezone_name: empty_rc_string(),
        message_entries: Vec::new(),
    }
}

impl Iterator for LogDataIterator<'_> {
    type Item = LogIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = self.stream.next_entry()?;
            match self.stream.resolve(&entry, self.provider) {
                Ok(log_data) => {
                    if self.exclude_missing && log_data.message.contains("<Missing message data>") {
                        continue; // skip, may find strings in later files
                    }
                    return Some(LogIteratorItem::Log(log_data));
                }
                Err(e) => {
                    if self.exclude_missing {
                        continue; // skip unresolvable
                    }
                    // Produce a minimal LogData with the error as message,
                    // matching old build_log behavior (never drops entries)
                    return Some(LogIteratorItem::Log(error_to_log_data(&entry, &e)));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filesystem::LogarchiveProvider;
    use crate::parser::collect_timesync;
    use crate::unified_log::{EventType, LogType};
    use std::{fs, path::PathBuf};

    fn test_path() -> PathBuf {
        let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        p.push("tests/test_data/system_logs_big_sur.logarchive");
        p
    }

    fn persist_file() -> Vec<u8> {
        let mut p = test_path();
        p.push("Persist/0000000000000002.tracev3");
        fs::read(p).unwrap()
    }

    #[test]
    fn test_log_data_iterator_count() {
        let archive_path = test_path();
        let mut provider = LogarchiveProvider::new(archive_path.as_path());
        let timesync_data = collect_timesync(&provider).unwrap();
        let buf = persist_file();

        let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);

        let mut total = 0;
        for item in iter {
            if let LogIteratorItem::Log(_) = item {
                total += 1;
            }
        }

        assert_eq!(total, 207_366);
    }

    #[test]
    fn test_log_data_iterator_entry_values() {
        let archive_path = test_path();
        let mut provider = LogarchiveProvider::new(archive_path.as_path());
        let timesync_data = collect_timesync(&provider).unwrap();
        let buf = persist_file();

        let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);

        let entries: Vec<LogData> = iter
            .filter_map(|item| match item {
                LogIteratorItem::Log(entry) => Some(entry),
                _ => None,
            })
            .take(11)
            .collect();

        assert!(entries.len() == 11);
        let entry = &entries[10];
        assert_eq!(entry.process.as_str(), "/usr/libexec/lightsoutmanagementd");
        assert_eq!(entry.subsystem.as_str(), "com.apple.lom");
        assert_eq!(entry.time, 1642302327364384800.0);
        assert_eq!(entry.activity_id, 0);
        assert_eq!(
            entry.library.as_str(),
            "/System/Library/PrivateFrameworks/AppleLOM.framework/Versions/A/AppleLOM"
        );
        assert_eq!(entry.message.as_str(), "<private> LOM isSupported : No");
        assert_eq!(entry.pid, 45);
        assert_eq!(entry.thread_id, 588);
        assert_eq!(entry.category.as_str(), "device");
        assert_eq!(entry.log_type, LogType::Default);
        assert_eq!(entry.event_type, EventType::Log);
        assert_eq!(entry.euid, 0);
        assert_eq!(
            entry.boot_uuid,
            Uuid::parse_str("80D194AF56A34C54867449D2130D41BB").unwrap()
        );
        assert_eq!(entry.timezone_name.as_str(), "Pacific");
        assert_eq!(
            entry.library_uuid,
            Uuid::parse_str("D8E5AF1CAF4F3CEB8731E6F240E8EA7D").unwrap()
        );
        assert_eq!(
            entry.process_uuid,
            Uuid::parse_str("6C3ADF991F033C1C96C4ADFAA12D8CED").unwrap()
        );
        assert_eq!(entry.raw_message.as_str(), "%@ LOM isSupported : %s");
    }

    #[test]
    fn test_oversize_cache_roundtrip() {
        let archive_path = test_path();
        let mut provider = LogarchiveProvider::new(archive_path.as_path());
        let timesync_data = collect_timesync(&provider).unwrap();
        let buf = persist_file();

        let mut iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);

        // Exhaust the iterator, then retrieve the cache
        while iter.next().is_some() {}
        let cache = iter.into_oversize_cache();

        // The Big Sur persist file contains oversize entries
        assert!(!cache.is_empty());
    }

    #[test]
    fn test_empty_input() {
        let archive_path = test_path();
        let mut provider = LogarchiveProvider::new(archive_path.as_path());
        let timesync_data = collect_timesync(&provider).unwrap();

        let iter = LogDataIterator::new(Vec::new(), &mut provider, &timesync_data, true);

        let count = iter.count();
        assert_eq!(count, 0);
    }
}
