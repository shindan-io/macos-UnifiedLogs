//! Output types for the rewrite pipeline.
//!
//! `LogEntry<'a>` is the zero-copy replacement for `LogData` + `RcString`.
//! All fields borrow from source data buffers. The `message` is formatted
//! on demand via `.message()` — no heap allocation until explicitly requested.

use chrono::{DateTime, Utc};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use uuid::Uuid;

use super::chunkset::firehose::flags::FirehoseFlags;
use super::chunkset::firehose::item::{parse_items_data, parse_trace_items};
use super::format::{AppleDecoder, NoDecoder, format_message};

/// Event type classification for a log entry.
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

/// Log severity/subtype for a log entry.
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

/// Raw data needed to format a message on demand.
/// Not public — callers use `LogEntry::message()`.
///
/// All variants store owned `Vec<u8>` because the mutable `ChunkSetReader`
/// iterator prevents zero-copy borrows from the tracev3 buffer. The items
/// data is small (typically 10–100 bytes), so cloning is much cheaper than
/// eagerly formatting the full message string.
#[derive(Debug)]
pub(crate) enum ItemsData {
  /// Activity/NonActivity/Signpost: raw item bytes.
  Regular { data: Vec<u8>, flags: FirehoseFlags },
  /// Trace: raw item bytes (parsed differently — reversed big-endian).
  Trace { data: Vec<u8> },
  /// Loss entry: formatted lazily from count + time range.
  Loss { count: u64, start_time: u64, end_time: u64 },
  /// No items (Unknown, or genuinely empty).
  None,
}

/// Zero-copy log entry — borrows strings from source data buffers.
///
/// All `&'a str` fields borrow from the tracev3 file buffer, DSC files,
/// or `UUIDText` files passed to [`super::tracev3::process_tracev3`].
///
/// The log message is **not** eagerly formatted. Call `.message()` to format
/// the message string on demand. This is the only allocation point.
#[derive(Debug)]
pub struct LogEntry<'a> {
  pub subsystem: Option<&'a str>,
  pub category: Option<&'a str>,
  pub thread_id: u64,
  pub pid: u64,
  pub euid: u32,
  pub library: Option<&'a str>,
  pub library_uuid: Uuid,
  pub activity_id: u64,
  pub time: f64,
  pub event_type: EventType,
  pub log_type: LogType,
  pub process: Option<&'a str>,
  pub process_uuid: Uuid,
  pub format_string: Option<&'a str>,
  pub boot_uuid: Uuid,
  pub timezone_name: &'a str,
  // Private: deferred message data
  pub(crate) items: ItemsData,
}

impl<'a> LogEntry<'a> {
  /// Format the log message on demand. This is the only allocation point.
  pub fn message(&self) -> String {
    self.message_with_decoder(&NoDecoder)
  }

  /// Format with a custom Apple decoder.
  pub fn message_with_decoder(&self, decoder: &dyn AppleDecoder) -> String {
    match &self.items {
      ItemsData::Regular { data, flags } => {
        let items = parse_items_data(data, *flags).map(|(_, d)| d.items).unwrap_or_default();
        format_message(self.format_string, &items, decoder)
      }
      ItemsData::Trace { data } => {
        let items = parse_trace_items(data);
        format_message(self.format_string, &items, decoder)
      }
      ItemsData::Loss {
        count,
        start_time,
        end_time,
      } => {
        format!("Lost {} log entries between {} and {}", count, start_time, end_time)
      }
      ItemsData::None => format_message(self.format_string, &[], decoder),
    }
  }

  /// Compute wall-clock timestamp on demand from `time` (nanoseconds since UNIX epoch).
  pub fn timestamp(&self) -> DateTime<Utc> {
    DateTime::from_timestamp_nanos(self.time as i64)
  }
}

impl Serialize for LogEntry<'_> {
  fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
    let mut state = serializer.serialize_struct("LogEntry", 17)?;
    state.serialize_field("subsystem", &self.subsystem)?;
    state.serialize_field("category", &self.category)?;
    state.serialize_field("thread_id", &self.thread_id)?;
    state.serialize_field("pid", &self.pid)?;
    state.serialize_field("euid", &self.euid)?;
    state.serialize_field("library", &self.library)?;
    state.serialize_field("library_uuid", &self.library_uuid)?;
    state.serialize_field("activity_id", &self.activity_id)?;
    state.serialize_field("time", &self.time)?;
    state.serialize_field("event_type", &self.event_type)?;
    state.serialize_field("log_type", &self.log_type)?;
    state.serialize_field("process", &self.process)?;
    state.serialize_field("process_uuid", &self.process_uuid)?;
    let message = self.message();
    state.serialize_field("message", &message)?;
    state.serialize_field("format_string", &self.format_string)?;
    state.serialize_field("boot_uuid", &self.boot_uuid)?;
    state.serialize_field("timezone_name", self.timezone_name)?;
    state.end()
  }
}
