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
use shared::common::taxonomy_common_vo::PatternList;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::config_system::taxonomy_config_vo::default_aes_config;
use shared::file_system::contract_system_port::IFileSystemPort;
use shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector;
use shared::source_parsing::taxonomy_path_vo::FilePath;

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
    pub fn check_file_similarity(
        &self,
        files: &[String],
        min_dup_lines: usize,
        threshold_pct: f64,
    ) -> Vec<AesCodeAnalysisViolation> {
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

                violations.push(AesCodeAnalysisViolation::CodeDuplication {
                    reason: Some(LintMessage::new(msg)),
                });
            }
        }

        violations
    }
}

impl ICodeMetricAnalyzerProtocol for CodeDuplicationAnalyzer {
    fn handle_duplicates(
        &self,
        path: Option<String>,
        fs: &dyn IFileSystemPort,
    ) -> Vec<AesCodeAnalysisViolation> {
        let root = crate::agent_code_analysis_orchestrator::resolve_target(path);
        let src = crate::agent_code_analysis_orchestrator::detect_source_dir(Path::new(&root));
        let config = default_aes_config();
        let ignored = PatternList {
            values: config
                .ignored_paths
                .values
                .iter()
                .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
                .collect(),
        };
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
            .first()
            .and_then(|r| r.code_analysis.duplication_threshold)
            .unwrap_or(50.0);

        let src_fp = match FilePath::new(src.to_string_lossy().to_string()) {
            Ok(fp) => fp,
            Err(_) => return Vec::new(),
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        let all_files = rt.block_on(fs.walk(&src_fp, Some(&ignored)));
        let file_strs: Vec<String> = all_files.values.iter().map(|fp| fp.value.clone()).collect();
        self.check_file_similarity(&file_strs, min_lines, threshold_pct)
    }
}

/// File content + path, ready for the sliding-window scan.
type FileEntry = (PathBuf, String);

/// Read each input file via std::fs; skip non-lintable / unreadable files.
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
            blocks.entry(key).or_default().push((path.clone(), idx + 1));
        }
    }
    blocks.into_values().filter(|v| v.len() > 1).collect()
}

/// Convert the grouped duplicate windows into AesCodeAnalysisViolation messages,
/// appending a summary violation when any duplicates were found.
fn build_violations(
    duplicates: &[Vec<(PathBuf, usize)>],
    total_loc: usize,
    min_lines: usize,
) -> Vec<AesCodeAnalysisViolation> {
    let mut violations = Vec::new();
    for locations in duplicates {
        let msg = format!(
            "Duplicate block found across {} files (similarity ~{}%). Locations: {}",
            locations.len(),
            100usize
                .saturating_sub(locations.len().saturating_sub(1) * 5)
                .min(100),
            locations
                .iter()
                .take(3)
                .map(|(p, l)| format!("{}:{}", p.display(), l))
                .collect::<Vec<_>>()
                .join(", ")
        );
        violations.push(AesCodeAnalysisViolation::CodeDuplication {
            reason: Some(LintMessage::new(msg)),
        });
    }

    if !duplicates.is_empty() {
        let total_dup_lines = duplicates.len() * min_lines;
        let pct = if total_loc > 0 {
            total_dup_lines as f64 / total_loc as f64 * 100.0
        } else {
            0.0
        };
        violations.push(AesCodeAnalysisViolation::CodeDuplication {
            reason: Some(LintMessage::new(format!(
                "Summary: {} duplicate blocks ({} lines) out of {} total LOC ({:.1}%)",
                duplicates.len(),
                total_dup_lines,
                total_loc,
                pct,
            ))),
        });
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// A .rs file with 3 identical lines appears in the lintable set; the
    /// duplicate must be reported with the real 1-indexed line number, not a
    /// byte-offset (regression guard for the prior as_ptr() bug).
    /// A single .rs file with the same 3-line block appearing twice must be
    /// reported with the real 1-indexed start line for each occurrence — not
    /// a byte-offset (regression guard for the prior as_ptr() bug).
    #[test]
    fn reported_line_numbers_are_1_indexed_not_byte_offsets() {
        let dir = tempdir();
        let file_path = dir.join("a.rs");
        let mut f = std::fs::File::create(&file_path).unwrap();
        // 5 lines of filler, the duplicate block, then 5 more filler lines,
        // then the SAME block again. With min_lines=3 the first occurrence
        // starts at line 6 and the second at line 14 (1-indexed).
        for i in 0..5 {
            writeln!(f, "filler line {i}").unwrap();
        }
        writeln!(f, "dup alpha").unwrap();
        writeln!(f, "dup beta").unwrap();
        writeln!(f, "dup gamma").unwrap();
        for i in 0..5 {
            writeln!(f, "more filler {i}").unwrap();
        }
        writeln!(f, "dup alpha").unwrap();
        writeln!(f, "dup beta").unwrap();
        writeln!(f, "dup gamma").unwrap();

        let analyzer = CodeDuplicationAnalyzer::new();
        let violations = analyzer.check_duplicates(&[file_path.to_string_lossy().to_string()], 3);

        let dup_msg = violations
            .iter()
            .map(|v| match v {
                AesCodeAnalysisViolation::CodeDuplication { reason } => {
                    reason.as_ref().map(|r| r.value.clone()).unwrap_or_default()
                }
                _ => String::new(),
            })
            .find(|m| m.contains("Duplicate block"))
            .expect("expected a CodeDuplication violation");

        // Reported lines MUST be 6 and 14 (the real 1-indexed starts).
        // The buggy version reported byte-offsets like 12992 here.
        assert!(
            dup_msg.contains(":6"),
            "expected line 6 in violation, got: {dup_msg}"
        );
        assert!(
            dup_msg.contains(":14"),
            "expected line 14 in violation, got: {dup_msg}"
        );
    }

    /// A single file with no duplicates must produce no per-block violations.
    #[test]
    fn no_duplicates_emits_no_block_violation() {
        let dir = tempdir();
        let file_path = dir.join("unique.rs");
        let mut f = std::fs::File::create(&file_path).unwrap();
        for i in 0..30 {
            writeln!(f, "fn unique_{i}() {{ println!({i}); }}").unwrap();
        }
        let violations = CodeDuplicationAnalyzer::new()
            .check_duplicates(&[file_path.to_string_lossy().to_string()], 5);
        assert!(
            violations.is_empty(),
            "expected no violations for unique content, got {}",
            violations.len()
        );
    }

    /// Two files sharing the same 4-line block must surface a single duplicate
    /// violation whose locations point at both files at the correct lines.
    #[test]
    fn two_files_sharing_block_reports_both_locations() {
        let dir = tempdir();
        let a = dir.join("a.rs");
        let b = dir.join("b.rs");
        for (path, prefix) in [(&a, "alpha"), (&b, "beta")] {
            let mut f = std::fs::File::create(path).unwrap();
            for i in 0..20 {
                writeln!(f, "{prefix}_line_{i}").unwrap();
            }
            writeln!(f, "shared one").unwrap();
            writeln!(f, "shared two").unwrap();
            writeln!(f, "shared three").unwrap();
            writeln!(f, "shared four").unwrap();
        }
        let violations = CodeDuplicationAnalyzer::new().check_duplicates(
            &[
                a.to_string_lossy().to_string(),
                b.to_string_lossy().to_string(),
            ],
            4,
        );
        let dup = violations
            .iter()
            .map(|v| match v {
                AesCodeAnalysisViolation::CodeDuplication { reason } => {
                    reason.as_ref().map(|r| r.value.clone()).unwrap_or_default()
                }
                _ => String::new(),
            })
            .find(|m| m.contains("Duplicate block"))
            .unwrap();
        assert!(dup.contains("a.rs:21"), "missing a.rs line in: {dup}");
        assert!(dup.contains("b.rs:21"), "missing b.rs line in: {dup}");
    }

    fn tempdir() -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "code_dup_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }
}
