//! Output types for the rewrite pipeline.
//!
//! `LogEntry<'a>` is the zero-copy replacement for `LogData` + `RcString`.
//! Only the `message` field is heap-allocated; all other string fields borrow
//! from source data buffers.

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

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

/// Zero-copy log entry -- borrows strings from source data buffers.
///
/// Only `message` is heap-allocated (freshly formatted per entry).
/// All `&'a str` fields borrow from the tracev3 file buffer, DSC files,
/// or `UUIDText` files passed to [`super::tracev3::process_tracev3`].
#[derive(Debug, Serialize)]
pub struct LogEntry<'a> {
  pub subsystem: &'a str,
  pub category: &'a str,
  pub thread_id: u64,
  pub pid: u64,
  pub euid: u32,
  pub library: &'a str,
  pub library_uuid: Uuid,
  pub activity_id: u64,
  pub time: f64,
  pub event_type: EventType,
  pub log_type: LogType,
  pub process: &'a str,
  pub process_uuid: Uuid,
  pub message: String,
  pub raw_message: &'a str,
  pub boot_uuid: Uuid,
  pub timezone_name: &'a str,
}

impl LogEntry<'_> {
  /// Compute wall-clock timestamp on demand from `time` (nanoseconds since UNIX epoch).
  pub fn timestamp(&self) -> DateTime<Utc> {
    DateTime::from_timestamp_nanos(self.time as i64)
  }
}
