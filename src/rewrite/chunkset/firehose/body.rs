use nom::bytes::complete::take;
use nom::number::complete::{be_u128, le_u8, le_u16, le_u32, le_u64};

use super::entry::{FirehoseActivityType, FirehoseLogType};

// --- Flag constants ---

const FLAG_HAS_CURRENT_AID: u16 = 0x0001;
const FLAG_HAS_UNIQUE_PID: u16 = 0x0010;
const FLAG_HAS_PRIVATE_DATA: u16 = 0x0100;
const FLAG_HAS_SUBSYSTEM: u16 = 0x0200;
const FLAG_HAS_RULES: u16 = 0x0400;
const FLAG_HAS_OVERSIZE: u16 = 0x0800;
const FLAG_HAS_NAME: u16 = 0x8000;

const FORMATTER_FLAG_MASK: u16 = 0x000e;
const FORMATTER_MAIN_EXE: u16 = 0x2;
const FORMATTER_SHARED_CACHE: u16 = 0x4;
const FORMATTER_ABSOLUTE: u16 = 0x8;
const FORMATTER_UUID_RELATIVE: u16 = 0xa;
const FORMATTER_LARGE_SHARED_CACHE: u16 = 0xc;
const FORMATTER_LARGE_OFFSET: u16 = 0x20;

// --- Formatter flags ---

/// Zero-copy formatter flags — replaces `FirehoseFormatters` without heap allocation.
///
/// `uuid_relative` is stored as raw `[u8; 16]` (big-endian) instead of `Uuid`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RawFormatterFlags {
    pub main_exe: bool,
    pub shared_cache: bool,
    pub absolute: bool,
    pub has_large_offset: u16,
    pub large_shared_cache: u16,
    pub alt_index: u16,
    pub uuid_relative: [u8; 16],
}

impl RawFormatterFlags {
    /// Parse formatter flags from entry data.
    ///
    /// Direct translation of `FirehoseFormatters::firehose_formatter_flags`
    /// from `src/chunks/firehose/flags.rs`.
    fn parse(input: &[u8], flags: u16) -> nom::IResult<&[u8], Self> {
        let mut result = Self::default();

        match flags & FORMATTER_FLAG_MASK {
            FORMATTER_LARGE_SHARED_CACHE => {
                let mut input = input;
                if (flags & FORMATTER_LARGE_OFFSET) != 0 {
                    let (i, val) = le_u16(input)?;
                    result.has_large_offset = val;
                    input = i;
                }
                let (input, val) = le_u16(input)?;
                result.large_shared_cache = val;
                Ok((input, result))
            }
            FORMATTER_ABSOLUTE => {
                result.absolute = true;
                if (flags & FORMATTER_MAIN_EXE) == 0 {
                    let (input, val) = le_u16(input)?;
                    result.alt_index = val;
                    Ok((input, result))
                } else {
                    Ok((input, result))
                }
            }
            FORMATTER_MAIN_EXE => {
                result.main_exe = true;
                Ok((input, result))
            }
            FORMATTER_SHARED_CACHE => {
                result.shared_cache = true;
                if (flags & FORMATTER_LARGE_OFFSET) != 0 {
                    let (input, val) = le_u16(input)?;
                    result.has_large_offset = val;
                    Ok((input, result))
                } else {
                    Ok((input, result))
                }
            }
            FORMATTER_UUID_RELATIVE => {
                let (input, val) = be_u128(input)?;
                result.uuid_relative = val.to_be_bytes();
                Ok((input, result))
            }
            _ => Err(nom::Err::Failure(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Switch,
            ))),
        }
    }
}

// --- Body types ---

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

/// Parsed Signpost entry body.
#[derive(Debug, Clone, Copy)]
pub struct RawSignpostBody<'a> {
    /// Activity ID — present if `FLAG_HAS_CURRENT_AID` (0x0001).
    pub activity_id: Option<(u32, u32)>,
    /// Private string (offset, size) — present if `FLAG_HAS_PRIVATE_DATA` (0x0100).
    pub private_strings: Option<(u16, u16)>,
    pub pc_id: u32,
    pub formatter: RawFormatterFlags,
    /// Subsystem — present if `FLAG_HAS_SUBSYSTEM` (0x0200).
    pub subsystem: Option<u16>,
    /// Always present in signpost entries.
    pub signpost_id: u64,
    /// TTL — present if `FLAG_HAS_RULES` (0x0400).
    pub ttl: Option<u8>,
    /// Oversize data reference — present if `FLAG_HAS_OVERSIZE` (0x0800).
    pub data_ref: Option<u32>,
    /// Signpost name — present if `FLAG_HAS_NAME` (0x8000).
    pub signpost_name: Option<u32>,
    pub items_data: &'a [u8],
}

/// Parsed Trace entry body.
#[derive(Debug, Clone, Copy)]
pub struct RawTraceBody<'a> {
    pub pc_id: u32,
    /// Raw message data — interpretation requires reversal (trace stores data backwards).
    pub items_data: &'a [u8],
}

/// Parsed Loss entry body (no lifetime — fully owned).
#[derive(Debug, Clone, Copy)]
pub struct RawLossBody {
    pub start_time: u64,
    pub end_time: u64,
    pub count: u64,
}

/// Dispatch enum for parsed firehose entry bodies.
#[derive(Debug, Clone, Copy)]
pub enum RawFirehoseBody<'a> {
    Activity(RawActivityBody<'a>),
    NonActivity(RawNonActivityBody<'a>),
    Signpost(RawSignpostBody<'a>),
    Trace(RawTraceBody<'a>),
    Loss(RawLossBody),
    Unknown(&'a [u8]),
}

// --- Parse implementations ---

impl<'a> RawActivityBody<'a> {
    /// Parse an Activity entry body from raw entry data.
    pub fn parse(
        data: &'a [u8],
        flags: u16,
        log_type: FirehoseLogType,
    ) -> nom::IResult<&'a [u8], Self> {
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

impl<'a> RawSignpostBody<'a> {
    /// Parse a Signpost entry body from raw entry data.
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

        let (mut input, signpost_id) = le_u64(input)?;

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

        let signpost_name = if (flags & FLAG_HAS_NAME) != 0 {
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

impl<'a> RawTraceBody<'a> {
    /// Parse a Trace entry body from raw entry data.
    pub fn parse(data: &'a [u8]) -> nom::IResult<&'a [u8], Self> {
        let (items_data, pc_id) = le_u32(data)?;
        Ok((&[], Self { pc_id, items_data }))
    }
}

impl RawLossBody {
    /// Parse a Loss entry body from raw entry data.
    pub fn parse(data: &[u8]) -> nom::IResult<&[u8], Self> {
        let (input, start_time) = le_u64(data)?;
        let (input, end_time) = le_u64(input)?;
        let (input, count) = le_u64(input)?;
        Ok((
            input,
            Self {
                start_time,
                end_time,
                count,
            },
        ))
    }
}

impl<'a> RawFirehoseBody<'a> {
    /// Parse a firehose entry body by dispatching on the activity type.
    pub fn parse(
        data: &'a [u8],
        log_activity_type: FirehoseActivityType,
        flags: u16,
        log_type: FirehoseLogType,
    ) -> Result<Self, nom::Err<nom::error::Error<&'a [u8]>>> {
        match log_activity_type {
            FirehoseActivityType::Activity => {
                let (_, body) = RawActivityBody::parse(data, flags, log_type)?;
                Ok(Self::Activity(body))
            }
            FirehoseActivityType::NonActivity => {
                let (_, body) = RawNonActivityBody::parse(data, flags)?;
                Ok(Self::NonActivity(body))
            }
            FirehoseActivityType::Signpost => {
                let (_, body) = RawSignpostBody::parse(data, flags)?;
                Ok(Self::Signpost(body))
            }
            FirehoseActivityType::Trace => {
                let (_, body) = RawTraceBody::parse(data)?;
                Ok(Self::Trace(body))
            }
            FirehoseActivityType::Loss => {
                let (_, body) = RawLossBody::parse(data)?;
                Ok(Self::Loss(body))
            }
            FirehoseActivityType::Unknown => Ok(Self::Unknown(data)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::RawFirehose;

    #[test]
    fn test_activity_body() {
        // From src/chunks/firehose/activity.rs test_parse_activity
        let test_data: &[u8] = &[
            178, 251, 0, 0, 0, 0, 0, 128, 236, 0, 0, 0, 0, 0, 0, 0, 178, 251, 0, 0, 0, 0, 0,
            128, 179, 251, 0, 0, 0, 0, 0, 128, 64, 63, 24, 18, 1, 0, 2, 0,
        ];
        let flags: u16 = 573;
        let log_type = FirehoseLogType::Info;

        let body =
            RawFirehoseBody::parse(test_data, FirehoseActivityType::Activity, flags, log_type)
                .unwrap();
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
    fn test_non_activity_body() {
        // From src/chunks/firehose/nonactivity.rs test_parse_non_activity
        let test_data: &[u8] = &[
            122, 179, 12, 13, 2, 0, 4, 0, 41, 0, 34, 9, 32, 4, 0, 0, 1, 0, 32, 4, 1, 0, 1, 0,
            32, 4, 2, 0, 14, 0, 0, 8, 2, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            8, 2, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 4, 1, 0, 0, 0, 0, 4, 1, 0, 0, 0, 0,
            0, 100, 105, 115, 112, 97, 116, 99, 104, 69, 118, 101, 110, 116, 0,
        ];
        let flags: u16 = 556;

        let body = RawFirehoseBody::parse(
            test_data,
            FirehoseActivityType::NonActivity,
            flags,
            FirehoseLogType::Default,
        )
        .unwrap();
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

    #[test]
    fn test_signpost_body() {
        // From src/chunks/firehose/signpost.rs test_parse_signpost
        let test_data: &[u8] = &[
            225, 244, 2, 0, 1, 0, 238, 238, 178, 178, 181, 176, 238, 238, 176, 63, 27, 0, 0, 0,
        ];
        let flags: u16 = 33282;

        let body = RawFirehoseBody::parse(
            test_data,
            FirehoseActivityType::Signpost,
            flags,
            FirehoseLogType::Default,
        )
        .unwrap();
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

    #[test]
    fn test_trace_body() {
        // From src/chunks/firehose/trace.rs test_parse_firehose_trace
        let test_data: &[u8] = &[106, 139, 3, 0, 0];

        let body = RawFirehoseBody::parse(
            test_data,
            FirehoseActivityType::Trace,
            0,
            FirehoseLogType::Default,
        )
        .unwrap();
        let trace = match body {
            RawFirehoseBody::Trace(t) => t,
            other => panic!("expected Trace, got {other:?}"),
        };

        assert_eq!(trace.pc_id, 232298);
        assert_eq!(trace.items_data, &[0]);
    }

    #[test]
    fn test_loss_body() {
        // From src/chunks/firehose/loss.rs test_parse_firehose_loss_monterey
        let test_data: &[u8] = &[
            72, 56, 43, 42, 0, 0, 0, 0, 231, 207, 114, 187, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0,
            0,
        ];

        let body = RawFirehoseBody::parse(
            test_data,
            FirehoseActivityType::Loss,
            0,
            FirehoseLogType::Default,
        )
        .unwrap();
        let loss = match body {
            RawFirehoseBody::Loss(l) => l,
            other => panic!("expected Loss, got {other:?}"),
        };

        assert_eq!(loss.start_time, 707475528);
        assert_eq!(loss.end_time, 3144863719);
        assert_eq!(loss.count, 63);
    }

    #[test]
    fn test_activity_body_from_entry_test_data() {
        // Same test data as entry.rs test_iterate_entries: 3 Activity entries with flags=4.
        let test_data: &[u8] = &[
            1, 96, 0, 0, 0, 0, 0, 0, 152, 0, 0, 0, 0, 0, 0, 0, 133, 16, 0, 0, 0, 0, 0, 0, 157,
            38, 0, 0, 0, 0, 0, 0, 136, 0, 0, 16, 0, 0, 0, 2, 42, 188, 25, 14, 104, 4, 0, 0, 2,
            1, 4, 0, 240, 243, 53, 0, 176, 232, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 12, 0, 176,
            249, 0, 0, 0, 0, 0, 128, 163, 133, 51, 0, 0, 0, 0, 0, 2, 1, 4, 0, 32, 250, 53, 0,
            177, 232, 0, 0, 0, 0, 0, 0, 209, 67, 85, 0, 16, 0, 12, 0, 177, 249, 0, 0, 0, 0, 0,
            128, 237, 115, 51, 0, 0, 0, 0, 0, 2, 1, 4, 0, 48, 57, 126, 0, 179, 232, 0, 0, 0, 0,
            0, 0, 40, 101, 197, 1, 16, 0, 12, 0, 178, 249, 0, 0, 0, 0, 0, 128, 105, 67, 61, 0,
            0, 0, 0, 0,
        ];

        // Skip 16-byte preamble, parse firehose
        let data = &test_data[16..];
        let (_, fh) = RawFirehose::parse(data).unwrap();
        let entries: Vec<_> = fh.entries().collect();
        assert_eq!(entries.len(), 3);

        for entry in &entries {
            let body = entry.parse_body().unwrap();
            let activity = match body {
                RawFirehoseBody::Activity(a) => a,
                other => panic!("expected Activity, got {other:?}"),
            };

            // flags=4, log_type=Info(0x01)
            // Not Useraction → activity_id present
            assert_eq!(activity.activity_id.unwrap().1, 0x80000000);
            // flags & 0x10 = 0 → no pid
            assert_eq!(activity.pid, None);
            // flags & 0x01 = 0 → no current_aid
            assert_eq!(activity.current_aid, None);
            // flags & 0x200 = 0 → no other_aid
            assert_eq!(activity.other_aid, None);
            // flags & 0xE = 0x4 = SHARED_CACHE
            assert!(activity.formatter.shared_cache);
            assert!(activity.items_data.is_empty());
        }

        // First entry specifics
        let first = match entries[0].parse_body().unwrap() {
            RawFirehoseBody::Activity(a) => a,
            _ => unreachable!(),
        };
        assert_eq!(first.activity_id, Some((63920, 0x80000000)));
        assert_eq!(first.pc_id, 0x003385A3);
    }
}
