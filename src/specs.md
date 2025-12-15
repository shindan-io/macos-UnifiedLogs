> The Apple Unified Logging and Activity Tracing formats are used to
> store system logs. This specification is based on the source code and
> documentation.
>
> This document is intended as a working document for the Apple Unified
> Logging and Activity Tracing file formats specification.

# Document information

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>Author(s):</p></td>
<td style="text-align: left;"><p>Joachim Metz &lt;<a
href="mailto:joachim.metz@gmail.com">joachim.metz@gmail.com</a>&gt;</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>Abstract:</p></td>
<td style="text-align: left;"><p>Apple Unified Logging and Activity
Tracing formats</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>Classification:</p></td>
<td style="text-align: left;"><p>Public</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>Keywords:</p></td>
<td style="text-align: left;"><p>Unified Logging, tracev3,
uuidtext</p></td>
</tr>
</tbody>
</table>

# License

    Copyright (C) 2019-2023, Joachim Metz <joachim.metz@gmail.com>.
    Permission is granted to copy, distribute and/or modify this document under the
    terms of the GNU Free Documentation License, Version 1.3 or any later version
    published by the Free Software Foundation; with no Invariant Sections, no
    Front-Cover Texts, and no Back-Cover Texts. A copy of the license is included
    in the section entitled "GNU Free Documentation License".

# Revision history

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Version</th>
<th style="text-align: left;">Author</th>
<th style="text-align: left;">Date</th>
<th style="text-align: left;">Comments</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0.0.1</p></td>
<td style="text-align: left;"><p>J.B. Metz</p></td>
<td style="text-align: left;"><p>January 2019</p></td>
<td style="text-align: left;"><p>Initial version.</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0.0.2</p></td>
<td style="text-align: left;"><p>J.B. Metz</p></td>
<td style="text-align: left;"><p>February 2019</p></td>
<td style="text-align: left;"><p>Additional changes based on work by Y.
Khatri.</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0.0.3</p></td>
<td style="text-align: left;"><p>Y. Khatri</p></td>
<td style="text-align: left;"><p>February 2019</p></td>
<td style="text-align: left;"><p>Additional changes based prior
work.</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0.0.4</p></td>
<td style="text-align: left;"><p>J.B. Metz</p></td>
<td style="text-align: left;"><p>February 2019</p></td>
<td style="text-align: left;"><p>Additional changes based on format
analysis.</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0.0.5</p></td>
<td style="text-align: left;"><p>J.B. Metz</p></td>
<td style="text-align: left;"><p>March 2019</p></td>
<td style="text-align: left;"><p>Additional changes based on format
analysis.</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0.0.6</p></td>
<td style="text-align: left;"><p>R. Asselin</p></td>
<td style="text-align: left;"><p>July 2022</p></td>
<td style="text-align: left;"><p>Additional changes based on format
analysis.</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0.0.7</p></td>
<td style="text-align: left;"><p>J.B. Metz</p></td>
<td style="text-align: left;"><p>August 2022</p></td>
<td style="text-align: left;"><p>Additional changes based on format
analysis.</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0.0.8</p></td>
<td style="text-align: left;"><p>Fry</p></td>
<td style="text-align: left;"><p>November 2022</p></td>
<td style="text-align: left;"><p>Additional changes based on format
analysis.</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0.0.9</p></td>
<td style="text-align: left;"><p>J.B. Metz</p></td>
<td style="text-align: left;"><p>May 2023</p></td>
<td style="text-align: left;"><p>Additional changes based on format
analysis.</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0.0.10</p></td>
<td style="text-align: left;"><p>J.B. Metz</p></td>
<td style="text-align: left;"><p>June 2023</p></td>
<td style="text-align: left;"><p>Additional changes based on format
analysis.</p></td>
</tr>
</tbody>
</table>

# Overview

Apple Unified Logging and Activity Tracing was introduced in macOS 10.12
(Sierra) and are used to store system logs.

Apple Unified Logging and Activity Tracing consists of:

-   Diagnostics data

-   Shared-Cache Strings data

# Diagnostics data

The diagnostics data can be stored in a logarchive (directory), as part
of a sysdiagnose dump for iOS, or on a Mac OS in the directory:

    /private/var/db/diagnostics/
    /var/db/diagnostics/

The diagnostics directory contains the following sub directories:

-   HighVolume

-   Persist

-   Signpost

-   Special

-   timesync

And files:

-   logdata.statistics.\[0-9\].txt

-   logd.\[0-9\].log

-   shutdown.log

-   version.plist

**<span class="yellow-background">TODO describe what HighVolume sub
directory contains</span>**

The Persist, Signpost and Special sub directories can contain zero or
more files with the extension .tracev3. These files are stored in the
[tracev3 file format](#tracev3_file_format).

The timesync sub directory can contain one or more files with the
extension .timesync. These files are stored in the [timesync database
file format](#timesync_database_file_format).

## logd.\[0-9\].log

The logd.log files contain logging messages from the file logging daemon
itself. They are in the single text line format:

    timestamp logd[pid]: message

For example:

    2022-10-23 19:15:32-0700 logd[25]: Unable to parse os_log_simple buffer. Error: 94 Received: 89

## logdata.statistics.\[0-9\].txt

The logdata.statistics files contain various statistics and metrics kept
by the logging daemon, about both individual tracev3 files and the
operations of the daemon itself.

These are delimited by headers of the form `--- !logd <type> record`
where type is one of:

-   purge

-   snapshot

-   statistics

-   uuid purge

### purge logdata statistics

The purge logdata statistics consist of:

-   Records of type "Persist Periodic Purge" detail tracev3 files in the
    `Persist/` subdirectory that have been purged, including the size of
    the file and how many files in the `Persist/` subdirectory remain.

-   Records of type "Signpost Periodic Compaction" lists the files that
    have undergone compaction, all observed have been in the `Signpost/`
    subdirectory.

-   Records of type "Special Periodic Compaction" lists the files that
    have undergone compaction, all observed have been in the `Special/`
    subdirectory.

### snapshot logdata statistics

The purpose of the snapshot logdata statistics is currently unknown. An
example of a log record:

    --- !logd snapshot record
    client : /usr/libexec/signpost_reporter
    time   : 2022-11-20 03:04:48+0000
    flags  : 0x3

### statistics logdata statistics

The statistics logdata statistics consist of:

-   Records of type "File Rotate" list, per file, how many records it
    contains and a list of each process that produced records in the
    file, sorted descendingly by total record count. Example:

-   Records of type "Memory Rollover" appear to be total number of
    records, and a list of each process that produced records, but
    globally and not for any particular file.

### uuid purge logdata statistics

The uuid perger logdata statistics consist of:

-   List of UUIDs that have been purged

## shutdown.log

shutdown.log contains a period list of the clients that still exist
while the system is attempting to shut down.

The logs are of the form:

    After <x>s, these clients are still here:
                    remaining client pid: <pid> (/<process>/<uuid>) (repeats)
    SIGTERM: [<timestamp>] All buffers flushed

## version.plist

**<span class="yellow-background">TODO describe what version.plist file
contains</span>**

# Shared-Cache Strings data

The Shared-Cache Strings data can be stored in a logarchive (directory)
or on a Mac OS system in the directory:

    /private/var/db/uuidtext/
    /var/db/uuidtext/

The uuidtext directory contains the following sub directories:

-   `[0-9A-F][0-9A-F]`

-   dsc

# `[0-9A-F][0-9A-F]` sub directory

The `[0-9A-F][0-9A-F]` sub directory contains zero or more files:

-   `[0-9A-F]{30}`

This path relates to an UUID, for example:

    AB/414C1EC0233A05AF22029CC5E160EA represents AB414C1E-C023-3A05-AF22-029CC5E160EA

# dsc sub directory

The dsc sub directory contains one or more files:

-   `[0-9A-F]{32}`

This filename relates to an UUID, for example:

    2576AD2587533C119308541E6149A88D represents 2576AD25-8753-3C11-9308-541E6149A88D

These files are store in the [Shared-Cache strings (dsc) file
format](#shared_cache_strings_file_format)

# Test versions

The following version of programs were used to test the information
within this document:

-   macOS 10.12 (Sierra)

-   macOS 10.13 (High Sierra)

-   macOS 10.14 (Mojave)

-   macOS 10.15 (Catalina)

-   macOS 11 (Big Sur)

-   macOS 12 (Monterey)

-   macOS 13 (Ventura)

# <span id="tracev3_file_format"></span>tracev3 file format

A tracev3 file consists of:

-   header chunk

-   one or more:

    -   catalog chunk

    -   chunk sets related to the catalog chunk

Note that a new catalog chunk overwrites the previous catalog.

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Characteristics</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>Byte order</p></td>
<td style="text-align: left;"><p>little-endian</p></td>
</tr>
</tbody>
</table>

# tracev3 chunk

A tracev3 chunk (tracev3\_chunk) is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk header
(tracev3_chunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk tag (tag)<br />
See section: <a href="#chunk_tag_types">Chunk tag types</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk sub tag (subtag)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk data size (length)<br />
<strong><span class="yellow-background">Note that only the lower 32-bit
has been observed to be used, so this could be 2 32-bit values as
well</span></strong></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>chunk_data_size</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk data</p></td>
</tr>
</tbody>
</table>

The chunk header is stored 64-bit aligned.

# <span id="chunk_tag_types"></span>Chunk tag types

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x1000</p></td>
<td style="text-align: left;"><p>Header</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x6001</p></td>
<td style="text-align: left;"><p>Firehose</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x6002</p></td>
<td style="text-align: left;"><p>Oversize</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x6003</p></td>
<td style="text-align: left;"><p>StateDump</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x6004</p></td>
<td style="text-align: left;"><p>SimpleDump</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x600b</p></td>
<td style="text-align: left;"><p>Catalog</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x600d</p></td>
<td style="text-align: left;"><p>ChunkSet</p></td>
<td style="text-align: left;"></td>
</tr>
</tbody>
</table>

# Header (chunk)

The header (chunk) is 224 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk header
(tracev3_chunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x1000</p></td>
<td style="text-align: left;"><p>Chunk tag (tag)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x0011</p></td>
<td style="text-align: left;"><p>Chunk sub tag (subtag)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>208</p></td>
<td style="text-align: left;"><p>Chunk data size (length)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk data
(tracev3_chunk_header)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(Mach) Timebase numerator (first number
in timebase # / #)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>20</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(Mach) Timebase denominator (second
number in timebase # / #)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Start time<br />
Contains a Mach continuous timestamp</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Timestamp (or wall clock time)<br />
Signed integer that contains the number of seconds since January 1, 1970
00:00:00 UTC (POSIX epoch), disregarding leap seconds</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>36</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>40</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>44</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Time zone offset in minutes<br />
Contains a signed integer that contains the number of minutes relative
from UTC, for example -60 represents UTC+1</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>48</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Daylight savings time (DST) flag<br />
0 = daylight savings time is not active (no-DST)<br />
1 = daylight savings time is active (DST)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>52</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><a href="#tracev3_header_flags">Header
flags</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>56</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><a
href="#header_continuous_time_sub_chunk">Header continuous time sub
chunk</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>72</p></td>
<td style="text-align: left;"><p>64</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><a
href="#header_system_information_sub_chunk">Header system information
sub chunk</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>136</p></td>
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><a
href="#header_generation_sub_chunk">Header generation sub
chunk</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>168</p></td>
<td style="text-align: left;"><p>56</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><a
href="#header_time_zone_sub_chunk">Header time zone sub
chunk</a></p></td>
</tr>
</tbody>
</table>

Timebase numerator / Timebase denominator is the timebase, which
contains the number of seconds per continuous time unit.

The start time contains the Mach continuous time representation of the
POSIX timestamp (or wall clock time).

Timestamp appears to be stored in UTC but the log tool shows the local
time zone.

## <span id="tracev3_header_flags"></span>Header flags

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x00000001</p></td>
<td style="text-align: left;"><p>64bits</p></td>
<td style="text-align: left;"><p>Is 64-bits</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x00000002</p></td>
<td style="text-align: left;"><p>is_boot</p></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Is boot
what?)</span></strong></p></td>
</tr>
</tbody>
</table>

Note that files without the "64bits" header flag have not yet been
observed.

## <span id="header_continuous_time_sub_chunk"></span>Header continuous time sub chunk

The header continuous time sub chunk is 16 bytes of size and consist of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>The sub chunk header
(tracev3_subchunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x6100</p></td>
<td style="text-align: left;"><p>Sub chunk tag</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sub chunk data size<br />
The size value does not include the 8 bytes of the sub chunk tag and
data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>The continuous time sub
chunk data (tracev3_subchunk_continuous)</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Continuous time<br />
Contains a Mach continuous timestamp</p></td>
</tr>
</tbody>
</table>

## <span id="header_system_information_sub_chunk"></span>Header system information sub chunk

The header system information sub chunk is 64 bytes of size and consist
of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>The sub chunk header
(tracev3_subchunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x6101</p></td>
<td style="text-align: left;"><p>Sub chunk tag</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sub chunk data size<br />
The size value does not include the 8 bytes of the sub chunk tag and
data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>The system information
sub chunk data (tracev3_subchunk_systeminfo)</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Build version<br />
Contains an UTF-8 encoded string with end-of-string character</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Hardware model string<br />
Contains an UTF-8 encoded string with end-of-string character</p></td>
</tr>
</tbody>
</table>

One of the unknowns is likely architecture like (x86\_64h).

## <span id="header_generation_sub_chunk"></span>Header generation sub chunk

The header generation sub chunk is 32 bytes of size and consist of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>The sub chunk header
(tracev3_subchunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x6102</p></td>
<td style="text-align: left;"><p>Sub chunk tag</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sub chunk data size<br />
The size value does not include the 8 bytes of the sub chunk tag and
data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>The generation sub
chunk data (tracev3_subchunk_generation)</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Boot identifier (Boot UUID)<br />
Contains a UUID stored in big-endian</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Process identifier (pid) of
logd</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Exit status (of logd)</p></td>
</tr>
</tbody>
</table>

## <span id="header_time_zone_sub_chunk"></span>Header time zone sub chunk

The time zone generation sub chunk is 56 bytes of size and consist of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>The sub chunk header
(tracev3_subchunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x6103</p></td>
<td style="text-align: left;"><p>Sub chunk tag</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sub chunk data size<br />
The size value does not include the 8 bytes of the sub chunk tag and
data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>The time zone sub chunk
data (tracev3_subchunk_timezone)</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>48</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Path to time zone information
file<br />
Contains an UTF-8 encoded string with end-of-string character</p></td>
</tr>
</tbody>
</table>

# Catalog chunk

The Catalog chunk is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk header
(tracev3_chunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x600b</p></td>
<td style="text-align: left;"><p>Chunk tag (tag)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk sub tag (subtag)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk data size (length)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk data
(tracev3_chunk_catalog_v2)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Offset of the catalog sub system
strings<br />
The offset is relative to the start of the catalog UUIDs</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>18</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Offset of the catalog process
information entries<br />
The offset is relative to the start of the catalog UUIDs</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>20</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of process information
entries<br />
</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>22</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Offset of the catalog sub chunks<br />
The offset is relative to the start of the catalog UUIDs</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of sub chunks</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>26</p></td>
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Reserved or
Padding)</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Earliest firehose timestamp<br />
Contains a Mach continuous timestamp</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>40</p></td>
<td style="text-align: left;"><p>16 x …</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Catalog UUIDs<br />
Contains an array of UUIDs stored in big-endian</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Catalog sub system strings<br />
Contains an array of strings with an end-of-string character</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Catalog process information
entries<br />
Contains an array of <a
href="#catalog_process_information_entry">Catalog process information
entries</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Catalog sub chunks<br />
Contains an array of <a href="#catalog_sub_chunk">Catalog sub
chunks</a></p></td>
</tr>
</tbody>
</table>

**<span class="yellow-background">TODO this appears version 3 of the
catalog, what about other versions?</span>**

## <span id="catalog_process_information_entry"></span>Catalog process information entry

The catalog process information entry is variable of size and consists
of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Entry index</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Index of the main UUID in the catalog
UUIDs</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Index of the dsc UUID in the catalog
UUIDs<br />
Where -1 (0xffff) represents not set</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>First number in proc_id @</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Second number in proc_id @</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>20</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Process identifier (pid)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Effective user identifier
(euid)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of UUID information entries
(uuidinfos)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>36</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>40</p></td>
<td style="text-align: left;"><p>16 x …</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID information entries array</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of sub system entries
(subsystems)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>6 x …</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sub system entries array</p></td>
</tr>
</tbody>
</table>

The catalog process information entry is stored 64-bit aligned.

### Catalog process information UUID information entry

The catalog process information UUID information entry is 16 bytes of
size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Size</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID index<br />
Contains an index of an UUID in the catalog UUIDs</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>10</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Load address (lower 32-bit)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>14</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Load address (upper 16-bit)</p></td>
</tr>
</tbody>
</table>

### Catalog process information sub system

The catalog process information sub system is 6 bytes of size and
consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sub system offset<br />
The offset is relative to the start of the catalog sub system
strings</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Category offset<br />
The offset is relative to the start of the catalog sub system
strings</p></td>
</tr>
</tbody>
</table>

## <span id="catalog_sub_chunk"></span>Catalog sub chunk

The catalog sub chunk describes metadata for the chunk to follow and
consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Start (earliest) time<br />
Contains a Mach continuous timestamp</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>End (latest) time<br />
Contains a Mach continuous timestamp</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Uncompressed size of chunk</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>20</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x100</p></td>
<td style="text-align: left;"><p>Compression algorithm used (0x100 =
LZ4)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of indexes (num_indexes or
procinfos)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>2 x Number of indexes</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Indexes (2 bytes each) pointing to
process info entry</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of string offsets (num_offsets
or subcats)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2 x Number of string offsets</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (offsets used as
cache?)</span></strong><br />
The offset is relative to the start of the catalog sub system
strings</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>64-bit alignment padding</p></td>
</tr>
</tbody>
</table>

# ChunkSet chunk

The ChunkSet chunk is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk header
(tracev3_chunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x600d</p></td>
<td style="text-align: left;"><p>Chunk tag (tag)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk sub tag (subtag)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk data size (length)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Contains compressed data<br />
The compressed data contains chunks</p></td>
</tr>
</tbody>
</table>

# LZ4 compressed data

LZ4 compressed data consists of:

-   compressed block marker

    -   compressed or uncompressed data

-   end of compressed stream marker

## LZ4 compressed data markers

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>"bv41"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>LZ4 compressed block marker<br />
See section: <a href="#lz4_compressed_block">LZ4 compressed
block</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"bv4-"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>LZ4 uncompressed block marker<br />
See section: <a href="#lz4_uncompressed_block">LZ4 uncompressed
block</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"bv4$"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>end of LZ4 compressed stream
marker</p></td>
</tr>
</tbody>
</table>

## <span id="lz4_compressed_block"></span>LZ4 compressed block

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>LZ4 compressed block
header</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>"bv41"</p></td>
<td style="text-align: left;"><p>LZ4 compressed block marker</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Uncompressed data size (in
bytes)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>LZ4 compressed data size (in
bytes)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>LZ4 compressed block
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>LZ4 compressed data</p></td>
</tr>
</tbody>
</table>

## <span id="lz4_uncompressed_block"></span>LZ4 uncompressed block

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>"bv4-"</p></td>
<td style="text-align: left;"><p>LZ4 uncompressed block marker</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Uncompressed data size (in
bytes)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>LZ4 uncompressed block
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Uncompressed data</p></td>
</tr>
</tbody>
</table>

# <span id="tracev3_firehose_chunk"></span>Firehose chunk

The firehose chunk (tracev3\_chunk\_firehose) is variable of size and
consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk header
(tracev3_chunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x6001</p></td>
<td style="text-align: left;"><p>Chunk tag (tag)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk sub tag (subtag)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk data size (length)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk data
(tracev3_chunk_log_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>First number in proc_id @</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Second number in proc_id @</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TTL</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>29</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Collapsed</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>30</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Reserved)</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Public data size
(size_pub_data)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>34</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Private data virtual offset<br />
Contains 4096 (0x1000) if there is no private data and
<code>-(4096 - offset)</code> as offset relative to the end of the
firehose chunk.</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>36</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>38</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Stream type</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>39</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown3</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>40</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Base continuous time for events in the
firehose chunk<br />
Contains a Mach continuous timestamp</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>48</p></td>
<td style="text-align: left;"><p>data size</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>data<br />
Contains one or more <a href="#tracev3_firehose_tracepoint">Firehose
tracepoints</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Private or remnant data</p></td>
</tr>
</tbody>
</table>

It appears that the size of a firehose chunk can grow to 4096 bytes,
with public data as the header of that 4096 block and private data at
the end (as a footer).

*Collapsed* indicates if the empty bytes in between have been removed to
shrink the block. Size of private data can be calculated by subtracting
virtual offset from 4096.

This chunk is usually 64-bit aligned with padding, but at times it is
not. It is unknown if there is a flag to control this behavior.

# Stream type

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x00</p></td>
<td style="text-align: left;"><p>persist</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x01</p></td>
<td style="text-align: left;"><p>special handling</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x02</p></td>
<td style="text-align: left;"><p>memory</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x04</p></td>
<td style="text-align: left;"><p>signpost</p></td>
<td style="text-align: left;"></td>
</tr>
</tbody>
</table>

# Unknown3

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x01</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x02</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x03</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
</tbody>
</table>

# <span id="tracev3_firehose_tracepoint"></span>Firehose tracepoint

A firehose tracepoint is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Record type<br />
See section: <a href="#tracev3_firehose_tracepoint_record_type">Record
type</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Log type<br />
See section: <a href="#tracev3_log_type">Log type</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Flags<br />
See section: <a
href="#tracev3_firehose_tracepoint_flags">Flags</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Format string reference (lower
32-bit)<br />
See section: <a
href="#tracev3_firehose_tracepoint_string_reference">String
reference</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Thread identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Continuous time delta<br />
Contains the delta relative to the base continuous time in the <a
href="#tracev3_firehose_chunk">Firehose chunk</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>22</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Start of
data</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data which contents depends on the
flags</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>End of
data</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>64-bit alignment padding</p></td>
</tr>
</tbody>
</table>

## <span id="tracev3_firehose_tracepoint_record_type"></span>Firehose tracepoint record type

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x00</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Unused (or Empty)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x02</p></td>
<td style="text-align: left;"><p>activity</p></td>
<td style="text-align: left;"><p>Activity<br />
See section: <a href="#tracev3_firehose_tracepoint_activity">Activity
firehose tracepoint</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x03</p></td>
<td style="text-align: left;"><p>trace</p></td>
<td style="text-align: left;"><p>Trace<br />
See section: <a href="#tracev3_firehose_tracepoint_trace">Trace firehose
tracepoint</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x04</p></td>
<td style="text-align: left;"><p>log</p></td>
<td style="text-align: left;"><p>Log<br />
See section: <a href="#tracev3_firehose_tracepoint_log">Log firehose
tracepoint</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x06</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Signpost<br />
See section: <a href="#tracev3_firehose_tracepoint_signpost">Signpost
firehose tracepoint</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x07</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Loss<br />
See section: <a href="#tracev3_firehose_tracepoint_loss">Loss firehose
loss</a></p></td>
</tr>
</tbody>
</table>

## <span id="tracev3_firehose_tracepoint_flags"></span>Firehose tracepoint flags

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x0001</p></td>
<td style="text-align: left;"><p>has_current_aid</p></td>
<td style="text-align: left;"><p>Has current activity
identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x000e</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><a
href="#tracev3_firehose_tracepoint_strings_file_type">Strings file
type</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x0010</p></td>
<td style="text-align: left;"><p>has_unique_pid</p></td>
<td style="text-align: left;"><p>Has process identifier value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x0020</p></td>
<td style="text-align: left;"><p>has_large_offset</p></td>
<td style="text-align: left;"><p>Has large offset data</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x0100</p></td>
<td style="text-align: left;"><p>has_private_data</p></td>
<td style="text-align: left;"><p>Has private data range</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x0200</p></td>
<td style="text-align: left;"><p>has_other_aid</p></td>
<td style="text-align: left;"><p>Has other activity identifier<br />
Note that appears to be only used by the "Activity" record type, other
record types use has_subsystem</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x0200</p></td>
<td style="text-align: left;"><p>has_subsystem</p></td>
<td style="text-align: left;"><p>Has sub system</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x0400</p></td>
<td style="text-align: left;"><p>has_rules</p></td>
<td style="text-align: left;"><p>Has rules</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x0800</p></td>
<td style="text-align: left;"><p>has_oversize</p></td>
<td style="text-align: left;"><p>Has oversize data reference</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x1000</p></td>
<td style="text-align: left;"><p>has_backtrace</p></td>
<td style="text-align: left;"><p>Has backtrace</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x8000</p></td>
<td style="text-align: left;"><p>has_name</p></td>
<td style="text-align: left;"><p>Has name string reference</p></td>
</tr>
</tbody>
</table>

### <span id="tracev3_firehose_tracepoint_strings_file_type"></span>Firehose tracepoint strings file type

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x0002</p></td>
<td style="text-align: left;"><p>main_exe</p></td>
<td style="text-align: left;"><p>Strings are stored in an uuidtext file
by proc_id</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x0004</p></td>
<td style="text-align: left;"><p>shared_cache</p></td>
<td style="text-align: left;"><p>Strings are stored in a Shared-Cache
Strings (dsc) file</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x0008</p></td>
<td style="text-align: left;"><p>absolute</p></td>
<td style="text-align: left;"><p>Strings are stored in an uuidtext file
by reference</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x000a</p></td>
<td style="text-align: left;"><p>uuid_relative</p></td>
<td style="text-align: left;"><p>Strings are stored in an uuidtext file
by identifier</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x000c</p></td>
<td style="text-align: left;"><p>large_shared_cache</p></td>
<td style="text-align: left;"><p>Strings are stored in a Shared-Cache
Strings (dsc) file</p></td>
</tr>
</tbody>
</table>

## <span id="tracev3_firehose_tracepoint_data_range"></span>Firehose tracepoint data range

A firehose tracepoint data range is 4 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Range offset</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Range size</p></td>
</tr>
</tbody>
</table>

## <span id="tracev3_firehose_tracepoint_data_item"></span>Firehose tracepoint data item

A firehose tracepoint data item is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value type</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data item data size</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data item data<br />
Contains inline data or a value data range</p></td>
</tr>
</tbody>
</table>

### Value type

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x00</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (integer or floating-point
value)</span></strong><br />
Contains a 32-bit or 64-bit value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x01</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (private value)</span></strong><br />
Contains a 32-bit value, formatted as "&lt;private&gt;"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x02</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (integer or floating-point
value)</span></strong><br />
Contains a 8-bit, 16-bit, 32-bit or 64-bit value</p></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x10</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (integer format
precision)</span></strong><br />
Contains a 32-bit value<br />
This value has been seen to be used in combination with format strings
like "%.0lld"</p></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x12</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (string format
precision)</span></strong><br />
Contains a 32-bit value<br />
This value has been seen to be used in combination with format strings
like "%.16s" and "%.*s", where this value contains the number of
characters of the string that should be printed.</p></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x20</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (string)</span></strong><br />
Consists of a <a
href="#tracev3_firehose_tracepoint_data_time_with_value_data_range">Firehose
tracepoint data item with value data range</a> where the value data
contains an UTF-8 encoded string with an optional end-of-string
character.</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x21</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (private value)</span></strong><br />
Consists of a <a
href="#tracev3_firehose_tracepoint_data_time_with_private_data_range">Firehose
tracepoint data item with private data range</a> where the value data
contains an UTF-8 encoded string with an optional end-of-string
character.</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x22</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (string)</span></strong><br />
Consists of a <a
href="#tracev3_firehose_tracepoint_data_time_with_value_data_range">Firehose
tracepoint data item with value data range</a> where the value data
contains an UTF-8 encoded string with an optional end-of-string
character.</p></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x25</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (sensitive
value)</span></strong><br />
Contains a 32-bit value, formatted as "&lt;private&gt;"</p></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x30</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (binary data)</span></strong><br />
Consists of a <a
href="#tracev3_firehose_tracepoint_data_time_with_value_data_range">Firehose
tracepoint data item with value data range</a> where the value data
contains binary data.</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x31</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (private value)</span></strong><br />
Contains a 32-bit value, formatted as "&lt;private&gt;"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x32</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (binary data)</span></strong><br />
Consists of a <a
href="#tracev3_firehose_tracepoint_data_time_with_value_data_range">Firehose
tracepoint data item with value data range</a> where the value data
contains binary data.</p></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x35</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (private
value?)</span></strong></p></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x40</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (string)</span></strong><br />
Consists of a <a
href="#tracev3_firehose_tracepoint_data_time_with_value_data_range">Firehose
tracepoint data item with value data range</a> where the value data
contains an UTF-8 encoded string with an optional end-of-string
character.</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x41</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (private value)</span></strong><br />
Consists of a <a
href="#tracev3_firehose_tracepoint_data_time_with_private_data_range">Firehose
tracepoint data item with private data range</a> where the value data
contains an UTF-8 encoded string with an optional end-of-string
character.</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x42</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (string)</span></strong><br />
Consists of a <a
href="#tracev3_firehose_tracepoint_data_time_with_value_data_range">Firehose
tracepoint data item with value data range</a> where the value data
contains an UTF-8 encoded string with an optional end-of-string
character.</p></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x45</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (private
value?)</span></strong></p></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0xf2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (binary data)</span></strong><br />
Consists of a <a
href="#tracev3_firehose_tracepoint_data_time_with_value_data_range">Firehose
tracepoint data item with value data range</a> where the value data
contains binary data.</p></td>
</tr>
</tbody>
</table>

Private value types are formatted as "&lt;private&gt;".

### <span id="tracev3_firehose_tracepoint_data_time_with_value_data_range"></span>Firehose tracepoint data item with value data range

A firehose tracepoint data item with value data range is 6 bytes of size
and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value type</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>Data item data size</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Data item
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value data (range) offset<br />
The offset is relative to the start of the values data</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value data (range) size</p></td>
</tr>
</tbody>
</table>

### <span id="tracev3_firehose_tracepoint_data_time_with_private_data_range"></span>Firehose tracepoint data item with private data range

A firehose tracepoint data item with private data range is 6 bytes of
size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value type</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>Data item data size</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Data item
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value data (range) offset<br />
The offset is relative to the start of the private data in the <a
href="#tracev3_firehose_chunk">Firehose chunk</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value data (range) size</p></td>
</tr>
</tbody>
</table>

## <span id="tracev3_firehose_tracepoint_backtrace_data"></span>Firehose tracepoint backtrace data

Firehose tracepoint backtrace data is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Seen: 0x01)</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Seen: 0x12)</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of (image) identifiers</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of frames</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"><p>number of identifiers</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Array of image identifiers
(UUIDs)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>number of frames</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Array of image offsets</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>number of frames</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Array of image identifier
indexes</p></td>
</tr>
</tbody>
</table>

## <span id="tracev3_firehose_tracepoint_activity"></span>Activity firehose tracepoint

An activity firehose tracepoint is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>0x02</p></td>
<td style="text-align: left;"><p>Record type<br />
See section: <a href="#tracev3_firehose_tracepoint_record_type">Record
type</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Log type<br />
See section: <a href="#tracev3_log_type">Log type</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Flags<br />
See section: <a
href="#tracev3_firehose_tracepoint_flags">Flags</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Format string reference (lower
32-bit)<br />
See section: <a
href="#tracev3_firehose_tracepoint_string_reference">String
reference</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Thread identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Continuous time delta<br />
Contains the delta relative to the base continuous time in the <a
href="#tracev3_firehose_chunk">Firehose chunk</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>22</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Start of
data</em></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has current activity
identifier flag (0x0001) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Current activity identifier</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has process identifier
value flag (0x0010) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Process identifier (pid)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has other activity
identifier (0x0200) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Other activity identifier</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Log type !=
0x03</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>New activity identifier</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID entry load address (lower
32-bit)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has large offset flag
(0x0020) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Large offset data</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If strings file type ==
0x0008</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID entry load address (upper
16-bit)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If strings file type ==
0x000a</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>uuidtext file identifier (or image
uuid)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If strings file type ==
0x000c</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Large shared cache data</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>End of
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>64-bit alignment padding</p></td>
</tr>
</tbody>
</table>

Note that "has private data range flag (0x0100)" has been observed to be
set but without any obvious changes to the activity firehose tracepoint
structure.

## <span id="tracev3_firehose_tracepoint_trace"></span>Trace firehose tracepoint

A trace firehose tracepoint is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>0x03</p></td>
<td style="text-align: left;"><p>Record type<br />
See section: <a href="#tracev3_firehose_tracepoint_record_type">Record
type</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Log type<br />
See section: <a href="#tracev3_log_type">Log type</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Flags<br />
See section: <a
href="#tracev3_firehose_tracepoint_flags">Flags</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Format string reference (lower
32-bit)<br />
See section: <a
href="#tracev3_firehose_tracepoint_string_reference">String
reference</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Thread identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Continuous time delta<br />
Contains the delta relative to the base continuous time in the <a
href="#tracev3_firehose_chunk">Firehose chunk</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>22</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Start of
data</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID entry load address (lower
32-bit)</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>If data size &gt;
5</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Values data</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>number of values</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value data size<br />
Seen: 4 and 8</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of values<br />
Seen: 0 and 1</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>End of
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>64-bit alignment padding</p></td>
</tr>
</tbody>
</table>

The value data sizes and values are stored front-to-back.

## <span id="tracev3_firehose_tracepoint_log"></span>Log firehose tracepoint

A log firehose tracepoint is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>0x04</p></td>
<td style="text-align: left;"><p>Record type<br />
See section: <a href="#tracev3_firehose_tracepoint_record_type">Record
type</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Log type<br />
See section: <a href="#tracev3_log_type">Log type</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Flags<br />
See section: <a
href="#tracev3_firehose_tracepoint_flags">Flags</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Format string reference (lower
32-bit)<br />
See section: <a
href="#tracev3_firehose_tracepoint_string_reference">String
reference</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Thread identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Continuous time delta<br />
Contains the delta relative to the base continuous time in the <a
href="#tracev3_firehose_chunk">Firehose chunk</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>22</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Start of
data</em></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has current activity
identifier flag (0x0001) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Current activity identifier</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has private data range
flag (0x0100) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><a
href="#tracev3_firehose_tracepoint_data_range">Private data
range</a><br />
Where the range offset is a virtual private strings offset in the <a
href="#tracev3_firehose_chunk">Firehose chunk</a></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID entry load address (lower
32-bit)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has large offset flag
(0x0020) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Large offset data</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If strings file type ==
0x0008</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID entry load address (upper
16-bit)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If strings file type ==
0x000a</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>uuidtext file identifier (or image
uuid)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If strings file type ==
0x000c</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Large shared cache data</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has sub system flag
(0x0200) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sub system identifier</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has rules flag (0x0400)
is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TTL</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has oversize data
reference flag (0x0800) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Oversize data reference</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of data items</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>number of data items</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Array of data items<br />
See section: <a href="#tracev3_firehose_tracepoint_data_item">Data
item</a></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Values
data</em></p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Has backtrace flag
(0x1000) is set</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Backtrace data<br />
See section: <a
href="#tracev3_firehose_tracepoint_backtrace_data">Backtrace
data</a></p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data items values data</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>End of values
data</em></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>End of
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>64-bit alignment padding</p></td>
</tr>
</tbody>
</table>

The backtrace data is stored as part of the values data. Value data
offsets of data items are relative from the start of the values data.

## <span id="tracev3_firehose_tracepoint_singpost"></span>Signpost firehose tracepoint

A signpost firehose tracepoint is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>0x06</p></td>
<td style="text-align: left;"><p>Record type<br />
See section: <a href="#tracev3_firehose_tracepoint_record_type">Record
type</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Log type<br />
See section: <a href="#tracev3_log_type">Log type</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Flags<br />
See section: <a
href="#tracev3_firehose_tracepoint_flags">Flags</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Format string reference (lower
32-bit)<br />
See section: <a
href="#tracev3_firehose_tracepoint_string_reference">String
reference</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Thread identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Continuous time delta<br />
Contains the delta relative to the base continuous time in the <a
href="#tracev3_firehose_chunk">Firehose chunk</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>22</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Start of
data</em></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has current activity
identifier flag (0x0001) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Current activity identifier</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has private data range
flag (0x0100) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><a
href="#tracev3_firehose_tracepoint_data_range">Private data
range</a><br />
Where the range offset is a virtual private strings offset in the <a
href="#tracev3_firehose_chunk">Firehose chunk</a></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID entry load address (lower
32-bit)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has large offset flag
(0x0020) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Large offset data</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If strings file type ==
0x0008</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID entry load address (upper
16-bit)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If strings file type ==
0x000a</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>uuidtext file identifier (or image
uuid)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If strings file type ==
0x000c</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Large shared cache data</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has sub system flag
(0x0200) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sub system identifier</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Signpost identifier</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has rules flag (0x0400)
is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TTL</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has oversize data
reference flag (0x0800) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Oversize data reference</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has name string
reference flag (0x8000) is set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Name string reference (lower
32-bit)<br />
See section: <a
href="#tracev3_firehose_tracepoint_string_reference">String
reference</a></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Has name string
reference flag (0x8000) and Has large offset flag (0x0020) are
set</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Name string reference (upper
16-bit)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of data items</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>number of data items</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Array of data items<br />
See section: <a href="#tracev3_firehose_tracepoint_data_item">Data
item</a></p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Values
data</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data items values data</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>End of values
data</em></p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>End of
data</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>64-bit alignment padding</p></td>
</tr>
</tbody>
</table>

## <span id="tracev3_firehose_tracepoint_loss"></span>Loss firehose tracepoint

A loss firehose tracepoint is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>0x07</p></td>
<td style="text-align: left;"><p>Record type<br />
See section: <a href="#tracev3_firehose_tracepoint_record_type">Record
type</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Log type<br />
See section: <a href="#tracev3_log_type">Log type</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>0x00</p></td>
<td style="text-align: left;"><p>Flags<br />
See section: <a
href="#tracev3_firehose_tracepoint_flags">Flags</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Format string reference (lower
32-bit)<br />
See section: <a
href="#tracev3_firehose_tracepoint_string_reference">String
reference</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Thread identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Continuous time delta<br />
Contains the delta relative to the base continuous time in the <a
href="#tracev3_firehose_chunk">Firehose chunk</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>22</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data size</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Start of
data</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Start time<br />
Contains the date and time the loss started</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>End time<br />
Contains the date and time the loss ended</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of messages<br />
Contains the number of messages lost</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>End of
data</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>64-bit alignment padding</p></td>
</tr>
</tbody>
</table>

## <span id="tracev3_firehose_tracepoint_string_reference"></span>Firehose tracepoint string reference

A string reference is used to look up strings in the uuidtext or
Shared-Cache Strings (dsc) files.

The most-significant bit (MSB) of the string reference lower 32-bit is a
"dynamic" flag. If set (and the string reference ⇐ 0xffffffff) the
string is "%s". In a Shared-Cache Strings (dsc) file the corresponding
image identifier and offset can be found comparing the remaining value
of the string referrence with the dsc text offset.

To calculate a format string reference:

-   ( large shared cache data &lt;&lt; 31 ) | ( format string reference
    lower 32-bit & 0x7fffffff )

-   ( large offset data &lt;&lt; 31 ) | ( format string reference lower
    32-bit & 0x7fffffff )

-   ( format string reference lower 32-bit & 0x7fffffff )

To calculate a name string reference:

-   ( name string reference upper 16-bit &lt;&lt; 31 ) | ( name string
    reference lower 32-bit & 0x7fffffff )

-   ( name string reference lower 32-bit & 0x7fffffff )

### Invalid shared cache code pointer offset

    tp 2872 + 117:      log info (has_current_aid, has_large_offset, large_shared_cache, has_subsystem)
        thread:         00000000000009ba
        time:           +42.683s
        walltime:       1659846550 - 2022-08-07 06:29:10 (Sunday)
        cur_aid:        8000000000001652
        location:       pc:0x8008ecf707e9 fmt:0x1117a0720
        image uuid:     CC386FB1-8C26-3CB7-8329-CC63095FCA7D
        format:         START %{public}@
        error:          ~~> <Invalid shared cache code pointer offset>
        subsystem:      21 com.apple.reminderkit.utility

    Format string reference                                                 : 0x117a0720
    UUID entry load address (lower 32-bit)                                  : 0xecf707e9
    Large offset data                                                       : 0x8008
    Large shared cache data                                                 : 0x0002

    Calculated format string reference                                      : 0x1117a0720
    Strings file identifier                                                 : cc386fb1-8c26-3cb7-8329-cc63095fca7d
    Image identifier                                                        : cc386fb1-8c26-3cb7-8329-cc63095fca7d
    Load address                                                            : 0x8008ecf707e9

Observed behavior:

-   Large shared cache data is used to calculate format string reference

-   Image identifier = strings file identifier (value from Shared-Cache
    Strings (dsc) is not used)

-   Image path is not set

-   Image text offset = 0

-   Load address = ( Large offset data &lt;&lt; 32 ) | UUID entry load
    address (lower 32-bit)

### Invalid bounds for uuidtext file

    tp 272 + 38:        activity create (has_large_offset, main_exe)
        thread:         000000000000117c
        time:           +1,284.168s
        walltime:       1659848875 - 2022-08-07 07:07:55 (Sunday)
        new_aid:        8000000000013b44
        location:       pc:0x744dd fmt:0x7fcbf77bca21
        image uuid:     3BC14712-721B-3B0E-A542-A289155FE74E
        image path:     /System/Library/PrivateFrameworks/GameCenterFoundation.framework/Versions/A/gamed
        error:          ~~> Invalid bounds -142882271 for 3BC14712-721B-3B0E-A542-A289155FE74E

    Format string reference                                                 : 0x777bca21
    UUID entry load address (lower 32-bit)                                  : 0x000744dd
    Large offset data                                                       : 0xff97

    Calculated format string reference                                      : 0x7fcbf77bca21
    Strings file identifier                                                 : 3bc14712-721b-3b0e-a542-a289155fe74e
    Image identifier                                                        : 3bc14712-721b-3b0e-a542-a289155fe74e
    Load address                                                            : 0x000744dd

Observed behavior:

-   Large offset data is used to calculate format string reference

-   Format string is not set

-   Load address = UUID entry load address (lower 32-bit)

## <span id="tracev3_firehose_tracepoint_format_string"></span>Firehose tracepoint format string

Format string operators are defined in the following format:

    %[value_type_decoder] [flags] [width] [.precision] [length_modifier] conversion_specifier

Where `%%` represents a literal `%`.

Also see `man 3 os_log` and `man 3 printf` on Mac OS.

A missing data item is formatted as "&lt;decode: missing data&gt;".

The built-in value type decoders are:

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>"bitrate"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a bit-rate value, for
example "123 kbps"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"bool"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a lower-case boolean
value, for example "true" or "false"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"BOOL"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a upper-case boolean
value, for example "YES" or "NO"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"bytes"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted a bytes value, for example
"4.72 kB"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"darwin.errno"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a system error, for
example "[32: Broken pipe]"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"darwin.mode"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a file mode value, for
example "drwxr-xr-x"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"darwin.signal"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a signal, for example
"[sigsegv: Segmentation Fault]"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"iec-bitrate"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as an IEC bit-rate value, for
example "118 Kibps"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"iec-bytes"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as IEC bytes value, for
example "4.61 KiB"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"in_addr"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as an IPv4 address, for
example "127.0.0.1"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"in6_addr"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as an IPv6 address, for
example "fe80::f:86ff:fee9:5c16"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"sockaddr"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#socket_address">socket address</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"time_t"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a seconds precision date
and time value, for example "2016-01-12 19:41:37"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"timespec"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a nanoseconds precision
date and time value, for example "2016-01-12
19:41:37.2382382823"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"timeval"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a microseconds precision
date and time value, for example "2016-01-12 19:41:37.774236"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"uuid_t"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as an UUID, for example
"10742E39-0657-41F8-AB99-878C5EC2DCAA"</p></td>
</tr>
</tbody>
</table>

Other observerd value type decoders are:

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>"bluetooth:OI_STATUS"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>No additional formatting</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"errno"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a system error, for
example "[32: Broken pipe]"</p></td>
</tr>
<tr class="odd">
<td
style="text-align: left;"><p>"location:_CLClientManagerStateTrackerState"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#locationd_core_location_client_manager_state_tracker_state">locationd
Core Location client manager (CLClientManager) state tracker
state</a></p></td>
</tr>
<tr class="even">
<td
style="text-align: left;"><p>"location:_CLLocationManagerStateTrackerState"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#locationd_core_location_location_manager_state_tracker_state">locationd
Core Location location manager (CLLocationManager) state tracker
state</a></p></td>
</tr>
<tr class="odd">
<td
style="text-align: left;"><p>"location:CLClientAuthorizationStatus"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#locationd_core_location_client_authorization_status">locationd
Core Location client authorization status</a></p></td>
</tr>
<tr class="even">
<td
style="text-align: left;"><p>"location:CLDaemonStatus_Type::Reachability"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#locationd_core_location_daemon_reachability_status_types">locationd
Core Location daemon reachability status types</a></p></td>
</tr>
<tr class="odd">
<td
style="text-align: left;"><p>"location:CLDaemonStatusStateTracker"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#locationd_core_location_daemon_tracker_state">locationd Core
Location daemon tracker state</a></p></td>
</tr>
<tr class="even">
<td
style="text-align: left;"><p>"location:CLSubHarvesterIdentifier"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#locationd_core_location_sub_harvester_identifier">locationd Core
Location sub harvester identifier</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"location:escape_only"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatter that escapes specific
characters such as: "/"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"location:IOMessage"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#locationd_io_message">locationd IO message</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"location:SqliteResult"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#locationd_sqlite_result">locationd SQLite result</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"mask.hash"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mask_hash">Mask hash</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"mdns:acceptable"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a boolean value, for
example "acceptable" or "unacceptable"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"mdns:addrmv"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a boolean value, for
example "add" or "rmv"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"mdns:dns.counts"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mdns_dns_counters">mDNS DNS countters</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"mdns:dns.idflags"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mdns_dns_identifier_and_flags">mDNS DNS identifier and
flags</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"mdns:dnshdr"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mdns_dns_header">mDNS DNS header</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"mdns:gaiopts"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mdns_getaddrinfo_options">mDNS getaddrinfo options</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"mdns:nreason"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mdns_reason">mDNS reason</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"mdns:protocol"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mdns_protocol">mDNS protocol</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"mdns:rd.svcb"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"mdns:rrtype"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mdns_resource_record_type">mDNS resource record (RR)
type</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"mdns:yesno"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a boolean value, for
example "yes" or "no"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"mdnsresponder:domain_name"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"mdnsresponder:ip_addr"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mdnsresponder_ip_address">mDNSResponder IP address</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"mdnsresponder:mac_addr"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#mdnsresponder_mac_address">mDNSResponder MAC address</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"name=NAME"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Name formatting argument, where NAME is
the name of the value, which has no additional formatting</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"Name:NAME"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown, see notes
below</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"network:in_addr"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as an IPv4 address, for
example "127.0.0.1"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"network:in6_addr"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as an IPv6 address, for
example "fe80::f:86ff:fee9:5c16"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"network:sockaddr"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#socket_address">socket address</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"network:tcp_flags"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"network:tcp_state"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"odtypes:mbr_details"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#open_directory_membership_details">Open Directory membership
details</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"odtypes:mbridtype"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#open_directory_membership_identifier_type">Open Directory
membership identifier type</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"odtypes:nt_sid_t"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#open_directory_windows_nt_sid">Open Directory Windows NT Security
Identifier (SID)</a></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"odtypes:ODError"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a <a
href="#open_directory_error">Open Directory error</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"private"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Private formatting argument, which is
formatted as "&lt;private&gt;"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"public"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Public formatting argument, which has
no additional formatting</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"sensitive"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sensitive formatting argument, which is
formatted as "&lt;private&gt;"</p></td>
</tr>
<tr class="odd">
<td
style="text-align: left;"><p>"signpost.description:attribute"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a signpost description
attribute, for example
<code>__##__signpost.description#____#attribute#_##_#efilogin-helper##__##</code></p></td>
</tr>
<tr class="even">
<td
style="text-align: left;"><p>"signpost.description:begin_time"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a signpost description
begin time, for example
<code>__##__signpost.description#____#begin_time#_##_#2180300470618##__##</code></p></td>
</tr>
<tr class="odd">
<td
style="text-align: left;"><p>"signpost.description:end_time"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Formatted as a signpost description end
time, for example
<code>__##__signpost.description#____#end_time#_##_#1005756624719##__##</code></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"signpost.telemetry:number1"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>For example
<code>__##__signpost.telemetry#____#number1#_##_#5.88671875##__##</code>,
where a avalue can be an integer or floating-point which is formatted as
(at least) "%.9g"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"signpost.telemetry:number2"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>For example
<code>__##__signpost.telemetry#____#number2#_##_#6.05859375##__##</code>,
where a avalue can be an integer or floating-point and which is
formatted as (at least) "%.9g"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"signpost.telemetry:number3"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>For example
<code>__##__signpost.telemetry#____#number3#_##_#6.05859375##__##</code>,
where a avalue can be an integer or floating-point and which is
formatted as (at least) "%.9g"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"signpost.telemetry:string1"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>For example
<code>__##__signpost.telemetry#____#string1#_##_#executeQueryBegin##__##</code></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"signpost.telemetry:string2"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>For example
<code>__##__signpost.telemetry#____#string2#_##_#executeQueryBegin##__##</code></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"xcode:size-in-bytes"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>No additional formatting</p></td>
</tr>
</tbody>
</table>

The multiple value type decoders can be used in combination for example
"%{public,uuid\_t}.16P" or "%{private, mask.hash,
mdnsresponder:ip\_addr}.20P".

The flags are defined as:

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>"#"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value should be converted to an
"alternate form". Note that "%#x" is formatted as "0x%x" with the
exception of "0".</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"0"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value should be padded with 0</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"-"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value should be left-justified instead
of right-justified</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>" "</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Value should be padded with
space</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"+"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Add the <em>+</em> sign in front of
positive values</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"'"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">TODO describe</span></strong></p></td>
</tr>
</tbody>
</table>

The length modifiers are defined as:

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>"h"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>short</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"hh"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>char</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"j"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>intmax_t or uintmax_t</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"l"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(unsigned) long</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"ll"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(unsigned) long long</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"q"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(unsigned) long long</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"t"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ptrdiff_t</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"z"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>size_t</p></td>
</tr>
</tbody>
</table>

The .precision is defined as:

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>"0"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Observed that this has no effect in
"%.0s"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"*"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>An additional integer argument supplies
the field width or precision.</p></td>
</tr>
</tbody>
</table>

The types are defined as:

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>"@"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Obj-C/CF/Swift object</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"a"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Floating-point value</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"A"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Floating-point value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"c"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Character value</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"C"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>wide character value, equivalent to
"lc"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"d"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Signed decimal integer value</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"D"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Long signed decimal integer value,
equivalent to "ld"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"e"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Floating-point value</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"E"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Floating-point value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"f"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Floating-point value</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"F"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Floating-point value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"g"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Floating-point value</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"G"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Floating-point value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"i"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Signed decimal integer value</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"m"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Signed decimal integer value
representing a system error (errno), formatted as "No route to
host"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"n"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">TODO describe</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"o"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Octal integer value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"O"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Long octal integer value, equivalent to
"lo"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"p"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Pointer value, equivalent to
"0x%x"</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"P"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Binary data</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"s"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>String value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"S"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Wide character string value, equivalent
to "ls"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"u"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Unsigned decimal integer value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"U"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Long unsigned decimal integer value,
equivalent to "lu"</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>"x"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Hexadecimal interger value, formatter
in lower case</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>"X"</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Hexadecimal interger value, formatter
in upper case</p></td>
</tr>
</tbody>
</table>

### Notes

For a format string like "%6.6d" 0 padding is used.

    Post "com.apple.system.config.network_change" (%s: %ld.%6.6d: 0x%x)

Formatted as:

    Post "com.apple.system.config.network_change" (delayed: 0.000001: 0x2)

For format string with a "Name:" custom formatter:

     enableTelemetry=YES ResultCount=%{public, signpost.telemetry:number1, Name:ResultCount}ld DataSize=%{public, signpost.telemetry:number2, Name:DataSize}ld

Seen:

     enableTelemetry=YES ResultCount=0 DataSize=256

Is "Name:NAME" used for some kind of formatting override?

    %{public}-22s: OFF

Formatted as:

    SFIManagedFocal       : OFF

Observed behavior:

If the number of values stored in the data items &gt; number of
formatters the values are formatted as "&lt;decode: missing data&gt;".

# Oversize chunk

Oversize chunks contain data that is too large to fit into a single log
entry, hence it is stored seperately and referenced by firehose
tracepoints.

The Oversize chunk is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk header
(tracev3_chunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x6002</p></td>
<td style="text-align: left;"><p>Chunk tag (tag)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk sub tag (subtag)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk data size (length)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk data
(tracev3_chunk_oversize)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>First number in proc_id @</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Second number in proc_id @</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TTL</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>29</p></td>
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Reserved?)</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Timestamp<br />
Contains a Mach continuous timestamp</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>40</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data reference (or message
identifier)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>44</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Size of data (buffdata_sz)<br />
Contains the size of the data items and values data</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>46</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Size of private data
(privdata_sz)</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Data</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>48</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>49</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of data items</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>50</p></td>
<td style="text-align: left;"><p>number of data items</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Array of data items<br />
See section: <a href="#tracev3_firehose_tracepoint_data_item">Data
item</a></p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Has backtrace flag
(0x1000) is set</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Backtrace data<br />
See section: <a
href="#tracev3_firehose_tracepoint_backtrace_data">Backtrace
data</a></p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data items values data</p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>End of
data</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Private data</p></td>
</tr>
</tbody>
</table>

# StateDump chunk

The StateDump chunk is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk header
(tracev3_chunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x6003</p></td>
<td style="text-align: left;"><p>Chunk tag (tag)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk sub tag (subtag)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk data size (length)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk data
(tracev3_chunk_statedump)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>First number in proc_id @</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Second number in proc_id @</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TTL</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>29</p></td>
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Reserved?)</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Timestamp<br />
Contains a Mach continuous timestamp</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>40</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Activity identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>48</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sender image identifier<br />
Contains a UUID stored in big-endian</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>64</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>State data type<br />
See section: <a href="#state_data_types">State data types</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>68</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>State data size</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>If state data type !=
3</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>72</p></td>
<td style="text-align: left;"><p>64</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong><br />
<strong><span class="yellow-background">Only used when data type is
3?</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>136</p></td>
<td style="text-align: left;"><p>64</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong><br />
<strong><span class="yellow-background">Only used when data type is
3?</span></strong></p></td>
</tr>
<tr class="even">
<td colspan="4" style="text-align: left;"><p><em>If state data type ==
3</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>72</p></td>
<td style="text-align: left;"><p>64</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Decoder library<br />
Contains an UTF-8 formatted string with an end-of-string
character</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>136</p></td>
<td style="text-align: left;"><p>64</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Decoder type<br />
Contains an UTF-8 formatted string with an end-of-string
character</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Common</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>200</p></td>
<td style="text-align: left;"><p>64</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Title (or Name)<br />
Contains an UTF-8 formatted string with an end-of-string
character</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>264</p></td>
<td style="text-align: left;"><p>state data size</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>State data</p></td>
</tr>
</tbody>
</table>

Note that both the decoder library and decoder type can contain remnant
data.

## <span id="state_data_types"></span>State data types

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>serialized NS/CF object<br />
State date contains a binary plist</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>protocol buffer (protobuf)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>custom<br />
State date contains decoder specific data</p></td>
</tr>
</tbody>
</table>

# SimpleDump chunk

The SimpleDump chunk is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk header
(tracev3_chunk_preamble)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>0x6004</p></td>
<td style="text-align: left;"><p>Chunk tag (tag)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk sub tag (subtag)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Chunk data size (length)</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Chunk data
(tracev3_chunk_simpledump)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>First number in proc_id @</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Second number in proc_id @</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TTL</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>29</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Log type<br />
See section: <a href="#tracev3_log_type">Log type</a></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>29</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Reserved?)</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Timestamp<br />
Contains a Mach continuous timestamp</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>40</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Thread identifier</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>48</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Load address (or offset)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>56</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Sender image identifier<br />
Contains a UUID stored in big-endian</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>72</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Shared-Cache Strings (dsc)
identifier<br />
Contains a UUID stored in big-endian</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>88</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (number of message
strings?)</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>92</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Subsystem string size</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>96</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Message string size</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>100</p></td>
<td style="text-align: left;"><p>subsystem string size</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Subsystem string<br />
Contains an UTF-8 formatted string with an end-of-string
character</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>message string size</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Message string<br />
Contains an UTF-8 formatted string with an end-of-string
character</p></td>
</tr>
</tbody>
</table>

# <span id="tracev3_log_type"></span>Log type

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x00</p></td>
<td style="text-align: left;"><p>default</p></td>
<td style="text-align: left;"><p>Default</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x01</p></td>
<td style="text-align: left;"><p>create</p></td>
<td style="text-align: left;"><p>Create<br />
Note that appears to be only used by the "Activity" record type, other
record types use info</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x01</p></td>
<td style="text-align: left;"><p>info</p></td>
<td style="text-align: left;"><p>Info</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x02</p></td>
<td style="text-align: left;"><p>debug</p></td>
<td style="text-align: left;"><p>Debug</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x03</p></td>
<td style="text-align: left;"><p>useraction</p></td>
<td style="text-align: left;"><p>Useraction<br />
Note that appears to be only used by the "Activity" record type</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x10</p></td>
<td style="text-align: left;"><p>error</p></td>
<td style="text-align: left;"><p>Error</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x11</p></td>
<td style="text-align: left;"><p>fault</p></td>
<td style="text-align: left;"><p>Fault</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"><p><em>Signpost thread scope
(0x40)</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x40</p></td>
<td style="text-align: left;"><p>thread, event</p></td>
<td style="text-align: left;"><p>Thread Signpost event</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x41</p></td>
<td style="text-align: left;"><p>thread, interval_begin</p></td>
<td style="text-align: left;"><p>Thread Signpost start</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x42</p></td>
<td style="text-align: left;"><p>thread, interval_end</p></td>
<td style="text-align: left;"><p>Thread Signpost end</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td colspan="3" style="text-align: left;"><p><em>Signpost process scope
(0x80)</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x80</p></td>
<td style="text-align: left;"><p>process, event</p></td>
<td style="text-align: left;"><p>Process Signpost event</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x81</p></td>
<td style="text-align: left;"><p>process, interval_begin</p></td>
<td style="text-align: left;"><p>Process Signpost start</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x82</p></td>
<td style="text-align: left;"><p>process, interval_end</p></td>
<td style="text-align: left;"><p>Process Signpost end</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"><p><em>Signpost system scope
(0xc0)</em></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0xc0</p></td>
<td style="text-align: left;"><p>system, event</p></td>
<td style="text-align: left;"><p>System Signpost event</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0xc1</p></td>
<td style="text-align: left;"><p>system, interval_begin</p></td>
<td style="text-align: left;"><p>System Signpost start</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0xc2</p></td>
<td style="text-align: left;"><p>system, interval_end</p></td>
<td style="text-align: left;"><p>System Signpost end</p></td>
</tr>
</tbody>
</table>

**<span class="yellow-background">Other values are marked as
"Default"?</span>**

# Notes

To view the contents of a tracev3 file on Mac OS:

    log raw-dump -f ${FILE}.tracev3

# <span id="timesync_database_file_format"></span>timesync database file format

A timesync database file consists of:

-   one or more:

    -   timesync boot record

    -   timesync sync records related to the boot record

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Characteristics</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>Byte order</p></td>
<td style="text-align: left;"><p>little-endian</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>Date and time values</p></td>
<td style="text-align: left;"><p>number of nanoseconds since January 1,
1970 00:00:00 UTC (POSIX epoch), disregarding leap seconds</p></td>
</tr>
</tbody>
</table>

# timesync boot record

The timesync boot record is 48 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>"\xb0\xbb"</p></td>
<td style="text-align: left;"><p>Signature</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>48</p></td>
<td style="text-align: left;"><p>Size of record</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Seen: 0)</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Boot identifier (boot UUID)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(Mach) Timebase numerator (first number
in timebase # / #)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(Mach) Timebase denominator (second
number in timebase # / #)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Timestamp (or boot time)<br />
Signed integer that contains the number of nanoseconds since January 1,
1970 00:00:00 UTC or 0 if not set</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>40</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Time zone offset in minutes<br />
Contains a signed integer that contains the number of minutes relative
from UTC, for example -60 represents UTC+1</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>44</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Daylight savings (DST) flag (0 = no
DST, 1 = DST)</p></td>
</tr>
</tbody>
</table>

Timebase numerator / Timebase denominator is the timebase, which
contains the number of seconds per continuous time unit.

Timestamp appears to be stored in UTC but the log tool shows the local
time zone.

## timesync sync record

The timesync sync record is 32 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>"Ts"</p></td>
<td style="text-align: left;"><p>Signature</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>Size of record</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (Seen: 0 and
1)</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Kernel time<br />
Contains a Mach continuous timestamp</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Timestamp (or wall clock time)<br />
Signed integer that contains the number of nanoseconds since January 1,
1970 00:00:00 UTC or 0 if not set</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Time zone offset in minutes<br />
Contains a signed integer that contains the number of minutes relative
from UTC, for example -60 represents UTC+1</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Daylight savings (DST) flag (0 = no
DST, 1 = DST)</p></td>
</tr>
</tbody>
</table>

The kernel time contains the Mach continuous time representation of the
POSIX timestamp (or wall clock time).

Timestamp appears to be stored in UTC but the log tool shows the local
time zone.

# Notes

To view the contents of timesync files on Mac OS:

    log raw-dump -t /var/db/diagnostics/timesync/

# <span id="shared_cache_strings_file_format"></span>Shared-Cache Strings (dsc) file format

A Shared-Cache Strings (dsc) file consist of:

-   Shared-Cache Strings (dsc) file header

-   Range descriptors

-   UUID descriptors

-   path strings

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Characteristics</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>Byte order</p></td>
<td style="text-align: left;"><p>little-endian</p></td>
</tr>
</tbody>
</table>

# Shared-Cache Strings (dsc) file header

The Shared-Cache Strings (dsc) file header is 16 bytes of size and
consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>"hcsd"</p></td>
<td style="text-align: left;"><p>Signature</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Format major version</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Format minor version</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of ranges (range count)</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of UUIDs (uuid count)</p></td>
</tr>
</tbody>
</table>

## Format versions

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">OS version</th>
<th style="text-align: left;">Format version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>macOS 10.12 (Sierra)</p></td>
<td style="text-align: left;"><p>1.0</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>macOS 10.13 (High Sierra)</p></td>
<td style="text-align: left;"><p>1.0</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>macOS 10.14 (Mojave)</p></td>
<td style="text-align: left;"><p>1.0</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>macOS 10.15 (Catalina)</p></td>
<td style="text-align: left;"><p>1.0</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>macOS 11 (Big Sur)</p></td>
<td style="text-align: left;"><p>1.0</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>macOS 12 (Monterey)</p></td>
<td style="text-align: left;"><p>2.0</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>macOS 13 (Ventura)</p></td>
<td style="text-align: left;"><p>2.0</p></td>
</tr>
</tbody>
</table>

# Shared-Cache Strings (dsc) range descriptor

## Shared-Cache Strings (dsc) range descriptor - version 1

A Shared-Cache Strings (dsc) range descriptor - version 1 is 16 bytes of
size and consist of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID descriptor index</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(dsc) range offset</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data offset<br />
The offset is relative to the start of the file</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(dsc) range size</p></td>
</tr>
</tbody>
</table>

## Shared-Cache Strings (dsc) range descriptor - version 2

A Shared-Cache Strings (dsc) range descriptor - version 2 is 24 bytes of
size and consist of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(dsc) range offset</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Data offset<br />
The offset is relative to the start of the file</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(dsc) range size</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID descriptor index</p></td>
</tr>
</tbody>
</table>

# Shared-Cache Strings (dsc) UUID descriptor

## Shared-Cache Strings (dsc) UUID descriptor - version 1

A Shared-Cache Strings (dsc) UUID descriptor - version 1 is 28 bytes of
size and consist of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(dsc) text offset<br />
Contains the base image offset (or base sender program counter)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(dsc) text size</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Image (process or library)
identifier<br />
Contains a UUID stored in big-endian</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Image (process or library) path
offset<br />
The offset is relative to the start of the file</p></td>
</tr>
</tbody>
</table>

## Shared-Cache Strings (dsc) UUID descriptor - version 2

A Shared-Cache Strings (dsc) UUID descriptor - version 2 is 32 bytes of
size and consist of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(dsc) text offset<br />
Contains the base image offset (or base sender program counter)</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>(dsc) text size</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Image (process or library)
identifier<br />
Contains a UUID stored in big-endian</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Image (process or library) path
offset<br />
The offset is relative to the start of the file</p></td>
</tr>
</tbody>
</table>

# Notes

To view the contents of a shared-cache strings (dsc) file on Mac OS:

    log raw-dump -s /var/db/uuidtext/dsc/${FILE}

# UUID text file format

An UUID text (uuidtext) file consist of:

-   UUID text (uuidtext) file header

-   UUID text (uuidtext) entries

-   UUID text (uuidtext) footer

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Characteristics</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>Byte order</p></td>
<td style="text-align: left;"><p>little-endian</p></td>
</tr>
</tbody>
</table>

# UUID text (uuidtext) file header

The UUID text (uuidtext) file header is variable of size and consists
of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>"\x99\x88\x77\x66"</p></td>
<td style="text-align: left;"><p>Signature</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (format major
version?)</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (format minor
version?)</span></strong></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of entries</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>8 x number of entries</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Array of entry descriptors</p></td>
</tr>
</tbody>
</table>

The UUID text (uuidtext) entry descriptor is 8 bytes of size and
consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Range start offset</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Entry size</p></td>
</tr>
</tbody>
</table>

# UUID text (uuidtext) file footer

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Image (process or library) path<br />
Contains an UTF-8 formatted string with an end-of-string
character</p></td>
</tr>
</tbody>
</table>

# <span id="converting_continuous_time"></span>Converting a Mach continuous in to wall clock time

The Apple Unified Logging and Activity Tracing formats contain both Mach
continuous time and POSIX timestamps. The exact method of converting
between both is undocumented but the following behavior has been
observed:

# Without corresponding timesync database file

If there is no timesync database corresponding to the boot identifier:

-   use the timebase numerator and timebase denominator of the .tracev3
    file to determine the timebase:

<!-- -->

    timebase = numerator / denominator

-   calculate the wall clock time from the continuous time

<!-- -->

    wall clock time = header wall clock time + (continuous time * timebase)

**<span class="yellow-background">TODO it is currently unknown if a
floor or ceil function is used to convert the timebase corrected
continuous time to an integer</span>**

# With corresponding timesync database file

If there is a timesync database corresponding to the boot identifier:

-   use the timebase numerator and timebase denominator of the
    corresponding timesync boot record to determine the timebase:

<!-- -->

    timebase = numerator / denominator

-   find the timesync sync record that corresponds to the continuous
    time, if none applicable the timesync boot record is used

-   determine the base continous time from the record, for timesync boot
    record the base is 0

-   calculate the wall clock time from the continuous time

<!-- -->

    wall clock time = record wall clock time + ((continuous time - base continuous time) * timebase)

**<span class="yellow-background">TODO it is currently unknown if a
floor or ceil function is used to convert the timebase corrected
continuous time to an integer</span>**

# Value type decoders

# <span id="locationd_core_location_client_authorization_status"></span>locationd Core Location client authorization status

The locationd Core Location client authorization status value is 8 bytes
of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Authorization status</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    "%s"

Where %s contains one of the following descriptions based on the value
for example:

    "Denied"

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Not Determined</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Restricted</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Denied</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Authorized Always</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Authorized When In Use</p></td>
</tr>
</tbody>
</table>

# <span id="locationd_core_location_daemon_reachability_status_types"></span>locationd Core Location daemon reachability status types

**<span class="yellow-background">TODO describe</span>**

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Reachability Unavailable</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Reachability Small</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Reachability Large</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>56</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Reachability Unachievable</p></td>
</tr>
</tbody>
</table>

# <span id="locationd_core_location_daemon_tracker_state"></span>locationd Core Location daemon tracker state

The locationd Core Location Daemon status tracker state
(CLDaemonStatusStateTracker) is 36 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Level<br />
Contains a floating-point value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Is charged<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>9</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Is connected<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>10</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Charger type</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>14</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Was connected<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>15</p></td>
<td style="text-align: left;"><p>9</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Reachability</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Thermal level<br />
Contains a signed integer</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Airplane mode (enabled)<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>33</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Battery saver mode (enabled)<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>34</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Push service<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>35</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Restricted mode (enabled)<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    {"thermalLevel":-1,"reachability":"kReachabilityLarge","airplaneMode":false,"batteryData":{"wasConnected":false,"charged":false,"level":-1,"connected":false,"chargerType":"kChargerTypeUnknown"},"restrictedMode":false,"batterySaverModeEnabled":false}"

Charger type values:

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>kChargerTypeUnknown</p></td>
<td style="text-align: left;"></td>
</tr>
</tbody>
</table>

Reachability values:

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>kReachabilityLarge</p></td>
<td style="text-align: left;"></td>
</tr>
</tbody>
</table>

# <span id="locationd_core_location_client_manager_state_tracker_state"></span>locationd Core Location client manager (CLClientManager) state tracker state

The locationd Core Location client manager (CLClientManager) state
tracker state is 8 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Location enabled status</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Location restricted<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
</tbody>
</table>

**<span class="yellow-background">TODO: confirm location enabled status
is the first value in the structure. Only seen data where both values
are 0.</span>**

The value is formatted as:

    {"locationRestricted":false,"locationServicesEnabledStatus":0}

# <span id="locationd_core_location_location_manager_state_tracker_state"></span>locationd Core Location location manager (CLLocationManager) state tracker state

The locationd Core Location location manager (CLLocationManager) state
tracker state is 64 or 72 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Distance filter<br />
Contains a floating-point value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Desired accuracy<br />
Contains a floating-point value</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Updating location<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>17</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Requestiong location<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>18</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Requestiong ranging<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>19</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Updating ranging<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>20</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Updating heading<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>21</p></td>
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Heading filter<br />
Contains a floating-point value</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>32</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Allows location prompts<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>33</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Allows altered accessory location<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>34</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Dynamic accuracy reduction
enabled<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>35</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Previous authorization status
valid<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>36</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Previous authorization status</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>40</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Limits precision<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>41</p></td>
<td style="text-align: left;"><p>7</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>48</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Activity type<br />
Contains a signed integer</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>56</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Pauses location updates
automatically<br />
Contains a signed integer</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>60</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Paused<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>61</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Allows background location
updates<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>62</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Shows background location
indicator<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>63</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Allows map correction<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td colspan="4" style="text-align: left;"><p><em>Additional values if
size &gt; 64</em></p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>64</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Batching location<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>65</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Updating vehicle speed<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>66</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Updating vehicle heading<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>67</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Match information enabled<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>68</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Ground altitude enabled<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>69</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Fusion information enabled<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>70</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Courtesy prompt needed<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>71</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Is authorized for widget updates<br />
Contains a boolean value where false if 0 or true otherwise</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    {"previousAuthorizationStatusValid":false,"paused":false,"requestingLocation":false,"updatingVehicleSpeed":false,"desiredAccuracy":100,"allowsBackgroundLocationUpdates":false,"dynamicAccuracyReductionEnabled":false,"distanceFilter":-1,"allowsLocationPrompts":true,"activityType":0,"groundAltitudeEnabled":false,"pausesLocationUpdatesAutomatially":1,"fusionInfoEnabled":false,"isAuthorizedForWidgetUpdates":false,"updatingVehicleHeading":false,"batchingLocation":false,"showsBackgroundLocationIndicator":false,"updatingLocation":false,"requestingRanging":false,"updatingHeading":false,"previousAuthorizationStatus":0,"allowsMapCorrection":true,"matchInfoEnabled":false,"allowsAlteredAccessoryLoctions":false,"updatingRanging":false,"limitsPrecision":false,"courtesyPromptNeeded":false,"headingFilter":1}

# <span id="locationd_core_location_sub_harvester_identifier"></span>locationd Core Location sub harvester identifier

**<span class="yellow-background">TODO describe</span>**

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Wifi</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Tracks</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Realtime</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>App</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Pass</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Indoor</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>7</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Pressure</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Poi</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>9</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Trace</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>10</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Avenger</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>11</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Altimeter</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Ionosphere</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>13</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Unknown</p></td>
</tr>
</tbody>
</table>

# <span id="locationd_io_message"></span>locationd IO message

**<span class="yellow-background">TODO describe</span>**

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>3758096400</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ServiceIsTerminated</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758096416</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ServiceIsSuspended</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758096432</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ServiceIsResumed</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758096640</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ServiceIsRequestingClose</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758096641</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ServiceIsAttemptingOpen</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758096656</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ServiceWasClosed</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758096672</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ServiceBusyStateChange</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758096680</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ConsoleSecurityChange</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758096688</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ServicePropertyChange</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758096896</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>CanDevicePowerOff</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758096912</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>DeviceWillPowerOff</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758096928</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>DeviceWillNotPowerOff</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758096944</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>DeviceHasPoweredOn</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758096976</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SystemWillPowerOff</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758096981</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SystemPagingOff</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758097008</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>CanSystemSleep</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758097024</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SystemWillSleep</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758097040</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SystemWillNotSleep</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758097152</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SystemHasPoweredOn</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758097168</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SystemWillRestart</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758097184</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SystemWillPowerOn</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758097200</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>CopyClientID</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3758097216</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SystemCapabilityChange</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3758097232</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>DeviceSignaledWakeup</p></td>
</tr>
</tbody>
</table>

# <span id="locationd_sqlite_result"></span>locationd SQLite result

The locationd SQLite result is 4 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SQLite result or error code</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    "%s"

Where "%s" contains one of the following identifiers based on the
[value](#locationd_sqlite_result_values) for example:

    "SQLITE_OK"

## <span id="locationd_sqlite_result_values"></span>locationd SQLite result values

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>SQLITE_OK</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>SQLITE_ERROR</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>SQLITE_INTERNAL</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"><p>SQLITE_PERM</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>SQLITE_ABORT</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5</p></td>
<td style="text-align: left;"><p>SQLITE_BUSY</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"><p>SQLITE_LOCKED</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>7</p></td>
<td style="text-align: left;"><p>SQLITE_NOMEM</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>SQLITE_READONLY</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>9</p></td>
<td style="text-align: left;"><p>SQLITE_INTERRUPT</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>10</p></td>
<td style="text-align: left;"><p>SQLITE_IOERR</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>11</p></td>
<td style="text-align: left;"><p>SQLITE_CORRUPT</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"><p>SQLITE_NOTFOUND</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>13</p></td>
<td style="text-align: left;"><p>SQLITE_FULL</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>14</p></td>
<td style="text-align: left;"><p>SQLITE_CANTOPEN</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>15</p></td>
<td style="text-align: left;"><p>SQLITE_PROTOCOL</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>SQLITE_EMPTY</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>17</p></td>
<td style="text-align: left;"><p>SQLITE_SCHEMA</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>18</p></td>
<td style="text-align: left;"><p>SQLITE_TOOBIG</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>19</p></td>
<td style="text-align: left;"><p>SQLITE_CONSTRAINT</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>20</p></td>
<td style="text-align: left;"><p>SQLITE_MISMATCH</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>21</p></td>
<td style="text-align: left;"><p>SQLITE_MISUSE</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>22</p></td>
<td style="text-align: left;"><p>SQLITE_NOLFS</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>23</p></td>
<td style="text-align: left;"><p>SQLITE_AUTH</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>SQLITE_FORMAT</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>25</p></td>
<td style="text-align: left;"><p>SQLITE_RANGE</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>26</p></td>
<td style="text-align: left;"><p>SQLITE_NOTADB</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>27</p></td>
<td style="text-align: left;"><p>SQLITE_NOTICE</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"><p>SQLITE_WARNING</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>100</p></td>
<td style="text-align: left;"><p>SQLITE_ROW</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>101</p></td>
<td style="text-align: left;"><p>SQLITE_DONE</p></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td colspan="3" style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>266</p></td>
<td style="text-align: left;"><p>SQLITE IO ERR READ</p></td>
<td style="text-align: left;"></td>
</tr>
</tbody>
</table>

# <span id="mask_hash"></span>Mask hash

The mask hash is 16 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    <mask.hash: '%s'>

Where "%s" contains the value encoded in base64, for example:

    <mask.hash: 'HR/T++mmRmq3Mn+2mGECsg=='>

# <span id="mdns_dns_counters"></span>mDNS DNS counters

The mDNS DNS counters are 8 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of additional records</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of authority records</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of answers</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of questions</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    1/0/0/0

where:

    questions / answers / authority records / additional records

# <span id="mdns_dns_header"></span>mDNS DNS header

The mDNS DNS header is 12 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Identifier</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Flags</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of questions</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of answers</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of authority records</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>10</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Number of additional records</p></td>
</tr>
</tbody>
</table>

The values of the mDNS DNS header are stored in big-endian.

The value is formatted as:

    id: 0x0000 (0), flags: 0x8180 (R/Query, RD, RA, NoError), counts: 1/0/0/0

Where "counts" consists of:

    questions / answers / authority records / additional records

# <span id="mdns_dns_identifier_and_flags"></span>mDNS DNS identifier and flags

The mDNS DNS identifier and flags are 8 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Flags</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Identifier</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown</span></strong></p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    id: 0x4BBB (19387), flags: 0x0100 (Q/Query, RD, NoError)

# <span id="mdns_getaddrinfo_options"></span>mDNS getaddrinfo options

**<span class="yellow-background">TODO describe flags</span>**

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x0004</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>in-app-browser</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x0008</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>use-failover</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0x0010</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>prohibit-encrypted-dns</p></td>
</tr>
</tbody>
</table>

# <span id="mdns_protocol"></span>mDNS protocol

The mDNS protocol value is 4 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>mDNS resource record (RR) type
value</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    %s

Where %s contains one of the following descriptions based on the value
for example:

    UDP

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UDP</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TCP</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>HTTPS</p></td>
</tr>
</tbody>
</table>

# <span id="mdns_reason"></span>mDNS reason

The mDNS reason value is 4 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>mDNS reason value</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    %s

Where %s contains one of the following descriptions based on the value
for example:

    query-suppressed

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>no-data</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>nxdomain</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>query-suppressed</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>no-dns-service</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>server error</p></td>
</tr>
</tbody>
</table>

**<span class="yellow-background">TODO check values 2, 4 and 5</span>**

# <span id="mdns_resource_record_type"></span>mDNS resource record (RR) type

The mDNS resource record (RR) type value is 4 bytes of size and consists
of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>mDNS resource record (RR) type
value</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    %s

Where %s contains one of the following descriptions based on the value
for example:

    A

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>A</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>NS</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>CNAME</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SOA</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>PTR</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>13</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>HINFO</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>15</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>MX</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TXT</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>17</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>RP</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>18</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>AFSDB</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SIG</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>25</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>KEY</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>28</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>AAAA</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>29</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>LOC</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>33</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SRV</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>35</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>NAPTR</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>36</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>KX</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>37</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>CERT</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>39</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>DNAME</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>42</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>APL</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>43</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>DS</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>44</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SSHFP</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>45</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>IPSECKEY</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>46</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>RRSIG</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>47</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>NSEC</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>48</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>DNSKEY</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>49</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>DHCID</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>50</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>NSEC3</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>51</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>NSEC3PARAM</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>52</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TLSA</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>53</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SMIMEA</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>55</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>HIP</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>59</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>CDS</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>60</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>CDNSKEY</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>61</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>OPENPGPKEY</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>62</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>CSYNC</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>63</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ZONEMD</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>64</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SVCB</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>65</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>HTTPS</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>108</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>EUI48</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>109</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>EUI64</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>249</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TKEY</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>250</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TSIG</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>256</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>URI</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>257</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>CAA</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>32768</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>TA</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>32769</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>DLV</p></td>
</tr>
</tbody>
</table>

# <span id="mdnsresponder_ip_address"></span>mDNSResponder IP address

It is currently unknown how the value is formatted is has only been
observed in combination with the "mask.hash" value type formatter.

# <span id="mdnsresponder_mac_address"></span>mDNSResponder MAC address

It is currently unknown how the value is formatted is has only been
observed in combination with the "mask.hash" value type formatter.

# <span id="open_directory_error"></span>Open Directory error

The Open Directory error code is 4 bytes of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Open Directory error code</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    %s

Where %s contains one of the following descriptions based on the value
for example:

    ODNoError

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>kODErrorSuccess</p></td>
<td style="text-align: left;"><p>ODNoError</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Not Found</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>1000</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorSessionLocalOnlyDaemonInUse</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1001</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorSessionNormalDaemonInUse</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>1002</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorSessionDaemonNotRunning</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1003</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorSessionDaemonRefused</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1100</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorSessionProxyCommunicationError</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>1101</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorSessionProxyVersionMismatch</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1102</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorSessionProxyIPUnreachable</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>1103</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorSessionProxyUnknownHost</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2000</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorNodeUnknownName</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>2001</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorNodeUnknownType</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2002</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorNodeDisabled</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2100</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorNodeConnectionFailed</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2200</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorNodeUnknownHost</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3000</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorQuerySynchronize</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3100</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorQueryInvalidMatchType</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3101</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorQueryUnsupportedMatchType</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>3102</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorQueryTimeout</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4000</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorRecordReadOnlyNode</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4001</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorRecordPermissionError</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4100</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorRecordParameterError</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4101</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorRecordInvalidType</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4102</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorRecordAlreadyExists</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4103</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorRecordTypeDisabled</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4104</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorRecordNoLongerExists</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4200</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorRecordAttributeUnknownType</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4201</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorRecordAttributeNotFound</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4202</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorRecordAttributeValueSchemaError</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4203</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorRecordAttributeValueNotFound</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5000</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorCredentialsInvalid</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5001</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsInvalidComputer</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5100</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsMethodNotSupported</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5101</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsNotAuthorized</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5102</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsParameterError</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5103</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsOperationFailed</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5200</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsServerUnreachable</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5201</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsServerNotFound</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5202</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorCredentialsServerError</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5203</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsServerTimeout</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5204</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsContactPrimary</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5205</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsServerCommunicationError</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5300</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsAccountNotFound</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5301</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsAccountDisabled</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5302</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsAccountExpired</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5303</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsAccountInactive</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5304</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsAccountTemporarilyLocked</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5305</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsAccountLocked</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5400</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsPasswordExpired</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5401</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsPasswordChangeRequired</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5402</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsPasswordQualityFailed</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5403</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsPasswordTooShort</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5404</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsPasswordTooLong</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5405</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsPasswordNeedsLetter</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5406</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsPasswordNeedsDigit</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5407</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsPasswordChangeTooSoon</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5408</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsPasswordUnrecoverable</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>5500</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorCredentialsInvalidLogonHours</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>6000</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorPolicyUnsupported</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>6001</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorPolicyOutOfRange</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>10000</p></td>
<td style="text-align: left;"></td>
<td
style="text-align: left;"><p>ODErrorPluginOperationNotSupported</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>10001</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorPluginError</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>10002</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorDaemonError</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>10003</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>ODErrorPluginOperationTimeout</p></td>
</tr>
</tbody>
</table>

## Notes

    List of Open Directory error codes:
    https://developer.apple.com/documentation/opendirectory/odframeworkerrors?changes=__2&language=objc

# <span id="open_directory_membership_details"></span>Open Directory membership details

The Open Directory membership details is variable of size and consists
of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Type indicator</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Membership data</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Domain<br />
Contains an UTF-8 encoded string with end-of-string character</p></td>
</tr>
</tbody>
</table>

Where membership data is type indicator dependent:

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Type indicator</th>
<th style="text-align: left;">Membership data</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0x23</p></td>
<td style="text-align: left;"><p>user identifier (UID) Contains a 32-bit
signed integer</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x24</p></td>
<td style="text-align: left;"><p>Username<br />
Contains an UTF-8 encoded string with end-of-string character</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0x44</p></td>
<td style="text-align: left;"><p>Group name<br />
Contains an UTF-8 encoded string with end-of-string character</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0xa0</p></td>
<td style="text-align: left;"><p>Username<br />
Contains an UTF-8 encoded string with end-of-string character</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>0xa3</p></td>
<td style="text-align: left;"><p>user identifier (UID) Contains a 32-bit
signed integer</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0xa4</p></td>
<td style="text-align: left;"><p>Username<br />
Contains an UTF-8 encoded string with end-of-string character</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>0xc3</p></td>
<td style="text-align: left;"><p>group identifier (GID) Contains a
32-bit signed integer</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    user: 501@/Local/Default

or

    group: wheel@/Local/Default

# <span id="open_directory_membership_identifier_type"></span>Open Directory membership identifier type

**<span class="yellow-background">TODO describe</span>**

    /Library/Developer/CommandLineTools/SDKs/MacOSX12.3.sdk/usr/include/membership.h

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UID</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>GID</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>3</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>SID</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Username</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>5</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Groupname</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>6</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>UUID</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>7</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>GROUP NFS</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>USER NFS</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>10</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>GSS EXPORT NAME</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>11</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>X509 DN</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>12</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>KERBEROS</p></td>
</tr>
</tbody>
</table>

# <span id="open_directory_windows_nt_sid"></span>Open Directory Windows NT Security Identifier (SID)

The format of a Windows NT Security Identifier (SID) is described in
[\[LIBFWNT\]](https://github.com/libyal/libfwnt/blob/main/documentation/Security%20Descriptor.asciidoc)

The value is formatted as:

    S-1-5-1-2-3-4-5

# <span id="socket_address"></span>Socket address

The socket address record is variable of size and consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Record size</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Address family</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>…</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Address data</p></td>
</tr>
</tbody>
</table>

## Address family

<table>
<colgroup>
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 71%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Identifier</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>AF_INET</p></td>
<td style="text-align: left;"><p>IPv4 address</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>30</p></td>
<td style="text-align: left;"><p>AF_INET6</p></td>
<td style="text-align: left;"><p>IPv6 address</p></td>
</tr>
</tbody>
</table>

# IPv4 socket address

The IPv4 socket address (sockaddr\_in) record is 16 bytes of size and
consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"><p>Record size</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>Address family</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Port number</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>IPv4 address</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p><strong><span
class="yellow-background">Unknown (reserved)</span></strong></p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    127.0.0.1

# IPv6 socket address

The IPv6 socket address (sockaddr\_in6) record is 28 bytes of size and
consists of:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 12%" />
<col style="width: 63%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Offset</th>
<th style="text-align: left;">Size</th>
<th style="text-align: left;">Value</th>
<th style="text-align: left;">Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>0</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>20</p></td>
<td style="text-align: left;"><p>Record size</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>1</p></td>
<td style="text-align: left;"><p>30</p></td>
<td style="text-align: left;"><p>Address family</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"><p>2</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Port number</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Flow information</p></td>
</tr>
<tr class="odd">
<td style="text-align: left;"><p>8</p></td>
<td style="text-align: left;"><p>16</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>IPv6 address</p></td>
</tr>
<tr class="even">
<td style="text-align: left;"><p>24</p></td>
<td style="text-align: left;"><p>4</p></td>
<td style="text-align: left;"></td>
<td style="text-align: left;"><p>Scope zone index</p></td>
</tr>
</tbody>
</table>

The value is formatted as:

    fe80::f:86ff:fee9:5c16

# Notes

    plutil -p /var/db/diagnostics/version.plist
    {
      "Identifier" => "9C956601-D721-47E0-BBB7-42AF4351FF4E"
      "ttl01" => {
        "ContinuousTime" => 393453185112398
        "UUID" => "BBF90666-3E6D-4DD5-9A57-99F2A94F4955"
      }
      "ttl03" => {
        "ContinuousTime" => 220653185112398
        "UUID" => "BBF90666-3E6D-4DD5-9A57-99F2A94F4955"
      }
      "ttl07" => {
        "ContinuousTime" => 211836946939114
        "UUID" => "83C643BF-0E8A-466E-8EFC-156EEADBA2D5"
      }
      "ttl14" => {
        "ContinuousTime" => 298223698807905
        "UUID" => "862A1404-20FC-4C3B-84A7-FB03D37E0EA0"
      }
      "ttl30" => {
        "ContinuousTime" => 406810835343916
        "UUID" => "E1693458-8845-48EF-A9AE-E9C8CA37E46E"
      }
      "Version" => 7
    }

**<span class="yellow-background">TODO what about .atrc and .ktrace
files?</span>**

# Output

    log raw-dump -A test.logarchive

Output starts with \*.tracev3 files under "Signpost" followed by
"logdata.LiveData.tracev3", "Special", "Persist"

# JSON output

`log show --style json --timezone UTC --backtrace --debug --info --loss --signpost --file *.tracev3`

traceID of a Firehose Tracepoint consists of:

    ( fmt lower 32-bit << 32 ) | ( tp flags << 16 ) | ( tp log type << 8 ) | ( tp record type )

traceID of SimpleDump and StateDump is 0

**<span class="yellow-background">TODO describe
senderProgramCounter</span>**

# References

`[REFERENCE]`

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Title:</th>
<th style="text-align: left;">Apple Developer: COMPRESSION_LZ4</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>URL:</p></td>
<td style="text-align: left;"><p><a
href="https://developer.apple.com/documentation/compression/compression_lz4">https://developer.apple.com/documentation/compression/compression_lz4</a></p></td>
</tr>
</tbody>
</table>

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<thead>
<tr class="header">
<th style="text-align: left;">Title:</th>
<th style="text-align: left;">Class dump of CDStructures.h</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td style="text-align: left;"><p>URL:</p></td>
<td style="text-align: left;"><p><a
href="https://github.com/w0lfschild/macOS_headers/blob/master/macOS/PrivateFrameworks/LoggingSupport/906.250.3/CDStructures.h">https://github.com/w0lfschild/macOS_headers/blob/master/macOS/PrivateFrameworks/LoggingSupport/906.250.3/CDStructures.h</a></p></td>
</tr>
</tbody>
</table>

# GNU Free Documentation License

Version 1.3, 3 November 2008 Copyright © 2000, 2001, 2002, 2007, 2008
Free Software Foundation, Inc. <http://fsf.org/>

Everyone is permitted to copy and distribute verbatim copies of this
license document, but changing it is not allowed.

# 0. PREAMBLE

The purpose of this License is to make a manual, textbook, or other
functional and useful document "free" in the sense of freedom: to assure
everyone the effective freedom to copy and redistribute it, with or
without modifying it, either commercially or noncommercially.
Secondarily, this License preserves for the author and publisher a way
to get credit for their work, while not being considered responsible for
modifications made by others.

This License is a kind of "copyleft", which means that derivative works
of the document must themselves be free in the same sense. It
complements the GNU General Public License, which is a copyleft license
designed for free software.

We have designed this License in order to use it for manuals for free
software, because free software needs free documentation: a free program
should come with manuals providing the same freedoms that the software
does. But this License is not limited to software manuals; it can be
used for any textual work, regardless of subject matter or whether it is
published as a printed book. We recommend this License principally for
works whose purpose is instruction or reference.

# 1. APPLICABILITY AND DEFINITIONS

This License applies to any manual or other work, in any medium, that
contains a notice placed by the copyright holder saying it can be
distributed under the terms of this License. Such a notice grants a
world-wide, royalty-free license, unlimited in duration, to use that
work under the conditions stated herein. The "Document", below, refers
to any such manual or work. Any member of the public is a licensee, and
is addressed as "you". You accept the license if you copy, modify or
distribute the work in a way requiring permission under copyright law.

A "Modified Version" of the Document means any work containing the
Document or a portion of it, either copied verbatim, or with
modifications and/or translated into another language.

A "Secondary Section" is a named appendix or a front-matter section of
the Document that deals exclusively with the relationship of the
publishers or authors of the Document to the Document’s overall subject
(or to related matters) and contains nothing that could fall directly
within that overall subject. (Thus, if the Document is in part a
textbook of mathematics, a Secondary Section may not explain any
mathematics.) The relationship could be a matter of historical
connection with the subject or with related matters, or of legal,
commercial, philosophical, ethical or political position regarding them.

The "Invariant Sections" are certain Secondary Sections whose titles are
designated, as being those of Invariant Sections, in the notice that
says that the Document is released under this License. If a section does
not fit the above definition of Secondary then it is not allowed to be
designated as Invariant. The Document may contain zero Invariant
Sections. If the Document does not identify any Invariant Sections then
there are none.

The "Cover Texts" are certain short passages of text that are listed, as
Front-Cover Texts or Back-Cover Texts, in the notice that says that the
Document is released under this License. A Front-Cover Text may be at
most 5 words, and a Back-Cover Text may be at most 25 words.

A "Transparent" copy of the Document means a machine-readable copy,
represented in a format whose specification is available to the general
public, that is suitable for revising the document straightforwardly
with generic text editors or (for images composed of pixels) generic
paint programs or (for drawings) some widely available drawing editor,
and that is suitable for input to text formatters or for automatic
translation to a variety of formats suitable for input to text
formatters. A copy made in an otherwise Transparent file format whose
markup, or absence of markup, has been arranged to thwart or discourage
subsequent modification by readers is not Transparent. An image format
is not Transparent if used for any substantial amount of text. A copy
that is not "Transparent" is called "Opaque".

Examples of suitable formats for Transparent copies include plain ASCII
without markup, Texinfo input format, LaTeX input format, SGML or XML
using a publicly available DTD, and standard-conforming simple HTML,
PostScript or PDF designed for human modification. Examples of
transparent image formats include PNG, XCF and JPG. Opaque formats
include proprietary formats that can be read and edited only by
proprietary word processors, SGML or XML for which the DTD and/or
processing tools are not generally available, and the machine-generated
HTML, PostScript or PDF produced by some word processors for output
purposes only.

The "Title Page" means, for a printed book, the title page itself, plus
such following pages as are needed to hold, legibly, the material this
License requires to appear in the title page. For works in formats which
do not have any title page as such, "Title Page" means the text near the
most prominent appearance of the work’s title, preceding the beginning
of the body of the text.

The "publisher" means any person or entity that distributes copies of
the Document to the public.

A section "Entitled XYZ" means a named subunit of the Document whose
title either is precisely XYZ or contains XYZ in parentheses following
text that translates XYZ in another language. (Here XYZ stands for a
specific section name mentioned below, such as "Acknowledgements",
"Dedications", "Endorsements", or "History".) To "Preserve the Title" of
such a section when you modify the Document means that it remains a
section "Entitled XYZ" according to this definition.

The Document may include Warranty Disclaimers next to the notice which
states that this License applies to the Document. These Warranty
Disclaimers are considered to be included by reference in this License,
but only as regards disclaiming warranties: any other implication that
these Warranty Disclaimers may have is void and has no effect on the
meaning of this License.

# 2. VERBATIM COPYING

You may copy and distribute the Document in any medium, either
commercially or noncommercially, provided that this License, the
copyright notices, and the license notice saying this License applies to
the Document are reproduced in all copies, and that you add no other
conditions whatsoever to those of this License. You may not use
technical measures to obstruct or control the reading or further copying
of the copies you make or distribute. However, you may accept
compensation in exchange for copies. If you distribute a large enough
number of copies you must also follow the conditions in section 3.

You may also lend copies, under the same conditions stated above, and
you may publicly display copies.

# 3. COPYING IN QUANTITY

If you publish printed copies (or copies in media that commonly have
printed covers) of the Document, numbering more than 100, and the
Document’s license notice requires Cover Texts, you must enclose the
copies in covers that carry, clearly and legibly, all these Cover Texts:
Front-Cover Texts on the front cover, and Back-Cover Texts on the back
cover. Both covers must also clearly and legibly identify you as the
publisher of these copies. The front cover must present the full title
with all words of the title equally prominent and visible. You may add
other material on the covers in addition. Copying with changes limited
to the covers, as long as they preserve the title of the Document and
satisfy these conditions, can be treated as verbatim copying in other
respects.

If the required texts for either cover are too voluminous to fit
legibly, you should put the first ones listed (as many as fit
reasonably) on the actual cover, and continue the rest onto adjacent
pages.

If you publish or distribute Opaque copies of the Document numbering
more than 100, you must either include a machine-readable Transparent
copy along with each Opaque copy, or state in or with each Opaque copy a
computer-network location from which the general network-using public
has access to download using public-standard network protocols a
complete Transparent copy of the Document, free of added material. If
you use the latter option, you must take reasonably prudent steps, when
you begin distribution of Opaque copies in quantity, to ensure that this
Transparent copy will remain thus accessible at the stated location
until at least one year after the last time you distribute an Opaque
copy (directly or through your agents or retailers) of that edition to
the public.

It is requested, but not required, that you contact the authors of the
Document well before redistributing any large number of copies, to give
them a chance to provide you with an updated version of the Document.

# 4. MODIFICATIONS

You may copy and distribute a Modified Version of the Document under the
conditions of sections 2 and 3 above, provided that you release the
Modified Version under precisely this License, with the Modified Version
filling the role of the Document, thus licensing distribution and
modification of the Modified Version to whoever possesses a copy of it.
In addition, you must do these things in the Modified Version:

1.  Use in the Title Page (and on the covers, if any) a title distinct
    from that of the Document, and from those of previous versions
    (which should, if there were any, be listed in the History section
    of the Document). You may use the same title as a previous version
    if the original publisher of that version gives permission.

2.  List on the Title Page, as authors, one or more persons or entities
    responsible for authorship of the modifications in the Modified
    Version, together with at least five of the principal authors of the
    Document (all of its principal authors, if it has fewer than five),
    unless they release you from this requirement.

3.  State on the Title page the name of the publisher of the Modified
    Version, as the publisher.

4.  Preserve all the copyright notices of the Document.

5.  Add an appropriate copyright notice for your modifications adjacent
    to the other copyright notices.

6.  Include, immediately after the copyright notices, a license notice
    giving the public permission to use the Modified Version under the
    terms of this License, in the form shown in the Addendum below.

7.  Preserve in that license notice the full lists of Invariant Sections
    and required Cover Texts given in the Document’s license notice.

8.  Include an unaltered copy of this License.

9.  Preserve the section Entitled "History", Preserve its Title, and add
    to it an item stating at least the title, year, new authors, and
    publisher of the Modified Version as given on the Title Page. If
    there is no section Entitled "History" in the Document, create one
    stating the title, year, authors, and publisher of the Document as
    given on its Title Page, then add an item describing the Modified
    Version as stated in the previous sentence.

10. Preserve the network location, if any, given in the Document for
    public access to a Transparent copy of the Document, and likewise
    the network locations given in the Document for previous versions it
    was based on. These may be placed in the "History" section. You may
    omit a network location for a work that was published at least four
    years before the Document itself, or if the original publisher of
    the version it refers to gives permission.

11. For any section Entitled "Acknowledgements" or "Dedications",
    Preserve the Title of the section, and preserve in the section all
    the substance and tone of each of the contributor acknowledgements
    and/or dedications given therein.

12. Preserve all the Invariant Sections of the Document, unaltered in
    their text and in their titles. Section numbers or the equivalent
    are not considered part of the section titles.

13. Delete any section Entitled "Endorsements". Such a section may not
    be included in the Modified Version.

14. Do not retitle any existing section to be Entitled "Endorsements" or
    to conflict in title with any Invariant Section.

15. Preserve any Warranty Disclaimers.

If the Modified Version includes new front-matter sections or appendices
that qualify as Secondary Sections and contain no material copied from
the Document, you may at your option designate some or all of these
sections as invariant. To do this, add their titles to the list of
Invariant Sections in the Modified Version’s license notice. These
titles must be distinct from any other section titles.

You may add a section Entitled "Endorsements", provided it contains
nothing but endorsements of your Modified Version by various parties—for
example, statements of peer review or that the text has been approved by
an organization as the authoritative definition of a standard.

You may add a passage of up to five words as a Front-Cover Text, and a
passage of up to 25 words as a Back-Cover Text, to the end of the list
of Cover Texts in the Modified Version. Only one passage of Front-Cover
Text and one of Back-Cover Text may be added by (or through arrangements
made by) any one entity. If the Document already includes a cover text
for the same cover, previously added by you or by arrangement made by
the same entity you are acting on behalf of, you may not add another;
but you may replace the old one, on explicit permission from the
previous publisher that added the old one.

The author(s) and publisher(s) of the Document do not by this License
give permission to use their names for publicity for or to assert or
imply endorsement of any Modified Version.

# 5. COMBINING DOCUMENTS

You may combine the Document with other documents released under this
License, under the terms defined in section 4 above for modified
versions, provided that you include in the combination all of the
Invariant Sections of all of the original documents, unmodified, and
list them all as Invariant Sections of your combined work in its license
notice, and that you preserve all their Warranty Disclaimers.

The combined work need only contain one copy of this License, and
multiple identical Invariant Sections may be replaced with a single
copy. If there are multiple Invariant Sections with the same name but
different contents, make the title of each such section unique by adding
at the end of it, in parentheses, the name of the original author or
publisher of that section if known, or else a unique number. Make the
same adjustment to the section titles in the list of Invariant Sections
in the license notice of the combined work.

In the combination, you must combine any sections Entitled "History" in
the various original documents, forming one section Entitled "History";
likewise combine any sections Entitled "Acknowledgements", and any
sections Entitled "Dedications". You must delete all sections Entitled
"Endorsements".

# 6. COLLECTIONS OF DOCUMENTS

You may make a collection consisting of the Document and other documents
released under this License, and replace the individual copies of this
License in the various documents with a single copy that is included in
the collection, provided that you follow the rules of this License for
verbatim copying of each of the documents in all other respects.

You may extract a single document from such a collection, and distribute
it individually under this License, provided you insert a copy of this
License into the extracted document, and follow this License in all
other respects regarding verbatim copying of that document.

# 7. AGGREGATION WITH INDEPENDENT WORKS

A compilation of the Document or its derivatives with other separate and
independent documents or works, in or on a volume of a storage or
distribution medium, is called an "aggregate" if the copyright resulting
from the compilation is not used to limit the legal rights of the
compilation’s users beyond what the individual works permit. When the
Document is included in an aggregate, this License does not apply to the
other works in the aggregate which are not themselves derivative works
of the Document.

If the Cover Text requirement of section 3 is applicable to these copies
of the Document, then if the Document is less than one half of the
entire aggregate, the Document’s Cover Texts may be placed on covers
that bracket the Document within the aggregate, or the electronic
equivalent of covers if the Document is in electronic form. Otherwise
they must appear on printed covers that bracket the whole aggregate.

# 8. TRANSLATION

Translation is considered a kind of modification, so you may distribute
translations of the Document under the terms of section 4. Replacing
Invariant Sections with translations requires special permission from
their copyright holders, but you may include translations of some or all
Invariant Sections in addition to the original versions of these
Invariant Sections. You may include a translation of this License, and
all the license notices in the Document, and any Warranty Disclaimers,
provided that you also include the original English version of this
License and the original versions of those notices and disclaimers. In
case of a disagreement between the translation and the original version
of this License or a notice or disclaimer, the original version will
prevail.

If a section in the Document is Entitled "Acknowledgements",
"Dedications", or "History", the requirement (section 4) to Preserve its
Title (section 1) will typically require changing the actual title.

# 9. TERMINATION

You may not copy, modify, sublicense, or distribute the Document except
as expressly provided under this License. Any attempt otherwise to copy,
modify, sublicense, or distribute it is void, and will automatically
terminate your rights under this License.

However, if you cease all violation of this License, then your license
from a particular copyright holder is reinstated (a) provisionally,
unless and until the copyright holder explicitly and finally terminates
your license, and (b) permanently, if the copyright holder fails to
notify you of the violation by some reasonable means prior to 60 days
after the cessation.

Moreover, your license from a particular copyright holder is reinstated
permanently if the copyright holder notifies you of the violation by
some reasonable means, this is the first time you have received notice
of violation of this License (for any work) from that copyright holder,
and you cure the violation prior to 30 days after your receipt of the
notice.

Termination of your rights under this section does not terminate the
licenses of parties who have received copies or rights from you under
this License. If your rights have been terminated and not permanently
reinstated, receipt of a copy of some or all of the same material does
not give you any rights to use it.

# 10. FUTURE REVISIONS OF THIS LICENSE

The Free Software Foundation may publish new, revised versions of the
GNU Free Documentation License from time to time. Such new versions will
be similar in spirit to the present version, but may differ in detail to
address new problems or concerns. See <http://www.gnu.org/copyleft/>.

Each version of the License is given a distinguishing version number. If
the Document specifies that a particular numbered version of this
License "or any later version" applies to it, you have the option of
following the terms and conditions either of that specified version or
of any later version that has been published (not as a draft) by the
Free Software Foundation. If the Document does not specify a version
number of this License, you may choose any version ever published (not
as a draft) by the Free Software Foundation. If the Document specifies
that a proxy can decide which future versions of this License can be
used, that proxy’s public statement of acceptance of a version
permanently authorizes you to choose that version for the Document.

# 11. RELICENSING

"Massive Multiauthor Collaboration Site" (or "MMC Site") means any World
Wide Web server that publishes copyrightable works and also provides
prominent facilities for anybody to edit those works. A public wiki that
anybody can edit is an example of such a server. A "Massive Multiauthor
Collaboration" (or "MMC") contained in the site means any set of
copyrightable works thus published on the MMC site.

"CC-BY-SA" means the Creative Commons Attribution-Share Alike 3.0
license published by Creative Commons Corporation, a not-for-profit
corporation with a principal place of business in San Francisco,
California, as well as future copyleft versions of that license
published by that same organization.

"Incorporate" means to publish or republish a Document, in whole or in
part, as part of another Document.

An MMC is "eligible for relicensing" if it is licensed under this
License, and if all works that were first published under this License
somewhere other than this MMC, and subsequently incorporated in whole or
in part into the MMC, (1) had no cover texts or invariant sections, and
(2) were thus incorporated prior to November 1, 2008.

The operator of an MMC Site may republish an MMC contained in the site
under CC-BY-SA on the same site at any time before August 1, 2009,
provided the MMC is eligible for relicensing.
