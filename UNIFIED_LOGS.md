# macOS Unified Logs — Binary Format Reference

## Context

This document describes the binary wire format of macOS Unified Logs (`tracev3` files and supporting files) as reverse-engineered from the `macos-UnifiedLogs` parser codebase. It is intended to help developers understand the streaming format and the different parsing steps needed to make sense of the raw bytes.

---

## 1. Logarchive Directory Layout

A `.logarchive` directory (or the live system paths) contains:

```
logarchive/
├── <tracev3 files>                      # Main log data
│   ├── logdata.LiveData.tracev3         # Live/special log
│   ├── Persist/
│   │   ├── 0000000000000001.tracev3     # Persistent logs (sorted by name = time order)
│   │   ├── 0000000000000002.tracev3
│   │   └── ...
│   ├── Signpost/
│   │   └── 0000000000000001.tracev3
│   └── HighVolume/
│       └── ...
├── dsc/                                 # Shared string caches (DSC)
│   ├── <UUID1>                          # One per shared cache version
│   └── <UUID2>
├── uuidtext/                            # Per-binary format string files
│   ├── <2-hex-prefix>/
│   │   ├── <remaining-UUID-hex>         # e.g. uuidtext/3D/05845F3F65358F9EBF2236E772AC01
│   │   └── ...
│   └── ...
└── timesync/                            # Time synchronization data
    ├── 0000000000000001.timesync
    └── ...
```

On a live system:
- tracev3: `/private/var/db/diagnostics/`
- DSC: `/private/var/db/uuidtext/dsc/`
- UUIDText: `/private/var/db/uuidtext/`
- Timesync: `/private/var/db/diagnostics/timesync/`

---

## 2. tracev3 File — Top-Level Structure

A tracev3 file is a flat sequence of **chunks**. Each chunk starts with a 16-byte **preamble**.

### 2.1 Chunk Preamble (16 bytes — every chunk)

```
Offset  Size  Type    Field
─────────────────────────────────
 0      4     u32LE   chunk_tag          ← identifies chunk type
 4      4     u32LE   chunk_sub_tag      ← version/sub-identifier
 8      8     u64LE   chunk_data_size    ← size of payload (excludes preamble)
```

Total chunk size = 16 + chunk_data_size, then padded to 8-byte alignment.

Source: `src/preamble.rs`

### 2.2 Chunk Tags

| Tag        | Hex      | Name             | Where found        |
|------------|----------|------------------|--------------------|
| HEADER     | `0x1000` | Header           | Top-level only     |
| CATALOG    | `0x600b` | Catalog          | Top-level only     |
| CHUNKSET   | `0x600d` | Chunkset         | Top-level only     |
| FIREHOSE   | `0x6001` | Firehose         | Inside chunkset    |
| OVERSIZE   | `0x6002` | Oversize         | Inside chunkset    |
| STATEDUMP  | `0x6003` | Statedump        | Inside chunkset    |
| SIMPLEDUMP | `0x6004` | Simpledump       | Inside chunkset    |

Source: `src/constants.rs`

### 2.3 Top-Level Iteration Loop

```
pos = 0
while pos < file.len():
    preamble = read 16 bytes at pos
    chunk_end = pos + 16 + preamble.chunk_data_size
    padding = (8 - (chunk_data_size % 8)) % 8

    match preamble.chunk_tag:
        HEADER_CHUNK  → parse header (boot UUID, timezone, etc.)
        CATALOG_CHUNK → parse catalog (process metadata, UUID table, subsystems)
        CHUNKSET_CHUNK → decompress → iterate inner chunks (firehose entries)

    pos = chunk_end + padding
```

Source: `src/chunk.rs:next_top_level_chunk()`, `src/noalloc_iterator.rs:try_advance_top_level_chunk()`

---

## 3. Header Chunk

The first chunk in a tracev3 file. Provides boot identity and time context.

### 3.1 Header Data Layout (after 16-byte preamble)

```
Offset  Size  Type     Field                      Example
────────────────────────────────────────────────────────────────
+0      4     u32LE    mach_time_numerator         1 (Intel) or 125 (ARM)
+4      4     u32LE    mach_time_denominator       1 (Intel) or 3 (ARM)
+8      8     u64LE    continuous_time              mach absolute time at boot
+16     8     u64LE    unknown_time                 possibly system start time
+24     4     u32LE    unknown
+28     4     u32LE    bias_min                     timezone offset in minutes (e.g. 300)
+32     4     u32LE    daylight_savings             0=no DST, 1=DST
+36     4     u32LE    unknown_flags

── Sub-chunk 0x6100 (time) ──
+40     4     u32LE    sub_chunk_tag                0x6100
+44     4     u32LE    sub_chunk_data_size          8
+48     8     u64LE    sub_chunk_continuous_time

── Sub-chunk 0x6101 (build info) ──
+56     4     u32LE    sub_chunk_tag_2              0x6101
+60     4     u32LE    sub_chunk_data_size_2        56
+64     4     u32LE    unknown_2
+68     4     u32LE    unknown_3
+72     16    UTF-8    build_version_string         "21A559\0..." (NUL-padded)
+88     32    UTF-8    hardware_model_string        "MacBookPro16,1\0..." (NUL-padded)

── Sub-chunk 0x6102 (boot UUID) ──
+120    4     u32LE    sub_chunk_tag_3              0x6102
+124    4     u32LE    sub_chunk_data_size_3        24
+128    16    u128BE   boot_uuid                    ← KEY: identifies this boot cycle
+144    4     u32LE    logd_pid
+148    4     u32LE    logd_exit_status

── Sub-chunk 0x6103 (timezone) ──
+152    4     u32LE    sub_chunk_tag_4              0x6103
+156    4     u32LE    sub_chunk_data_size_4        48
+160    48    UTF-8    timezone_path                "/var/db/timezone/zoneinfo/America/New_York\0..."
```

Total header data: **208 bytes** (offsets relative to after preamble).

Source: `src/header.rs:parse_header()`

---

## 4. Catalog Chunk

Stores metadata needed to resolve log entries: process info, UUID references, subsystem/category names.

### 4.1 Catalog Header (after 16-byte preamble)

```
Offset  Size  Type     Field
────────────────────────────────────────────
+0      2     u16LE    subsystem_strings_offset    offset to subsystem string table
+2      2     u16LE    process_info_offset         offset to process info entries
+4      2     u16LE    number_process_info_entries  count
+6      2     u16LE    sub_chunks_offset           offset to catalog subchunks
+8      2     u16LE    number_sub_chunks           count
+10     6     bytes    unknown                     reserved
+16     8     u64LE    earliest_firehose_timestamp
+24     N*16  u128BE[] catalog_uuids               array of UUIDs (N = subsystem_strings_offset / 16)
```

### 4.2 Process Info Entry (variable-length, repeated)

```
+0      2     u16LE    index
+2      2     u16LE    unknown
+4      2     u16LE    main_uuid_index             → index into catalog_uuids (for UUIDText)
+6      2     u16LE    dsc_uuid_index              → index into catalog_uuids (for DSC)
+8      8     u64LE    first_number_proc_id        ← key part 1 (matches firehose preamble)
+16     4     u32LE    second_number_proc_id       ← key part 2
+20     4     u32LE    pid                         actual process ID
+24     4     u32LE    effective_user_id            euid
+28     4     u32LE    unknown2
+32     4     u32LE    number_uuid_entries
+36     4     u32LE    unknown3
+40     N*20  bytes    uuid_info_entries[]          ProcessUUIDEntry (20 bytes each)
then:
        4     u32LE    number_subsystems
        4     u32LE    unknown4
        N*6   bytes    subsystem_entries[]          (6 bytes each: u16 id + u32 offset)
        ...   bytes    padding to 8-byte alignment
```

**Key concept:** The tuple `(first_proc_id, second_proc_id)` in the firehose preamble is looked up in the catalog to find the process's PID, euid, UUID files, and subsystem names.

Source: `src/catalog.rs`

---

## 5. Chunkset Chunk — Compression Layer

Chunksets wrap compressed (or uncompressed) inner chunks. This is the main container for actual log entries.

### 5.1 Chunkset Data (after 16-byte preamble)

```
Offset  Size  Type     Field
──────────────────────────────────
+0      4     u32LE    signature        "bv41" (0x31347662) = LZ4 compressed
                                        "bv4-" (0x2D347662) = uncompressed
+4      4     u32LE    uncompressed_size
```

**If compressed** (`bv41`):
```
+8      4     u32LE    block_size       size of compressed data
+12     N     bytes    compressed_data  LZ4 block (N = block_size)
```

**If uncompressed** (`bv4-`):
```
+8      N     bytes    raw_data         (N = uncompressed_size)
```

### 5.2 Inner Chunk Iteration

After decompression, the data contains a sequence of inner chunks (same preamble format). **Difference from top-level:** padding is zero-byte (skip `\0` bytes) instead of 8-byte alignment.

```
pos = 0
while pos < decompressed.len():
    preamble = read 16 bytes at pos
    chunk_end = pos + 16 + preamble.chunk_data_size

    match preamble.chunk_tag:
        FIREHOSE   (0x6001) → parse firehose preamble → yield log entries
        OVERSIZE   (0x6002) → parse oversize → cache for later reference
        SIMPLEDUMP (0x6004) → parse simpledump → yield directly
        STATEDUMP  (0x6003) → parse statedump → yield directly

    pos = chunk_end + skip_zero_bytes(remaining)
```

Source: `src/chunk.rs:next_inner_chunk()`, `src/noalloc_iterator.rs:try_advance_inner_chunk()`

---

## 6. Firehose Chunk — Log Entry Container

A firehose chunk contains a **preamble** (process context + base timestamp) followed by one or more **log entries**.

### 6.1 Firehose Preamble (48 bytes total, including 16-byte chunk preamble)

```
Offset  Size  Type     Field                         Notes
──────────────────────────────────────────────────────────────
+0      4     u32LE    chunk_tag                      0x6001
+4      4     u32LE    chunk_sub_tag
+8      8     u64LE    chunk_data_size
+16     8     u64LE    first_number_proc_id           ← catalog lookup key (part 1)
+24     4     u32LE    second_number_proc_id          ← catalog lookup key (part 2)
+28     1     u8       ttl
+29     1     u8       collapsed                      affects private data padding
+30     2     bytes    unknown                        reserved
+32     2     u16LE    public_data_size               includes 16 bytes of overhead*
+34     2     u16LE    private_data_virtual_offset    0x1000 = no private data
+36     2     u16LE    unknown2
+38     2     u16LE    unknown3
+40     8     u64LE    base_continuous_time           ← base timestamp for delta encoding
+48     ...   bytes    public_data                    ← sequential log entries start here
```

*`public_data_size` includes the 16 bytes from offset +32 to +47, so actual entry data = `public_data_size - 16` bytes starting at offset +48.

All entries within one firehose preamble share the same `(first_proc_id, second_proc_id)`, `ttl`, `collapsed`, and `base_continuous_time`.

Source: `src/chunks/firehose/firehose_log.rs:parse_firehose_preamble()`

### 6.2 Firehose Entry Header (24 bytes — each entry)

```
Offset  Size  Type     Field                         Values
──────────────────────────────────────────────────────────────
+0      1     u8       log_activity_type              0x2=Activity, 0x3=Trace, 0x4=NonActivity,
                                                       0x6=Signpost, 0x7=Loss, 0x0=Remnant (end)
+1      1     u8       log_type                       0x01=Info, 0x02=Debug, 0x03=Useraction,
                                                       0x10=Error, 0x11=Fault, 0x40-0xC2=Signpost*
+2      2     u16LE    flags                          bitmask (see §6.4)
+4      4     u32LE    format_string_location         offset into format string file
+8      8     u64LE    thread_id
+16     4     u32LE    continuous_time_delta           lower 32 bits of time delta from base
+20     2     u16LE    continuous_time_delta_upper     upper 16 bits (extends to 48-bit delta)
+22     2     u16LE    data_size                       bytes of sub-type data following this header
+24     N     bytes    sub_type_data                   (N = data_size, variable per type)
```

**Wall-clock time** = timesync lookup of `base_continuous_time + (delta_upper << 32 | delta)`.

**Entry chaining:** next entry starts at `+24 + data_size + padding_to_8(data_size)`. Loop ends when `log_activity_type == 0x0` (REMNANT_DATA) or remaining bytes < 24.

Source: `src/chunks/firehose/firehose_log.rs:parse_firehose()`, `src/noalloc_iterator.rs:parse_entry_from_data()`

### 6.3 Sub-Type Data (inside `data_size` bytes)

The structure of the sub-type data depends on `log_activity_type`. Fields are conditional on `flags`.

#### NonActivity (0x4)

```
[if FLAG_HAS_CURRENT_AID (0x0001)]   u32 activity_id + u32 sentinel
[if FLAG_HAS_PRIVATE_DATA (0x0100)]  u16 private_strings_offset + u16 private_strings_size
[if FLAG_HAS_UNKNOWN_REF (0x0008)]   u32 unknown_ref
[always]                             u32 unknown_pc_id
[formatter_flags_data]               variable (see §6.5)
[if FLAG_HAS_SUBSYSTEM (0x0200)]     u16 subsystem_value
[if FLAG_HAS_RULES (0x0400)]         u8  ttl_value
[if FLAG_HAS_OVERSIZE (0x0800)]      u32 data_ref_value  ← cross-ref to oversize chunk
```

Source: `src/chunks/firehose/nonactivity.rs`

#### Activity (0x2)

```
[always unless USERACTION log_type]  u32 activity_id + u32 sentinel (=0x80000000)
[if FLAG_HAS_UNIQUE_PID (0x0010)]    u64 pid
[if FLAG_HAS_CURRENT_AID (0x0001)]   u32 activity_id_2 + u32 sentinel_2
[if FLAG_HAS_SUBSYSTEM (0x0200)]     u32 other_current_aid + u32 sentinel_3
[always]                             u32 unknown_pc_id
[formatter_flags_data]               variable (see §6.5)
```

Source: `src/chunks/firehose/activity.rs`

#### Signpost (0x6)

```
[if FLAG_HAS_CURRENT_AID (0x0001)]   u32 activity_id + u32 sentinel
[if FLAG_HAS_PRIVATE_DATA (0x0100)]  u16 private_strings_offset + u16 private_strings_size
[always]                             u32 unknown_pc_id
[formatter_flags_data]               variable (see §6.5)
[if FLAG_HAS_SUBSYSTEM (0x0200)]     u16 subsystem_value
[always]                             u64 signpost_id
[if FLAG_HAS_RULES (0x0400)]         u8  ttl_value
[if FLAG_HAS_OVERSIZE (0x0800)]      u32 data_ref_value
[if FLAG_HAS_NAME (0x8000)]          u32 signpost_name (or u64 if large)
```

Source: `src/chunks/firehose/signpost.rs`

#### Trace (0x3)

```
[always]                             u32 unknown_pc_id
[remaining]                          message_data (reversed byte order!)
```

Trace is special: the remaining bytes after `unknown_pc_id` are stored in **reverse order** and must be reversed before parsing items.

Source: `src/chunks/firehose/trace.rs`

#### Loss (0x7)

Minimal entry. Indicates dropped log entries (no sub-type data of interest).

### 6.4 Flags Bitmask (u16)

| Bit    | Hex      | Constant             | Meaning                                    |
|--------|----------|----------------------|--------------------------------------------|
| 0      | `0x0001` | FLAG_HAS_CURRENT_AID | Activity ID pair present                   |
| 1-3    | `0x000e` | FORMATTER_FLAG_MASK  | Format string location type (see §6.5)     |
| 3      | `0x0008` | FLAG_HAS_UNKNOWN_REF | Unknown message string ref present          |
| 4      | `0x0010` | FLAG_HAS_UNIQUE_PID  | Unique PID field (activity only)            |
| 5      | `0x0020` | FORMATTER_LARGE_OFFSET | Large offset flag (u32 instead of u16)   |
| 8      | `0x0100` | FLAG_HAS_PRIVATE_DATA | Private data section follows               |
| 9      | `0x0200` | FLAG_HAS_SUBSYSTEM   | Subsystem value present                     |
| 10     | `0x0400` | FLAG_HAS_RULES       | TTL/rules value present                     |
| 11     | `0x0800` | FLAG_HAS_OVERSIZE    | Oversize data_ref present                   |
| 12     | `0x1000` | FLAG_HAS_CONTEXT_DATA | Backtrace/context data follows             |
| 15     | `0x8000` | FLAG_HAS_NAME        | Signpost name present                       |

### 6.5 Formatter Flags (where is the format string?)

The lower nibble `(flags & 0x000e)` determines where to find the printf-style format string:

| Value  | Constant                     | Meaning                                      |
|--------|------------------------------|----------------------------------------------|
| `0x2`  | FORMATTER_MAIN_EXE           | In a UUIDText file (main executable)          |
| `0x4`  | FORMATTER_SHARED_CACHE       | In a DSC file (shared cache)                  |
| `0x8`  | FORMATTER_ABSOLUTE           | Alternative UUID index from catalog            |
| `0xa`  | FORMATTER_UUID_RELATIVE      | UUID embedded inline (16 bytes follow)         |
| `0xc`  | FORMATTER_LARGE_SHARED_CACHE | Large shared cache offset                      |

Additional data appended by formatter type:
- `0x2` (MAIN_EXE): 0 extra bytes
- `0x4` (SHARED_CACHE): 0-2 bytes (u16 if LARGE_OFFSET set)
- `0x8` (ABSOLUTE): 0-2 bytes (u16 alt_uuid_index if not also MAIN_EXE)
- `0xa` (UUID_RELATIVE): 16 bytes (inline u128BE UUID)
- `0xc` (LARGE_SHARED): 2-4 bytes depending on LARGE_OFFSET

Source: `src/chunks/firehose/flags.rs`

### 6.6 After Sub-Type: Items and Message Data

After the sub-type-specific fields, the remaining data in `data_size` bytes contains:

```
+0      1     u8       unknown_item       (purpose unclear)
+1      1     u8       number_items       count of firehose items
+2      ...   bytes    item_data[]        see below
```

Each **firehose item** is:
```
+0      1     u8       item_type          (see item type table)
+1      1     u8       item_size          size in bytes
+2      2     u16LE    offset             (for string/object types: offset into string area)
+4      2     u16LE    message_string_size (for string/object types)
```

Number items (type 0x00, 0x01, 0x02, 0x10, 0x12) only have `item_type` + `item_size`, then the number data follows inline.

**Item Types:**

| Type   | Hex    | Name                |
|--------|--------|---------------------|
| Number | `0x00` | ITEM_NUMBER         |
| Private number | `0x01` | ITEM_PRIVATE_NUMBER |
| Precision | `0x10` | ITEM_PRECISION   |
| String | `0x20` | ITEM_STRING         |
| Private string | `0x21` | ITEM_PRIVATE_STRING |
| Arbitrary | `0x30` | ITEM_ARBITRARY   |
| Object | `0x40` | ITEM_OBJECT         |
| Private object | `0x41` | ITEM_PRIVATE_OBJECT |
| Base64 raw | `0xf2` | ITEM_BASE64_RAW |

If `FLAG_HAS_CONTEXT_DATA (0x1000)` is set, backtrace data follows the items.

### 6.7 Private Data

If `private_data_virtual_offset != 0x1000`, private string data is located after the public data section in the preamble. The offset calculation:

```
private_data_start = public_data_end + zero_padding
actual_offset = private_strings_offset - private_data_virtual_offset
```

Private data contains the actual string values for items marked as private (types 0x01, 0x21, 0x25, 0x35, 0x41, 0x81, 0xf1).

---

## 7. Oversize Chunk

For log messages too large to fit in a standard firehose entry. Referenced by `data_ref_value` in NonActivity/Signpost entries.

```
Offset  Size  Type     Field
──────────────────────────────────
+0      16    bytes    chunk preamble (tag=0x6002)
+16     8     u64LE    first_proc_id
+24     4     u32LE    second_proc_id
+28     1     u8       ttl
+29     3     bytes    unknown/reserved
+32     8     u64LE    continuous_time
+40     4     u32LE    data_ref_index       ← matched by firehose entry's data_ref_value
+44     2     u16LE    public_data_size
+46     2     u16LE    private_data_size
+48     N     bytes    message_data         (N = public_data_size + private_data_size)
```

The `message_data` contains: `u8 unknown + u8 item_count + items[]` (same format as firehose items).

**Lookup:** `(data_ref_index, first_proc_id, second_proc_id)` must all match.

**Important:** Oversize entries can appear in earlier tracev3 files than the firehose entry that references them, so an oversize cache must be carried across files.

Source: `src/chunks/oversize.rs`

---

## 8. Simpledump & Statedump Chunks

### 8.1 Simpledump (0x6004)

Lightweight metadata capture (Monterey+).

```
+16     8     u64LE    first_proc_id
+24     8     u64LE    second_proc_id
+32     8     u64LE    continuous_time
+40     8     u64LE    thread_id
+48     4     u32LE    unknown_offset
+52     2     u16LE    unknown_ttl
+54     2     u16LE    unknown_type
+56     16    bytes    sender_uuid
+72     16    bytes    dsc_uuid
+88     4     u32LE    unknown_number_message_strings
+92     4     u32LE    subsystem_string_size
+96     4     u32LE    message_string_size
+100    N     UTF-8    subsystem_string     (NUL-terminated)
+100+N  M     UTF-8    message_string       (NUL-terminated)
```

Source: `src/chunks/simpledump.rs`

### 8.2 Statedump (0x6003)

Complex state snapshots (plist, protobuf, or custom objects).

```
+16     8     u64LE    first_proc_id
+24     4     u32LE    second_proc_id
+28     1     u8       ttl
+29     3     bytes    unknown/reserved
+32     8     u64LE    continuous_time
+40     8     u64LE    activity_id
+48     16    bytes    uuid
+64     4     u32LE    data_type            1=plist, 2=protobuf, 3=custom object
+68     4     u32LE    data_size
[if data_type == 3:]
+72     64    UTF-8    decoder_library      (NUL-padded)
+136    64    UTF-8    decoder_type         (NUL-padded)
+200    64    UTF-8    title_name           (NUL-padded)
+264    N     bytes    statedump_data       (N = data_size)
[else:]
+72     64    UTF-8    title_name           (NUL-padded)
+136    N     bytes    statedump_data       (N = data_size)
```

Source: `src/chunks/statedump.rs`

---

## 9. Supporting Files

### 9.1 Timesync Files

Map kernel continuous time to wall-clock timestamps.

**Boot record:**
```
+0      2     u16LE    signature            0xBBB0
+2      2     u16LE    header_size
+4      4     u32LE    unknown
+8      16    u128BE   boot_uuid            ← matches header chunk's boot_uuid
+24     4     u32LE    timebase_numerator    1 (Intel) or 125 (ARM)
+28     4     u32LE    timebase_denominator  1 (Intel) or 3 (ARM)
+32     8     i64LE    boot_time             nanoseconds since Unix epoch
+40     4     u32LE    timezone_offset_mins
+44     4     u32LE    daylight_savings
```

**Sync record** (repeats after boot record):
```
+0      4     u32LE    signature            0x00207354
+4      4     u32LE    unknown_flags
+8      8     u64LE    kernel_time           mach continuous_time
+16     8     i64LE    walltime              nanoseconds since Unix epoch
+24     4     u32LE    timezone
+28     4     u32LE    daylight_savings
```

**Timestamp calculation:**
```
adjustment = timebase_numerator / timebase_denominator  (1.0 on Intel, ~41.67 on ARM)
delta = (entry_continuous_time - nearest_sync.kernel_time) * adjustment
wall_clock = nearest_sync.walltime + delta
```

Source: `src/timesync.rs`

### 9.2 DSC Files (Shared Cache Strings)

Deduplicated format strings shared across all processes.

**Header:**
```
+0      4     u32LE    signature            0x64736368 ("hcsd" reversed)
+4      2     u16LE    major_version         1 (≤Big Sur) or 2 (Monterey+)
+6      2     u16LE    minor_version
+8      4     u32LE    number_ranges
+12     4     u32LE    number_uuids
```

**Range descriptors** (v1: 16 bytes each, v2: 24 bytes each):
- `range_offset`: where in file the string data starts
- `data_offset`: offset within that range
- `range_size`: size of the string data
- `unknown_uuid_index`: which UUID owns this range

**UUID descriptors** (v1: 28 bytes each, v2: 32 bytes each):
- `text_offset`: offset in file to this UUID's string table
- `text_size`: size (typically 8192)
- `uuid`: 16-byte big-endian UUID
- `path_offset`: offset to the library's path string

Source: `src/dsc.rs`

### 9.3 UUIDText Files

Per-binary format string storage. Filename IS the UUID (e.g., `uuidtext/3D/05845F3F65358F9EBF2236E772AC01`).

**Header:**
```
+0      4     u32LE    signature            0x66778899
+4      4     u32LE    major_version         typically 2
+8      4     u32LE    minor_version         typically 1
+12     4     u32LE    number_entries
```

**Entry descriptors** (8 bytes each, repeated `number_entries` times):
```
+0      4     u32LE    range_start_offset
+4      4     u32LE    entry_size
```

String data follows the entry table. Each entry is at `range_start_offset` with `entry_size` bytes.

Source: `src/uuidtext.rs`

---

## 10. Complete Parsing Pipeline

```
                        ┌─────────────────┐
                        │  timesync files  │──→ HashMap<boot_uuid, TimesyncBoot>
                        └────────┬────────┘
                                 │
                                 ▼
┌──────────────┐    ┌────────────────────────┐
│  tracev3     │    │   Iteration Loop       │
│  file bytes  │──→ │                        │
└──────────────┘    │  1. HEADER chunk       │──→ boot_uuid, timezone
                    │     ↓                  │
                    │  2. CATALOG chunk      │──→ process info, UUID table, subsystems
                    │     ↓                  │
                    │  3. CHUNKSET chunk     │──→ decompress (LZ4 or raw)
                    │     │                  │
                    │     ├─ FIREHOSE chunk  │──→ preamble → entries (see below)
                    │     ├─ OVERSIZE chunk  │──→ cache for later lookup
                    │     ├─ SIMPLEDUMP      │──→ yield directly
                    │     └─ STATEDUMP       │──→ yield directly
                    │                        │
                    │  repeat 2-3 until EOF  │
                    └────────────────────────┘

For each firehose entry:
  ┌──────────────────────────────────┐
  │  entry header (24 bytes)         │
  │  + sub-type data (variable)      │
  │  + items + private data          │
  └──────────────┬───────────────────┘
                 │
      ┌──────────┴──────────┐
      │                     │
      ▼                     ▼
  ┌────────┐          ┌──────────┐
  │ DSC    │          │ UUIDText │
  │ files  │          │ files    │
  └────┬───┘          └────┬─────┘
       │                   │
       └───────┬───────────┘
               ▼
     format_string (printf-style)
               │
               ▼
     expand with item values
               │
               ▼
     final log message string
```

**Key insight:** The tracev3 file is self-contained for scalar data (timestamps, PIDs, flags, item values). But **string resolution** (process name, library name, format string, subsystem/category) requires external files (DSC, UUIDText) and the catalog's UUID table to locate them.

---

## Verification

This document can be verified against the actual codebase:
- Constants: `src/constants.rs`
- Preamble: `src/preamble.rs`
- Header: `src/header.rs` (test at line 190 has real byte data)
- Catalog: `src/catalog.rs`
- Chunkset/Chunk iteration: `src/chunk.rs`, `src/noalloc_iterator.rs`
- Firehose preamble + entries: `src/chunks/firehose/firehose_log.rs`
- Sub-types: `src/chunks/firehose/{nonactivity,activity,signpost,trace}.rs`
- Flags: `src/chunks/firehose/flags.rs`
- Oversize: `src/chunks/oversize.rs`
- Simpledump: `src/chunks/simpledump.rs`
- Statedump: `src/chunks/statedump.rs`
- Timesync: `src/timesync.rs`
- DSC: `src/dsc.rs`
- UUIDText: `src/uuidtext.rs`
