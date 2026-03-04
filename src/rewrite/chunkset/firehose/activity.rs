use nom::number::complete::{le_u32, le_u64};

use super::entry::FirehoseLogType;
use super::flags::{FLAG_HAS_CURRENT_AID, FLAG_HAS_SUBSYSTEM, FLAG_HAS_UNIQUE_PID, RawFormatterFlags};
use super::item::{RawFirehoseItemData, parse_items_data};

/// Parsed Activity entry body.
#[derive(Debug, Clone, Copy)]
pub struct RawActivityBody<'a> {
  /// Activity ID + sentinel (absent for `Useraction` `log_type`).
  pub activity_id: Option<(u32, u32)>,
  /// Unique PID — present if `FLAG_HAS_UNIQUE_PID` (0x0010).
  pub pid: Option<u64>,
  /// Current activity ID — present if `FLAG_HAS_CURRENT_AID` (0x0001).
  pub current_aid: Option<(u32, u32)>,
  /// Other activity ID — present if `FLAG_HAS_SUBSYSTEM` (0x0200), reinterpreted for Activity.
  pub other_aid: Option<(u32, u32)>,
  pub pc_id: u32,
  pub formatter: RawFormatterFlags,
  pub items_data: &'a [u8],
}

impl<'a> RawActivityBody<'a> {
  /// Parse an Activity entry body from raw entry data.
  /// Parse items from this activity body's `items_data`.
  pub fn parse_items(&self, flags: u16) -> RawFirehoseItemData<'a> {
    parse_items_data(self.items_data, flags)
      .map(|(_, data)| data)
      .unwrap_or_else(|_| RawFirehoseItemData {
        unknown_item: 0,
        items: Vec::new(),
        backtrace_data: None,
      })
  }

  pub fn parse(data: &'a [u8], flags: u16, log_type: FirehoseLogType) -> nom::IResult<&'a [u8], Self> {
    let mut input = data;

    // Useraction activity type does not have the first Activity ID or sentinel
    let activity_id = if log_type != FirehoseLogType::Useraction {
      let (i, id) = le_u32(input)?;
      let (i, sentinel) = le_u32(i)?;
      input = i;
      Some((id, sentinel))
    } else {
      None
    };

    let pid = if (flags & FLAG_HAS_UNIQUE_PID) != 0 {
      let (i, val) = le_u64(input)?;
      input = i;
      Some(val)
    } else {
      None
    };

    let current_aid = if (flags & FLAG_HAS_CURRENT_AID) != 0 {
      let (i, id) = le_u32(input)?;
      let (i, sentinel) = le_u32(i)?;
      input = i;
      Some((id, sentinel))
    } else {
      None
    };

    // In Activity entries, FLAG_HAS_SUBSYSTEM means "has other activity ID"
    let other_aid = if (flags & FLAG_HAS_SUBSYSTEM) != 0 {
      let (i, id) = le_u32(input)?;
      let (i, sentinel) = le_u32(i)?;
      input = i;
      Some((id, sentinel))
    } else {
      None
    };

    let (input, pc_id) = le_u32(input)?;
    let (items_data, formatter) = RawFormatterFlags::parse(input, flags)?;

    Ok((
      &[],
      Self {
        activity_id,
        pid,
        current_aid,
        other_aid,
        pc_id,
        formatter,
        items_data,
      },
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::super::body::RawFirehoseBody;
  use super::super::entry::FirehoseActivityType;
  use super::*;

  #[test]
  fn test_activity_body() {
    // From src/chunks/firehose/activity.rs test_parse_activity
    let test_data: &[u8] = &[
      178, 251, 0, 0, 0, 0, 0, 128, 236, 0, 0, 0, 0, 0, 0, 0, 178, 251, 0, 0, 0, 0, 0, 128, 179, 251, 0, 0, 0, 0, 0, 128, 64, 63, 24, 18,
      1, 0, 2, 0,
    ];
    let flags: u16 = 573;
    let log_type = FirehoseLogType::Info;

    let body = RawFirehoseBody::parse(test_data, FirehoseActivityType::Activity, flags, log_type).unwrap();
    let activity = match body {
      RawFirehoseBody::Activity(a) => a,
      other => panic!("expected Activity, got {other:?}"),
    };

    assert_eq!(activity.activity_id, Some((64434, 0x80000000)));
    assert_eq!(activity.pid, Some(236));
    assert_eq!(activity.current_aid, Some((64434, 0x80000000)));
    assert_eq!(activity.other_aid, Some((64435, 0x80000000)));
    assert_eq!(activity.pc_id, 303578944);
    assert_eq!(activity.formatter.has_large_offset, 1);
    assert_eq!(activity.formatter.large_shared_cache, 2);
    assert!(!activity.formatter.main_exe);
    assert!(!activity.formatter.shared_cache);
    assert!(!activity.formatter.absolute);
    assert_eq!(activity.formatter.alt_index, 0);
    assert_eq!(activity.formatter.uuid_relative, [0; 16]);
    assert!(activity.items_data.is_empty());
  }

  #[test]
  fn test_activity_parse_items() {
    let test_data: &[u8] = &[
      178, 251, 0, 0, 0, 0, 0, 128, 236, 0, 0, 0, 0, 0, 0, 0, 178, 251, 0, 0, 0, 0, 0, 128, 179, 251, 0, 0, 0, 0, 0, 128, 64, 63, 24, 18,
      1, 0, 2, 0,
    ];
    let flags: u16 = 573;
    let log_type = FirehoseLogType::Info;

    let body = RawFirehoseBody::parse(test_data, FirehoseActivityType::Activity, flags, log_type).unwrap();
    let activity = match body {
      RawFirehoseBody::Activity(a) => a,
      other => panic!("expected Activity, got {other:?}"),
    };

    let result = activity.parse_items(flags);
    assert_eq!(result.items.len(), 0);
  }
}
