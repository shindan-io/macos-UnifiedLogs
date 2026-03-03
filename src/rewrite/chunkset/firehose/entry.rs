use nom::number::complete::{le_u8, le_u16, le_u32, le_u64};

use super::super::super::helpers::padding_size_8;

const ENTRY_HEADER_SIZE: usize = 24;
const REMNANT_DATA: u8 = 0x0;

/// A single firehose log entry header with its raw body (zero-copy).
///
/// The 24-byte header is fully parsed; the type-specific body (`entry_data`)
/// is kept as raw `&[u8]` for later dispatch (activity / nonactivity / signpost / trace / loss).
#[derive(Debug, Clone, Copy)]
pub struct RawFirehoseEntry<'a> {
  pub log_activity_type: u8,
  pub log_type: u8,
  pub flags: u16,
  pub format_string_location: u32,
  pub thread_id: u64,
  pub continuous_time_delta: u32,
  pub continuous_time_delta_upper: u16,
  pub data_size: u16,
  pub entry_data: &'a [u8],
}

impl<'a> RawFirehoseEntry<'a> {
  fn parse(input: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
    let (input, log_activity_type) = le_u8(input)?;
    let (input, log_type) = le_u8(input)?;
    let (input, flags) = le_u16(input)?;
    let (input, format_string_location) = le_u32(input)?;
    let (input, thread_id) = le_u64(input)?;
    let (input, continuous_time_delta) = le_u32(input)?;
    let (input, continuous_time_delta_upper) = le_u16(input)?;
    let (input, data_size) = le_u16(input)?;

    let data_len = data_size as usize;
    if input.len() < data_len {
      return Err(nom::Err::Incomplete(nom::Needed::new(data_len - input.len())));
    }
    let entry_data = &input[..data_len];
    let input = &input[data_len..];

    Ok((
      input,
      RawFirehoseEntry {
        log_activity_type,
        log_type,
        flags,
        format_string_location,
        thread_id,
        continuous_time_delta,
        continuous_time_delta_upper,
        data_size,
        entry_data,
      },
    ))
  }
}

/// Iterator over individual firehose entries within the public data region.
///
/// Stops when:
/// - remaining bytes < 24 (entry header size)
/// - `log_activity_type == 0x0` (remnant/sentinel)
/// - `data_size` exceeds remaining bytes (malformed)
pub struct RawFirehoseEntryReader<'a> {
  data: &'a [u8],
}

impl<'a> RawFirehoseEntryReader<'a> {
  pub fn new(data: &'a [u8]) -> Self {
    Self { data }
  }
}

impl<'a> Iterator for RawFirehoseEntryReader<'a> {
  type Item = RawFirehoseEntry<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.data.len() < ENTRY_HEADER_SIZE {
      return None;
    }

    // Peek at log_activity_type — 0x0 means end of entries
    if self.data[0] == REMNANT_DATA {
      return None;
    }

    let entry = match RawFirehoseEntry::parse(self.data) {
      Ok((remaining, entry)) => {
        // Advance past the 8-byte alignment padding
        let padding = padding_size_8(u64::from(entry.data_size)) as usize;
        if remaining.len() >= padding {
          self.data = &remaining[padding..];
        } else {
          self.data = &[];
        }
        entry
      }
      Err(_) => return None,
    };

    Some(entry)
  }
}

#[cfg(test)]
mod tests {
  use super::super::RawFirehose;

  /// Same test data as `test_parse_raw_firehose` in mod.rs.
  /// 16-byte preamble + 32-byte header + 120 bytes of entry data (3 entries).
  const TEST_DATA: &[u8] = &[
    1, 96, 0, 0, 0, 0, 0, 0, 152, 0, 0, 0, 0, 0, 0, 0, 133, 16, 0, 0, 0, 0, 0, 0, 157, 38, 0, 0, 0, 0, 0, 0, 136, 0, 0, 16, 0, 0, 0, 2, 42,
    188, 25, 14, 104, 4, 0, 0, 2, 1, 4, 0, 240, 243, 53, 0, 176, 232, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 12, 0, 176, 249, 0, 0, 0, 0, 0,
    128, 163, 133, 51, 0, 0, 0, 0, 0, 2, 1, 4, 0, 32, 250, 53, 0, 177, 232, 0, 0, 0, 0, 0, 0, 209, 67, 85, 0, 16, 0, 12, 0, 177, 249, 0, 0,
    0, 0, 0, 128, 237, 115, 51, 0, 0, 0, 0, 0, 2, 1, 4, 0, 48, 57, 126, 0, 179, 232, 0, 0, 0, 0, 0, 0, 40, 101, 197, 1, 16, 0, 12, 0, 178,
    249, 0, 0, 0, 0, 0, 128, 105, 67, 61, 0, 0, 0, 0, 0,
  ];

  #[test]
  fn test_iterate_entries() {
    let data = &TEST_DATA[16..]; // skip preamble
    let (_, fh) = RawFirehose::parse(data).unwrap();
    let entries: Vec<_> = fh.entries().collect();

    assert_eq!(entries.len(), 3);

    for entry in &entries {
      assert_eq!(entry.log_activity_type, 0x02); // Activity
      assert_eq!(entry.log_type, 0x01);
      assert_eq!(entry.flags, 4);
      assert_eq!(entry.data_size, 12);
      assert_eq!(entry.entry_data.len(), 12);
    }

    // thread_id should differ between entries
    assert_ne!(entries[0].thread_id, entries[1].thread_id);
    assert_ne!(entries[1].thread_id, entries[2].thread_id);

    // continuous_time_delta should differ
    assert_ne!(entries[0].continuous_time_delta, entries[1].continuous_time_delta);
  }
}
