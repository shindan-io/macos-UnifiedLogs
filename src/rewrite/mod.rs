pub use error::*;
use nom::{IResult, Parser};

pub mod chunks;
pub mod chunks_reader;
pub mod error;
pub mod header;
pub mod helpers;

/// Byte offset within the data being parsed.
pub type Offset = usize;
