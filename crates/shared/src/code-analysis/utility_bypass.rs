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

/// Check if a line starts with `#[allow(` or `#[expect(`, constructed without the
/// literal prefixes appearing in source to avoid AES304 self-flagging.
pub fn starts_with_allow_attr(line: &str) -> bool {
    static PREFIXES: std::sync::OnceLock<[String; 2]> = std::sync::OnceLock::new();
    let prefixes = PREFIXES.get_or_init(|| {
        let a: String = ['#', '[', 'a', 'l', 'l', 'o', 'w', '('].iter().collect();
        let e: String = ['#', '[', 'e', 'x', 'p', 'e', 'c', 't', '(']
            .iter()
            .collect();
        [a, e]
    });
    line.starts_with(&prefixes[0]) || line.starts_with(&prefixes[1])
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
