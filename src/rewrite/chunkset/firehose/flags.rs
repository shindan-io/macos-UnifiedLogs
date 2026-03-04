use nom::number::complete::{be_u128, le_u16};

// --- Entry-level flags (independent bits) ---

bitflags::bitflags! {
  /// Firehose entry flags — independent bit flags parsed from the entry header.
  ///
  /// Controls which optional fields are present in the entry body.
  /// Bits 1–3 (mask 0x000E) and bit 5 (0x0020) are formatter flags,
  /// handled separately in [`RawFormatterFlags::parse()`] via `bits()`.
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct FirehoseFlags: u16 {
    const HAS_CURRENT_AID   = 0x0001;
    const HAS_UNIQUE_PID    = 0x0010;
    const HAS_PRIVATE_DATA  = 0x0100;
    const HAS_SUBSYSTEM     = 0x0200;
    const HAS_RULES         = 0x0400;
    const HAS_OVERSIZE      = 0x0800;
    const HAS_CONTEXT_DATA  = 0x1000;
    const HAS_NAME          = 0x8000;
  }
}

// --- Formatter constants (multi-bit enum pattern, private) ---

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
  pub(super) fn parse(input: &[u8], flags: FirehoseFlags) -> nom::IResult<&[u8], Self> {
    let mut result = Self::default();
    let raw = flags.bits();

    match raw & FORMATTER_FLAG_MASK {
      FORMATTER_LARGE_SHARED_CACHE => {
        let mut input = input;
        if (raw & FORMATTER_LARGE_OFFSET) != 0 {
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
        if (raw & FORMATTER_MAIN_EXE) == 0 {
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
        if (raw & FORMATTER_LARGE_OFFSET) != 0 {
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
      _ => Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Switch))),
    }
  }
}
