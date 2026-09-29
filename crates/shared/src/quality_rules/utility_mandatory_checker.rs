// PURPOSE: Stateless utility functions for mandatory definition checking (AES303)
// Extracted from capabilities_mandatory_definition_checker.rs — pure functions, no &self, no I/O

/// Check if a line declares a Rust struct/enum/trait/type using word-boundary matching.
/// Handles visibility modifiers (pub, pub(crate)), tuple structs, and avoids
/// substring false-positives like "obstruction", "structure", "instruction".
pub fn rust_declares_type(line: &str) -> bool {
    let keywords = ["struct", "enum", "trait", "type"];
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }
    for kw in &keywords {
        if keyword_at_word_boundary(trimmed, kw) {
            return true;
        }
    }
    false
}

/// Word-boundary keyword matcher — checks if `token` appears as a complete word in `line`.
fn keyword_at_word_boundary(line: &str, token: &str) -> bool {
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
