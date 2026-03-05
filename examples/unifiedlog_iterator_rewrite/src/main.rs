use chrono::SecondsFormat;
use clap::{Parser, ValueEnum, builder};
use csv::Writer;
use log::{LevelFilter, error, info};
use macos_unifiedlogs::rewrite::log_entry::LogEntry;
use macos_unifiedlogs::rewrite::logarchive::visit_logarchive;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version, about = "Parse macOS Unified Logs using the rewrite API")]
struct Args {
  /// Path to logarchive formatted directory
  #[clap(short, long)]
  input: PathBuf,

  /// Filename to save results to (default: stdout)
  #[clap(short, long)]
  output: Option<PathBuf>,

  /// Output format
  #[clap(short, long, default_value = Format::Jsonl)]
  format: Format,

  /// Append to output file instead of overwriting
  #[clap(short, long, default_value = "false")]
  append: bool,
}

#[derive(Parser, Debug, Clone, ValueEnum)]
enum Format {
  Csv,
  Jsonl,
  /// The non type will just print nothing
  None,
}

impl From<Format> for builder::OsStr {
  fn from(value: Format) -> Self {
    match value {
      Format::Csv => "csv".into(),
      Format::Jsonl => "jsonl".into(),
      Format::None => "none".into(),
    }
  }
}

fn main() {
  TermLogger::init(LevelFilter::Warn, Config::default(), TerminalMode::Stderr, ColorChoice::Auto).expect("Failed to initialize logger");

  let args = Args::parse();

  let handle: Box<dyn Write> = if let Some(path) = args.output {
    Box::new(
      fs::OpenOptions::new()
        .append(args.append)
        .write(!args.append)
        .truncate(!args.append)
        .create(true)
        .open(path)
        .unwrap(),
    )
  } else {
    Box::new(std::io::stdout())
  };

  let mut writer = OutputWriter::new(handle, &args.format).unwrap();
  let mut log_count: u64 = 0;

  if let Err(e) = visit_logarchive(&args.input, |entry| {
    log_count += 1;
    if let Err(err) = writer.write_record(&entry) {
      error!("Failed to output log data: {err:?}");
    }
  }) {
    error!("Failed to parse logarchive: {e}");
    std::process::exit(1);
  }

  writer.flush().unwrap();
  info!("Parsed {log_count} log entries");
}

struct OutputWriter {
  writer: OutputWriterEnum,
}

enum OutputWriterEnum {
  Csv(Box<Writer<Box<dyn Write>>>),
  Json(Box<dyn Write>),
  None,
}

impl OutputWriter {
  fn new(writer: Box<dyn Write>, format: &Format) -> Result<Self, Box<dyn Error>> {
    let writer_enum = match format {
      Format::Csv => {
        let mut csv_writer = Writer::from_writer(writer);
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
          "Format String",
          "Boot UUID",
          "System Timezone Name",
        ])?;
        csv_writer.flush()?;
        OutputWriterEnum::Csv(Box::new(csv_writer))
      }
      Format::Jsonl => OutputWriterEnum::Json(writer),
      Format::None => OutputWriterEnum::None,
    };
    Ok(OutputWriter { writer: writer_enum })
  }

  fn write_record(&mut self, record: &LogEntry<'_, '_>) -> Result<(), Box<dyn Error>> {
    match &mut self.writer {
      OutputWriterEnum::Csv(csv_writer) => {
        let timestamp = record.timestamp().to_rfc3339_opts(SecondsFormat::Millis, true);
        let message = record.message();
        csv_writer.write_record(&[
          timestamp,
          format!("{:?}", record.event_type),
          format!("{:?}", record.log_type),
          record.effective_subsystem().unwrap_or("").to_string(),
          record.thread_id.to_string(),
          record.pid.to_string(),
          record.euid.to_string(),
          record.library.unwrap_or("").to_string(),
          record.library_uuid.to_string(),
          record.activity_id.to_string(),
          record.category.unwrap_or("").to_string(),
          record.process.unwrap_or("").to_string(),
          record.process_uuid.to_string(),
          message,
          record.format_string.unwrap_or("").to_string(),
          record.boot_uuid.to_string(),
          record.timezone_name.to_string(),
        ])?;
      }
      OutputWriterEnum::Json(json_writer) => {
        writeln!(json_writer, "{}", serde_json::to_string(record).unwrap())?;
      }
      OutputWriterEnum::None => { /* Do nothing */ }
    }
    Ok(())
  }

  fn flush(&mut self) -> Result<(), Box<dyn Error>> {
    match &mut self.writer {
      OutputWriterEnum::Csv(csv_writer) => csv_writer.flush()?,
      OutputWriterEnum::Json(json_writer) => json_writer.flush()?,
      OutputWriterEnum::None => { /* Do nothing */ }
    }
    Ok(())
  }
}
