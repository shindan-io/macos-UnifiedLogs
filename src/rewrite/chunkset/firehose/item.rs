use nom::{
    bytes::complete::take,
    number::complete::{be_u16, be_u32, be_u64, be_u8, le_i16, le_i32, le_i64, le_i8, le_u16, le_u8},
};

use super::super::super::helpers::{padding_size, utf8_str};
use super::flags::FirehoseFlags;

/// Classification of the `item_type` byte into parsing categories.
///
/// Multiple raw byte values map to the same logical kind (e.g. 0x20 and 0x22 are
/// both `String`). Uses `num_enum::FromPrimitive` with `alternatives` to handle this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, num_enum::IntoPrimitive, num_enum::FromPrimitive)]
#[repr(u8)]
pub enum RawItemKind {
    #[num_enum(alternatives = [0x02])]
    Number = 0x00,
    PrivateNumber = 0x01,
    #[num_enum(alternatives = [0x45, 0x85])]
    Sensitive = 0x05,
    #[num_enum(alternatives = [0x12])]
    Precision = 0x10,
    #[num_enum(alternatives = [0x22])]
    String = 0x20,
    #[num_enum(alternatives = [0x25, 0x35, 0x81, 0xf1])]
    PrivateString = 0x21,
    #[num_enum(alternatives = [0x32])]
    Arbitrary = 0x30,
    PrivateArbitrary = 0x31,
    #[num_enum(alternatives = [0x42])]
    Object = 0x40,
    PrivateObject = 0x41,
    BaseRaw = 0xf2,
    #[num_enum(default)]
    Unknown,
}

/// Zero-copy item value — borrows strings directly from the input buffer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RawItemValue<'a> {
    /// Precision items, unknowns.
    Empty,
    /// 1/2/4/8-byte signed LE numbers (standard items).
    I64(i64),
    /// 8-byte unsigned (trace items only, BE).
    U64(u64),
    /// Borrowed UTF-8 string from the input, null-trimmed.
    Str(&'a str),
    /// Arbitrary/base64 data, raw bytes (not yet encoded).
    Bytes(&'a [u8]),
    /// Private/sensitive marker — value is redacted.
    Private,
    /// Object with `string_size == 0`.
    Null,
}

/// A single parsed firehose item.
#[derive(Debug, Clone, Copy)]
pub struct RawFirehoseItem<'a> {
    pub item_type: u8,
    /// For string-like items: the `string_size` from the header.
    /// For number items: the raw `item_size` (1/2/4/8).
    pub item_size: u16,
    pub value: RawItemValue<'a>,
}

/// Result of parsing `items_data`.
#[derive(Debug, Clone)]
pub struct RawFirehoseItemData<'a> {
    pub unknown_item: u8,
    pub items: Vec<RawFirehoseItem<'a>>,
    /// Raw backtrace bytes (unparsed). `None` if no backtrace present.
    pub backtrace_data: Option<&'a [u8]>,
}

// --- Standard items parsing (Activity / NonActivity / Signpost) ---

/// Parse standard items data from Activity, `NonActivity`, or Signpost entries.
///
/// Two-pass algorithm:
/// 1. Parse item headers — numbers are resolved inline, string metadata is deferred.
/// 2. After optional backtrace skip, read string data sequentially.
pub fn parse_items_data<'a>(data: &'a [u8], flags: FirehoseFlags) -> nom::IResult<&'a [u8], RawFirehoseItemData<'a>> {
    if data.len() < 2 {
        return Ok((
            data,
            RawFirehoseItemData {
                unknown_item: 0,
                items: Vec::new(),
                backtrace_data: None,
            },
        ));
    }

    let (input, unknown_item) = le_u8(data)?;
    let (input, number_items) = le_u8(input)?;

    let mut items: Vec<RawFirehoseItem<'_>> = Vec::with_capacity(number_items as usize);
    // Indices of items that need string data in pass 2.
    let mut deferred: Vec<usize> = Vec::new();
    let mut input = input;

    // --- Pass 1: headers ---
    for _ in 0..number_items {
        let (rest, item_type) = le_u8(input)?;
        let (rest, item_size) = le_u8(rest)?;
        let kind = RawItemKind::from(item_type);

        match kind {
            RawItemKind::Number => {
                let (rest, value) = parse_item_number(rest, u16::from(item_size))?;
                items.push(RawFirehoseItem {
                    item_type,
                    item_size: u16::from(item_size),
                    value,
                });
                input = rest;
            }
            RawItemKind::PrivateNumber => {
                // Has offset+str_size metadata like strings, but value is private.
                let (rest, _offset) = le_u16(rest)?;
                let (rest, str_size) = le_u16(rest)?;
                items.push(RawFirehoseItem {
                    item_type,
                    item_size: str_size,
                    value: RawItemValue::Private,
                });
                input = rest;
            }
            RawItemKind::Precision => {
                // Skip item_size bytes (precision data for the next item, e.g. %*s).
                let (rest, _) = take(item_size as usize)(rest)?;
                items.push(RawFirehoseItem {
                    item_type,
                    item_size: u16::from(item_size),
                    value: RawItemValue::Empty,
                });
                input = rest;
            }
            RawItemKind::Sensitive | RawItemKind::PrivateString | RawItemKind::PrivateArbitrary | RawItemKind::PrivateObject => {
                // Has offset+str_size metadata, but value is private/sensitive.
                let (rest, _offset) = le_u16(rest)?;
                let (rest, str_size) = le_u16(rest)?;
                items.push(RawFirehoseItem {
                    item_type,
                    item_size: str_size,
                    value: RawItemValue::Private,
                });
                input = rest;
            }
            RawItemKind::String | RawItemKind::Object => {
                // Read offset+str_size metadata, defer string extraction to pass 2.
                let (rest, _offset) = le_u16(rest)?;
                let (rest, str_size) = le_u16(rest)?;
                if str_size == 0 && kind == RawItemKind::Object {
                    items.push(RawFirehoseItem {
                        item_type,
                        item_size: str_size,
                        value: RawItemValue::Null,
                    });
                } else {
                    let idx = items.len();
                    items.push(RawFirehoseItem {
                        item_type,
                        item_size: str_size,
                        value: RawItemValue::Empty, // placeholder
                    });
                    deferred.push(idx);
                }
                input = rest;
            }
            RawItemKind::Arbitrary | RawItemKind::BaseRaw => {
                // Read offset+str_size metadata, defer byte extraction to pass 2.
                let (rest, _offset) = le_u16(rest)?;
                let (rest, str_size) = le_u16(rest)?;
                let idx = items.len();
                items.push(RawFirehoseItem {
                    item_type,
                    item_size: str_size,
                    value: RawItemValue::Empty, // placeholder
                });
                deferred.push(idx);
                input = rest;
            }
            RawItemKind::Unknown => {
                // Best-effort: treat like a number (inline data).
                if item_size > 0 {
                    let take_size = (item_size as usize).min(rest.len());
                    let (rest, _) = take(take_size)(rest)?;
                    items.push(RawFirehoseItem {
                        item_type,
                        item_size: u16::from(item_size),
                        value: RawItemValue::Empty,
                    });
                    input = rest;
                } else {
                    items.push(RawFirehoseItem {
                        item_type,
                        item_size: 0,
                        value: RawItemValue::Empty,
                    });
                    input = rest;
                }
            }
        }
    }

    // --- Backtrace skip ---
    let has_backtrace = flags.contains(FirehoseFlags::HAS_CONTEXT_DATA)
        || (input.len() > 3 && input[..3] == [1, 0, 18]);
    let backtrace_data = if has_backtrace {
        let (rest, bt) = skip_backtrace(input)?;
        input = rest;
        Some(bt)
    } else {
        None
    };

    // --- Pass 2: strings/bytes ---
    for &idx in &deferred {
        let item = &items[idx];
        let str_size = item.item_size as usize;
        let kind = RawItemKind::from(item.item_type);

        if input.is_empty() {
            break;
        }

        // Clamp to available data.
        let actual_size = str_size.min(input.len());
        let (rest, raw_bytes) = take(actual_size)(input)?;
        input = rest;

        let value = match kind {
            RawItemKind::String | RawItemKind::Object => RawItemValue::Str(utf8_str(raw_bytes)),
            RawItemKind::Arbitrary | RawItemKind::BaseRaw => RawItemValue::Bytes(raw_bytes),
            _ => RawItemValue::Empty,
        };

        items[idx] = RawFirehoseItem {
            item_type: item.item_type,
            item_size: item.item_size,
            value,
        };
    }

    Ok((
        input,
        RawFirehoseItemData {
            unknown_item,
            items,
            backtrace_data,
        },
    ))
}

// --- Trace items parsing ---

/// Parse trace `items_data` (reversed byte order, big-endian, numeric only).
///
/// The trace data is stored reversed in the log. This function takes the raw
/// `items_data` bytes, reverses them, then parses: count, sizes[], values[].
/// The resulting items are reversed again to restore the original order.
///
/// Returns `Vec` with `'static` lifetime since all values are numeric (no borrows).
pub fn parse_trace_items(data: &[u8]) -> Vec<RawFirehoseItem<'static>> {
    let minimum_size = 4;
    if data.len() < minimum_size {
        return Vec::new();
    }

    let mut reversed = data.to_vec();
    reversed.reverse();

    parse_trace_items_inner(&reversed)
        .map(|(_, items)| items)
        .unwrap_or_default()
}

/// Inner parser for trace items — uses `IResult` so nom's type inference works.
fn parse_trace_items_inner(data: &[u8]) -> nom::IResult<&[u8], Vec<RawFirehoseItem<'static>>> {
    let (mut input, count) = le_u8(data)?;

    let mut sizes = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let (rest, size) = le_u8(input)?;
        sizes.push(size);
        input = rest;
    }

    let mut items = Vec::with_capacity(count as usize);
    for entry_size in sizes {
        let (rest, msg_data) = take(entry_size as usize)(input)?;
        let value = match entry_size {
            1 => {
                let (_, v) = be_u8(msg_data)?;
                RawItemValue::I64(i64::from(v))
            }
            2 => {
                let (_, v) = be_u16(msg_data)?;
                RawItemValue::I64(i64::from(v))
            }
            4 => {
                let (_, v) = be_u32(msg_data)?;
                RawItemValue::I64(i64::from(v))
            }
            8 => {
                let (_, v) = be_u64(msg_data)?;
                RawItemValue::U64(v)
            }
            _ => {
                log::warn!("Unhandled trace item size: {entry_size}. Defaulting to size 1.");
                let (_, v) = le_u8(msg_data)?;
                RawItemValue::I64(i64::from(v))
            }
        };
        items.push(RawFirehoseItem {
            item_type: 0,
            item_size: 0,
            value,
        });
        input = rest;
    }

    items.reverse();
    Ok((input, items))
}

// --- Helpers ---

/// Parse a little-endian number value from item data.
fn parse_item_number(data: &[u8], item_size: u16) -> nom::IResult<&[u8], RawItemValue<'static>> {
    match item_size {
        1 => {
            let (rest, v) = le_i8(data)?;
            Ok((rest, RawItemValue::I64(i64::from(v))))
        }
        2 => {
            let (rest, v) = le_i16(data)?;
            Ok((rest, RawItemValue::I64(i64::from(v))))
        }
        4 => {
            let (rest, v) = le_i32(data)?;
            Ok((rest, RawItemValue::I64(i64::from(v))))
        }
        8 => {
            let (rest, v) = le_i64(data)?;
            Ok((rest, RawItemValue::I64(v)))
        }
        _ => {
            log::warn!("Unknown number item size: {item_size}");
            Ok((data, RawItemValue::I64(-9999)))
        }
    }
}

/// Skip backtrace data, returning `(remaining, raw_backtrace_bytes)`.
///
/// Layout: 3 unknown bytes, `uuid_count` (u8), `offset_count` (u16 LE),
/// UUIDs (`uuid_count` × 16), offsets (`offset_count` × 4), indexes (`offset_count` × 1),
/// then padding to align `offset_count` to 4 bytes.
fn skip_backtrace(data: &[u8]) -> nom::IResult<&[u8], &[u8]> {
    let start = data;
    let (input, _unknown) = take(3_usize)(data)?;
    let (input, uuid_count) = le_u8(input)?;
    let (input, offset_count) = le_u16(input)?;

    // UUIDs: uuid_count × 16 bytes
    let uuid_bytes = uuid_count as usize * 16;
    let (input, _) = take(uuid_bytes)(input)?;

    // Offsets: offset_count × 4 bytes
    let offset_bytes = offset_count as usize * 4;
    let (input, _) = take(offset_bytes)(input)?;

    // Indexes: offset_count × 1 byte
    let (input, _) = take(offset_count as usize)(input)?;

    // Padding to align offset_count to 4 bytes
    let pad = padding_size(u64::from(offset_count), 4) as usize;
    let (input, _) = take(pad)(input)?;

    let consumed = start.len() - input.len();
    let backtrace_slice = &start[..consumed];

    Ok((input, backtrace_slice))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_item_kind_from() {
        // Number variants
        assert_eq!(RawItemKind::from(0x00), RawItemKind::Number);
        assert_eq!(RawItemKind::from(0x02), RawItemKind::Number);

        // Private number
        assert_eq!(RawItemKind::from(0x01), RawItemKind::PrivateNumber);

        // Sensitive variants
        assert_eq!(RawItemKind::from(0x05), RawItemKind::Sensitive);
        assert_eq!(RawItemKind::from(0x45), RawItemKind::Sensitive);
        assert_eq!(RawItemKind::from(0x85), RawItemKind::Sensitive);

        // Precision variants
        assert_eq!(RawItemKind::from(0x10), RawItemKind::Precision);
        assert_eq!(RawItemKind::from(0x12), RawItemKind::Precision);

        // String variants
        assert_eq!(RawItemKind::from(0x20), RawItemKind::String);
        assert_eq!(RawItemKind::from(0x22), RawItemKind::String);

        // Private string variants
        assert_eq!(RawItemKind::from(0x21), RawItemKind::PrivateString);
        assert_eq!(RawItemKind::from(0x25), RawItemKind::PrivateString);
        assert_eq!(RawItemKind::from(0x35), RawItemKind::PrivateString);
        assert_eq!(RawItemKind::from(0x81), RawItemKind::PrivateString);
        assert_eq!(RawItemKind::from(0xf1), RawItemKind::PrivateString);

        // Arbitrary variants
        assert_eq!(RawItemKind::from(0x30), RawItemKind::Arbitrary);
        assert_eq!(RawItemKind::from(0x32), RawItemKind::Arbitrary);

        // Private arbitrary
        assert_eq!(RawItemKind::from(0x31), RawItemKind::PrivateArbitrary);

        // Object variants
        assert_eq!(RawItemKind::from(0x40), RawItemKind::Object);
        assert_eq!(RawItemKind::from(0x42), RawItemKind::Object);

        // Private object
        assert_eq!(RawItemKind::from(0x41), RawItemKind::PrivateObject);

        // BaseRaw
        assert_eq!(RawItemKind::from(0xf2), RawItemKind::BaseRaw);

        // Unknown
        assert_eq!(RawItemKind::from(0xFF), RawItemKind::Unknown);
    }

    #[test]
    fn test_parse_nonactivity_items() {
        // Exact items_data from the NonActivity test (test_data[10..], 84 bytes).
        // 3 string items + 6 number items, then 16 bytes of string data.
        let items_data: &[u8] = &[
            34, 9, // unknown_item=34, number_items=9
            32, 4, 0, 0, 1, 0, // item 0: type=0x20(String), size=4, offset=0, str_size=1
            32, 4, 1, 0, 1, 0, // item 1: type=0x20(String), size=4, offset=1, str_size=1
            32, 4, 2, 0, 14, 0, // item 2: type=0x20(String), size=4, offset=2, str_size=14
            0, 8, 2, 0, 0, 0, 0, 0, 0, 0, // item 3: Number(2)
            0, 8, 0, 0, 0, 0, 0, 0, 0, 0, // item 4: Number(0)
            0, 8, 2, 0, 0, 0, 0, 0, 0, 0, // item 5: Number(2)
            0, 4, 0, 0, 0, 0, // item 6: Number(0)
            0, 4, 1, 0, 0, 0, // item 7: Number(1)
            0, 4, 1, 0, 0, 0, // item 8: Number(1)
            // String data: 2 null bytes (items 0 & 1), then "dispatchEvent\0" (item 2)
            0, 0, 100, 105, 115, 112, 97, 116, 99, 104, 69, 118, 101, 110, 116, 0,
        ];

        let (remaining, result) = parse_items_data(items_data, FirehoseFlags::empty()).unwrap();
        assert_eq!(result.unknown_item, 34);
        assert_eq!(result.items.len(), 9);
        assert_eq!(result.backtrace_data, None);

        // Items 0,1: strings (1 byte each → empty after null trim)
        assert_eq!(result.items[0].value, RawItemValue::Str(""));
        assert_eq!(result.items[1].value, RawItemValue::Str(""));

        // Item 2: string "dispatchEvent"
        assert_eq!(result.items[2].value, RawItemValue::Str("dispatchEvent"));

        // Items 3-8: numbers
        assert_eq!(result.items[3].value, RawItemValue::I64(2));
        assert_eq!(result.items[4].value, RawItemValue::I64(0));
        assert_eq!(result.items[5].value, RawItemValue::I64(2));
        assert_eq!(result.items[6].value, RawItemValue::I64(0));
        assert_eq!(result.items[7].value, RawItemValue::I64(1));
        assert_eq!(result.items[8].value, RawItemValue::I64(1));

        assert!(remaining.is_empty());
    }

    #[test]
    fn test_parse_activity_items_empty() {
        // Activity entries from the test data have empty items_data.
        let items_data: &[u8] = &[];
        let (_, result) = parse_items_data(items_data, FirehoseFlags::from_bits_retain(4)).unwrap();
        assert_eq!(result.items.len(), 0);
    }

    #[test]
    fn test_parse_trace_items_single() {
        // From trace.rs test: [200, 0, 0, 0, 0, 0, 0, 0, 8, 1]
        // After reverse: [1, 8, 0, 0, 0, 0, 0, 0, 0, 200]
        // count=1, size=8, value=be_u64([0,0,0,0,0,0,0,200])=200
        let data: &[u8] = &[200, 0, 0, 0, 0, 0, 0, 0, 8, 1];
        let items = parse_trace_items(data);
        assert_eq!(items.len(), 1);
        // 200 fits in u64, original code uses UNumber for 8-byte trace values
        assert_eq!(items[0].value, RawItemValue::U64(200));
    }

    #[test]
    fn test_parse_trace_items_multiple() {
        // From trace.rs test_parse_trace_message_multiple:
        // The test data is already reversed in the original test.
        // Original (pre-reversal): some bytes, reversed gives:
        // [2, 8, 8, 0, 0, 0, 0, 0, 0, 0, 200, 0, 0, 127, 251, 75, 225, 96, 176]
        // count=2, sizes=[8,8]
        // value1 = be_u64([0,0,0,0,0,0,0,200]) = 200  → UNumber
        // value2 = be_u64([0,0,127,251,75,225,96,176]) = 140717286580400 → UNumber
        // After reverse: [value2, value1] → item[0]=140717286580400, item[1]=200

        // The original test data is already in reversed form:
        let reversed_data: &[u8] = &[2, 8, 8, 0, 0, 0, 0, 0, 0, 0, 200, 0, 0, 127, 251, 75, 225, 96, 176];
        // We need to un-reverse it (since parse_trace_items will reverse it again)
        let mut data = reversed_data.to_vec();
        data.reverse();
        let items = parse_trace_items(&data);
        assert_eq!(items.len(), 2);
        // After parsing and reversing back, item[0] should be 140717286580400
        assert_eq!(items[0].value, RawItemValue::U64(140_717_286_580_400));
        assert_eq!(items[1].value, RawItemValue::U64(200));
    }

    #[test]
    fn test_parse_trace_items_too_small() {
        let data: &[u8] = &[1, 2, 3];
        let items = parse_trace_items(data);
        assert!(items.is_empty());
    }

    #[test]
    fn test_parse_items_data_minimal() {
        // Just unknown_item + number_items=0
        let data: &[u8] = &[0, 0];
        let (remaining, result) = parse_items_data(data, FirehoseFlags::empty()).unwrap();
        assert_eq!(result.unknown_item, 0);
        assert_eq!(result.items.len(), 0);
        assert_eq!(result.backtrace_data, None);
        assert!(remaining.is_empty());
    }

    #[test]
    fn test_signpost_items_small() {
        // Signpost test data has 2 bytes of items_data: [0, 0]
        // unknown_item=0, number_items=0
        let data: &[u8] = &[0, 0];
        let (_, result) = parse_items_data(data, FirehoseFlags::empty()).unwrap();
        assert_eq!(result.items.len(), 0);
    }
}
