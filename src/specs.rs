//! Constants & notes from the Unified Logging and Activity Tracing documentation.
//! https://github.com/libyal/dtformats/blob/main/documentation/Apple%20Unified%20Logging%20and%20Activity%20Tracing%20formats.asciidoc#2-tracev3-file-format

#![allow(dead_code)]

/// **Chunk tag types**
/// https://github.com/libyal/dtformats/blob/main/documentation/Apple%20Unified%20Logging%20and%20Activity%20Tracing%20formats.asciidoc#22-chunk-tag-types
pub mod chunck_tag_types{
    /// Header
    pub const HEADER: u16 = 0x1000;
    /// Firehose
    pub const FIREHOSE: u16 = 0x6001;
    /// Oversize
    pub const OVERSIZE: u16 = 0x6002;
    /// StateDump
    pub const STATEDUMP: u16 = 0x6003;
    /// SimpleDump
    pub const SIMPLEDUMP: u16 = 0x6004;
    /// Catalog
    pub const CATALOG: u16 = 0x600b;
    /// ChunkSet
    pub const CHUNKSET: u16 = 0x600d;
} 

/// **Header flags**
/// https://github.com/libyal/dtformats/blob/main/documentation/Apple%20Unified%20Logging%20and%20Activity%20Tracing%20formats.asciidoc#231-header-flags
pub mod header_flags {
    /// Is 64-bits
    pub const IS_64BITS: u32 = 0x00000001;
    /// Unknown (Is boot what?)
    pub const IS_BOOT: u32 = 0x00000002;
}

/// **Firehose stream type**
/// https://github.com/libyal/dtformats/blob/main/documentation/Apple%20Unified%20Logging%20and%20Activity%20Tracing%20formats.asciidoc#28-stream-type
pub mod firehose_stream_type {
    /// Persist
    pub const PERSIST: u8 = 0x00;
    /// Special handling
    pub const SPECIAL_HANDLING: u8 = 0x01;
    /// Memory
    pub const MEMORY: u8 = 0x02;
    /// Signpost
    pub const SIGNPOST: u8 = 0x04;
}


/// **Firehose tracepoint record type**
/// https://github.com/libyal/dtformats/blob/main/documentation/Apple%20Unified%20Logging%20and%20Activity%20Tracing%20formats.asciidoc#2101-firehose-tracepoint-record-type
/// 
/// | Value | Identifier | Description |
/// |-------|------------|-------------|
/// | 0x00 | | Unused (or Empty) |
/// | | | |
/// | 0x02 | activity | Activity |
/// | 0x03 | trace | Trace |
/// | 0x04 | log | Log |
/// | | | |
/// | 0x06 | | Signpost |
/// | 0x07 | | Loss |
pub mod firehose_record_type{
    /// Unused (or Empty)
    pub const UNUSED: u8 = 0x00;
    /// Activity
    pub const ACTIVITY: u8 = 0x02;
    /// Trace
    pub const TRACE: u8 = 0x03;
    /// Log
    pub const LOG: u8 = 0x04;
    /// Signpost
    pub const SIGNPOST: u8 = 0x06;
    /// Loss
    pub const LOSS: u8 = 0x07;
}


/// **Firehose tracepoint flags**
/// https://github.com/libyal/dtformats/blob/main/documentation/Apple%20Unified%20Logging%20and%20Activity%20Tracing%20formats.asciidoc#2102-firehose-tracepoint-flags
///  
/// | Value | Identifier | Description |
/// |-------|------------|-------------|
/// | 0x0001 | has_current_aid | Has current activity identifier |
/// | 0x000e | | Strings file type |
/// | 0x0010 | has_unique_pid | Has process identifier value |
/// | 0x0020 | has_large_offset | Has large offset data |
/// | | | |
/// | 0x0100 | has_private_data | Has private data range |
/// | 0x0200 | has_other_aid | Has other activity identifier. Note that appears to be only used by the "Activity" record type, other record types use has_subsystem |
/// | 0x0200 | has_subsystem | Has sub system |
/// | 0x0400 | has_rules | Has rules |
/// | 0x0800 | has_oversize | Has oversize data reference |
/// | 0x1000 | has_backtrace | Has backtrace |
/// | | | |
/// | 0x8000 | has_name | Has name string reference |
pub mod firehose_tracepoint_flags {
    /// Has current activity identifier
    pub const HAS_CURRENT_AID: u16 = 0x0001;
    /// Strings file type
    pub const STRINGS_FILE_TYPE: u16 = 0x000e;
    /// Has process identifier value
    pub const HAS_UNIQUE_PID: u16 = 0x0010;
    /// Has large offset data
    pub const HAS_LARGE_OFFSET: u16 = 0x0020;
    /// Has private data range
    pub const HAS_PRIVATE_DATA: u16 = 0x0100;
    /// Has other activity identifier. Note that appears to be only used by the "Activity" record type, other record types use has_subsystem
    pub const HAS_OTHER_AID: u16 = 0x0200;
    /// Has sub system
    pub const HAS_SUBSYSTEM: u16 = 0x0200;
    /// Has rules
    pub const HAS_RULES: u16 = 0x0400;
    /// Has oversize data reference
    pub const HAS_OVERSIZE: u16 = 0x0800;
    /// Has backtrace
    pub const HAS_BACKTRACE: u16 = 0x1000;
    /// Has name string reference
    pub const HAS_NAME: u16 = 0x8000;
}

/// **Firehose tracepoint strings file type**
/// https://github.com/libyal/dtformats/blob/main/documentation/Apple%20Unified%20Logging%20and%20Activity%20Tracing%20formats.asciidoc#2102-firehose-tracepoint-flags
/// 
/// | Value | Identifier | Description |
/// |-------|------------|-------------|
/// | 0x0002 | main_exe | Strings are stored in an uuidtext file by proc_id |
/// | 0x0004 | shared_cache | Strings are stored in a Shared-Cache Strings (dsc) file |
/// | 0x0008 | absolute | Strings are stored in an uuidtext file by reference |
/// | 0x000a | uuid_relative | Strings are stored in an uuidtext file by identifier |
/// | 0x000c | large_shared_cache | Strings are stored in a Shared-Cache Strings (dsc) file |
pub mod firehose_tracepoint_strings_file_type {
    /// Strings are stored in an uuidtext file by proc_id
    pub const MAIN_EXE: u16 = 0x0002;
    /// Strings are stored in a Shared-Cache Strings (dsc) file
    pub const SHARED_CACHE: u16 = 0x0004;
    /// Strings are stored in an uuidtext file by reference
    pub const ABSOLUTE: u16 = 0x0008;
    /// Strings are stored in an uuidtext file by identifier
    pub const UUID_RELATIVE: u16 = 0x000a;
    /// Strings are stored in a Shared-Cache Strings (dsc) file
    pub const LARGE_SHARED_CACHE: u16 = 0x000c;
}


/// **Firehose tracepoint data item type**
/// https://github.com/libyal/dtformats/blob/main/documentation/Apple%20Unified%20Logging%20and%20Activity%20Tracing%20formats.asciidoc#value-type
/// 
/// | Value | Identifier | Description |
/// |-------|------------|-------------|
/// | 0x00 | | Unknown (integer or floating-point value) - Contains a 32-bit or 64-bit value |
/// | 0x01 | | Unknown (private value) - Contains a 32-bit value, formatted as "<private>" |
/// | 0x02 | | Unknown (integer or floating-point value) - Contains a 8-bit, 16-bit, 32-bit or 64-bit value |
/// | | | |
/// | 0x10 | | Unknown (integer format precision) - Contains a 32-bit value. This value has been seen to be used in combination with format strings like "%.0lld" |
/// | | | |
/// | 0x12 | | Unknown (string format precision) - Contains a 32-bit value. This value has been seen to be used in combination with format strings like "%.16s" and "%.*s", where this value contains the number of characters of the string that should be printed. |
/// | | | |
/// | 0x20 | | Unknown (string) - Consists of a Firehose tracepoint data item with value data range where the value data contains an UTF-8 encoded string with an optional end-of-string character. |
/// | 0x21 | | Unknown (private value) - Consists of a Firehose tracepoint data item with private data range where the value data contains an UTF-8 encoded string with an optional end-of-string character. |
/// | 0x22 | | Unknown (string) - Consists of a Firehose tracepoint data item with value data range where the value data contains an UTF-8 encoded string with an optional end-of-string character. |
/// | | | |
/// | 0x25 | | Unknown (sensitive value) - Contains a 32-bit value, formatted as "<private>" |
/// | | | |
/// | 0x30 | | Unknown (binary data) - Consists of a Firehose tracepoint data item with value data range where the value data contains binary data. |
/// | 0x31 | | Unknown (private value) - Contains a 32-bit value, formatted as "<private>" |
/// | 0x32 | | Unknown (binary data) - Consists of a Firehose tracepoint data item with value data range where the value data contains binary data. |
/// | | | |
/// | 0x35 | | Unknown (private value?) |
/// | | | |
/// | 0x40 | | Unknown (string) - Consists of a Firehose tracepoint data item with value data range where the value data contains an UTF-8 encoded string with an optional end-of-string character. |
/// | 0x41 | | Unknown (private value) - Consists of a Firehose tracepoint data item with private data range where the value data contains an UTF-8 encoded string with an optional end-of-string character. |
/// | 0x42 | | Unknown (string) - Consists of a Firehose tracepoint data item with value data range where the value data contains an UTF-8 encoded string with an optional end-of-string character. |
/// | | | |
/// | 0x45 | | Unknown (private value?) |
/// | | | |
/// | 0xf2 | | Unknown (binary data) - Consists of a Firehose tracepoint data item with value data range where the value data contains binary data. |
pub mod firehose_tracepoint_data_item_type {
    /// Unknown (integer or floating-point value) - Contains a 32-bit or 64-bit value
    pub const UNKNOWN: u8 = 0x00;
    /// Unknown (private value) - Contains a 32-bit value, formatted as "<private>"
    pub const UNKNOWN_PRIVATE_1: u8 = 0x01;
    /// Unknown (integer or floating-point value) - Contains a 8-bit, 16-bit, 32-bit or 64-bit value
    pub const UNKNOWN_2: u8 = 0x02;
    /// Unknown (integer format precision) - Contains a 32-bit value. This value has been seen to be used in combination with format strings like "%.0lld"
    pub const UNKNOWN_INTEGER_FORMAT_PRECISION: u8 = 0x10;
    /// Unknown (string format precision) - Contains a 32-bit value. This value has been seen to be used in combination with format strings like "%.16s" and "%.*s", where this value contains the number of characters of the string that should be printed.
    pub const UNKNOWN_STRING_FORMAT_PRECISION: u8 = 0x12;
    /// Unknown (string) - Consists of a Firehose tracepoint data item with value data range where the value data contains an UTF-8 encoded string with an optional end-of-string character.
    pub const UNKNOWN_STRING: u8 = 0x20;
    /// Unknown (private value) - Consists of a Firehose tracepoint data item with private data range where the value data contains an UTF-8 encoded string with an optional end-of-string character.
    pub const UNKNOWN_PRIVATE_2: u8 = 0x21;
    /// Unknown (string) - Consists of a Firehose tracepoint data item with value data range where the value data contains an UTF-8 encoded string with an optional end-of-string character.
    pub const UNKNOWN_STRING_2: u8 = 0x22;
    /// Unknown (sensitive value) - Contains a 32-bit value, formatted as "<private>"
    pub const SENSITIVE: u8 = 0x25;
    /// Unknown (binary data) - Consists of a Firehose tracepoint data item with value data range where the value data contains binary data.
    pub const BINARY_DATA: u8 = 0x30;
    /// Unknown (private value) - Contains a 32-bit value, formatted as "<private>"
    pub const PRIVATE_3: u8 = 0x31;
    /// Unknown (binary data) - Consists of a Firehose tracepoint data item with value data range where the value data contains binary data.

}