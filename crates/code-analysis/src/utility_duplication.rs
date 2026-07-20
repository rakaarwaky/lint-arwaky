// PURPOSE: Stateless utility functions for code duplication analysis (AES305)
// Extracted from capabilities_code_duplication_analyzer.rs — pure functions, no &self, no I/O

use std::collections::HashMap;
use std::path::PathBuf;

use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;

/// Normalize a single line: trim, keep only alphanumeric and whitespace.
pub fn normalize_line(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

/// Normalize a window of lines into a single hash key.
pub fn normalize_window(window: &[&str]) -> String {
    window
        .iter()
        .map(|s| normalize_line(s))
        .collect::<Vec<_>>()
        .join("|")
}

/// Slide a normalized `min_lines` window across each file and group identical windows.
/// Returns one entry per duplicated block, each holding the (path, 1-indexed start_line)
/// of every occurrence.
pub fn scan_duplicate_blocks(
    entries: Vec<(PathBuf, String)>,
    min_lines: usize,
) -> Vec<Vec<(PathBuf, usize)>> {
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
    blocks
        .into_values()
        .filter(|locs| locs.len() >= 2)
        .collect()
}

/// Build violation list from duplicated blocks.
pub fn build_violations(
    blocks: &[Vec<(PathBuf, usize)>],
    total_loc: usize,
    min_dup_lines: usize,
) -> Vec<AesCodeAnalysisViolation> {
    if blocks.is_empty() || total_loc == 0 {
        return Vec::new();
    }
    let dup_lines: usize = blocks.iter().map(|b| b.len() * min_dup_lines).sum();
    let pct = dup_lines as f64 / total_loc as f64 * 100.0;
    if pct < 10.0 {
        return Vec::new();
    }
    let mut locations: Vec<String> = blocks
        .iter()
        .flat_map(|b| {
            b.iter()
                .map(|(path, line)| format!("{}:{}", path.display(), line))
        })
        .collect();
    locations.sort();
    locations.dedup();
    vec![AesCodeAnalysisViolation::CodeDuplication {
        reason: Some(LintMessage::new(format!(
            "AES305: Duplicate block ({} lines) at {} — {:.1}% of total across {} occurrence(s).",
            min_dup_lines,
            locations.join(", "),
            pct,
            blocks.iter().map(|b| b.len()).sum::<usize>()
        ))),
    }]
}

/// Collect file entries: (PathBuf, content_string) for each lintable file.
pub fn collect_file_entries(files: &[String]) -> Vec<(PathBuf, String)> {
    let mut out = Vec::new();
    for file_str in files {
        let fp = match FilePath::new(file_str.clone()) {
            Ok(f) => f,
            Err(_) => continue,
        };
        if !fp.is_lintable() {
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
