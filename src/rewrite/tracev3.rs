//! `TraceV3` file processor — threads all parsing modules together to produce log entries.

use std::collections::HashMap;

use log::warn;
use uuid::Uuid;

use super::catalog::RawCatalogChunk;
use super::chunk::{ChunksReader, TopChunk};
use super::chunks::ChunkTag;
use super::chunkset::firehose::RawFirehose;
use super::chunkset::firehose::body::{RawFirehoseBody, RawFormatterFlags};
use super::chunkset::firehose::entry::FirehoseLogType;
use super::chunkset::oversize::RawOversize;
use super::dsc::RawSharedCacheStrings;
use super::error::{NomExt, ParseError};
use super::header::RawHeaderChunk;
use super::log_entry::{EventType, ItemsData, LogEntry, LogType};
use super::resolve::resolve_strings;
use super::timesync::TimestampResolver;
use super::uuidtext::RawUUIDText;

// ---------------------------------------------------------------------------
// OversizeCache
// ---------------------------------------------------------------------------

/// Cache for oversize log entries, threaded across chunksets and tracev3 files.
///
/// Oversize entries carry strings too large for regular firehose entries.
/// They must be cached and looked up when a firehose entry references them
/// via `data_ref`.
#[derive(Debug, Default)]
pub struct OversizeCache {
  entries: HashMap<(u32, u64, u32), Vec<u8>>,
}

impl OversizeCache {
  pub fn new() -> Self {
    Self::default()
  }

  fn insert(&mut self, oversize: &RawOversize<'_>) {
    self.entries.insert(
      (oversize.data_ref_index, oversize.first_proc_id, oversize.second_proc_id),
      oversize.oversize_data.to_vec(),
    );
  }

  fn get(&self, data_ref: u32, first_proc_id: u64, second_proc_id: u32) -> Option<&[u8]> {
    self
      .entries
      .get(&(data_ref, first_proc_id, second_proc_id))
      .map(|v| v.as_slice())
  }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Process a single tracev3 file buffer, emitting `LogEntry` via callback.
///
/// The callback receives each log entry as it is produced. Entry-level errors
/// (bad body parse, missing oversize data) are logged as warnings and skipped.
#[allow(clippy::too_many_arguments)]
pub fn visit_tracev3<'a>(
  data: &'a [u8],
  resolver: &TimestampResolver,
  dsc_files: &'a HashMap<Uuid, RawSharedCacheStrings<'a>>,
  uuidtext_files: &'a HashMap<Uuid, RawUUIDText<'a>>,
  oversize_cache: &mut OversizeCache,
  mut callback: impl FnMut(LogEntry<'a>),
) -> Result<(), ParseError> {
  let mut current_header: Option<RawHeaderChunk<'a>> = None;
  let mut current_catalog: Option<RawCatalogChunk<'a>> = None;

  for top_chunk in ChunksReader::new(data) {
    match top_chunk? {
      TopChunk::Header(h) => current_header = Some(h),
      TopChunk::Catalog(c) => current_catalog = Some(c),
      TopChunk::Chunkset(mut reader) => {
        while let Some(inner) = reader.next() {
          let inner = inner?;
          match inner.preamble.tag {
            ChunkTag::Oversize => match RawOversize::parse(inner.data) {
              Ok((_, ov)) => oversize_cache.insert(&ov),
              Err(e) => {
                warn!("Failed to parse oversize chunk: {}", e.to_parse_error());
              }
            },
            ChunkTag::Firehose => {
              let fh = match RawFirehose::parse(inner.data) {
                Ok((_, fh)) => fh,
                Err(e) => {
                  warn!("Failed to parse firehose chunk: {}", e.to_parse_error());
                  continue;
                }
              };

              let Some(header) = &current_header else {
                continue;
              };
              let Some(catalog) = &current_catalog else {
                continue;
              };

              process_firehose_entries(
                &fh,
                header,
                catalog,
                resolver,
                dsc_files,
                uuidtext_files,
                oversize_cache,
                &mut callback,
              );
            }
            _ => {} // skip simpledump/statedump for now
          }
        }
      }
      TopChunk::Unknown(_) => {}
    }
  }

  Ok(())
}

// ---------------------------------------------------------------------------
// Per-entry processing
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn process_firehose_entries<'a>(
  fh: &RawFirehose<'_>,
  header: &RawHeaderChunk<'a>,
  catalog: &RawCatalogChunk<'a>,
  resolver: &TimestampResolver,
  dsc_files: &'a HashMap<Uuid, RawSharedCacheStrings<'a>>,
  uuidtext_files: &'a HashMap<Uuid, RawUUIDText<'a>>,
  oversize_cache: &OversizeCache,
  callback: &mut impl FnMut(LogEntry<'a>),
) {
  let boot_uuid = header.boot_uuid;
  let timezone_name = extract_timezone_name(header.timezone_path);

  for entry in fh.entries() {
    let body = match entry.parse_body() {
      Ok(body) => body,
      Err(e) => {
        warn!("Failed to parse firehose entry body: {e}");
        continue;
      }
    };

    // Extract body-specific fields
    let (event_type, log_type, activity_id, subsystem_value, data_ref, pc_id, formatter) = match &body {
      RawFirehoseBody::Activity(b) => (
        EventType::Activity,
        map_activity_log_type(entry.log_type),
        combine_activity_id(b.activity_id),
        None,
        None,
        b.pc_id,
        b.formatter,
      ),
      RawFirehoseBody::NonActivity(b) => (
        EventType::Log,
        map_default_log_type(entry.log_type),
        combine_activity_id(b.activity_id),
        b.subsystem,
        b.data_ref,
        b.pc_id,
        b.formatter,
      ),
      RawFirehoseBody::Signpost(b) => (
        EventType::Signpost,
        map_signpost_log_type(entry.log_type),
        combine_activity_id(b.activity_id),
        b.subsystem,
        b.data_ref,
        b.pc_id,
        b.formatter,
      ),
      RawFirehoseBody::Trace(b) => (
        EventType::Trace,
        LogType::Default,
        0,
        None,
        None,
        b.pc_id,
        RawFormatterFlags::default(),
      ),
      RawFirehoseBody::Loss(b) => {
        let abs_ct = entry.absolute_continuous_time(fh.base_continuous_time);
        let time = resolver.resolve(&boot_uuid, abs_ct, fh.base_continuous_time);
        callback(LogEntry {
          subsystem: "",
          category: "",
          thread_id: entry.thread_id,
          pid: 0,
          euid: 0,
          library: "",
          library_uuid: Uuid::nil(),
          activity_id: 0,
          time,
          event_type: EventType::Loss,
          log_type: LogType::Loss,
          process: "",
          process_uuid: Uuid::nil(),
          format_string: None,
          boot_uuid,
          timezone_name,
          items: ItemsData::Loss {
            count: b.count,
            start_time: b.start_time,
            end_time: b.end_time,
          },
        });
        continue;
      }
      RawFirehoseBody::Unknown(_) => continue,
    };

    // Timestamp
    let abs_ct = entry.absolute_continuous_time(fh.base_continuous_time);
    let time = resolver.resolve(&boot_uuid, abs_ct, fh.base_continuous_time);

    // Resolve strings (format string, library, process paths)
    let resolved = resolve_strings(
      entry.format_string_location,
      pc_id,
      &formatter,
      fh.first_proc_id,
      fh.second_proc_id,
      catalog,
      dsc_files,
      uuidtext_files,
    );

    // Build deferred items data — message formatted on demand via LogEntry::message()
    // All variants clone raw bytes into Vec<u8> because the mutable ChunkSetReader
    // iterator prevents zero-copy borrows. Items data is small (typically 10–100 bytes).
    let items = if let Some(data_ref) = data_ref {
      match oversize_cache.get(data_ref, fh.first_proc_id, fh.second_proc_id) {
        Some(d) => ItemsData::Regular {
          data: d.to_vec(),
          flags: entry.flags,
        },
        None => {
          warn!(
            "Missing oversize data for data_ref={data_ref}, \
             proc=({}, {})",
            fh.first_proc_id, fh.second_proc_id
          );
          ItemsData::None
        }
      }
    } else {
      match &body {
        RawFirehoseBody::Trace(t) => ItemsData::Trace {
          data: t.items_data.to_vec(),
        },
        _ => match body.standard_items_data() {
          Some(d) => ItemsData::Regular {
            data: d.to_vec(),
            flags: entry.flags,
          },
          None => ItemsData::None,
        },
      }
    };

    // Catalog lookups
    let (subsystem, category) = subsystem_value
      .and_then(|sv| catalog.get_subsystem(sv, fh.first_proc_id, fh.second_proc_id))
      .map_or(("", ""), |s| (s.subsystem, s.category));
    let pid = catalog.get_pid(fh.first_proc_id, fh.second_proc_id).unwrap_or(0);
    let euid = catalog.get_euid(fh.first_proc_id, fh.second_proc_id).unwrap_or(0);

    callback(LogEntry {
      subsystem,
      category,
      thread_id: entry.thread_id,
      pid,
      euid,
      library: resolved.library.unwrap_or(""),
      library_uuid: resolved.library_uuid,
      activity_id,
      time,
      event_type,
      log_type,
      process: resolved.process.unwrap_or(""),
      process_uuid: resolved.process_uuid,
      format_string: resolved.format_string,
      boot_uuid,
      timezone_name,
      items,
    });
  }
}

// ---------------------------------------------------------------------------
// Mapping helpers
// ---------------------------------------------------------------------------

fn map_activity_log_type(log_type: FirehoseLogType) -> LogType {
  match log_type {
    FirehoseLogType::Info => LogType::Create,
    FirehoseLogType::Useraction => LogType::Useraction,
    _ => LogType::Default,
  }
}

fn map_default_log_type(log_type: FirehoseLogType) -> LogType {
  match log_type {
    FirehoseLogType::Debug => LogType::Debug,
    FirehoseLogType::Info => LogType::Info,
    FirehoseLogType::Error => LogType::Error,
    FirehoseLogType::Fault => LogType::Fault,
    _ => LogType::Default,
  }
}

fn map_signpost_log_type(log_type: FirehoseLogType) -> LogType {
  match log_type {
    FirehoseLogType::ProcessSignpostEvent => LogType::ProcessSignpostEvent,
    FirehoseLogType::ProcessSignpostStart => LogType::ProcessSignpostStart,
    FirehoseLogType::ProcessSignpostEnd => LogType::ProcessSignpostEnd,
    FirehoseLogType::SystemSignpostEvent => LogType::SystemSignpostEvent,
    FirehoseLogType::SystemSignpostStart => LogType::SystemSignpostStart,
    FirehoseLogType::SystemSignpostEnd => LogType::SystemSignpostEnd,
    FirehoseLogType::ThreadSignpostEvent => LogType::ThreadSignpostEvent,
    FirehoseLogType::ThreadSignpostStart => LogType::ThreadSignpostStart,
    FirehoseLogType::ThreadSignpostEnd => LogType::ThreadSignpostEnd,
    _ => LogType::Default,
  }
}

fn combine_activity_id(ids: Option<(u32, u32)>) -> u64 {
  match ids {
    Some((lo, hi)) => u64::from(lo) | (u64::from(hi) << 32),
    None => 0,
  }
}

fn extract_timezone_name(timezone_path: &str) -> &str {
  timezone_path.rsplit('/').next().unwrap_or(timezone_path)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
  use super::*;

  // --- map_log_type tests ---

  #[test]
  fn test_map_activity_log_type_info_is_create() {
    assert_eq!(map_activity_log_type(FirehoseLogType::Info), LogType::Create);
  }

  #[test]
  fn test_map_activity_log_type_useraction() {
    assert_eq!(map_activity_log_type(FirehoseLogType::Useraction), LogType::Useraction);
  }

  #[test]
  fn test_map_activity_log_type_default_fallback() {
    assert_eq!(map_activity_log_type(FirehoseLogType::Debug), LogType::Default);
    assert_eq!(map_activity_log_type(FirehoseLogType::Error), LogType::Default);
    assert_eq!(map_activity_log_type(FirehoseLogType::Default), LogType::Default);
  }

  #[test]
  fn test_map_default_log_type_all() {
    assert_eq!(map_default_log_type(FirehoseLogType::Debug), LogType::Debug);
    assert_eq!(map_default_log_type(FirehoseLogType::Info), LogType::Info);
    assert_eq!(map_default_log_type(FirehoseLogType::Error), LogType::Error);
    assert_eq!(map_default_log_type(FirehoseLogType::Fault), LogType::Fault);
    assert_eq!(map_default_log_type(FirehoseLogType::Default), LogType::Default);
  }

  #[test]
  fn test_map_signpost_log_type_all_variants() {
    assert_eq!(
      map_signpost_log_type(FirehoseLogType::ProcessSignpostEvent),
      LogType::ProcessSignpostEvent
    );
    assert_eq!(
      map_signpost_log_type(FirehoseLogType::ProcessSignpostStart),
      LogType::ProcessSignpostStart
    );
    assert_eq!(
      map_signpost_log_type(FirehoseLogType::ProcessSignpostEnd),
      LogType::ProcessSignpostEnd
    );
    assert_eq!(
      map_signpost_log_type(FirehoseLogType::SystemSignpostEvent),
      LogType::SystemSignpostEvent
    );
    assert_eq!(
      map_signpost_log_type(FirehoseLogType::SystemSignpostStart),
      LogType::SystemSignpostStart
    );
    assert_eq!(
      map_signpost_log_type(FirehoseLogType::SystemSignpostEnd),
      LogType::SystemSignpostEnd
    );
    assert_eq!(
      map_signpost_log_type(FirehoseLogType::ThreadSignpostEvent),
      LogType::ThreadSignpostEvent
    );
    assert_eq!(
      map_signpost_log_type(FirehoseLogType::ThreadSignpostStart),
      LogType::ThreadSignpostStart
    );
    assert_eq!(
      map_signpost_log_type(FirehoseLogType::ThreadSignpostEnd),
      LogType::ThreadSignpostEnd
    );
    assert_eq!(map_signpost_log_type(FirehoseLogType::Default), LogType::Default);
  }

  // --- combine_activity_id tests ---

  #[test]
  fn test_combine_activity_id_none() {
    assert_eq!(combine_activity_id(None), 0);
  }

  #[test]
  fn test_combine_activity_id_some() {
    assert_eq!(combine_activity_id(Some((0xDEAD, 0xBEEF))), 0xBEEF_0000_DEAD);
  }

  #[test]
  fn test_combine_activity_id_zero() {
    assert_eq!(combine_activity_id(Some((0, 0))), 0);
  }

  // --- extract_timezone_name tests ---

  #[test]
  fn test_extract_timezone_name_full_path() {
    assert_eq!(extract_timezone_name("/var/db/timezone/zoneinfo/America/New_York"), "New_York");
  }

  #[test]
  fn test_extract_timezone_name_short_path() {
    assert_eq!(extract_timezone_name("/usr/share/zoneinfo/Pacific"), "Pacific");
  }

  #[test]
  fn test_extract_timezone_name_no_slash() {
    assert_eq!(extract_timezone_name("UTC"), "UTC");
  }

  #[test]
  fn test_extract_timezone_name_empty() {
    assert_eq!(extract_timezone_name(""), "");
  }

  // --- OversizeCache tests ---

  #[test]
  fn test_oversize_cache_insert_and_get() {
    let mut cache = OversizeCache::new();
    cache.entries.insert((1, 100, 200), vec![1, 2, 3, 4]);
    assert_eq!(cache.get(1, 100, 200), Some(&[1, 2, 3, 4][..]));
  }

  #[test]
  fn test_oversize_cache_miss() {
    let cache = OversizeCache::new();
    assert_eq!(cache.get(1, 100, 200), None);
  }

  #[test]
  fn test_oversize_cache_different_key() {
    let mut cache = OversizeCache::new();
    cache.entries.insert((1, 100, 200), vec![1, 2, 3]);
    // Different data_ref
    assert_eq!(cache.get(2, 100, 200), None);
    // Different first_proc_id
    assert_eq!(cache.get(1, 101, 200), None);
    // Different second_proc_id
    assert_eq!(cache.get(1, 100, 201), None);
  }

}
