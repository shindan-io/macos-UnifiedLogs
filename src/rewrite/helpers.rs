/// Calculate 8-byte alignment padding for `n` bytes.
#[inline]
pub fn padding_size_8(n: usize) -> usize {
  (8 - (n & 7)) & 7
}

#[cfg(test)]
pub mod tests {
  use std::path::PathBuf;

  pub fn test_data_path() -> PathBuf {
    let path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("tests/test_data");
    path
  }
}
