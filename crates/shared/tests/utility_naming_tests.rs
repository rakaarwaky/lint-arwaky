extern crate shared_lint_arwaky as shared;

use shared::naming_rules::utility_naming::{get_stem, get_suffix};

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

/// Regression test for Phase 3.4: get_stem handles dotfiles correctly.
/// Dotfiles like `.gitignore` should return the entire filename (not empty string).
#[test]
fn get_stem_dotfile_returns_full_name() {
    assert_eq!(get_stem(".gitignore"), Some(".gitignore"));
    assert_eq!(get_stem(".eslintrc"), Some(".eslintrc"));
    assert_eq!(get_stem(".prettierrc"), Some(".prettierrc"));
}

/// Regression test: get_stem handles normal files correctly.
#[test]
fn get_stem_normal_file() {
    assert_eq!(get_stem("checker.rs"), Some("checker"));
    assert_eq!(get_stem("mod.rs"), Some("mod"));
    assert_eq!(get_stem("foo.spec.rs"), Some("foo.spec"));
}

/// Regression test: get_stem handles files with no extension.
#[test]
fn get_stem_no_extension() {
    assert_eq!(get_stem("Makefile"), Some("Makefile"));
    assert_eq!(get_stem("README"), Some("README"));
}

/// Regression test: get_stem handles empty string.
#[test]
fn get_stem_empty_string() {
    assert_eq!(get_stem(""), Some(""));
}

/// Regression test: get_suffix extracts suffix after underscore.
#[test]
fn get_suffix_extracts_last_word() {
    assert_eq!(get_suffix("foo_bar"), Some("bar"));
    assert_eq!(get_suffix("hello_world_foo"), Some("foo"));
    // "no_underscore" has an underscore, so it returns Some("underscore")
    assert_eq!(get_suffix("no_underscore"), Some("underscore"));
}

/// Regression test: get_suffix handles strings without underscores.
#[test]
fn get_suffix_no_underscore_returns_none() {
    assert_eq!(get_suffix("nounderscore"), None);
    assert_eq!(get_suffix("simple"), None);
}

/// Regression test: get_suffix handles empty string.
#[test]
fn get_suffix_empty_string() {
    assert_eq!(get_suffix(""), None);
}
