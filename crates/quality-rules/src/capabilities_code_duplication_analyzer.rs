use shared::quality_rules::{AesCodeAnalysisViolation, ICodeMetricAnalyzerProtocol};

use shared::common::LintMessage;
use shared::config_system::ArchitectureConfig;
use std::collections::hash_map::DefaultHasher;

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
    fn handle_duplicates(
        &self,
        _path: Option<shared::common::taxonomy_path_vo::DirectoryPath>,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        // Legacy path: caller must pre-fetch entries and pass them via handle_duplicates_entries.
        Vec::new()
    }

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

        let str_entries: Vec<(String, String)> = entries
            .iter()
            .map(|(p, c)| (p.display().to_string(), c.clone()))
            .collect();
        self.check_file_similarity_entries(&str_entries, min_lines, threshold_pct)
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
        entries: &[(String, String)],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        if entries.is_empty() {
            return Vec::new();
        }

        // P2.1/P2.2/P2.3 fix: Hash-based dedup with single-pass normalization.
        // - Store normalized window hash → file indices (P2.3: no line tuples)
        // - Normalize each window only once (P2.1: cache per-file hashes)
        // - Remove unused interned_keys storage (P2.2)

        fn hash_key(key: &str) -> u64 {
            let mut hasher = DefaultHasher::new();
            std::hash::Hash::hash(key, &mut hasher);
            std::hash::Hasher::finish(&hasher)
        }

        // First pass: build global map + cache per-file unique hashes (P2.1: normalize once)
        // P2.3: HashMap<u64, HashSet<usize>> — hash-based, file-only
        let mut global: HashMap<u64, HashSet<usize>> = HashMap::with_capacity(entries.len());
        let mut file_unique_hashes: Vec<Vec<u64>> = Vec::with_capacity(entries.len());

        for (fi, (_, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                file_unique_hashes.push(Vec::new());
                continue;
            }
            let mut file_hashes: HashSet<u64> = HashSet::new();
            for w in lines.windows(min_dup_lines) {
                // P2.1: normalize once — cache hash for second pass
                let key = crate::utility_code_duplication_detector::normalize_window(w);
                let id = hash_key(&key);
                global.entry(id).or_default().insert(fi);
                file_hashes.insert(id);
            }
            file_unique_hashes.push(file_hashes.into_iter().collect());
        }

        // Identify keys that appear in 2+ different files (P2.3: use u64 hash)
        let shared_ids: HashSet<u64> = global
            .iter()
            .filter(|(_, file_indices)| file_indices.len() > 1)
            .map(|(id, _)| *id)
            .collect();

        // Count shared windows per file using cached hashes (P2.1: no re-normalization)
        let mut shared_counts: Vec<usize> = vec![0; entries.len()];
        for fi in 0..entries.len() {
            if entries[fi].1.len() < min_dup_lines {
                continue;
            }
            for hash in &file_unique_hashes[fi] {
                if shared_ids.contains(hash) {
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
            let total_win = lines.len() - min_dup_lines + 1;
            let shared_count = shared_counts[fi];

            let pct = shared_count as f64 / total_win as f64 * 100.0;
            if pct > threshold_pct {
                let other_indices = &file_to_others[fi];
                let mut other_files: Vec<String> = other_indices
                    .iter()
                    .map(|&ofi| entries[ofi].0.clone())
                    .collect();
                other_files.sort();

                let mut msg = format!(
                    "AES305: {:.0}% of this file's content appears in other files (threshold: {:.0}%). {} of {} windows are non-unique.",
                    pct, threshold_pct, shared_count, total_win,
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
                    file_path.clone(),
                    AesCodeAnalysisViolation::CodeDuplication {
                        reason: Some(LintMessage::new(msg)),
                    },
                ));
            }
        }

        violations
    }
}

use std::sync::Arc;
