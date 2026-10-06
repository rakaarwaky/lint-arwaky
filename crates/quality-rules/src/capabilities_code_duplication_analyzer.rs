use shared_quality_rules::contract_quality_protocol::ICodeMetricAnalyzerProtocol;
use shared_quality_rules::taxonomy_quality_rules_vo::AesCodeAnalysisViolation;

use shared_common::LintMessage;
use shared_config_system::ArchitectureConfig;
use std::collections::hash_map::DefaultHasher;
use std::sync::Arc;

// PURPOSE: CodeDuplicationAnalyzer — AES305: detect files with excessive duplication across the codebase
// ALGORITHM (file-level similarity, not per-block):
//   1. Accept pre-fetched (path, content) entries from caller
//   2. For each file, tokenize content into lines
//   3. Slide a window of `min_lines` over lines; normalize each window (trim, alphanumeric-only)
//   4. Use normalized window as hash key in global map; store file indices
//   5. Identify which normalized keys appear in 2+ files (shared keys)
//   6. For each file, calculate what % of its windows are shared
//   7. If a file's shared % exceeds `threshold_pct`, emit a single violation per file

use std::collections::{HashMap, HashSet};

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CodeDuplicationAnalyzer {
    /// P1.6 fix: carry injected config instead of calling default_aes_config()
    config: Arc<ArchitectureConfig>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates_entries(
        &self,
        entries: &[(std::path::PathBuf, String)],
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        let config = self.config.as_ref();
        let min_lines = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES305")
            .map(|r| r.code_analysis.min_lines.value as usize)
            .filter(|&v| v > 0)
            .unwrap_or(10);
        let threshold_pct = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES305")
            .and_then(|r| r.code_analysis.duplication_threshold)
            .unwrap_or(50.0);

        // Borrow path/content as &str instead of cloning into (String, String)
        let borrowed: Vec<(&str, &str)> = entries
            .iter()
            .map(|(p, c)| (p.to_str().unwrap_or_default(), c.as_str()))
            .collect();
        self.check_file_similarity_entries(&borrowed, min_lines, threshold_pct)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CodeDuplicationAnalyzer {
    pub fn from_config(config: Arc<ArchitectureConfig>) -> Self {
        Self { config }
    }
}

impl CodeDuplicationAnalyzer {
    /// File-level similarity analysis using pre-read entries.
    /// Instead of one violation per sliding-window match, calculates what % of a file's
    /// normalized windows also appear in other files. Only files exceeding `threshold_pct`
    /// are flagged — one violation per file.
    /// Returns (file_path, violation) tuples so the caller can attach the file path.
    pub fn check_file_similarity_entries(
        &self,
        entries: &[(&str, &str)],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        if entries.is_empty() {
            return Vec::new();
        }

        fn hash_key(key: &str) -> u64 {
            let mut hasher = DefaultHasher::new();
            std::hash::Hash::hash(key, &mut hasher);
            std::hash::Hasher::finish(&hasher)
        }

        // Build global map: normalized window hash → file indices that contain it.
        // Windows that consist entirely of import/`use`/`from` lines are
        // scaffolding, not logic, so they are excluded from both the global
        // map and the per-file shared count (AES305 counts logic duplication).
        let mut global: HashMap<u64, HashSet<usize>> = HashMap::with_capacity(entries.len());
        for (fi, (_, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            for w in lines.windows(min_dup_lines) {
                if w.iter().all(|l| {
                    let t = l.trim();
                    !t.is_empty()
                        && shared_quality_rules::utility_code_duplication_detector::is_import_line(t)
                }) {
                    continue;
                }
                let key =
                    shared_quality_rules::utility_code_duplication_detector::normalize_window(w);
                global.entry(hash_key(&key)).or_default().insert(fi);
            }
        }

        // Identify keys that appear in 2+ different files
        let shared_ids: HashSet<u64> = global
            .iter()
            .filter(|(_, file_indices)| file_indices.len() > 1)
            .map(|(id, _)| *id)
            .collect();

        // Count shared window OCCURRENCES per file: the numerator must reflect how
        // many of a file's windows are shared, not just how many distinct shared
        // hashes it has — otherwise repeated blocks deflate the similarity %.
        // Import-only windows are excluded from both numerator and denominator
        // so that mechanical scaffolding never counts toward duplication.
        let mut shared_counts: Vec<usize> = vec![0; entries.len()];
        let mut total_counts: Vec<usize> = vec![0; entries.len()];
        for (fi, (_, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            for w in lines.windows(min_dup_lines) {
                if w.iter().all(|l| {
                    let t = l.trim();
                    !t.is_empty()
                        && shared_quality_rules::utility_code_duplication_detector::is_import_line(t)
                }) {
                    continue;
                }
                total_counts[fi] += 1;
                let key =
                    shared_quality_rules::utility_code_duplication_detector::normalize_window(w);
                if shared_ids.contains(&hash_key(&key)) {
                    shared_counts[fi] += 1;
                }
            }
        }

        // Build O(1) file_to_others map
        let mut file_to_others: Vec<HashSet<usize>> = Vec::with_capacity(entries.len());
        for _ in 0..entries.len() {
            file_to_others.push(HashSet::new());
        }
        for file_indices in global.values() {
            if file_indices.len() > 1 {
                let unique: Vec<usize> = file_indices.iter().copied().collect();
                for fi in &unique {
                    for other in &unique {
                        if other != fi {
                            file_to_others[*fi].insert(*other);
                        }
                    }
                }
            }
        }

        // Generate violations (pre-allocate with capacity hint)
        let mut violations = Vec::with_capacity(entries.len());
        for (fi, (file_path, _)) in entries.iter().enumerate() {
            let lines: Vec<&str> = entries[fi].1.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            let total_non_import = total_counts[fi];
            let shared_count = shared_counts[fi];

            if total_non_import == 0 {
                continue;
            }
            let pct = shared_count as f64 / total_non_import as f64 * 100.0;
            if pct > threshold_pct {
                let other_indices = &file_to_others[fi];
                let mut other_files: Vec<String> = other_indices
                    .iter()
                    .map(|&ofi| entries[ofi].0.to_string())
                    .collect();
                other_files.sort();

                let mut msg = format!(
                    "AES305: {:.0}% of this file's non-import content appears in other files (threshold: {:.0}%). {} of {} windows are non-unique.",
                    pct, threshold_pct, shared_count, total_non_import,
                );
                if !other_files.is_empty() {
                    msg.push_str(&format!(
                        " Similar files ({}): {}",
                        other_files.len(),
                        other_files
                            .iter()
                            .take(5)
                            .map(|s| s.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }

                violations.push((
                    file_path.to_string(),
                    AesCodeAnalysisViolation::CodeDuplication {
                        reason: Some(LintMessage::new(msg)),
                    },
                ));
            }
        }

        violations
    }
}
