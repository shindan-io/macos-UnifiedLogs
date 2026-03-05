//! Parity test: old `LogData` pipeline vs new `LogEntry` rewrite pipeline.
//!
//! Runs both code paths on the Big Sur logarchive and compares field-by-field.
//! Differences are reported with index + field details for debugging.

use std::collections::HashMap;
use std::path::PathBuf;

use macos_unifiedlogs::filesystem::LogarchiveProvider;
use macos_unifiedlogs::log_data_iterator::iterate_all_logs_callback;
use macos_unifiedlogs::parser::collect_timesync;
use macos_unifiedlogs::rewrite::logarchive::visit_logarchive;
use macos_unifiedlogs::unified_log::LogData;
use uuid::Uuid;

/// Owned record extracted from either pipeline for comparison.
#[derive(Debug, PartialEq)]
struct CompareRecord {
    subsystem: String,
    category: String,
    thread_id: u64,
    pid: u64,
    euid: u32,
    library: String,
    library_uuid: Uuid,
    activity_id: u64,
    time: f64,
    event_type: String,
    log_type: String,
    process: String,
    process_uuid: Uuid,
    message: String,
    boot_uuid: Uuid,
    timezone_name: String,
}

impl CompareRecord {
    fn from_old(entry: &LogData) -> Self {
        Self {
            subsystem: entry.subsystem.as_str().to_owned(),
            category: entry.category.as_str().to_owned(),
            thread_id: entry.thread_id,
            pid: entry.pid,
            euid: entry.euid,
            library: entry.library.as_str().to_owned(),
            library_uuid: entry.library_uuid,
            activity_id: entry.activity_id,
            time: entry.time,
            event_type: format!("{:?}", entry.event_type),
            log_type: format!("{:?}", entry.log_type),
            process: entry.process.as_str().to_owned(),
            process_uuid: entry.process_uuid,
            message: entry.message.as_str().to_owned(),
            boot_uuid: entry.boot_uuid,
            timezone_name: entry.timezone_name.as_str().to_owned(),
        }
    }
}

/// Single field mismatch between old and new records.
struct FieldDiff {
    index: usize,
    field: &'static str,
    old: String,
    new: String,
}

/// Compare two records, returning all differing fields.
fn diff_records(index: usize, old: &CompareRecord, new: &CompareRecord) -> Vec<FieldDiff> {
    let mut diffs = Vec::new();
    macro_rules! cmp {
        ($field:ident) => {
            if old.$field != new.$field {
                diffs.push(FieldDiff {
                    index,
                    field: stringify!($field),
                    old: format!("{:?}", old.$field),
                    new: format!("{:?}", new.$field),
                });
            }
        };
    }
    cmp!(subsystem);
    cmp!(category);
    cmp!(thread_id);
    cmp!(pid);
    cmp!(euid);
    cmp!(library);
    cmp!(library_uuid);
    cmp!(activity_id);
    cmp!(time);
    cmp!(event_type);
    cmp!(log_type);
    cmp!(process);
    cmp!(process_uuid);
    cmp!(message);
    cmp!(boot_uuid);
    cmp!(timezone_name);
    diffs
}

fn test_data_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/test_data")
}

#[test]
fn parity_big_sur() {
    let archive = test_data_path().join("system_logs_big_sur.logarchive");

    // --- Old pipeline ---
    let mut provider = LogarchiveProvider::new(archive.as_path());
    let timesync_data = collect_timesync(&provider).unwrap();

    let mut old_records: Vec<CompareRecord> = Vec::new();
    iterate_all_logs_callback(&mut provider, &timesync_data, false, &mut |entry| {
        old_records.push(CompareRecord::from_old(&entry));
    });

    // --- New pipeline ---
    let mut new_records: Vec<CompareRecord> = Vec::new();
    visit_logarchive(&archive, |entry| {
        new_records.push(CompareRecord {
            subsystem: entry.effective_subsystem().unwrap_or("").to_owned(),
            category: entry.category.unwrap_or("").to_owned(),
            thread_id: entry.thread_id,
            pid: entry.pid,
            euid: entry.euid,
            library: entry.library.unwrap_or("").to_owned(),
            library_uuid: entry.library_uuid,
            activity_id: entry.activity_id,
            time: entry.time,
            event_type: format!("{:?}", entry.event_type),
            log_type: format!("{:?}", entry.log_type),
            process: entry.process.unwrap_or("").to_owned(),
            process_uuid: entry.process_uuid,
            message: entry.message(),
            boot_uuid: entry.boot_uuid,
            timezone_name: entry.timezone_name.to_owned(),
        });
    })
    .unwrap();

    // --- Sort both by stable identity fields ---
    let sort_key = |r: &CompareRecord| {
        (
            r.time.to_bits(),
            r.thread_id,
            r.pid,
            r.event_type.clone(),
            r.log_type.clone(),
        )
    };
    old_records.sort_by(|a, b| sort_key(a).cmp(&sort_key(b)));
    new_records.sort_by(|a, b| sort_key(a).cmp(&sort_key(b)));

    // --- Compare ---
    eprintln!("Old pipeline: {} entries", old_records.len());
    eprintln!("New pipeline: {} entries", new_records.len());

    let compare_len = old_records.len().min(new_records.len());

    // Collect stats
    let max_stored = 100;
    let mut stored_diffs: Vec<FieldDiff> = Vec::new();
    let mut total_mismatched_entries: usize = 0;
    let mut activity_id_only: usize = 0;
    let mut field_totals: HashMap<&'static str, usize> = HashMap::new();
    // Track unique message diff patterns (truncated for grouping)
    let mut message_diff_examples: Vec<(usize, String, String)> = Vec::new();

    for i in 0..compare_len {
        let entry_diffs = diff_records(i, &old_records[i], &new_records[i]);
        if entry_diffs.is_empty() {
            continue;
        }
        total_mismatched_entries += 1;

        let only_activity =
            entry_diffs.len() == 1 && entry_diffs[0].field == "activity_id";
        if only_activity {
            activity_id_only += 1;
        }

        for d in entry_diffs {
            *field_totals.entry(d.field).or_insert(0) += 1;
            if d.field == "message" && message_diff_examples.len() < 30 {
                message_diff_examples.push((d.index, d.old.clone(), d.new.clone()));
            }
            if stored_diffs.len() < max_stored {
                stored_diffs.push(d);
            }
        }
    }

    // --- Report ---
    let has_problems = old_records.len() != new_records.len() || total_mismatched_entries > 0;

    eprintln!("\n=== PARITY REPORT ===");
    if old_records.len() != new_records.len() {
        eprintln!(
            "COUNT MISMATCH: old={} new={} (delta={})",
            old_records.len(),
            new_records.len(),
            new_records.len() as i64 - old_records.len() as i64,
        );
    } else {
        eprintln!("COUNT OK: {}", old_records.len());
    }

    eprintln!(
        "\n{total_mismatched_entries} / {compare_len} entries have at least one field mismatch"
    );
    eprintln!("  of which {activity_id_only} are activity_id-only (high bit masking)");
    let other = total_mismatched_entries - activity_id_only;
    eprintln!("  remaining with content diffs: {other}");

    // Field totals
    if !field_totals.is_empty() {
        eprintln!("\nMismatches by field:");
        let mut sorted: Vec<_> = field_totals.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        for (field, count) in &sorted {
            let pct = *count as f64 / compare_len as f64 * 100.0;
            eprintln!("  {field:>15}: {count:>7} ({pct:.2}%)");
        }
    }

    // Message diff examples (unique patterns)
    if !message_diff_examples.is_empty() {
        eprintln!("\nMessage diff examples:");
        for (idx, old, new) in &message_diff_examples {
            // Truncate long messages for readability
            let old_trunc = if old.len() > 120 {
                format!("{}...", &old[..120])
            } else {
                old.clone()
            };
            let new_trunc = if new.len() > 120 {
                format!("{}...", &new[..120])
            } else {
                new.clone()
            };
            eprintln!("  [{idx}] old: {old_trunc}");
            eprintln!("       new: {new_trunc}");
        }
    }

    // First non-activity_id diffs
    let non_activity: Vec<_> = stored_diffs
        .iter()
        .filter(|d| d.field != "activity_id")
        .take(20)
        .collect();
    if !non_activity.is_empty() {
        eprintln!("\nFirst {} non-activity_id diffs:", non_activity.len());
        for d in &non_activity {
            let old_trunc = if d.old.len() > 100 {
                format!("{}...", &d.old[..100])
            } else {
                d.old.clone()
            };
            let new_trunc = if d.new.len() > 100 {
                format!("{}...", &d.new[..100])
            } else {
                d.new.clone()
            };
            eprintln!(
                "  [entry {}] {}: old={} new={}",
                d.index, d.field, old_trunc, new_trunc
            );
        }
    }

    eprintln!("=== END REPORT ===\n");

    if has_problems {
        panic!(
            "Parity check failed: count_match={}, {} entries with diffs ({} activity_id-only, {} other)",
            old_records.len() == new_records.len(),
            total_mismatched_entries,
            activity_id_only,
            other,
        );
    }

    eprintln!(
        "Parity OK: {} entries match across all fields",
        old_records.len()
    );
}
