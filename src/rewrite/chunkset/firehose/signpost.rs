use nom::bytes::complete::take;
use nom::number::complete::{le_u8, le_u16, le_u32, le_u64};

use super::flags::{FirehoseFlags, RawFormatterFlags};

/// Parsed Signpost entry body.
#[derive(Debug, Clone, Copy)]
pub struct RawSignpostBody<'a> {
  /// Activity ID — present if `HAS_CURRENT_AID` (0x0001).
  pub activity_id: Option<(u32, u32)>,
  /// Private string (offset, size) — present if `HAS_PRIVATE_DATA` (0x0100).
  pub private_strings: Option<(u16, u16)>,
  pub pc_id: u32,
  pub formatter: RawFormatterFlags,
  /// Subsystem — present if `HAS_SUBSYSTEM` (0x0200).
  pub subsystem: Option<u16>,
  /// Always present in signpost entries.
  pub signpost_id: u64,
  /// TTL — present if `HAS_RULES` (0x0400).
  pub ttl: Option<u8>,
  /// Oversize data reference — present if `HAS_OVERSIZE` (0x0800).
  pub data_ref: Option<u32>,
  /// Signpost name — present if `HAS_NAME` (0x8000).
  pub signpost_name: Option<u32>,
  pub items_data: &'a [u8],
}

impl<'a> RawSignpostBody<'a> {
  /// Parse a Signpost entry body from raw entry data.
  pub fn parse(data: &'a [u8], flags: FirehoseFlags) -> nom::IResult<&'a [u8], Self> {
    let mut input = data;

    let activity_id = if flags.contains(FirehoseFlags::HAS_CURRENT_AID) {
      let (i, id) = le_u32(input)?;
      let (i, sentinel) = le_u32(i)?;
      input = i;
      Some((id, sentinel))
    } else {
      None
    };

    let private_strings = if flags.contains(FirehoseFlags::HAS_PRIVATE_DATA) {
      let (i, offset) = le_u16(input)?;
      let (i, size) = le_u16(i)?;
      input = i;
      Some((offset, size))
    } else {
      None
    };

    let (input, pc_id) = le_u32(input)?;
    let (mut input, formatter) = RawFormatterFlags::parse(input, flags)?;

    let subsystem = if flags.contains(FirehoseFlags::HAS_SUBSYSTEM) {
      let (i, val) = le_u16(input)?;
      input = i;
      Some(val)
    } else {
      None
    };

    let (mut input, signpost_id) = le_u64(input)?;

    let ttl = if flags.contains(FirehoseFlags::HAS_RULES) {
      let (i, val) = le_u8(input)?;
      input = i;
      Some(val)
    } else {
      None
    };

    let data_ref = if flags.contains(FirehoseFlags::HAS_OVERSIZE) {
      let (i, val) = le_u32(input)?;
      input = i;
      Some(val)
    } else {
      None
    };

    let signpost_name = if flags.contains(FirehoseFlags::HAS_NAME) {
      let (i, val) = le_u32(input)?;
      input = i;
      // If the signpost has large_shared_cache flag, skip 2 extra bytes
      if formatter.large_shared_cache != 0 {
        let (i, _) = take(2_usize)(input)?;
        input = i;
      }
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
        signpost_id,
        ttl,
        data_ref,
        signpost_name,
        items_data: input,
      },
    ))
  }
}

#[cfg(test)]
mod tests {
  use super::super::body::RawFirehoseBody;
  use super::super::entry::{FirehoseActivityType, FirehoseLogType};
  use super::*;

  #[test]
  fn test_signpost_body() {
    // From src/chunks/firehose/signpost.rs test_parse_signpost
    let test_data: &[u8] = &[225, 244, 2, 0, 1, 0, 238, 238, 178, 178, 181, 176, 238, 238, 176, 63, 27, 0, 0, 0];
    let flags = FirehoseFlags::from_bits_retain(33282);

    let body = RawFirehoseBody::parse(test_data, FirehoseActivityType::Signpost, flags, FirehoseLogType::Default).unwrap();
    let sp = match body {
      RawFirehoseBody::Signpost(sp) => sp,
      other => panic!("expected Signpost, got {other:?}"),
    };

    assert_eq!(sp.activity_id, None);
    assert_eq!(sp.private_strings, None);
    assert_eq!(sp.pc_id, 193761);
    assert!(sp.formatter.main_exe);
    assert!(!sp.formatter.shared_cache);
    assert_eq!(sp.formatter.has_large_offset, 0);
    assert_eq!(sp.formatter.large_shared_cache, 0);
    assert!(!sp.formatter.absolute);
    assert_eq!(sp.subsystem, Some(1));
    assert_eq!(sp.signpost_id, 17216892719917625070);
    assert_eq!(sp.signpost_name, Some(1785776));
    assert_eq!(sp.ttl, None);
    assert_eq!(sp.data_ref, None);
    // 20 bytes - 4 (pc_id) - 0 (main_exe) - 2 (subsystem) - 8 (signpost_id) - 4 (name) = 2
    assert_eq!(sp.items_data.len(), 2);
  }
}
