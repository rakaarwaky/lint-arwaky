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
use std::path::{Path, PathBuf};

use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::contract_system_port::IFileSystemPort;
use shared::common::taxonomy_language_detector_helper::LanguageDetector;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::default_aes_config;

pub struct CodeDuplicationAnalyzer {}

impl CodeDuplicationAnalyzer {
    pub fn new() -> Self {
        Self {}
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
        let detector = LanguageDetector::new();
        let entries = collect_file_entries(files, &detector);
        let total_loc = entries.iter().map(|(_, c)| c.lines().count()).sum();
        let blocks = scan_duplicate_blocks(entries, min_dup_lines);
        build_violations(&blocks, total_loc, min_dup_lines)
    }

    /// File-level similarity analysis.
    /// Instead of one violation per sliding-window match, calculates what % of a file's
    /// normalized windows also appear in other files. Only files exceeding `threshold_pct`
    /// are flagged — one violation per file.
    /// Returns (file_path, violation) tuples so the caller can attach the file path.
    pub fn check_file_similarity(
        &self,
        files: &[String],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<(String, AesCodeAnalysisViolation)> {
        let detector = LanguageDetector::new();
        let entries = collect_file_entries(files, &detector);
        if entries.is_empty() {
            return Vec::new();
        }

        // Build global map: normalized key → Vec<(file_idx, line_number)>
        let mut global: HashMap<String, Vec<(usize, usize)>> = HashMap::new();
        for (fi, (_, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            for (li, w) in lines.windows(min_dup_lines).enumerate() {
                let key = normalize_window(w);
                global.entry(key).or_default().push((fi, li + 1));
            }
        }

        // Identify keys that appear in 2+ different files
        let shared_keys: HashSet<String> = global
            .iter()
            .filter(|(_, locs)| {
                let unique_files: HashSet<usize> = locs.iter().map(|(fi, _)| *fi).collect();
                unique_files.len() > 1
            })
            .map(|(k, _)| k.clone())
            .collect();

        // Per-file similarity calculation
        let mut violations = Vec::new();
        for (fi, (_path, content)) in entries.iter().enumerate() {
            let lines: Vec<&str> = content.lines().collect();
            if lines.len() < min_dup_lines {
                continue;
            }
            let total_win = lines.len() - min_dup_lines + 1;
            let shared_count = lines
                .windows(min_dup_lines)
                .enumerate()
                .filter(|(_, w)| shared_keys.contains(&normalize_window(w)))
                .count();

            let pct = shared_count as f64 / total_win as f64 * 100.0;
            if pct > threshold_pct {
                // Collect which other files share content with this file
                let mut other_files: Vec<String> = Vec::new();
                for (other_fi, (other_path, _)) in entries.iter().enumerate() {
                    if other_fi == fi {
                        continue;
                    }
                    if lines.windows(min_dup_lines).any(|w| {
                        let key = normalize_window(w);
                        global
                            .get(&key)
                            .is_some_and(|locs| locs.iter().any(|(ofi, _)| *ofi == other_fi))
                    }) {
                        other_files.push(other_path.display().to_string());
                    }
                }
                other_files.sort();
                other_files.dedup();

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

                let file_path = _path.display().to_string();
                violations.push((
                    file_path,
                    AesCodeAnalysisViolation::CodeDuplication {
                        reason: Some(LintMessage::new(msg)),
                    },
                ));
            }
        }

        violations
    }
}

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(
        &self,
        path: Option<String>,
        _fs: &dyn IFileSystemPort,
    ) -> Vec<AesCodeAnalysisViolation> {
        let root = crate::agent_code_analysis_orchestrator::resolve_target(path);
        let src = crate::agent_code_analysis_orchestrator::detect_source_dir(Path::new(&root));
        let config = default_aes_config();
        let ignored_vec: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        let min_lines = match config
            .rules
            .first()
            .map(|r| r.code_analysis.min_lines.value)
            .filter(|&v| v > 0)
        {
            Some(v) => v as usize,
            None => 10,
        };
        let threshold_pct = config
            .rules
            .iter()
            .find(|r| r.rule_type.to_string() == "AES305")
            .map(|r| {
                if r.code_analysis.min_lines.value > 0 {
                    r.code_analysis.min_lines.value as f64
                } else {
                    50.0
                }
            })
            .unwrap_or(50.0);

        let dir_path = match shared::common::taxonomy_path_vo::DirectoryPath::new(
            src.to_string_lossy().to_string(),
        ) {
            Ok(dp) => dp,
            Err(_) => return Vec::new(),
        };
        let source_files =
            crate::agent_code_analysis_orchestrator::collect_source_files(&src, &dir_path, &ignored_vec);
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        self.check_file_similarity(&file_strs, min_lines, threshold_pct)
            .into_iter()
            .map(|(_, v)| v)
            .collect()
    }
}

/// Collect file entries: (PathBuf, content_string) for each lintable file.
type FileEntry = (PathBuf, String);

fn collect_file_entries(files: &[String], detector: &LanguageDetector) -> Vec<FileEntry> {
    let mut out = Vec::new();
    for file_str in files {
        let fp = match FilePath::new(file_str.clone()) {
            Ok(f) => f,
            Err(_) => continue,
        };
        if !detector.is_lintable(&fp) {
            continue;
        }
        let content = match std::fs::read_to_string(&fp.value) {
            Ok(c) => c,
            Err(_) => continue,
        };
        out.push((PathBuf::from(&fp.value), content));
    }
    out
}

/// Normalize a single line: trim, keep only alphanumeric and whitespace.
fn normalize_line(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

/// Normalize a window of lines into a single hash key.
fn normalize_window(window: &[&str]) -> String {
    window
        .iter()
        .map(|s| normalize_line(s))
        .collect::<Vec<_>>()
        .join("|")
}

/// Slide a normalized `min_lines` window across each file and group identical windows.
/// Returns one entry per duplicated block, each holding the (path, 1-indexed start_line)
/// of every occurrence.
fn scan_duplicate_blocks(entries: Vec<FileEntry>, min_lines: usize) -> Vec<Vec<(PathBuf, usize)>> {
    let mut blocks: HashMap<String, Vec<(PathBuf, usize)>> = HashMap::new();
    for (path, content) in entries {
        let lines: Vec<&str> = content.lines().collect();
        if lines.len() < min_lines {
            continue;
        }
        for (idx, w) in lines.windows(min_lines).enumerate() {
            let key = normalize_window(w);
            blocks
                .entry(key)
                .or_default()
                .push((path.clone(), idx + 1));
        }
    }
    blocks
        .into_values()
        .filter(|locs| {
            let unique_files: HashSet<_> = locs.iter().map(|(p, _)| p).collect();
            unique_files.len() > 1
        })
        .collect()
}

/// Build violation list from duplicated blocks.
fn build_violations(
    blocks: &[Vec<(PathBuf, usize)>],
    total_loc: usize,
    min_dup_lines: usize,
) -> Vec<AesCodeAnalysisViolation> {
    if blocks.is_empty() || total_loc == 0 {
        return Vec::new();
    }
    let dup_lines: usize = blocks
        .iter()
        .map(|b| b.len() * min_dup_lines)
        .sum();
    let pct = dup_lines as f64 / total_loc as f64 * 100.0;
    if pct < 10.0 {
        return Vec::new();
    }
    vec![AesCodeAnalysisViolation::CodeDuplication {
        reason: Some(LintMessage::new(format!(
            "AES305: {:.1}% of lines appear in duplicate blocks across {} files.",
            pct,
            blocks.len()
        ))),
    }]
}
