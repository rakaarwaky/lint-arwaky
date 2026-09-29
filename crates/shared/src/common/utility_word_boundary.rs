// PURPOSE: Word-boundary helpers and inline-comment stripping — pure,
// stateless utilities used by auto-fix capability structs.
//
// Kept in shared/ so each capability remains thin and the logic is
// reusable without pulling in struct definitions or trait impls.

/// Strip an inline comment from a code line, preserving leading whitespace.
///
/// For `    let x = foo()  // FIXME: refactor` → `    let x = foo()  `
/// When `hash_comments` is true (Python files), `#` is also treated as a
/// comment start, but Rust-style `#[` attribute lines are left alone.
pub fn strip_inline_comment(line: &str, hash_comments: bool) -> String {
    if let Some(pos) = line.find("//") {
        return line[..pos].to_string();
    }
    if hash_comments {
        if let Some(pos) = line.find('#') {
            if !line[pos..].starts_with("#[") {
                return line[..pos].to_string();
            }
        }
    }
    line.to_string()
}

/// Count occurrences of `target` that match word boundaries in `text`.
///
/// A match is at a word boundary when the character before (if any) and the
/// character after (if any) are both non-identifier characters.
pub fn word_boundary_count(text: &str, target: &str) -> usize {
    let mut count = 0;
    let target_len = target.len();
    let bytes = text.as_bytes();
    let target_bytes = target.as_bytes();

    for i in 0..bytes.len() {
        if i + target_len > bytes.len() {
            break;
        }
        if &bytes[i..i + target_len] == target_bytes && is_word_boundary(bytes, i, target_len) {
            count += 1;
        }
    }
    count
}

/// Replace occurrences of `target` with `replacement` only at word boundaries.
pub fn word_boundary_replace(text: &str, target: &str, replacement: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let target_len = target.len();
    let bytes = text.as_bytes();
    let target_bytes = target.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if i + target_len <= bytes.len()
            && &bytes[i..i + target_len] == target_bytes
            && is_word_boundary(bytes, i, target_len)
        {
            result.push_str(replacement);
            i += target_len;
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    result
}

/// Check if a match at position `pos` of length `len` is at a word boundary.
pub fn is_word_boundary(bytes: &[u8], pos: usize, len: usize) -> bool {
    let before_ok = pos == 0 || !bytes[pos - 1].is_ascii_alphanumeric() && bytes[pos - 1] != b'_';
    let after_ok = pos + len >= bytes.len()
        || !bytes[pos + len].is_ascii_alphanumeric() && bytes[pos + len] != b'_';
    before_ok && after_ok
}
