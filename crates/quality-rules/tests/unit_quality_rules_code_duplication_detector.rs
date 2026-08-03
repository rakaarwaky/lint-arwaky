// Unit tests for code duplication detector — normalization, windowing, violation building.
use quality_rules_lint_arwaky::utility_code_duplication_detector::{
    build_violations, normalize_line, normalize_window, scan_duplicate_blocks,
};
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
