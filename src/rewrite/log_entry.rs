//! Output types for the rewrite pipeline.
//!
//! `LogEntry<'a, 'b>` is the zero-copy replacement for `LogData` + `RcString`.
//! All fields borrow from source data buffers. The `message` is formatted
//! on demand via `.message()` — no heap allocation until explicitly requested.

use base64::Engine;
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
/// Borrows raw item bytes with lifetime `'b` from the tracev3 chunk data
/// or oversize cache — zero-copy. The `'b` lifetime is scoped to a single
/// iteration of the chunkset reader, which outlives the callback invocation.
#[derive(Debug)]
pub(crate) enum ItemsData<'b> {
  /// Activity/NonActivity/Signpost: raw item bytes.
  Regular { data: &'b [u8], flags: FirehoseFlags },
  /// Trace: raw item bytes (parsed differently — reversed big-endian).
  Trace { data: &'b [u8] },
  /// Loss entry: formatted lazily from count + time range.
  Loss { count: u64, start_time: u64, end_time: u64 },
  /// SimpleDump: message is pre-formatted in the chunk data.
  Simpledump { subsystem: &'b str, message: &'b str },
  /// StateDump: raw data to be decoded based on data_type.
  Statedump {
    title_name: &'b str,
    decoder_library: &'b str,
    decoder_type: &'b str,
    statedump_data: &'b [u8],
    data_type: u32,
  },
  /// No items (Unknown, or genuinely empty).
  None,
}

/// Zero-copy log entry — borrows strings from source data buffers.
///
/// All `&'a str` fields borrow from the tracev3 file buffer, DSC files,
/// or `UUIDText` files passed to [`super::tracev3::process_tracev3`].
/// The `'b` lifetime covers raw item bytes borrowed from chunk data or
/// the oversize cache — scoped to a single chunkset iteration.
///
/// The log message is **not** eagerly formatted. Call `.message()` to format
/// the message string on demand. This is the only allocation point.
#[derive(Debug)]
pub struct LogEntry<'a, 'b> {
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
  pub(crate) items: ItemsData<'b>,
}

impl<'a, 'b> LogEntry<'a, 'b> {
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
      ItemsData::Simpledump { message, .. } => message.to_string(),
      ItemsData::Statedump {
        title_name,
        decoder_library,
        decoder_type,
        statedump_data,
        data_type,
      } => {
        let data_string = format_statedump_data(*data_type, statedump_data, title_name);
        format!(
          "title: {title_name}\nObject Type: {decoder_library}\nObject Type: {decoder_type}\n{data_string}"
        )
      }
      ItemsData::None => format_message(self.format_string, &[], decoder),
    }
  }

  /// Compute wall-clock timestamp on demand from `time` (nanoseconds since UNIX epoch).
  pub fn timestamp(&self) -> DateTime<Utc> {
    DateTime::from_timestamp_nanos(self.time as i64)
  }
}

// Statedump data type constants (matches src/constants.rs)
const STATEDUMP_DATA_PLIST: u32 = 1;
const STATEDUMP_DATA_PROTOBUF: u32 = 2;
const STATEDUMP_DATA_OBJECT: u32 = 3;

/// Format statedump data based on its type (plist, protobuf, custom object, or raw string).
fn format_statedump_data(data_type: u32, data: &[u8], title_name: &str) -> String {
  match data_type {
    STATEDUMP_DATA_PLIST => {
      if data.is_empty() {
        return String::from("Empty plist data");
      }
      match plist::from_bytes::<plist::Value>(data) {
        Ok(value) => serde_json::to_string(&value)
          .unwrap_or_else(|_| String::from("Failed to convert plist data to json")),
        Err(_) => String::from("Failed to get plist data"),
      }
    }
    STATEDUMP_DATA_PROTOBUF => match sunlight::light::extract_protobuf(data) {
      Ok(map) => serde_json::to_string(&map)
        .unwrap_or_else(|_| String::from("Failed to serialize Protobuf HashMap")),
      Err(_) => format!(
        "Failed to parse StateDump protobuf: {}",
        base64::engine::general_purpose::STANDARD.encode(data)
      ),
    },
    STATEDUMP_DATA_OBJECT => {
      // Custom object decoding (location, DNS, network config) not yet ported to rewrite.
      format!("Unsupported custom statedump object: {title_name}")
    }
    _ => std::str::from_utf8(data)
      .map(|s| s.to_string())
      .unwrap_or_else(|_| String::from("Failed to extract statedump data")),
  }
}

impl Serialize for LogEntry<'_, '_> {
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
