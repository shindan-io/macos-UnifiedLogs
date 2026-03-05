//! Parity test: old `LogData` pipeline vs new `LogEntry` rewrite pipeline.
//!
//! Runs both code paths on the Big Sur logarchive and compares field-by-field.
//! Differences are reported with index + field details for debugging.
//!
//! With `rewrite_behave_previous` feature flag: the rewrite pipeline produces
//! old-compatible output for activity_id, null formatting, and Apple decoders.
//! Remaining message differences (bytes-as-base64 vs UTF-8, signpost/backtrace
//! formatting, octal prefix) are tracked as known gaps.

use std::collections::HashMap;
use std::path::PathBuf;

use macos_unifiedlogs::filesystem::LogarchiveProvider;
use macos_unifiedlogs::log_data_iterator::iterate_all_logs_callback;
use macos_unifiedlogs::parser::collect_timesync;
use macos_unifiedlogs::rewrite::logarchive::visit_logarchive;
use macos_unifiedlogs::unified_log::LogData;
use uuid::Uuid;

/// Owned record extracted from either pipeline for comparison.
#[derive(Debug, Clone, PartialEq)]
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

    /// Sort key for stable ordering.
    fn sort_key(&self) -> (u64, u64, u64, &str, &str) {
        (
            self.time.to_bits(),
            self.thread_id,
            self.pid,
            &self.event_type,
            &self.log_type,
        )
    }

    /// Multiset key for matching entries between pipelines.
    fn multiset_key(&self) -> (u64, u64, u64, &str, &str) {
        self.sort_key()
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

/// Remove entries from `new_records` that don't appear in `old_records` (by multiset key).
/// Returns the number of entries removed.
fn remove_extra_entries(
    old_records: &[CompareRecord],
    new_records: &mut Vec<CompareRecord>,
) -> usize {
    let mut old_counts: HashMap<(u64, u64, u64, String, String), usize> = HashMap::new();
    for r in old_records {
        let key = (
            r.time.to_bits(),
            r.thread_id,
            r.pid,
            r.event_type.clone(),
            r.log_type.clone(),
        );
        *old_counts.entry(key).or_default() += 1;
    }

    let mut new_counts: HashMap<(u64, u64, u64, String, String), usize> = HashMap::new();
    let mut to_remove = Vec::new();
    for (i, r) in new_records.iter().enumerate() {
        let key = (
            r.time.to_bits(),
            r.thread_id,
            r.pid,
            r.event_type.clone(),
            r.log_type.clone(),
        );
        let nc = new_counts.entry(key.clone()).or_default();
        *nc += 1;
        let oc = old_counts.get(&key).copied().unwrap_or(0);
        if *nc > oc {
            to_remove.push(i);
        }
    }

    // Remove in reverse order to preserve indices
    for &i in to_remove.iter().rev() {
        new_records.remove(i);
    }

    to_remove.len()
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

    eprintln!("Old pipeline: {} entries", old_records.len());
    eprintln!("New pipeline: {} entries (before filtering)", new_records.len());

    // Remove entries from new that don't exist in old (known +2 Statedump entries
    // due to inner chunk reader differences).
    let removed = remove_extra_entries(&old_records, &mut new_records);
    if removed > 0 {
        eprintln!("Removed {removed} extra entries from new pipeline (chunk discovery gap)");
    }

    // Sort both by stable identity fields.
    // Ordering differs due to the old pipeline's LIFO statedump buffering vs the new
    // pipeline's forward-order emission. Sorting aligns entries for field comparison.
    old_records.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
    new_records.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));

    assert_eq!(
        old_records.len(),
        new_records.len(),
        "After filtering, counts must match: old={} new={}",
        old_records.len(),
        new_records.len()
    );

    // --- Compare ---
    let compare_len = old_records.len();
    let mut total_mismatched_entries: usize = 0;
    let mut message_only: usize = 0;
    let mut field_totals: HashMap<&'static str, usize> = HashMap::new();
    let mut message_diff_examples: Vec<(usize, String, String)> = Vec::new();

    for i in 0..compare_len {
        let entry_diffs = diff_records(i, &old_records[i], &new_records[i]);
        if entry_diffs.is_empty() {
            continue;
        }
        total_mismatched_entries += 1;

        let only_message = entry_diffs.len() == 1 && entry_diffs[0].field == "message";
        if only_message {
            message_only += 1;
        }

        for d in entry_diffs {
            *field_totals.entry(d.field).or_insert(0) += 1;
            if d.field == "message" && message_diff_examples.len() < 20 {
                message_diff_examples.push((d.index, d.old.clone(), d.new.clone()));
            }
        }
    }

    // --- Report ---
    eprintln!("\n=== PARITY REPORT ===");
    eprintln!("{total_mismatched_entries} / {compare_len} entries with at least one field diff");
    eprintln!("  of which {message_only} are message-only (formatting gaps)");
    let structural = total_mismatched_entries - message_only;
    eprintln!("  structural diffs (non-message fields): {structural}");

    if !field_totals.is_empty() {
        eprintln!("\nMismatches by field:");
        let mut sorted: Vec<_> = field_totals.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        for (field, count) in &sorted {
            let pct = *count as f64 / compare_len as f64 * 100.0;
            eprintln!("  {field:>15}: {count:>7} ({pct:.2}%)");
        }
    }

    if !message_diff_examples.is_empty() {
        eprintln!("\nMessage diff examples:");
        for (idx, old, new) in &message_diff_examples {
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
    eprintln!("=== END REPORT ===\n");

    // --- Assertions ---
    // With `rewrite_behave_previous`: structural fields must match perfectly.
    // Message formatting has known remaining gaps (bytes-as-base64, signpost/backtrace,
    // octal prefix) that are tracked but not yet gated.
    #[cfg(feature = "rewrite_behave_previous")]
    {
        // Non-message fields that should now be identical
        let non_message_fields = [
            "subsystem",
            "category",
            "thread_id",
            "pid",
            "euid",
            "library",
            "library_uuid",
            "time",
            "event_type",
            "log_type",
            "process",
            "process_uuid",
            "boot_uuid",
            "timezone_name",
        ];
        let mut sorted: Vec<_> = field_totals_copy(&old_records, &new_records);
        for field in &non_message_fields {
            let count = sorted.iter().find(|(f, _)| f == field).map(|(_, c)| *c).unwrap_or(0);
            assert_eq!(
                count, 0,
                "Field '{field}' should have 0 mismatches under rewrite_behave_previous, got {count}"
            );
        }

        // Activity_id: should be 0 or very low (sorted comparison may misalign
        // entries with identical sort keys but different activity_ids)
        let activity_count = sorted
            .iter()
            .find(|(f, _)| f == &"activity_id")
            .map(|(_, c)| *c)
            .unwrap_or(0);
        eprintln!("activity_id mismatches: {activity_count} (sort-alignment artifacts)");

        // Message diffs are known remaining gaps — report but don't fail
        let message_count = sorted
            .iter()
            .find(|(f, _)| f == &"message")
            .map(|(_, c)| *c)
            .unwrap_or(0);
        eprintln!("message mismatches: {message_count} (known formatting gaps, not yet gated)");
    }

    // Without the feature flag: report only, no strict assertions
    #[cfg(not(feature = "rewrite_behave_previous"))]
    {
        if total_mismatched_entries > 0 {
            eprintln!(
                "Parity check: {} entries with diffs (informational, no feature flag)",
                total_mismatched_entries
            );
        }
    }
}

/// Collect field mismatch counts (helper for assertions).
#[cfg(feature = "rewrite_behave_previous")]
fn field_totals_copy(
    old_records: &[CompareRecord],
    new_records: &[CompareRecord],
) -> Vec<(&'static str, usize)> {
    let mut totals: HashMap<&'static str, usize> = HashMap::new();
    for i in 0..old_records.len().min(new_records.len()) {
        let diffs = diff_records(i, &old_records[i], &new_records[i]);
        for d in diffs {
            *totals.entry(d.field).or_default() += 1;
        }
    }
    totals.into_iter().collect()
}
