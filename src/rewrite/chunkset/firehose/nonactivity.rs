use nom::number::complete::{le_u8, le_u16, le_u32};

use super::flags::{FLAG_HAS_CURRENT_AID, FLAG_HAS_OVERSIZE, FLAG_HAS_PRIVATE_DATA, FLAG_HAS_RULES, FLAG_HAS_SUBSYSTEM, RawFormatterFlags};

/// Parsed Non-Activity entry body.
#[derive(Debug, Clone, Copy)]
pub struct RawNonActivityBody<'a> {
  /// Activity ID — present if `FLAG_HAS_CURRENT_AID` (0x0001).
  pub activity_id: Option<(u32, u32)>,
  /// Private string (offset, size) — present if `FLAG_HAS_PRIVATE_DATA` (0x0100).
  pub private_strings: Option<(u16, u16)>,
  pub pc_id: u32,
  pub formatter: RawFormatterFlags,
  /// Subsystem — present if `FLAG_HAS_SUBSYSTEM` (0x0200), after formatter.
  pub subsystem: Option<u16>,
  /// TTL — present if `FLAG_HAS_RULES` (0x0400).
  pub ttl: Option<u8>,
  /// Oversize data reference — present if `FLAG_HAS_OVERSIZE` (0x0800).
  pub data_ref: Option<u32>,
  pub items_data: &'a [u8],
}

impl<'a> RawNonActivityBody<'a> {
  /// Parse a Non-Activity entry body from raw entry data.
  pub fn parse(data: &'a [u8], flags: u16) -> nom::IResult<&'a [u8], Self> {
    let mut input = data;

    let activity_id = if (flags & FLAG_HAS_CURRENT_AID) != 0 {
      let (i, id) = le_u32(input)?;
      let (i, sentinel) = le_u32(i)?;
      input = i;
      Some((id, sentinel))
    } else {
      None
    };

    let private_strings = if (flags & FLAG_HAS_PRIVATE_DATA) != 0 {
      let (i, offset) = le_u16(input)?;
      let (i, size) = le_u16(i)?;
      input = i;
      Some((offset, size))
    } else {
      None
    };

    let (input, pc_id) = le_u32(input)?;
    let (mut input, formatter) = RawFormatterFlags::parse(input, flags)?;

    let subsystem = if (flags & FLAG_HAS_SUBSYSTEM) != 0 {
      let (i, val) = le_u16(input)?;
      input = i;
      Some(val)
    } else {
      None
    };

    let ttl = if (flags & FLAG_HAS_RULES) != 0 {
      let (i, val) = le_u8(input)?;
      input = i;
      Some(val)
    } else {
      None
    };

    let data_ref = if (flags & FLAG_HAS_OVERSIZE) != 0 {
      let (i, val) = le_u32(input)?;
      input = i;
      Some(val)
    } else {
      None
    };

    Ok((
      &[],
      Self {
        activity_id,
        private_strings,
        pc_id,
        formatter,
        subsystem,
        ttl,
        data_ref,
        items_data: input,
      },
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::super::body::RawFirehoseBody;
  use super::super::entry::{FirehoseActivityType, FirehoseLogType};

  #[test]
  fn test_non_activity_body() {
    // From src/chunks/firehose/nonactivity.rs test_parse_non_activity
    let test_data: &[u8] = &[
      122, 179, 12, 13, 2, 0, 4, 0, 41, 0, 34, 9, 32, 4, 0, 0, 1, 0, 32, 4, 1, 0, 1, 0, 32, 4, 2, 0, 14, 0, 0, 8, 2, 0, 0, 0, 0, 0, 0, 0,
      0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 2, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 4, 1, 0, 0, 0, 0, 4, 1, 0, 0, 0, 0, 0, 100, 105,
      115, 112, 97, 116, 99, 104, 69, 118, 101, 110, 116, 0,
    ];
    let flags: u16 = 556;

    let body = RawFirehoseBody::parse(test_data, FirehoseActivityType::NonActivity, flags, FirehoseLogType::Default).unwrap();
    let na = match body {
      RawFirehoseBody::NonActivity(na) => na,
      other => panic!("expected NonActivity, got {other:?}"),
    };

    assert_eq!(na.activity_id, None);
    assert_eq!(na.private_strings, None);
    assert_eq!(na.pc_id, 218936186);
    assert_eq!(na.formatter.has_large_offset, 2);
    assert_eq!(na.formatter.large_shared_cache, 4);
    assert!(!na.formatter.main_exe);
    assert!(!na.formatter.shared_cache);
    assert!(!na.formatter.absolute);
    assert_eq!(na.formatter.alt_index, 0);
    assert_eq!(na.formatter.uuid_relative, [0; 16]);
    assert_eq!(na.subsystem, Some(41));
    assert_eq!(na.ttl, None);
    assert_eq!(na.data_ref, None);
    // 94 total bytes - 4 (pc_id) - 4 (formatter) - 2 (subsystem) = 84 items bytes
    assert_eq!(na.items_data.len(), 84);
  }
}
