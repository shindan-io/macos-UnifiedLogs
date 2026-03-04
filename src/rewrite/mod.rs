pub use error::*;
use nom::{IResult, Parser};

pub mod catalog;
pub mod chunk;
pub mod chunks;
pub mod chunks_reader;
pub mod chunkset;
pub mod error;
pub mod header;
pub mod helpers;
pub mod dsc;
pub mod timesync;
pub mod uuidtext;

/// Byte offset within the data being parsed.
pub type Offset = usize;
