// Unit tests for utility_path_filter — is_path_ignored pattern matching.

use shared_lint_arwaky::common::utility_path_filter::is_path_ignored;

#[test]
fn is_path_ignored_empty_path_returns_false() {
    let ignored: Vec<String> = vec!["target".to_string()];
    assert!(!is_path_ignored("", &ignored));
}

#[test]
fn is_path_ignored_empty_pattern_skipped() {
    let ignored: Vec<String> = vec![String::new(), "target".to_string()];
    assert!(is_path_ignored("target/main.rs", &ignored));
}

// ─── Prefix pattern: /dir/** ──────────────────────────────────

#[test]
fn is_path_ignored_prefix_pattern_exact_match() {
    let ignored = vec!["/build".to_string()];
    assert!(is_path_ignored("build/output.o", &ignored));
}

#[test]
fn is_path_ignored_prefix_pattern_nested() {
    let ignored = vec!["/build/sub".to_string()];
    assert!(is_path_ignored("build/sub/cache.bin", &ignored));
}

#[test]
fn is_path_ignored_prefix_pattern_no_false_positive() {
    let ignored = vec!["/build".to_string()];
    assert!(!is_path_ignored("src/build.rs", &ignored));
}

// ─── Recursive glob: **/*.ext ────────────────────────────────

#[test]
fn is_path_ignored_recursive_glob_extension() {
    let ignored = vec!["**/*.rs".to_string()];
    assert!(is_path_ignored("src/main.rs", &ignored));
    assert!(is_path_ignored("deep/nested/file.rs", &ignored));
}

#[test]
fn is_path_ignored_recursive_glob_no_extension_match() {
    let ignored = vec!["**/*.rs".to_string()];
    assert!(!is_path_ignored("src/main.txt", &ignored));
}

// ─── Single-level wildcard: dir/* ────────────────────────────

#[test]
fn is_path_ignored_single_level_wildcard() {
    let ignored = vec!["target/*".to_string()];
    assert!(is_path_ignored("target/output.o", &ignored));
}

#[test]
fn is_path_ignored_single_level_wildcard_no_false_positive() {
    let ignored = vec!["target/*".to_string()];
    assert!(!is_path_ignored("src/target.rs", &ignored));
}

// ─── Extension pattern: *.ext ────────────────────────────────

#[test]
fn is_path_ignored_extension_pattern() {
    let ignored = vec!["*.rs".to_string()];
    assert!(is_path_ignored("src/main.rs", &ignored));
    assert!(is_path_ignored("deep/nested/file.rs", &ignored));
}

#[test]
fn is_path_ignored_extension_pattern_no_match() {
    let ignored = vec!["*.rs".to_string()];
    assert!(!is_path_ignored("src/main.txt", &ignored));
}

// ─── Dot-prefixed literal: .git, .env ────────────────────────

#[test]
fn is_path_ignored_dot_literal_segment() {
    let ignored = vec![".git".to_string()];
    assert!(is_path_ignored(".git/config", &ignored));
    assert!(!is_path_ignored("src/.gitignore", &ignored)); // exact match only
}

// ─── Literal path match: single segment ──────────────────────

#[test]
fn is_path_ignored_literal_single_segment() {
    let ignored = vec!["target".to_string()];
    assert!(is_path_ignored("target/output.o", &ignored));
}

#[test]
fn is_path_ignored_literal_single_segment_no_match() {
    let ignored = vec!["target".to_string()];
    assert!(!is_path_ignored("src/target.rs", &ignored));
}

// ─── Multi-segment literal path ──────────────────────────────

#[test]
fn is_path_ignored_literal_multi_segment() {
    let ignored = vec!["build/release".to_string()];
    assert!(is_path_ignored("build/release/output.o", &ignored));
}

// ─── Backslash separator support ─────────────────────────────

#[test]
fn is_path_ignored_backslash_separator() {
    let ignored = vec!["build".to_string()];
    assert!(is_path_ignored("build\\output.o", &ignored));
}

// ─── Multiple patterns ──────────────────────────────────────

#[test]
fn is_path_ignored_multiple_patterns_first_match() {
    let ignored = vec!["*.rs".to_string(), "*.txt".to_string()];
    assert!(is_path_ignored("src/main.rs", &ignored));
}

#[test]
fn is_path_ignored_multiple_patterns_second_match() {
    let ignored = vec!["*.rs".to_string(), "*.txt".to_string()];
    assert!(is_path_ignored("doc/readme.txt", &ignored));
}

#[test]
fn is_path_ignored_multiple_patterns_no_match() {
    let ignored = vec!["*.rs".to_string(), "*.txt".to_string()];
    assert!(!is_path_ignored("src/main.c", &ignored));
}

// ─── Path with no segments (edge case) ──────────────────────

#[test]
fn is_path_ignored_single_segment_path() {
    let ignored = vec!["target".to_string()];
    assert!(is_path_ignored("target", &ignored));
}

#[test]
fn is_path_ignored_double_slash_no_segments() {
    let ignored = vec!["target".to_string()];
    assert!(!is_path_ignored("//", &ignored));
}
