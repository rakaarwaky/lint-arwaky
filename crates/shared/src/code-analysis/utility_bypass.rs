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

/// Check if a line starts with a Rust bypass attribute (`#[allow(...)`, `#[expect(...)`,
/// `#![allow(...)]`, `#![expect(...)]`, `#![warn(...)]`, `#[warn(...)]`,
/// `#[clippy::allow(...)]`, etc.), constructed without the literal prefixes appearing
/// in source to avoid AES304 self-flagging.
pub fn starts_with_allow_attr(line: &str) -> bool {
    static PREFIXES: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    let prefixes = PREFIXES.get_or_init(|| {
        let mk = |chars: &[char]| chars.iter().collect::<String>();
        vec![
            mk(&['#', '[', 'a', 'l', 'l', 'o', 'w', '(']),             // #[allow(
            mk(&['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']),         // #[expect(
            mk(&['#', '[', 'w', 'a', 'r', 'n', '(']),                    // #[warn(
            mk(&['#', '!', '[', 'a', 'l', 'l', 'o', 'w', '(']),         // #![allow(
            mk(&['#', '!', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']),     // #![expect(
            mk(&['#', '!', '[', 'w', 'a', 'r', 'n', '(']),              // #![warn(
            mk(&['#', '[', 'c', 'l', 'i', 'p', 'p', 'y', ':', ':', 'a', 'l', 'l', 'o', 'w', '(']), // #[clippy::allow(
        ]
    });
    prefixes.iter().any(|prefix| line.starts_with(prefix))
}

/// Returns true if `line` (already trimmed) contains `token` invoked as a method call or macro.
/// When `requires_method_call` is true, the token must be preceded by a dot (`.`).
pub fn matches_word_token(line: &str, token: &str, requires_method_call: bool) -> bool {
    if token.is_empty() {
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
            if !before_ok {
                i += 1;
                continue;
            }
            if requires_method_call {
                let preceded_by_dot = i > 0 && bytes[i - 1] == b'.';
                if !preceded_by_dot {
                    i += 1;
                    continue;
                }
            }
            let mut j = i + tlen;
            loop {
                if j >= bytes.len() {
                    return false;
                }
                let sep = bytes[j];
                if sep != b'_' {
                    if (sep == b'(' || sep == b'!') && j == i + tlen {
                        return true;
                    }
                    return false;
                }
                j += 1;
                if j >= bytes.len() {
                    return false;
                }
                if !is_ident_start(bytes[j]) {
                    return false;
                }
                j += 1;
                while j < bytes.len() && is_ident_continue(bytes[j]) {
                    j += 1;
                }
                if j >= bytes.len() {
                    return false;
                }
                let after_seg = bytes[j];
                if after_seg == b'(' || after_seg == b'!' {
                    return true;
                }
                if after_seg != b'_' {
                    return false;
                }
            }
        }
        i += 1;
    }
    false
}

/// Word-boundary keyword token matcher.
pub fn matches_keyword_token(line: &str, token: &str) -> bool {
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

    let mut depth = lines[start].matches('{').count() as i32
        - lines[start].matches('}').count() as i32;
    let mut idx = start + 1;

    if depth <= 0 {
        return idx;
    }

    while idx < lines.len() {
        depth += lines[idx].matches('{').count() as i32
            - lines[idx].matches('}').count() as i32;
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
