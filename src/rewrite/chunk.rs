use super::{
  catalog::RawCatalogChunk,
  chunks::ChunkTag,
  chunks_reader::{RawChunk, RawChunksReader},
  chunkset::{ChunksetPayload, simpledump::RawSimpleDump, statedump::RawStatedump},
  error::{NomExt, ParseError},
  header::RawHeaderChunk,
};

#[derive(Debug)]
pub enum Chunk<'a> {
  Header(super::header::RawHeaderChunk<'a>),
  Catalog(RawCatalogChunk<'a>),
  Simpledump(RawSimpleDump<'a>),
  Statedump(RawStatedump<'a>),
  Unknown(RawChunk<'a>),
}

/// A parsed, typed chunk from a tracev3 file.
///
/// Inner chunkset types (Firehose, Oversize, etc.) are not yet dispatched —
/// they land in the `Unknown` variant when encountered.
#[derive(Debug)]
pub enum TopChunk<'a> {
  Header(super::header::RawHeaderChunk<'a>),
  Catalog(RawCatalogChunk<'a>),
  Chunkset(ChunkSetReader<'a>),
  Unknown(RawChunk<'a>),
}

#[derive(Debug)]
pub struct ChunksReader<'a> {
  inner: RawChunksReader<'a>,
}

impl<'a> ChunksReader<'a> {
  pub fn new(input: &'a [u8]) -> Self {
    Self {
      inner: RawChunksReader::new_top_level(input),
    }
  }
}

#[derive(Debug)]
pub struct ChunkSetReader<'a> {
  payload: ChunksetPayload<'a>,
  current_offset: usize,
}

impl<'a> ChunkSetReader<'a> {
  pub fn new(payload: ChunksetPayload<'a>) -> Self {
    Self {
      payload,
      current_offset: 0,
    }
  }

  pub fn next(&mut self) -> Option<Result<RawChunk<'_>, ParseError>> {
    let data = self.payload.as_bytes();
    if self.current_offset >= data.len() {
      return None;
    }

    let input = &data[self.current_offset..];
    let mut raw_reader = RawChunksReader::new_chunckset(input);
    let next = raw_reader.next();
    self.current_offset += raw_reader.current_offset();
    next
  }
}

impl<'a> Iterator for ChunksReader<'a> {
  type Item = Result<TopChunk<'a>, ParseError>;

  fn next(&mut self) -> Option<Self::Item> {
    let raw = match self.inner.next()? {
      Ok(raw) => raw,
      Err(e) => return Some(Err(e)),
    };

    Some(match raw.preamble.tag {
      ChunkTag::Header => RawHeaderChunk::parse(raw.data)
        .map_err(|e| e.to_parse_error())
        .map(|(_, c)| TopChunk::Header(c)),
      ChunkTag::Catalog => RawCatalogChunk::parse(raw.data)
        .map_err(|e| e.to_parse_error())
        .map(|(_, c)| TopChunk::Catalog(c)),
      ChunkTag::Chunkset => {
        let payload = match ChunksetPayload::parse(raw.data) {
          Ok(p) => p,
          Err(e) => return Some(Err(e)),
        };
        let reader = ChunkSetReader::new(payload);
        Ok(TopChunk::Chunkset(reader))
      }
      _ => Ok(TopChunk::Unknown(raw)),
    })
  }
}

impl ChunksReader<'_> {
  pub fn visit(&mut self, mut f: impl FnMut(Chunk<'_>) -> ()) -> Result<(), ParseError> {
    for chunk in self {
      let chunk = chunk?;
      match chunk {
        TopChunk::Header(c) => f(Chunk::Header(c)),
        TopChunk::Catalog(c) => f(Chunk::Catalog(c)),
        TopChunk::Chunkset(mut reader) => {
          while let Some(inner) = reader.next() {
            let inner = inner?;
            match inner.preamble.tag {
              ChunkTag::Simpledump => {
                let (_, sd) = RawSimpleDump::parse(inner.data).map_err(|e| e.to_parse_error())?;
                f(Chunk::Simpledump(sd));
              }
              ChunkTag::Statedump => {
                let (_, sd) = RawStatedump::parse(inner.data).map_err(|e| e.to_parse_error())?;
                f(Chunk::Statedump(sd));
              }
              _ => f(Chunk::Unknown(inner)),
            }
          }
        }
        TopChunk::Unknown(raw) => f(Chunk::Unknown(raw)),
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::super::helpers::tests::test_data_path;
  use super::*;
  use std::collections::HashMap;

  #[test]
  fn parse_catalog_chunk() -> anyhow::Result<()> {
    let data = std::fs::read(test_data_path().join("Catalog Tests/big_sur_catalog.raw"))?;
    let reader = ChunksReader::new(&data);

    let chunks = reader.collect::<Result<Vec<_>, _>>()?;
    assert_eq!(chunks.len(), 1);
    assert!(matches!(chunks[0], TopChunk::Catalog(_)));
    Ok(())
  }

  #[test]
  fn parse() -> anyhow::Result<()> {
    let data = std::fs::read(test_data_path().join("Bad Data/TraceV3/Bad_header_0000000000000005.tracev3"))?;

    let reader = ChunksReader::new(&data);
    let chunks = reader.collect::<Result<Vec<_>, _>>()?;
    assert_eq!(chunks.len(), 251);

    Ok(())
  }

  #[test]
  fn visit() -> anyhow::Result<()> {
    let data = std::fs::read(test_data_path().join("Bad Data/TraceV3/Bad_header_0000000000000005.tracev3"))?;

    let mut reader = ChunksReader::new(&data);
    let mut count = 0;
    let mut count_by_type = HashMap::new();
    reader.visit(|chunk| {
      count += 1;
      match chunk {
        Chunk::Header(_) => {
          *count_by_type.entry(ChunkTag::Header).or_insert(0) += 1;
        }
        Chunk::Catalog(_) => {
          *count_by_type.entry(ChunkTag::Catalog).or_insert(0) += 1;
        }
        Chunk::Simpledump(_) => {
          *count_by_type.entry(ChunkTag::Simpledump).or_insert(0) += 1;
        }
        Chunk::Statedump(_) => {
          *count_by_type.entry(ChunkTag::Statedump).or_insert(0) += 1;
        }
        Chunk::Unknown(_) => {
          *count_by_type.entry(ChunkTag::Unknown).or_insert(0) += 1;
        }
      }
    })?;
    assert_eq!(count, 4082);
    dbg!(&count_by_type);
    assert_eq!(count_by_type.get(&ChunkTag::Catalog), Some(&36));
    assert_eq!(count_by_type.get(&ChunkTag::Simpledump), None);
    assert_eq!(count_by_type.get(&ChunkTag::Statedump), None);
    assert_eq!(count_by_type.get(&ChunkTag::Unknown), Some(&4046));

    Ok(())
  }
}
