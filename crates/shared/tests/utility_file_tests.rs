extern crate shared_lint_arwaky as shared;

use shared::common::utility_file_handler::is_path_ignored;

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

/// Regression test for Phase 3.2: is_path_ignored extension matching includes dot prefix.
/// Pattern `*.rs` should match files ending in `.rs`, not bare `rs`.
#[test]
fn is_path_ignored_extension_with_dot_prefix() {
    // *.rs should match "foo.rs" (with dot)
    assert!(is_path_ignored("foo.rs", &["*.rs".to_string()]));

    // *.rs should NOT match "bars" (without dot prefix)
    assert!(!is_path_ignored("bars", &["*.rs".to_string()]));

    // Multiple extension patterns
    assert!(is_path_ignored("test.py", &["*.py".to_string()]));
    assert!(is_path_ignored("app.ts", &["*.ts".to_string()]));
    assert!(is_path_ignored("style.css", &["*.css".to_string()]));
}

/// Regression test: is_path_ignored handles glob patterns with wildcards.
#[test]
fn is_path_ignored_glob_patterns() {
    // **/*.rs should match nested files
    assert!(is_path_ignored("src/foo.rs", &["**/*.rs".to_string()]));

    // Prefix patterns like "target/*" should match
    assert!(is_path_ignored("target/debug", &["target/*".to_string()]));
}

/// Regression test: is_path_ignored handles exact path matches.
#[test]
fn is_path_ignored_exact_match() {
    // Exact path patterns
    assert!(is_path_ignored(".gitignore", &[".gitignore".to_string()]));
    assert!(!is_path_ignored("gitignore", &[".gitignore".to_string()]));
}

/// Regression test: is_path_ignored handles segment-based matching.
#[test]
fn is_path_ignored_segment_matching() {
    // Segment-based patterns (without wildcards) match any segment
    assert!(is_path_ignored(
        "node_modules/foo/bar",
        &["node_modules".to_string()]
    ));
    assert!(!is_path_ignored(
        "my_node_modules/foo",
        &["node_modules".to_string()]
    ));
}

/// Regression test: is_path_ignored handles empty patterns list.
#[test]
fn is_path_ignored_empty_patterns() {
    let patterns: Vec<String> = vec![];
    assert!(!is_path_ignored("anything.rs", &patterns));
}

/// Regression test: is_path_ignored handles empty path.
#[test]
fn is_path_ignored_empty_path() {
    assert!(!is_path_ignored("", &["*.rs".to_string()]));
}
