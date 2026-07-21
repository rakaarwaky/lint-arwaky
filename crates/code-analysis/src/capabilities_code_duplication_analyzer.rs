use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use shared::common::taxonomy_message_vo::LintMessage;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;

// PURPOSE: CodeDuplicationAnalyzer — AES305: detect files with excessive duplication across the codebase
// ALGORITHM (file-level similarity, not per-block):
//   1. Resolve target directory (default: ".")
//   2. Walk all lintable files via utility_target::collect_source_files (handles ignored patterns)
//   3. For each file, read content and tokenize into lines
//   4. Slide a window of `min_lines` over lines; normalize each window (trim, alphanumeric-only)
//   5. Use normalized window as hash key in global map; store (file_idx, line)
//   6. Identify which normalized keys appear in 2+ files (shared keys)
//   7. For each file, calculate what % of its windows are shared
//   8. If a file's shared % exceeds `threshold_pct`, emit a single violation per file

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct CodeDuplicationAnalyzer {
    /// P1.6 fix: carry injected config instead of calling default_aes_config()
    config: Arc<ArchitectureConfig>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(&self, path: Option<String>) -> Vec<AesCodeAnalysisViolation> {
        let root = shared::code_analysis::utility_target::resolve_target(path);
        let src =
            shared::code_analysis::utility_target::detect_source_dir(std::path::Path::new(&root));
        // P1.6 fix: use injected config (self.config) instead of default_aes_config()
        let config = self.config.as_ref();
        let ignored_vec: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
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

        let dir_path = match shared::common::taxonomy_path_vo::DirectoryPath::new(
            src.to_string_lossy().to_string(),
        ) {
            Ok(dp) => dp,
            Err(_) => return Vec::new(),
        };
        let source_files = shared::code_analysis::utility_target::collect_source_files(
            &src,
            &dir_path,
            &ignored_vec,
        );
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        self.check_file_similarity(&file_strs, min_lines, threshold_pct)
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl CodeDuplicationAnalyzer {
    /// P1.6 fix: new() uses default config; prefer from_config() for injected config
    pub fn new() -> Self {
        Self {
            config: Arc::new(ArchitectureConfig::default()),
        }
    }

    /// Create with an injected ArchitectureConfig (P1.6 fix).
    pub fn from_config(config: Arc<ArchitectureConfig>) -> Self {
        Self { config }
    }
}

impl Default for CodeDuplicationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeDuplicationAnalyzer {
    /// Legacy per-block duplication detection.
    /// Kept for backward compatibility; prefer `check_file_similarity`.
    pub fn check_duplicates(
        &self,
        files: &[String],
        min_dup_lines: usize,
    ) -> Vec<AesCodeAnalysisViolation> {
        let entries = shared::code_analysis::utility_duplication::collect_file_entries(files);
        let total_loc = entries.iter().map(|(_, c)| c.lines().count()).sum();
        let blocks = shared::code_analysis::utility_duplication::scan_duplicate_blocks(
            entries,
            min_dup_lines,
        );
        shared::code_analysis::utility_duplication::build_violations(
            &blocks,
            total_loc,
            min_dup_lines,
        )
    }

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
            key.hash(&mut hasher);
            hasher.finish()
        }

        // First pass: build global map + cache per-file unique hashes (P2.1: normalize once)
        // P2.3: HashMap<u64, HashSet<usize>> — hash-based, file-only
        let mut global: HashMap<u64, HashSet<usize>> = HashMap::new();
        let mut file_unique_hashes: Vec<Vec<u64>> = vec![Vec::new(); entries.len()];

        for (fi, (_, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            let mut file_hashes: HashSet<u64> = HashSet::new();
            for w in lines.windows(min_dup_lines) {
                // P2.1: normalize once — cache hash for second pass
                let key = shared::code_analysis::utility_duplication::normalize_window(w);
                let id = hash_key(&key);
                global.entry(id).or_default().insert(fi);
                file_hashes.insert(id);
            }
            file_unique_hashes[fi] = file_hashes.into_iter().collect();
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
        let mut file_to_others: Vec<HashSet<usize>> = vec![HashSet::new(); entries.len()];
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

        // Generate violations
        let mut violations = Vec::new();
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

    /// File-level similarity analysis (legacy API — reads files internally).
    /// Prefer `check_file_similarity_entries` to avoid double I/O.
    pub fn check_file_similarity(
        &self,
        files: &[String],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        let entries = shared::code_analysis::utility_duplication::collect_file_entries(files);
        self.check_file_similarity_entries(
            &entries
                .iter()
                .map(|(p, c)| (p.display().to_string(), c.clone()))
                .collect::<Vec<_>>(),
            min_dup_lines,
            threshold_pct,
        )
    }
}
