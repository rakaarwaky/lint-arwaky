// PURPOSE: Stateless utility functions for code duplication analysis (AES305)
// Extracted from capabilities_code_duplication_analyzer.rs — pure functions, no &self, no I/O

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;

use std::path::PathBuf;

use shared::common::taxonomy_message_vo::LintMessage;
use shared::quality_rules::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;

const MAX_LOCATIONS_PER_BLOCK: usize = 128;

type BlockKey = (u64, u64);

#[derive(Debug, Default)]
struct BlockHits {
    count: usize,
    locations: Vec<(PathBuf, usize)>,
}

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

fn hash_window(window: &[&str]) -> BlockKey {
    let normalized = window
        .iter()
        .map(|line| normalize_line(line))
        .collect::<Vec<_>>()
        .join("|");

    let mut hasher = DefaultHasher::new();
    std::hash::Hash::hash(&normalized, &mut hasher);

    let primary = std::hash::Hasher::finish(&hasher);
    let secondary = normalized.len() as u64;

    (primary, secondary)
}

/// Slide a normalized `min_lines` window across each file and group identical windows.
/// Returns one entry per duplicated block, each holding the (path, 1-indexed start_line)
/// of every occurrence.
pub fn scan_duplicate_blocks(
    entries: Vec<(PathBuf, String)>,
    min_lines: usize,
) -> Vec<Vec<(PathBuf, usize)>> {
    let mut blocks: HashMap<BlockKey, BlockHits> = HashMap::new();

    for (path, content) in entries {
        let lines: Vec<&str> = content.lines().collect();

        if lines.len() < min_lines {
            continue;
        }

        for (index, window) in lines.windows(min_lines).enumerate() {
            let key = hash_window(window);
            let line_number = index + 1;

            let entry = blocks.entry(key).or_default();
            entry.count += 1;

            if entry.locations.len() < MAX_LOCATIONS_PER_BLOCK {
                entry.locations.push((path.clone(), line_number));
            }
        }
    }

    blocks
        .into_values()
        .filter(|block| block.count >= 2)
        .map(|block| block.locations)
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

    let mut duplicated_lines: HashSet<(PathBuf, usize)> = HashSet::new();

    for locs in blocks {
        for (path, start) in locs {
            for line in *start..(*start + min_dup_lines) {
                duplicated_lines.insert((path.clone(), line));
            }
        }
    }

    let dup_lines = duplicated_lines.len();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn normalize_line_trims_and_filters() {
        assert_eq!(normalize_line("  hello, world!  "), "hello world");
        assert_eq!(normalize_line("fn foo() -> i32"), "fn foo  i32");
        assert_eq!(normalize_line(""), "");
    }

    #[test]
    fn normalize_window_joins_with_pipe() {
        let window = vec!["fn foo()", "  let x = 1;", "  x + 1"];
        let result = normalize_window(&window);
        assert_eq!(result, "fn foo|let x  1|x  1");
    }

    #[test]
    fn scan_duplicate_blocks_finds_matches() {
        let entries = vec![
            (PathBuf::from("a.rs"), "line1\nline2\nline3\n".to_string()),
            (PathBuf::from("b.rs"), "line1\nline2\nline3\n".to_string()),
        ];
        let blocks = scan_duplicate_blocks(entries, 2);
        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn scan_duplicate_blocks_no_match_different_content() {
        let entries = vec![
            (PathBuf::from("a.rs"), "aaa\nbbb\nccc\n".to_string()),
            (PathBuf::from("b.rs"), "xxx\nyyy\nzzz\n".to_string()),
        ];
        let blocks = scan_duplicate_blocks(entries, 2);
        assert!(blocks.is_empty());
    }

    #[test]
    fn scan_duplicate_blocks_skips_short_files() {
        let entries = vec![
            (PathBuf::from("a.rs"), "line1\n".to_string()),
            (PathBuf::from("b.rs"), "line1\n".to_string()),
        ];
        let blocks = scan_duplicate_blocks(entries, 3);
        assert!(blocks.is_empty());
    }

    #[test]
    fn build_violations_empty_returns_empty() {
        let violations = build_violations(&[], 100, 10);
        assert!(violations.is_empty());
    }

    #[test]
    fn build_violations_low_percentage_returns_empty() {
        let blocks = vec![vec![(PathBuf::from("a.rs"), 1), (PathBuf::from("b.rs"), 1)]];
        // 20 duplicated lines / 1000 total = 2%, below 10% threshold
        let violations = build_violations(&blocks, 1000, 10);
        assert!(violations.is_empty());
    }
}
