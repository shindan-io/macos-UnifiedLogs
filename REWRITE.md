# `src/rewrite/` — Catalog Comparison & Module Status

## Module Structure

```
src/rewrite/
├── mod.rs              # Re-exports, Offset type alias
├── error.rs            # ParseError (thiserror + NomExt bridge)
├── chunks.rs           # ChunkTag (num_enum), ChunkPreamble
├── chunks_reader.rs    # RawChunksReader iterator (configurable padding)
├── header.rs           # RawHeaderChunk<'a> (zero-copy strings)
├── catalog.rs          # RawCatalogChunk<'a> (zero-copy strings, eager subsystem lookup)
├── helpers.rs          # Padding math, utf8_str, utf8_str_from_cstring, test helpers
└── chunkset/           # ChunksetPayload (bv41/bv4- decompression) + placeholder inner types
```

## Design Decisions (vs. original plan)

### Kept nom instead of `BinaryReader`

The plan proposed a custom `BinaryReader<'a>` cursor to replace nom. The rewrite keeps nom throughout. This is pragmatic: nom is already a dependency, well-tested, and the `(input, val)` threading — while verbose — is a known pattern in the codebase. Switching to a custom cursor can happen later if the verbosity becomes a real bottleneck.

### `num_enum` instead of manual `from_u32`/`as_u32`

`ChunkTag` uses `#[derive(num_enum::IntoPrimitive, num_enum::FromPrimitive)]` with `#[repr(u32)]` instead of hand-written match arms. This eliminates boilerplate and gives a compile-time guarantee that the discriminant values are correct. An `Unknown` variant with `#[num_enum(default)]` replaces the `ParseError::UnknownChunkTag` path — unknown tags are silently representable rather than immediately fatal.

### `Self::parse()` convention

All struct/enum types that parse from binary data expose a `pub fn parse(input: &'a [u8]) -> Result<Self, ParseError>` (or `IResult`) as an associated function. This keeps parsing logic co-located with the type and gives a consistent API across the codebase (`ChunkPreamble::parse()`, `RawHeaderChunk::parse()`, `RawCatalogChunk::parse()`, `ChunksetPayload::parse()`).

### `thiserror` instead of manual `Display`/`Error` impls

`ParseError` is a `#[derive(thiserror::Error)]` enum. Cleaner than manual `impl Display` + `impl Error`, and the `#[error(...)]` format strings serve as documentation.

### `NomExt` bridge trait

A `NomExt` trait on `nom::Err<...>` provides `.to_parse_error()` to convert nom errors into `ParseError`. This lets `RawChunksReader` return `Result<_, ParseError>` while internally using nom, avoiding a forced choice between the two error worlds.

## Catalog: Old vs. Rewrite — Detailed Comparison

### Type-level changes

| Aspect | Old (`src/catalog.rs`) | Rewrite (`src/rewrite/catalog.rs`) |
|--------|----------------------|-----------------------------------|
| Main type | `CatalogChunk` (owns everything) | `RawCatalogChunk<'a>` (borrows from input) |
| Preamble | Stored inline (`chunk_tag`, `chunk_sub_tag`, `chunk_data_size`) | Parsed separately, not duplicated in struct |
| Subsystem strings | `Vec<u8>` (heap copy) | `&'a [u8]` (zero-copy borrow) |
| Subsystem cache | `RefCell<HashMap<..., (RcString, RcString)>>` — lazy, interior mutability | `HashMap<..., (&'a str, &'a str)>` — eager, immutable, borrowed |
| String type | `RcString` (Rc-wrapped heap String) | `&'a str` (zero-copy borrow) |
| Return type for `get_subsystem` | `nom::IResult<&[u8], SubsystemInfo>` (leaks nom into public API) | `Option<SubsystemInfo<'a>>` (clean Rust API) |
| Return type for `get_pid` | `u64` (returns 0 on miss + logs warning) | `Option<u64>` (caller decides) |
| Return type for `get_euid` | `u32` (returns 0 on miss + logs warning) | `Option<u32>` |
| Process parsing | Monolithic method on `CatalogChunk` | Standalone `ProcessInfoEntry::parse()` |
| Subchunk parsing | Method on `CatalogChunk` | Standalone `CatalogSubchunk::parse()` |
| Subsystem parsing | Method on `CatalogChunk` | Standalone `ProcessInfoSubsystem::parse()` |

### Structural improvements

**1. Zero-copy strings**

Old: `catalog_subsystem_strings` is `subsystem_strings_data.to_vec()` — an unconditional heap copy of the subsystem string table. Every `get_subsystem` call then does `rc_string!(subsystem_string)` — another heap allocation per lookup (cached on second access).

Rewrite: `catalog_subsystem_strings` is `&'a [u8]` — borrows directly from the input buffer. The `subsystems_strings` HashMap stores `(&'a str, &'a str)` pairs — zero-copy borrows into the subsystem string table, resolved eagerly at parse time. No heap allocations for string lookups at all.

**2. Eager vs. lazy subsystem resolution**

Old: Subsystems are resolved lazily on first `get_subsystem()` call, using a `RefCell<HashMap>` cache. This requires interior mutability on what is logically an immutable data structure, and means the first lookup for each key pays a parsing cost.

Rewrite: All subsystem strings are resolved during `parse()` and stored in `subsystems_strings: HashMap<(u16, u64, u32), (&'a str, &'a str)>`. `get_subsystem()` is a simple HashMap lookup returning `Option<SubsystemInfo>`. No `RefCell`, no runtime parsing cost on lookup.

**3. Preamble separation**

Old: `parse_catalog()` parses the 16-byte preamble internally and stores `chunk_tag`/`chunk_sub_tag`/`chunk_data_size` as fields on `CatalogChunk`. This couples preamble parsing with catalog body parsing.

Rewrite: The preamble is parsed externally by `ChunkPreamble::parse()`, and `RawCatalogChunk::parse()` receives only the body bytes. The preamble fields don't pollute the catalog struct.

**4. Clean public API**

Old: `get_subsystem()` returns `nom::IResult<&[u8], SubsystemInfo>` — exposing nom's error type and the `&[u8]` remaining-input pattern through the public API. Callers must destructure a tuple even though the remaining input is always `&[]`.

Rewrite: `get_subsystem()` returns `Option<SubsystemInfo<'a>>`. Simple, idiomatic. `get_pid()` and `get_euid()` return `Option<T>` instead of silently returning 0 and logging warnings.

**5. Decomposed parsing methods**

Old: All parsing methods (`parse_catalog_process_entry`, `parse_process_info_uuid_entry`, `parse_process_info_subystem`, `parse_catalog_subchunk`) are private methods on `CatalogChunk`. This makes unit testing harder — the test for `parse_process_info_subystem` has to call `CatalogChunk::parse_process_info_subystem()`.

Rewrite: Each sub-type has its own `parse()` as an associated function (`ProcessInfoEntry::parse()`, `ProcessUUIDEntry::parse()`, `ProcessInfoSubsystem::parse()`, `CatalogSubchunk::parse()`). Independently testable.

### What stays the same

- Binary layout parsing is identical (same fields, same order, same types)
- `ProcessInfoEntry`, `ProcessUUIDEntry`, `ProcessInfoSubsystem`, `CatalogSubchunk` have the same fields
- `CatalogProcessInfoKey(u64, u32)` tuple struct is unchanged
- The 6-byte `load_address` hack (pad to 8 bytes, parse as `le_u64`) is preserved as-is
- Padding calculations use the same `anticipated_padding_size_8` logic
- Test data and assertions are identical

## Header: Old vs. Rewrite

| Aspect | Old (`src/header.rs`) | Rewrite (`src/rewrite/header.rs`) |
|--------|----------------------|----------------------------------|
| Type | `HeaderChunk<S>` generic over string type | `RawHeaderChunk<'a>` with `&'a str` |
| String handling | Generic `S: Default + ToString`, needs `into_owned()` for `RcString` | Direct `&'a str`, uses shared `utf8_str()` helper |
| Preamble | Parsed inline, stored as fields | Parsed externally by `ChunkPreamble` |
| UTF-8 errors | Per-field `from_utf8().inspect_err().unwrap_or(INVALID_UTF8)` (3 copies) | Shared `utf8_str()` helper (DRY) |

The old code has a `HeaderChunkStr<'a>` / `HeaderChunkOwned` type alias pair with an `into_owned()` method that manually copies every field. The rewrite eliminates this entirely — there's only one type with borrowed strings.

## Chunks & Reader: Old vs. Rewrite

| Aspect | Old (`src/chunk.rs` + `src/preamble.rs`) | Rewrite (`src/rewrite/chunks.rs` + `chunks_reader.rs`) |
|--------|----------------------------------------|-------------------------------------------------------|
| Tag type | `u32` constants | `ChunkTag` enum with `num_enum` |
| Preamble | Separate `LogPreamble` struct | `ChunkPreamble` (same fields, typed tag) |
| Iteration | Free functions `next_top_level_chunk()` / `next_inner_chunk()` | `RawChunksReader` iterator with configurable padding |
| Padding | Hardcoded 8-byte (top-level) vs. zero-skip (inner) | Constructor param: `new_top_level()` (8) or `new(data, n)` |
| Result | `Option<(RawChunkRef, usize)>` — manual offset tracking | `Iterator<Item = Result<RawChunk, ParseError>>` |
| Chunk ref | `RawChunkRef { tag: u32, start: usize, end: usize }` | `RawChunk { preamble: ChunkPreamble, data: &[u8] }` |

The iterator approach is cleaner — consumers use a standard `for chunk in reader` loop instead of manually tracking `pos` with a `while let` loop.

## Error Handling

| Old | Rewrite |
|-----|---------|
| `nom::IResult` everywhere | `ParseError` enum + `NomExt` bridge |
| No byte offsets in errors | `offset: Offset` on each variant |
| No context labels | `context: Option<&'static str>` on each variant |
| Silent fallback values (`0`, `""`, `"Unknown"`) | `Option<T>` return types where appropriate |
| `log::warn!` / `log::error!` on parse failures | Errors propagated via `Result`, caller decides |

## Chunkset Decompression

| Aspect | Old (`src/chunkset.rs` / `src/noalloc_iterator.rs`) | Rewrite (`src/rewrite/chunkset/mod.rs`) |
|--------|------------------------------------------------------|----------------------------------------|
| Type | Free function `decompress_chunkset()` / inline in `process_chunkset_range()` | `ChunksetPayload<'a>` enum with `Self::parse()` |
| Signature dispatch | Match on `u32` constants `BV41_COMPRESSED` / `BV41_UNCOMPRESSED` | Same constants, private to module |
| Uncompressed | Returns `Vec<u8>` (copies data) | `ChunksetPayload::Uncompressed(&'a [u8])` — zero-copy borrow |
| Compressed | `lz4_flex::decompress()` → `Vec<u8>` | `ChunksetPayload::Decompressed(Vec<u8>)` — owned, same decompression |
| Inner iteration | Caller passes decompressed bytes to `next_inner_chunk()` manually | `payload.inner_chunks()` returns a `RawChunksReader` directly |
| Error handling | `log::error!` + empty fallback | `ParseError::DecompressError` / `ParseError::UnexpectedEof` propagated |

The key design is the `ChunksetPayload` enum that cleanly models the owned-vs-borrowed split: `bv4-` data borrows from the tracev3 buffer (`&'a [u8]`), while `bv41` data must be owned (`Vec<u8>`) after LZ4 decompression. Both variants expose `as_bytes()` and `inner_chunks()` uniformly.

## What's Left to Implement

The `chunkset/` subdirectory has placeholder files (firehose, oversize, simpledump, statedump). These are the inner chunk types that appear inside decompressed chunksets — the core of the log entry parsing pipeline.

Also not yet in `src/rewrite/`:
- Firehose entry parsing (the 24-byte entry header + flag-dependent body)
- Timesync, DSC, UUIDText file parsing
- Message resolution (printf format expansion)
- The streaming iterator that ties everything together
