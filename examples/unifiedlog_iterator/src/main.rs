// Copyright 2022 Mandiant, Inc. All Rights Reserved
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software distributed under the License
// is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and limitations under the License.

use chrono::{SecondsFormat, TimeZone, Utc};
use log::{LevelFilter, error, info};
use macos_unifiedlogs::filesystem::{LiveSystemProvider, LogarchiveProvider};
use macos_unifiedlogs::log_data_iterator::{iterate_all_logs_callback, LogDataIterator};
use macos_unifiedlogs::parser::collect_timesync;
use macos_unifiedlogs::unified_log::LogData;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum, builder};
use csv::Writer;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Mode of operation
    #[clap(short, long)]
    mode: Mode,

    /// Path to logarchive formatted directory (log-archive mode) or tracev3 file (single-file
    /// mode)
    #[clap(short, long)]
    input: Option<PathBuf>,

    /// Filename to save results to
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Output format. Options: csv, jsonl
    #[clap(short, long, default_value = Format::Jsonl)]
    format: Format,

    /// Append to output file.
    /// If false, will overwrite output file
    #[clap(short, long, default_value = "false")]
    append: bool,
}

#[derive(Parser, Debug, Clone, ValueEnum)]
enum Mode {
    Live,
    LogArchive,
    SingleFile,
}

#[derive(Parser, Debug, Clone, ValueEnum)]
enum Format {
    Csv,
    Jsonl,
}

impl From<Format> for builder::OsStr {
    fn from(value: Format) -> Self {
        match value {
            Format::Csv => "csv".into(),
            Format::Jsonl => "jsonl".into(),
        }
    }
}

impl From<Format> for &str {
    fn from(value: Format) -> Self {
        match value {
            Format::Csv => "csv",
            Format::Jsonl => "jsonl",
        }
    }
}

fn main() {
    TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )
    .expect("Failed to initialize simple logger");
    info!("Starting Unified Log parser...");

    let args = Args::parse();
    let output_format = args.format;

    let handle: Box<dyn Write> = if let Some(path) = args.output {
        Box::new(
            fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap(),
        )
    } else {
        Box::new(std::io::stdout())
    };

    let mut writer = OutputWriter::new(Box::new(handle), output_format.into()).unwrap();

    match (args.mode, args.input) {
        (Mode::Live, None) => {
            parse_live_system(&mut writer);
        }
        (Mode::LogArchive, Some(path)) => {
            parse_log_archive(&path, &mut writer);
        }
        (Mode::SingleFile, Some(path)) => {
            parse_single_file(&path, &mut writer);
        }
        _ => {
            error!("log-archive and single-file modes require an --input argument");
        }
    }
}

fn parse_single_file(path: &Path, writer: &mut OutputWriter) {
    let mut provider = LogarchiveProvider::new(path);
    let timesync_data = collect_timesync(&provider).unwrap();

    let buf = match fs::read(path) {
        Ok(buf) => buf,
        Err(e) => {
            error!("Failed to read {path:?}: {e}");
            return;
        }
    };

    let iter = LogDataIterator::new(buf, &mut provider, &timesync_data, false);
    for entry in iter {
        if let Err(e) = writer.write_record(&entry) {
            error!("Error writing record: {e}");
        }
    }
}

// Parse a provided directory path. Currently, expect the path to follow macOS log collect structure
fn parse_log_archive(path: &Path, writer: &mut OutputWriter) {
    let mut provider = LogarchiveProvider::new(path);
    let timesync_data = collect_timesync(&provider).unwrap();

    let mut log_count: u64 = 0;
    iterate_all_logs_callback(
        &mut provider,
        &timesync_data,
        false,
        &mut |entry| {
            log_count += 1;
            if let Err(err) = writer.write_record(&entry) {
                log::error!("Failed to output log data: {err:?}");
            }
        },
    );
    info!("Parsed {log_count} log entries");
}

// Parse a live macOS system
fn parse_live_system(writer: &mut OutputWriter) {
    let mut provider = LiveSystemProvider::default();
    let timesync_data = collect_timesync(&provider).unwrap();

    let mut log_count: u64 = 0;
    iterate_all_logs_callback(
        &mut provider,
        &timesync_data,
        false,
        &mut |entry| {
            log_count += 1;
            if let Err(err) = writer.write_record(&entry) {
                log::error!("Failed to output log data: {err:?}");
            }
        },
    );
    info!("Parsed {log_count} log entries");
}

pub struct OutputWriter {
    writer: OutputWriterEnum,
}

enum OutputWriterEnum {
    Csv(Box<Writer<Box<dyn Write>>>),
    Json(Box<dyn Write>),
}

impl OutputWriter {
    pub fn new(writer: Box<dyn Write>, output_format: &str) -> Result<Self, Box<dyn Error>> {
        let writer_enum = match output_format {
            "csv" => {
                let mut csv_writer = Writer::from_writer(writer);
                // Write CSV headers
                csv_writer.write_record([
                    "Timestamp",
                    "Event Type",
                    "Log Type",
                    "Subsystem",
                    "Thread ID",
                    "PID",
                    "EUID",
                    "Library",
                    "Library UUID",
                    "Activity ID",
                    "Category",
                    "Process",
                    "Process UUID",
                    "Message",
                    "Raw Message",
                    "Boot UUID",
                    "System Timezone Name",
                ])?;
                csv_writer.flush()?;
                OutputWriterEnum::Csv(Box::new(csv_writer))
            }
            "jsonl" => OutputWriterEnum::Json(writer),
            _ => {
                error!("Unsupported output format: {output_format}");
                std::process::exit(1);
            }
        };

        Ok(OutputWriter {
            writer: writer_enum,
        })
    }

    pub fn write_record(&mut self, record: &LogData) -> Result<(), Box<dyn Error>> {
        match &mut self.writer {
            OutputWriterEnum::Csv(csv_writer) => {
                let date_time = Utc.timestamp_nanos(record.time as i64);
                csv_writer.write_record(&[
                    date_time.to_rfc3339_opts(SecondsFormat::Millis, true),
                    format!("{:?}", record.event_type),
                    format!("{:?}", record.log_type),
                    record.subsystem.to_string(),
                    record.thread_id.to_string(),
                    record.pid.to_string(),
                    record.euid.to_string(),
                    record.library.to_string(),
                    record.library_uuid.to_string(),
                    record.activity_id.to_string(),
                    record.category.to_string(),
                    record.process.to_string(),
                    record.process_uuid.to_string(),
                    record.message.to_string(),
                    record.raw_message.to_string(),
                    record.boot_uuid.to_string(),
                    record.timezone_name.to_string(),
                ])?;
            }
            OutputWriterEnum::Json(json_writer) => {
                writeln!(json_writer, "{}", serde_json::to_string(record).unwrap())?;
            }
        }
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        match &mut self.writer {
            OutputWriterEnum::Csv(csv_writer) => csv_writer.flush()?,
            OutputWriterEnum::Json(json_writer) => json_writer.flush()?,
        }
        Ok(())
    }
}
