// PURPOSE: CodeDuplicationAnalyzer — AES305: detect files with excessive duplication across the codebase
// ALGORITHM (file-level similarity, not per-block):
//   1. Resolve target directory (default: ".")
//   2. Walk all lintable files via IFileSystemPort (handles ignored patterns)
//   3. For each file, read content and tokenize into lines
//   4. Slide a window of `min_lines` over lines; normalize each window (trim, alphanumeric-only)
//   5. Use normalized window as hash key in global map; store (file_idx, line)
//   6. Identify which normalized keys appear in 2+ files (shared keys)
//   7. For each file, calculate what % of its windows are shared
//   8. If a file's shared % exceeds `threshold_pct`, emit a single violation per file
use std::collections::{HashMap, HashSet};

use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::taxonomy_duplication_utility::{
    build_violations, collect_file_entries, normalize_window, scan_duplicate_blocks,
};
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::contract_system_port::IFileSystemPort;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::config_system::taxonomy_config_vo::default_aes_config;

// Block 1: struct Definition
pub struct CodeDuplicationAnalyzer {}

// Block 3: constructors
impl Default for CodeDuplicationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeDuplicationAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    /// Legacy per-block duplication detection.
    pub fn check_duplicates(
        &self,
        files: &[String],
        min_dup_lines: usize,
    ) -> Vec<AesCodeAnalysisViolation> {
        let entries = collect_file_entries(files);
        let total_loc = entries.iter().map(|(_, c)| c.lines().count()).sum();
        let blocks = scan_duplicate_blocks(entries, min_dup_lines);
        build_violations(&blocks, total_loc, min_dup_lines)
    }

    /// File-level similarity analysis using pre-read entries.
    pub fn check_file_similarity_entries(
        &self,
        entries: &[(String, String)],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        if entries.is_empty() {
            return Vec::new();
        }

        let mut interner: HashMap<String, u32> = HashMap::new();
        let mut interned_keys: Vec<String> = Vec::new();
        let mut get_id = |key: String| -> u32 {
            if let Some(&id) = interner.get(&key) {
                return id;
            }
            let id = interner.len() as u32;
            interned_keys.push(key.clone());
            interner.insert(key, id);
            id
        };

        let mut global: HashMap<u32, Vec<(usize, usize)>> = HashMap::new();
        for (fi, (_, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            for (li, w) in lines.windows(min_dup_lines).enumerate() {
                let key = normalize_window(w);
                let id = get_id(key);
                global.entry(id).or_default().push((fi, li + 1));
            }
        }

        let shared_ids: HashSet<u32> = global
            .iter()
            .filter(|(_, locs)| {
                let unique_files: HashSet<usize> = locs.iter().map(|(fi, _)| *fi).collect();
                unique_files.len() > 1
            })
            .map(|(id, _)| *id)
            .collect();

        let mut file_to_others: Vec<HashSet<usize>> = vec![HashSet::new(); entries.len()];
        for locs in global.values() {
            let unique: HashSet<usize> = locs.iter().map(|(fi, _)| *fi).collect();
            if unique.len() > 1 {
                for &fi in &unique {
                    for &other in &unique {
                        if other != fi {
                            file_to_others[fi].insert(other);
                        }
                    }
                }
            }
        }

        let mut violations = Vec::new();
        for (fi, (file_path, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            let total_win = lines.len() - min_dup_lines + 1;
            let shared_count = lines
                .windows(min_dup_lines)
                .enumerate()
                .filter(|(_, w)| {
                    let key = normalize_window(w);
                    let id = get_id(key);
                    shared_ids.contains(&id)
                })
                .count();

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
    pub fn check_file_similarity(
        &self,
        files: &[String],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        let entries = collect_file_entries(files);
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

// Block 2: impl Trait for Struct (Public Contract)
impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(
        &self,
        path: Option<String>,
        _fs: &dyn IFileSystemPort,
    ) -> Vec<AesCodeAnalysisViolation> {
        let root = crate::agent_code_analysis_orchestrator::resolve_target(path);
        let src =
            crate::agent_code_analysis_orchestrator::detect_source_dir(std::path::Path::new(&root));
        let config = default_aes_config();
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
        let source_files = crate::agent_code_analysis_orchestrator::collect_source_files(
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
