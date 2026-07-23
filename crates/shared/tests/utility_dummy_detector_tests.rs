extern crate shared_lint_arwaky as shared;

use shared::common::taxonomy_common_vo::LineNumber;
use shared::import_rules::utility_dummy_detector::{
    contains_ident, is_short_marker, js_imported_symbols, python_imported_symbols, symbol_used_real,
};

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

/// Regression test for Phase 3.6: Python import alias handling uses `as` keyword.
/// Import aliases like `import foo as bar` should extract `bar`, not `foo`.
#[test]
fn python_import_alias_extraction() {
    let lines = &[
        "import os",
        "import foo as bar",
        "from module import baz as qux",
    ];
    let symbols = python_imported_symbols(lines);

    // Should extract "bar" from "import foo as bar"
    assert!(
        symbols.iter().any(|(name, _)| name.value() == "bar"),
        "Python import alias 'bar' should be extracted from 'import foo as bar'"
    );

    // Should extract "qux" from "from module import baz as qux"
    assert!(
        symbols.iter().any(|(name, _)| name.value() == "qux"),
        "Python import alias 'qux' should be extracted from 'from module import baz as qux'"
    );

    // Should extract "os" (no alias)
    assert!(
        symbols.iter().any(|(name, _)| name.value() == "os"),
        "Python import 'os' (no alias) should be extracted"
    );
}

/// Regression test for Phase 3.7: JS import alias handling uses `as` keyword.
/// Import aliases like `import { foo as bar }` should extract `bar`, not `foo`.
#[test]
fn js_import_alias_extraction() {
    let lines = &[
        "import foo from 'foo'",
        "import { baz as qux } from 'module'",
    ];
    let symbols = js_imported_symbols(lines);

    // Should extract "qux" from "import { baz as qux }"
    assert!(
        symbols.iter().any(|(name, _)| name.value() == "qux"),
        "JS import alias 'qux' should be extracted from 'import {{ baz as qux }}'"
    );

    // Should extract "foo" (default import, no alias)
    assert!(
        symbols.iter().any(|(name, _)| name.value() == "foo"),
        "JS default import 'foo' (no alias) should be extracted"
    );
}

/// Regression test for Phase 3.8: contains_ident checks word boundaries.
/// Symbol matching should not match substrings — "unwrap" should NOT match "my_unwrap".
#[test]
fn contains_ident_word_boundary_rejection() {
    // "unwrap" should NOT match inside "my_unwrap_something"
    assert!(!contains_ident("my_unwrap_something", "unwrap"));

    // "unwrap" SHOULD match at word boundaries
    assert!(contains_ident("unwrap", "unwrap"));
    assert!(contains_ident("let x = unwrap();", "unwrap"));
    assert!(contains_ident("foo.unwrap()", "unwrap"));

    // Symbol with underscore prefix should be matched
    assert!(contains_ident("_unused_var", "_unused_var"));
}

/// Regression test: contains_ident handles multi-byte UTF-8 characters.
#[test]
fn contains_ident_utf8_safe() {
    // Multi-byte characters should not cause panics or incorrect matches
    let haystack = "hello world — test"; // em dash is 3 bytes in UTF-8
    assert!(contains_ident(haystack, "world"));
    assert!(contains_ident(haystack, "—")); // em dash is surrounded by spaces

    // Test with Unicode characters that are multi-byte
    let haystack = "café"; // é is 2 bytes in UTF-8
    assert!(contains_ident(haystack, "café")); // whole word match
    assert!(!contains_ident(haystack, "caf")); // not a whole identifier (followed by é)
}

/// Regression test: contains_ident handles empty needle.
#[test]
fn contains_ident_empty_needle() {
    assert!(!contains_ident("anything", ""));
}

/// Regression test: contains_ident handles empty haystack.
#[test]
fn contains_ident_empty_haystack() {
    assert!(!contains_ident("", "needle"));
}

/// Regression test: symbol_used_real correctly identifies used symbols.
#[test]
fn symbol_used_real_detection() {
    let lines = &[
        "fn main() {",
        "    let x = my_function();",
        "    println!(\"{}\", x);",
        "}",
    ];
    let dummy_ranges: Vec<(LineNumber, LineNumber)> = vec![];
    let dummy_impl_traits: Vec<String> = vec![];

    // "my_function" should be detected as used (not in dummy range)
    assert!(symbol_used_real(
        lines,
        "my_function",
        &dummy_ranges,
        &dummy_impl_traits
    ));

    // "nonexistent" should NOT be detected
    assert!(!symbol_used_real(
        lines,
        "nonexistent",
        &dummy_ranges,
        &dummy_impl_traits
    ));
}

/// Regression test: is_short_marker identifies short marker patterns.
#[test]
fn is_short_marker_identifies_todo() {
    assert!(is_short_marker("todo!()"));
}

#[test]
fn is_short_marker_identifies_unimplemented() {
    assert!(is_short_marker("unimplemented!()"));
}

#[test]
fn is_short_marker_identifies_panic() {
    assert!(is_short_marker("panic!()"));
}

#[test]
fn is_short_marker_identifies_unreachable() {
    assert!(is_short_marker("unreachable!()"));
}

#[test]
fn is_short_marker_non_marker_returns_false() {
    assert!(!is_short_marker("println!()"));
    assert!(!is_short_marker("normal_function()"));
}
