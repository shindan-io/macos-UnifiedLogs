use chunks::{ChunkPreamble, PREAMBLE_SIZE};
use error::ParseError;
use nom::{IResult, Parser};

pub mod chunks;
pub mod chunks_reader;
pub mod error;
pub mod helpers;

/// Byte offset within the data being parsed.
pub type Offset = usize;

#[cfg(test)]
mod tests {
  // use super::*;

  // /// Build a minimal chunk: 16-byte preamble + body + 8-byte aligned padding (zeroed).
  // fn make_chunk(tag: ChunkTag, body: &[u8]) -> Vec<u8> {
  //   let mut buf = Vec::new();
  //   buf.extend_from_slice(&tag.as_u32().to_le_bytes()); // tag
  //   buf.extend_from_slice(&0x11u32.to_le_bytes()); // sub_tag
  //   buf.extend_from_slice(&(body.len() as u64).to_le_bytes()); // data_size
  //   buf.extend_from_slice(body);
  //   // pad to 8-byte alignment
  //   let pad = padding_size_8(body.len());
  //   buf.extend(std::iter::repeat_n(0u8, pad));
  //   buf
  // }

  // #[test]
  // fn iterate_empty_buffer() {
  //   let reader = RootChunksReader::new(&[]);
  //   assert_eq!(reader.count(), 0);
  // }

  // #[test]
  // fn iterate_single_header_chunk() {
  //   let body = vec![0xAA; 208]; // typical header body size
  //   let buf = make_chunk(ChunkTag::Header, &body);

  //   let chunks: Vec<_> = RootChunksReader::new(&buf).collect::<Result<_, _>>().unwrap();
  //   assert_eq!(chunks.len(), 1);
  //   assert_eq!(chunks[0].tag, ChunkTag::Header);
  //   assert_eq!(chunks[0].sub_tag, 0x11);
  //   assert_eq!(chunks[0].data.len(), 208);
  //   assert_eq!(chunks[0].data[0], 0xAA);
  // }

  // #[test]
  // fn iterate_multiple_chunks() {
  //   let mut buf = Vec::new();
  //   buf.extend(make_chunk(ChunkTag::Header, &[0; 208]));
  //   buf.extend(make_chunk(ChunkTag::Catalog, &[0; 100]));
  //   buf.extend(make_chunk(ChunkTag::Chunkset, &[0; 50]));

  //   let tags: Vec<_> = RootChunksReader::new(&buf)
  //     .map(|r| r.map(|c| c.tag))
  //     .collect::<Result<_, _>>()
  //     .unwrap();
  //   assert_eq!(tags, [ChunkTag::Header, ChunkTag::Catalog, ChunkTag::Chunkset]);
  // }

  // #[test]
  // fn padding_alignment() {
  //   // body of 5 bytes → needs 3 bytes padding to reach 8-byte alignment
  //   assert_eq!(padding_size_8(5), 3);
  //   assert_eq!(padding_size_8(8), 0);
  //   assert_eq!(padding_size_8(0), 0);
  //   assert_eq!(padding_size_8(1), 7);
  //   assert_eq!(padding_size_8(16), 0);
  // }

  // #[test]
  // fn truncated_body_is_error() {
  //   // Preamble claims 100 bytes of body, but buffer only has preamble
  //   let mut buf = Vec::new();
  //   buf.extend_from_slice(&ChunkTag::Header.as_u32().to_le_bytes());
  //   buf.extend_from_slice(&0x11u32.to_le_bytes());
  //   buf.extend_from_slice(&100u64.to_le_bytes());
  //   // no body bytes

  //   let mut reader = RootChunksReader::new(&buf);
  //   let result = reader.next().unwrap();
  //   assert!(result.is_err());
  // }
}
