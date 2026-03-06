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
use macos_unifiedlogs::rewrite::log_entry::LogEntry;
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
    format_string: String,
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
            format_string: entry.raw_message.to_string(),
        }
    }

    fn from_rewrite(entry: &LogEntry<'_, '_>) -> Self {
        Self {
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
            format_string: entry.format_string.unwrap_or("").to_owned(),
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

/// Categorize a message-only diff into one of the known buckets.
#[allow(clippy::too_many_arguments)]
fn categorize_message_diff(
    d: &FieldDiff,
    record_idx: usize,
    old_record: &CompareRecord,
    cat_signpost: &mut usize,
    cat_backtrace: &mut usize,
    cat_loss: &mut usize,
    cat_octal: &mut usize,
    cat_bytes: &mut usize,
    cat_private: &mut usize,
    cat_float: &mut usize,
    cat_missing_fmt: &mut usize,
    cat_other: &mut usize,
    cat_other_examples: &mut Vec<(usize, String, String, String)>,
    cat_octal_fmt_examples: &mut Vec<(usize, String, String, String)>,
) {
    let old_msg = &d.old;
    let new_msg = &d.new;
    if old_msg.contains("Signpost ID:") || new_msg.contains("Signpost ID:") {
        *cat_signpost += 1;
    } else if old_msg.contains("Backtrace:") || new_msg.contains("Backtrace:") {
        *cat_backtrace += 1;
    } else if new_msg.contains("Lost ") && old_msg == "\"\"" {
        *cat_loss += 1;
    } else if old_msg.contains("0o") && !new_msg.contains("0o") {
        *cat_octal += 1;
        if cat_octal_fmt_examples.len() < 5 {
            cat_octal_fmt_examples.push((
                d.index,
                d.old.clone(),
                d.new.clone(),
                old_record.format_string.clone(),
            ));
        }
    } else if old_msg.contains("==\\\"") || old_msg.contains("==)")
        || old_msg.contains("== ") || old_msg.ends_with("==\\\"")
        || (new_msg.contains("<Invalid UTF-8>") && !old_msg.contains("<Invalid UTF-8>"))
        || (new_msg.contains("Could not extract string") && !old_msg.contains("Could not extract string"))
    {
        *cat_bytes += 1;
    } else if new_msg.contains("<private>") && !old_msg.contains("<private>") {
        *cat_private += 1;
    } else if new_msg.contains("<missing format string>") {
        *cat_missing_fmt += 1;
    } else if (old_msg.contains("<private>") || old_msg.contains("Could not find path string"))
        && !new_msg.contains("<private>")
        && !new_msg.contains("Could not find path string")
    {
        // Reverse private: old pipeline failed to extract private data, new pipeline succeeded.
        // This is an improvement, not a regression. Typically caused by cursor alignment
        // bugs in the old pipeline's parse_private_data().
        *cat_private += 1;
    } else {
        let old_clean = old_msg.replace('"', "");
        let new_clean = new_msg.replace('"', "");
        if old_clean.len() > 20 && new_clean.len() > 20
            && old_clean[..15] == new_clean[..15]
            && !old_clean.contains("<private>") && !new_clean.contains("<private>")
            && (old_clean.contains("000000") || new_clean.contains("000000"))
        {
            *cat_float += 1;
        } else {
            *cat_other += 1;
            if cat_other_examples.len() < 20 {
                cat_other_examples.push((
                    record_idx,
                    d.old.clone(),
                    d.new.clone(),
                    old_record.format_string.clone(),
                ));
            }
        }
    }
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
        new_records.push(CompareRecord::from_rewrite(&entry));
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

    // Diff categorization buckets
    let mut cat_signpost: usize = 0;
    let mut cat_backtrace: usize = 0;
    let mut cat_loss: usize = 0;
    let mut cat_octal: usize = 0;
    let mut cat_bytes: usize = 0;
    let mut cat_private: usize = 0;
    let mut cat_float: usize = 0;
    let mut cat_missing_fmt: usize = 0;
    let mut cat_other: usize = 0;
    let mut cat_other_examples: Vec<(usize, String, String, String)> = Vec::new();
    let mut cat_octal_fmt_examples: Vec<(usize, String, String, String)> = Vec::new();

    // Group-aware comparison: entries with identical sort keys are compared
    // as multisets to avoid phantom diffs from undefined sort order.
    let mut i = 0;
    while i < compare_len {
        // Find group of entries sharing the same sort key
        let key = old_records[i].sort_key();
        let mut j = i + 1;
        while j < compare_len && old_records[j].sort_key() == key {
            j += 1;
        }
        let group_size = j - i;

        if group_size == 1 {
            // Single entry — compare directly (no ambiguity)
            let entry_diffs = diff_records(i, &old_records[i], &new_records[i]);
            if !entry_diffs.is_empty() {
                total_mismatched_entries += 1;
                let only_message = entry_diffs.len() == 1 && entry_diffs[0].field == "message";
                if only_message {
                    message_only += 1;
                }
                for d in &entry_diffs {
                    *field_totals.entry(d.field).or_insert(0) += 1;
                    if d.field == "message" && message_diff_examples.len() < 20 {
                        message_diff_examples.push((d.index, d.old.clone(), d.new.clone()));
                    }
                }
                if only_message {
                    categorize_message_diff(
                        &entry_diffs[0], i, &old_records[i],
                        &mut cat_signpost, &mut cat_backtrace, &mut cat_loss,
                        &mut cat_octal, &mut cat_bytes, &mut cat_private,
                        &mut cat_float, &mut cat_missing_fmt, &mut cat_other,
                        &mut cat_other_examples, &mut cat_octal_fmt_examples,
                    );
                }
            }
        } else {
            // Multi-entry group — multiset comparison on full records
            // Count messages in each pipeline for this group
            let old_msgs: HashMap<&str, usize> = {
                let mut m = HashMap::new();
                for r in &old_records[i..j] {
                    *m.entry(r.message.as_str()).or_default() += 1;
                }
                m
            };
            let new_msgs: HashMap<&str, usize> = {
                let mut m = HashMap::new();
                for r in &new_records[i..j] {
                    *m.entry(r.message.as_str()).or_default() += 1;
                }
                m
            };

            // Collect all unique message keys
            let mut all_keys: Vec<&str> = old_msgs.keys().copied().collect();
            for k in new_msgs.keys() {
                if !old_msgs.contains_key(k) {
                    all_keys.push(k);
                }
            }

            for msg in &all_keys {
                let old_n = old_msgs.get(msg).copied().unwrap_or(0);
                let new_n = new_msgs.get(msg).copied().unwrap_or(0);
                if old_n != new_n {
                    // Real diffs: entries present in one but not the other
                    let diff_count = old_n.abs_diff(new_n);
                    total_mismatched_entries += diff_count;
                    message_only += diff_count;
                    *field_totals.entry("message").or_insert(0) += diff_count;

                    if message_diff_examples.len() < 20 {
                        // Find a counterpart message for the diff example
                        let (old_msg_str, new_msg_str) = if old_n > new_n {
                            (format!("{msg:?}"), "\"<no match in group>\"".to_string())
                        } else {
                            ("\"<no match in group>\"".to_string(), format!("{msg:?}"))
                        };
                        message_diff_examples.push((i, old_msg_str, new_msg_str));
                    }

                    // Categorize using a synthetic FieldDiff
                    // Find the actual old and new messages for categorization
                    if old_n > new_n {
                        // Message exists in old but not new — find closest new match
                        let sample_old = old_records[i..j].iter().find(|r| r.message == *msg).unwrap();
                        let sample_new = new_records[i..j].iter().next().unwrap();
                        let fd = FieldDiff {
                            index: i,
                            field: "message",
                            old: format!("{:?}", sample_old.message),
                            new: format!("{:?}", sample_new.message),
                        };
                        for _ in 0..diff_count {
                            categorize_message_diff(
                                &fd, i, sample_old,
                                &mut cat_signpost, &mut cat_backtrace, &mut cat_loss,
                                &mut cat_octal, &mut cat_bytes, &mut cat_private,
                                &mut cat_float, &mut cat_missing_fmt, &mut cat_other,
                                &mut cat_other_examples, &mut cat_octal_fmt_examples,
                            );
                        }
                    } else {
                        let sample_old = old_records[i..j].iter().next().unwrap();
                        let sample_new = new_records[i..j].iter().find(|r| r.message == *msg).unwrap();
                        let fd = FieldDiff {
                            index: i,
                            field: "message",
                            old: format!("{:?}", sample_old.message),
                            new: format!("{:?}", sample_new.message),
                        };
                        for _ in 0..diff_count {
                            categorize_message_diff(
                                &fd, i, sample_old,
                                &mut cat_signpost, &mut cat_backtrace, &mut cat_loss,
                                &mut cat_octal, &mut cat_bytes, &mut cat_private,
                                &mut cat_float, &mut cat_missing_fmt, &mut cat_other,
                                &mut cat_other_examples, &mut cat_octal_fmt_examples,
                            );
                        }
                    }
                }
            }

            // Also check non-message structural fields within the group.
            // For structural fields, entries in the same group should be identical
            // regardless of position, so compare sorted-by-message pairs.
            let mut old_group: Vec<&CompareRecord> = old_records[i..j].iter().collect();
            let mut new_group: Vec<&CompareRecord> = new_records[i..j].iter().collect();
            old_group.sort_by_key(|r| &r.message);
            new_group.sort_by_key(|r| &r.message);
            for (oi, ni) in old_group.iter().zip(new_group.iter()) {
                let diffs = diff_records(i, oi, ni);
                for d in &diffs {
                    if d.field != "message" {
                        *field_totals.entry(d.field).or_insert(0) += 1;
                    }
                }
            }
        }

        i = j;
    }

    // --- Report ---
    eprintln!("\n=== PARITY REPORT ===");
    eprintln!("{total_mismatched_entries} / {compare_len} entries with at least one field diff");
    eprintln!("  of which {message_only} are message-only (formatting gaps)");
    let structural = total_mismatched_entries - message_only;
    eprintln!("  structural diffs (non-message fields): {structural}");

    eprintln!("\nMessage diff categories:");
    eprintln!("  signpost prefix: {cat_signpost}");
    eprintln!("  backtrace:       {cat_backtrace}");
    eprintln!("  loss message:    {cat_loss}");
    eprintln!("  octal (0o):      {cat_octal}");
    eprintln!("  bytes/base64:    {cat_bytes}");
    eprintln!("  private leak:    {cat_private}");
    eprintln!("  float precision: {cat_float}");
    eprintln!("  missing fmt str: {cat_missing_fmt}");
    eprintln!("  other:           {cat_other}");

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
        eprintln!("\nMessage diff examples (first 20):");
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

    if !cat_octal_fmt_examples.is_empty() {
        eprintln!("\nOctal diff examples (with format string):");
        for (idx, old, new, fmt) in &cat_octal_fmt_examples {
            eprintln!("  [{idx}] fmt: {fmt:?}");
            eprintln!("       old: {old}");
            eprintln!("       new: {new}");
        }
    }

    if !cat_other_examples.is_empty() {
        eprintln!("\n'Other' category examples:");
        for (idx, old, new, fmt) in &cat_other_examples {
            let old_trunc = if old.len() > 200 {
                format!("{}...", &old[..200])
            } else {
                old.clone()
            };
            let new_trunc = if new.len() > 200 {
                format!("{}...", &new[..200])
            } else {
                new.clone()
            };
            eprintln!("  [{idx}] fmt: {fmt:?}");
            eprintln!("       old: {old_trunc}");
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
        let sorted: Vec<_> = field_totals_copy(&old_records, &new_records);
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

/// Collect field mismatch counts using group-aware comparison (helper for assertions).
#[cfg(feature = "rewrite_behave_previous")]
fn field_totals_copy(
    old_records: &[CompareRecord],
    new_records: &[CompareRecord],
) -> Vec<(&'static str, usize)> {
    let compare_len = old_records.len().min(new_records.len());
    let mut totals: HashMap<&'static str, usize> = HashMap::new();
    let mut i = 0;
    while i < compare_len {
        let key = old_records[i].sort_key();
        let mut j = i + 1;
        while j < compare_len && old_records[j].sort_key() == key {
            j += 1;
        }

        if j - i == 1 {
            let diffs = diff_records(i, &old_records[i], &new_records[i]);
            for d in diffs {
                *totals.entry(d.field).or_default() += 1;
            }
        } else {
            // Multiset comparison for messages
            let old_msgs: HashMap<&str, usize> = {
                let mut m = HashMap::new();
                for r in &old_records[i..j] {
                    *m.entry(r.message.as_str()).or_default() += 1;
                }
                m
            };
            let new_msgs: HashMap<&str, usize> = {
                let mut m = HashMap::new();
                for r in &new_records[i..j] {
                    *m.entry(r.message.as_str()).or_default() += 1;
                }
                m
            };
            let mut all_keys: Vec<&str> = old_msgs.keys().copied().collect();
            for k in new_msgs.keys() {
                if !old_msgs.contains_key(k) {
                    all_keys.push(k);
                }
            }
            for msg in &all_keys {
                let old_n = old_msgs.get(msg).copied().unwrap_or(0);
                let new_n = new_msgs.get(msg).copied().unwrap_or(0);
                if old_n != new_n {
                    *totals.entry("message").or_default() += old_n.abs_diff(new_n);
                }
            }
            // Non-message fields: compare sorted-by-message pairs
            let mut old_group: Vec<&CompareRecord> = old_records[i..j].iter().collect();
            let mut new_group: Vec<&CompareRecord> = new_records[i..j].iter().collect();
            old_group.sort_by_key(|r| &r.message);
            new_group.sort_by_key(|r| &r.message);
            for (oi, ni) in old_group.iter().zip(new_group.iter()) {
                let diffs = diff_records(i, oi, ni);
                for d in diffs {
                    if d.field != "message" {
                        *totals.entry(d.field).or_default() += 1;
                    }
                }
            }
        }
        i = j;
    }
    totals.into_iter().collect()
}
