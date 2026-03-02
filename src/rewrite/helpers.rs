pub(crate) const INVALID_UTF8: &str = "<Invalid UTF-8>";

#[cfg(test)]
pub mod tests {
  use std::path::PathBuf;

  pub fn test_data_path() -> PathBuf {
    let path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join("tests/test_data");
    path
  }
}
