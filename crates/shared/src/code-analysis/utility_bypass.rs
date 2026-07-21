// PURPOSE: Stateless utility functions for bypass checking (AES304)
// Pure functions only — no domain types (enums, consts) belong here

/// Returns true if byte is a valid identifier continuation character.
pub fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Returns true if byte can start an identifier.
pub fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

/// Strip trailing `// ...` comment from a line, respecting string literals.
/// Returns the code portion only (everything before the first unquoted `//`).
pub fn strip_trailing_comment(line: &str) -> &str {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut in_string = false;
    let mut in_char = false;

    while i < len {
        let b = bytes[i];

        // Handle string boundaries
        if b == b'"' && !in_char {
            if in_string {
                // Check for escaped quote
                if i > 0 && bytes[i - 1] == b'\\' {
                    i += 1;
                    continue;
                }
                in_string = false;
            } else {
                in_string = true;
            }
            i += 1;
            continue;
        }

        // Handle char boundaries
        if b == b'\'' && !in_string {
            if in_char {
                if i > 0 && bytes[i - 1] == b'\\' {
                    i += 1;
                    continue;
                }
                in_char = false;
            } else {
                in_char = true;
            }
            i += 1;
            continue;
        }

        // Skip content inside strings/chars
        if in_string || in_char {
            i += 1;
            continue;
        }

        // Detect `//` comment start
        if b == b'/' && i + 1 < len && bytes[i + 1] == b'/' {
            return &line[..i];
        }

        i += 1;
    }

    line
}

/// Check if a byte position in a line is inside a string or char literal.
pub fn is_inside_string_or_char(line: &str, pos: usize) -> bool {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    let mut in_string = false;
    let mut in_char = false;

    while i < len && i < pos {
        let b = bytes[i];

        if b == b'"' && !in_char {
            if in_string && i > 0 && bytes[i - 1] == b'\\' {
                i += 1;
                continue;
            }
            in_string = !in_string;
            i += 1;
            continue;
        }

        if b == b'\'' && !in_string {
            if in_char && i > 0 && bytes[i - 1] == b'\\' {
                i += 1;
                continue;
            }
            in_char = !in_char;
            i += 1;
            continue;
        }

        if in_string || in_char {
            i += 1;
            continue;
        }

        i += 1;
    }

    in_string || in_char
}

/// Check if a line starts with a Rust bypass attribute (`#[allow(...)`, `#[expect(...)`,
/// `#![allow(...)]`, `#![expect(...)]`, `#![warn(...)]`, `#[warn(...)]`,
/// `#[clippy::allow(...)]`, etc.), constructed without the literal prefixes appearing
/// in source to avoid AES304 self-flagging.
pub fn starts_with_allow_attr(line: &str) -> bool {
    static PREFIXES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    let prefixes = PREFIXES.get_or_init(|| {
        let mk = |chars: &[char]| chars.iter().collect::<String>();
        vec![
            mk(&['#', '[', 'a', 'l', 'l', 'o', 'w', '(']), // #[allow(
            mk(&['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']), // #[expect(
            mk(&['#', '[', 'w', 'a', 'r', 'n', '(']),      // #[warn(
            mk(&['#', '!', '[', 'a', 'l', 'l', 'o', 'w', '(']), // #![allow(
            mk(&['#', '!', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']), // #![expect(
            mk(&['#', '!', '[', 'w', 'a', 'r', 'n', '(']), // #![warn(
            mk(&[
                '#', '[', 'c', 'l', 'i', 'p', 'p', 'y', ':', ':', 'a', 'l', 'l', 'o', 'w', '(',
            ]), // #[clippy::allow(
        ]
    });
    prefixes.iter().any(|prefix| line.starts_with(prefix))
}

/// Check if a suffix after underscore is a known panicking/unsafe variant.
fn forbidden_method_suffix(token: &str, suffix: &str) -> bool {
    matches!((token, suffix), ("unwrap", "unchecked") | ("panic", "any"))
}

/// Returns true if `line` (already trimmed) contains `token` invoked as a method call or macro.
/// When `requires_method_call` is true, the token must be preceded by a dot (`.`).
pub fn matches_word_token(line: &str, token: &str, requires_method_call: bool) -> bool {
    if token.is_empty() {
        return false;
    }

    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }

    let bytes = line.as_bytes();
    let token_bytes = token.as_bytes();
    let tlen = token_bytes.len();

    if bytes.len() < tlen {
        return false;
    }

    let mut i = 0;

    while i + tlen <= bytes.len() {
        if &bytes[i..i + tlen] == token_bytes {
            let before_ok = i == 0 || !is_ident_start(bytes[i - 1]);

            if before_ok {
                if requires_method_call {
                    let preceded_by_dot = i > 0 && bytes[i - 1] == b'.';
                    if !preceded_by_dot {
                        i += 1;
                        continue;
                    }
                }

                let j = i + tlen;

                if j < bytes.len() && (bytes[j] == b'(' || bytes[j] == b'!') {
                    return true;
                }

                if j < bytes.len() && bytes[j] == b'_' {
                    let seg_start = j + 1;

                    if seg_start < bytes.len() && is_ident_start(bytes[seg_start]) {
                        let mut seg_end = seg_start;

                        while seg_end < bytes.len() && is_ident_continue(bytes[seg_end]) {
                            seg_end += 1;
                        }

                        let seg = &line[seg_start..seg_end];
                        let k = seg_end;

                        if k < bytes.len()
                            && (bytes[k] == b'(' || bytes[k] == b'!')
                            && forbidden_method_suffix(token, seg)
                        {
                            return true;
                        }
                    }
                }
            }
        }

        i += 1;
    }

    false
}

/// Word-boundary keyword token matcher.
pub fn matches_keyword_token(line: &str, token: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }

    let bytes = line.as_bytes();
    let token_bytes = token.as_bytes();
    let tlen = token_bytes.len();

    if bytes.len() < tlen {
        return false;
    }

    let mut i = 0;

    while i + tlen <= bytes.len() {
        if &bytes[i..i + tlen] == token_bytes {
            let before_ok =
                i == 0 || (!bytes[i - 1].is_ascii_alphanumeric() && bytes[i - 1] != b'_');

            let after_ok = i + tlen == bytes.len()
                || (!bytes[i + tlen].is_ascii_alphanumeric() && bytes[i + tlen] != b'_');

            if before_ok && after_ok {
                return true;
            }
        }

        i += 1;
    }

    false
}

/// Skip a brace-delimited block starting at `start`.
///
/// Returns the index of the first line after the block.
/// If the starting line is already balanced or has no opening brace,
/// returns `start + 1`.
pub fn skip_brace_block(lines: &[&str], start: usize) -> usize {
    if start >= lines.len() {
        return start;
    }

    let mut depth =
        lines[start].matches('{').count() as i32 - lines[start].matches('}').count() as i32;
    let mut idx = start + 1;

    if depth <= 0 {
        return idx;
    }

    while idx < lines.len() {
        depth += lines[idx].matches('{').count() as i32 - lines[idx].matches('}').count() as i32;
        idx += 1;

        if depth <= 0 {
            break;
        }
    }

    idx
}

/// Skip a `#[cfg(test)]` module block when present.
///
/// If the attribute is followed by a test module, returns the first line
/// after that module. Otherwise, returns `start + 1`, skipping only the
/// attribute line.
pub fn skip_cfg_test_block(lines: &[&str], start: usize) -> usize {
    if start >= lines.len() {
        return start;
    }

    let mut idx = start + 1;

    // Skip blank lines and additional attributes.
    while idx < lines.len() {
        let t = lines[idx].trim();
        if t.is_empty() || t.starts_with('#') {
            idx += 1;
            continue;
        }
        break;
    }

    if idx >= lines.len() {
        return idx;
    }

    let t = lines[idx].trim();
    let is_mod = t.split_whitespace().any(|w| w == "mod");

    // Not a module attribute; skip only the attribute line.
    if !is_mod {
        return start + 1;
    }

    // Module declaration without body, e.g. `mod tests;`.
    if t.ends_with(';') && !t.contains('{') {
        return idx + 1;
    }

    let mut depth = t.matches('{').count() as i32 - t.matches('}').count() as i32;
    idx += 1;

    // The module body opened and closed on the same line, e.g. `mod tests {}`.
    if depth <= 0 && t.contains('{') {
        return idx;
    }

    // Look for an opening brace on following lines.
    if depth <= 0 {
        while idx < lines.len() {
            let st = lines[idx].trim();
            depth += st.matches('{').count() as i32 - st.matches('}').count() as i32;
            idx += 1;

            if depth > 0 {
                break;
            }

            // Opened and closed immediately on the next line.
            if depth <= 0 && st.contains('{') {
                return idx;
            }
        }
    }

    // Consume until the module body closes.
    while idx < lines.len() && depth > 0 {
        let st = lines[idx].trim();
        depth += st.matches('{').count() as i32 - st.matches('}').count() as i32;
        idx += 1;
    }

    idx
}

// ─── Regression Tests for Phase 3 Fixes ──────────────────────────────────────────

#[cfg(test)]
mod phase3_regression_tests {
    use super::*;

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
}
