extern crate shared_lint_arwaky as shared;

use shared::code_analysis::utility_bypass_detector::{matches_keyword_token, matches_word_token};

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

/// Regression test for Phase 3.9: matches_word_token restricts underscore suffix
/// to known panicking variants (unwrap_unchecked, panic_any).
#[test]
fn matches_word_token_restricted_underscore_suffixes() {
    // Known unsafe variants should match
    assert!(matches_word_token("x.unwrap_unchecked()", "unwrap", true));
    assert!(matches_word_token("x.panic_any()", "panic", true));

    // Unknown suffixes should NOT match when requires_method_call is true
    // This prevents false positives like matching "foo_something()" for token "foo"
    assert!(!matches_word_token("x.foo_something()", "foo", true));
    assert!(!matches_word_token("x.bar_unsafe()", "bar", true));

    // Direct method calls should still match
    assert!(matches_word_token("x.unwrap()", "unwrap", true));
    assert!(matches_word_token("x.panic()", "panic", true));
}

/// Regression test: matches_word_token handles keyword tokens correctly.
#[test]
fn matches_keyword_token_word_boundaries() {
    // Keywords should match at proper word boundaries
    assert!(matches_keyword_token("let x = unwrap();", "unwrap"));
    assert!(matches_keyword_token("fn main() { unwrap(); }", "main"));

    // Substrings should NOT match
    assert!(!matches_keyword_token("my_unwrap()", "unwrap"));
    assert!(!matches_keyword_token("unwrap_something()", "unwrap"));
}

/// Regression test: matches_word_token skips comment lines.
#[test]
fn matches_word_token_skips_comments() {
    // Lines starting with // should be skipped
    assert!(!matches_word_token("// unwrap()", "unwrap", true));
    assert!(!matches_word_token("/* unwrap */", "unwrap", true));
    assert!(!matches_word_token("* unwrap *", "unwrap", true));
}

/// Regression test: matches_word_token handles empty token.
#[test]
fn matches_word_token_empty_token() {
    assert!(!matches_word_token("anything", "", true));
}

/// Regression test: matches_word_token handles line shorter than token.
#[test]
fn matches_word_token_short_line() {
    assert!(!matches_word_token("foo", "foobar", true));
}
