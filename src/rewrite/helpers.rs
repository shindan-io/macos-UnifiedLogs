use nom::{
  Parser,
  bytes::complete::{take, take_while},
  combinator::opt,
};

pub(crate) const INVALID_UTF8: &str = "<Invalid UTF-8>";
const NULL_BYTE: u8 = 0;

/// Returns the padding to consume in order to align to 8 bytes
/// Actual total size is computed as `items_count` * `items_size`
pub(crate) fn anticipated_padding_size_8(items_count: u64, items_size: u64) -> u64 {
  anticipated_padding_size(items_count, items_size, 8)
}

/// Returns the padding to consume in order to align to 'alignment' bytes
/// Actual total size is computed as `items_count` * `items_size`
pub(crate) fn anticipated_padding_size(items_count: u64, items_size: u64, alignment: u64) -> u64 {
  let total_size = items_count * items_size;
  padding_size(total_size, alignment)
}

/// Calculate padding based on provided `alignment`
pub(crate) fn padding_size(data_size: u64, alignment: u64) -> u64 {
  (alignment - (data_size & (alignment - 1))) & (alignment - 1)
}

pub(crate) fn u64_to_usize(n: u64) -> Option<usize> {
  usize::try_from(n).ok()
}

pub(crate) fn utf8_str(data: &[u8]) -> &str {
  std::str::from_utf8(data)
    .inspect_err(|err| log::warn!("{err}"))
    .map(|s| s.trim_end_matches('\0'))
    .unwrap_or(INVALID_UTF8)
}
/// Extract an UTF8 string from a byte array, stops at `NULL_BYTE` or END OF STRING
/// Consumes the end byte
/// Fails if the string is empty
pub(crate) fn utf8_str_from_cstring(input: &[u8]) -> nom::IResult<&[u8], &str> {
  if input.is_empty() {
    return Ok((input, ""));
  }
  let mut tup = (take_while(|b: u8| b != NULL_BYTE), opt(take(1_usize)));
  let (input, (str_part, _)) = tup.parse(input)?;
  let str_part = utf8_str(str_part);
  Ok((input, str_part))
}

#[cfg(test)]
pub mod tests {
  use std::path::PathBuf;

  pub fn test_data_path() -> PathBuf {
    let path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("tests/test_data");
    path
  }
}
