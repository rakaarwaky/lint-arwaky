extern crate shared_lint_arwaky as shared;

use shared::code_analysis::utility_column_index::{byte_offset_to_column, compute_column};

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

/// Regression test for Phase 3.1: compute_column returns character columns (not byte offsets).
/// For multi-byte UTF-8 characters, byte offset and character column differ.
#[test]
fn compute_column_returns_character_columns() {
    // "café" — 'é' is 2 bytes in UTF-8 (0xC3 0xA9)
    // Character positions: c=1, a=2, f=3, é=4
    // Byte offsets: c=0, a=1, f=2, é=3, é_end=5
    let line = "café";

    // "café" pattern starts at byte offset 0, character column 1
    assert_eq!(compute_column(line, "café"), 1);

    // "f" is at byte offset 2, but character column 3
    assert_eq!(compute_column(line, "f"), 3);

    // "é" starts at byte offset 3, but it's the 4th character
    assert_eq!(compute_column(line, "é"), 4);
}

/// Regression test: compute_column handles ASCII correctly.
#[test]
fn compute_column_ascii() {
    let line = "hello world";
    // "world" starts at char position 7 (1-indexed)
    assert_eq!(compute_column(line, "world"), 7);
    // "hello" starts at char position 1
    assert_eq!(compute_column(line, "hello"), 1);
    // "lo" starts at char position 4 (1-indexed)
    assert_eq!(compute_column(line, "lo"), 4);
}

/// Regression test: byte_offset_to_column converts byte offset to character column.
#[test]
fn byte_offset_to_column_conversion() {
    // For ASCII line, byte offset and char column have a simple relationship
    assert_eq!(byte_offset_to_column("hello", 0), 1); // first char
    assert_eq!(byte_offset_to_column("hello", 4), 5); // fifth char (index 4)

    // For multi-byte UTF-8, byte offset != character column
    let line = "café";
    // 'é' starts at byte offset 3, but it's the 4th character
    assert_eq!(byte_offset_to_column(line, 3), 4);
}

/// Regression test: compute_column handles empty pattern.
#[test]
fn compute_column_empty_pattern() {
    // Empty pattern should return column 1 (found at position 0)
    assert_eq!(compute_column("hello", ""), 1);
}

/// Regression test: compute_column returns 0 for not-found patterns.
#[test]
fn compute_column_not_found() {
    assert_eq!(compute_column("hello", "xyz"), 0);
}

/// Regression test: compute_column handles Unicode emoji and complex scripts.
#[test]
fn compute_column_complex_unicode() {
    // Emoji are multi-byte (4 bytes each in UTF-8)
    let line = "🎉🎊";
    assert_eq!(compute_column(line, "🎊"), 2);

    // CJK characters (3 bytes each)
    let line = "こんにちは";
    assert_eq!(compute_column(line, "にち"), 3);
}
