// PURPOSE: utility_dummy_helper — pure utility functions for dummy function, block, and trait detection
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::import_rules::taxonomy_language_vo::LanguageVO;

pub fn dummy_function_ranges(lines: &[&str], lang: LanguageVO) -> Vec<(LineNumber, LineNumber)> {
    match lang {
        LanguageVO::Rust => rust_dummy_function_ranges(lines),
        LanguageVO::Python => python_dummy_function_ranges(lines),
        LanguageVO::JavaScript => js_dummy_function_ranges(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn imported_symbols(lines: &[&str], lang: LanguageVO) -> Vec<(SymbolName, LineNumber)> {
    match lang {
        LanguageVO::Rust => rust_imported_symbols(lines),
        LanguageVO::Python => python_imported_symbols(lines),
        LanguageVO::JavaScript => js_imported_symbols(lines),
        LanguageVO::Unknown => Vec::new(),
    }
}

pub fn dummy_impl_traits_with_lines(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut traits = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                let (end, body_lines) = impl_block(lines, i);
                if trait_impl_is_dummy(&body_lines) {
                    traits.push((SymbolName::new(trait_name), LineNumber::new(i as i64 + 1)));
                }
                i = end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    traits
}

pub fn symbol_used_real(
    lines: &[&str],
    symbol: &str,
    dummy_ranges: &[(LineNumber, LineNumber)],
    dummy_impl_traits: &[String],
) -> bool {
    let dummy_ranges_usize: Vec<(usize, usize)> = dummy_ranges
        .iter()
        .map(|(a, b)| (a.value() as usize, b.value() as usize))
        .collect();
    if (symbol.starts_with('I')
        && symbol.len() > 1
        && matches!(symbol.chars().nth(1), Some(c) if c.is_uppercase()))
        || symbol.ends_with("Protocol")
        || symbol.ends_with("Trait")
        || symbol.ends_with("Aggregate")
        || symbol.ends_with("Ext")
        || symbol == "Default"
        || symbol == "Debug"
        || symbol == "Display"
        || symbol == "Clone"
        || symbol == "Copy"
        || symbol == "From"
        || symbol == "Into"
        || symbol == "TryFrom"
        || symbol == "TryInto"
        || symbol == "AsRef"
        || symbol == "AsMut"
        || symbol == "Deref"
        || symbol == "DerefMut"
        || symbol == "Iterator"
        || symbol == "IntoIterator"
        || symbol == "Future"
        || symbol == "Stream"
        || symbol == "Read"
        || symbol == "Write"
        || symbol == "BufRead"
        || symbol == "Seek"
        || symbol == "Hash"
        || symbol == "PartialEq"
        || symbol == "Eq"
        || symbol == "PartialOrd"
        || symbol == "Ord"
        || symbol == "Send"
        || symbol == "Sync"
        || symbol == "Unpin"
        || symbol == "Sized"
        || symbol == "Drop"
        || symbol == "Fn"
        || symbol == "FnMut"
        || symbol == "FnOnce"
        || symbol == "async_trait"
        || symbol == "Parser"
        || symbol == "Digest"
        || symbol == "Manager"
        || symbol == "Emitter"
        || symbol == "Serialize"
        || symbol == "Deserialize"
    {
        return true;
    }

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if in_dummy_range(line_no, &dummy_ranges_usize)
            || trimmed.starts_with("use ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*")
            || trimmed.starts_with("*/")
            || (trimmed.starts_with("#") && !trimmed.starts_with("#["))
            || trimmed.contains("PhantomData")
        {
            continue;
        }

        if !contains_ident(trimmed, symbol) {
            continue;
        }

        // If the symbol only appears inside string literals, it's not real usage
        if is_symbol_only_in_strings(trimmed, symbol) {
            continue;
        }

        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                if dummy_impl_traits.contains(&trait_name) {
                    continue;
                }
            }
        }

        return true;
    }

    false
}

// ─── Private Helpers ───

/// Check if `haystack` contains `needle` as a whole identifier (not a substring).
pub fn contains_ident(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }

    let mut start = 0usize;

    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;
        let end = abs + needle.len();

        let before_ok = abs == 0 || {
            let before_char = haystack[..abs].chars().next_back().unwrap_or(' ');
            !before_char.is_alphanumeric() && before_char != '_'
        };

        let after_ok = end == haystack.len() || {
            let after_char = haystack[end..].chars().next().unwrap_or(' ');
            !after_char.is_alphanumeric() && after_char != '_'
        };

        if before_ok && after_ok {
            return true;
        }

        start = abs + needle.len();
    }

    false
}

/// Check if all occurrences of `needle` in `haystack` appear strictly inside
/// double-quoted string literals. Returns true when the symbol is never used
/// as a code identifier (only inside strings, comments, or doc lines).
pub fn is_symbol_only_in_strings(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() || !haystack.contains(needle) {
        return false;
    }

    let mut start = 0usize;
    let mut found_anywhere = false;

    while let Some(pos) = haystack[start..].find(needle) {
        let abs = start + pos;

        // Determine which string literal (if any) this occurrence falls inside.
        // We scan backwards from `abs` to find the opening quote that is not
        // escaped and not inside a comment.
        let opening_quote = match find_enclosing_string_start(haystack, abs) {
            Some(q) => q,
            None => return false, // not inside a string — real usage
        };

        // Verify the quote is an actual string delimiter (not escaped)
        let before_quote = &haystack[..opening_quote];
        let backslash_count = before_quote
            .chars()
            .rev()
            .take_while(|c| *c == '\\')
            .count();
        if backslash_count % 2 != 0 {
            // Quote is escaped — treat as real usage
            return false;
        }

        // Check if this is a comment line (starts with //, #, or /*)
        let line_start = find_line_start(haystack, abs);
        let line_prefix = &haystack[line_start..abs];
        if line_prefix.trim().starts_with("//")
            || line_prefix.trim().starts_with("///")
            || line_prefix.trim().starts_with("#")
            || line_prefix.trim().ends_with("/*")
        {
            // Inside a comment — skip, not real usage
            start = abs + needle.len();
            found_anywhere = true;
            continue;
        }

        // It's inside a string literal — skip, not real usage
        start = abs + needle.len();
        found_anywhere = true;
    }

    // If we never found the symbol outside strings/comments, return true
    found_anywhere
}

/// Find the position of the opening double-quote that encloses the character
/// at `pos`. Returns None if `pos` is not inside a string literal.
fn find_enclosing_string_start(haystack: &str, pos: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut string_start = 0usize;
    let mut i = 0usize;

    while i <= pos {
        if i >= haystack.len() {
            break;
        }
        let ch = haystack[i..].chars().next()?;
        let ch_len = ch.len_utf8();

        if in_string {
            if ch == '"' && i > string_start {
                // Check not escaped
                let before = &haystack[..i];
                let bs = before.chars().rev().take_while(|c| *c == '\\').count();
                if bs % 2 == 0 {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        in_string = false;
                    }
                }
            }
        } else if ch == '"' {
            // Check not escaped
            let before = &haystack[..i];
            let bs = before.chars().rev().take_while(|c| *c == '\\').count();
            if bs % 2 == 0 {
                in_string = true;
                string_start = i;
                depth += 1;
            }
        }

        if in_string && i == pos {
            return Some(string_start);
        }

        i += ch_len;
    }

    None
}

/// Find the start of the line containing position `pos`.
fn find_line_start(haystack: &str, pos: usize) -> usize {
    haystack[..pos]
        .rfind('\n')
        .map(|n| n + 1)
        .unwrap_or(0)
}

/// Iterate `lines`, invoking `is_header(trimmed_line)` to identify function
/// definitions and `body_extent(start, lines)` to compute the body end line
/// for that definition. Returns `[(start_line, end_line), ...]` of all ranges.
///
/// The two language-specific differences (Rust/JS brace-counting vs. Python
/// indent-based termination) live in the closures passed in.
fn collect_ranges<F, G>(
    lines: &[&str],
    is_header: F,
    body_extent: G,
) -> Vec<(LineNumber, LineNumber)>
where
    F: Fn(&str) -> bool,
    G: Fn(usize, &[&str]) -> usize,
{
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if is_header(lines[i].trim()) {
            let start = i + 1;
            let end = body_extent(i, lines);
            ranges.push((LineNumber::new(start as i64), LineNumber::new(end as i64)));
            i = end;
        }
        i += 1;
    }
    ranges
}

/// Brace-counting body extenter for Rust/JS-like brace-delimited languages.
fn brace_extent(start: usize, lines: &[&str]) -> usize {
    let mut depth = 0usize;
    let mut end = start + 1;
    for (idx, line) in lines.iter().enumerate().skip(start) {
        let t = line.trim();
        depth = depth.saturating_add(t.matches('{').count());
        depth = depth.saturating_sub(t.matches('}').count());
        end = idx + 1;
        if depth == 0 && t.contains('}') {
            break;
        }
    }
    end
}

/// Indent-based body extenter for Python. Returns the line *after* the
/// `def` block ends (the next non-empty, non-comment line at the same or
/// shallower indent).
fn indent_extent(start: usize, lines: &[&str]) -> usize {
    let mut end = start + 1;
    let indent = lines[start].len() - lines[start].trim_start().len();
    for (idx, line) in lines.iter().enumerate().skip(start + 1) {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            end = idx + 1;
            continue;
        }
        let line_indent = line.len() - line.trim_start().len();
        if line_indent <= indent && !t.is_empty() {
            break;
        }
        end = idx + 1;
    }
    end
}

fn rust_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| t.starts_with("fn _use_") || t.starts_with("fn dummy_"),
        brace_extent,
    )
}

fn python_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| t.starts_with("def _use_") || t.starts_with("def dummy_"),
        indent_extent,
    )
}

fn js_dummy_function_ranges(lines: &[&str]) -> Vec<(LineNumber, LineNumber)> {
    collect_ranges(
        lines,
        |t| {
            t.starts_with("function _use")
                || t.starts_with("function dummy")
                || t.starts_with("const _use")
                || t.starts_with("const dummy")
        },
        brace_extent,
    )
}

fn rust_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use ") || !trimmed.ends_with(';') {
            continue;
        }

        if trimmed == "use super::*;" {
            continue;
        }

        let body = trimmed
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim();

        if body.contains('{') {
            if let Some(open) = body.find('{') {
                if let Some(close) = body.rfind('}') {
                    let inside = &body[open + 1..close];
                    for part in inside.split(',') {
                        if let Some(symbol) = rust_imported_symbol_from_part(part.trim()) {
                            symbols
                                .push((SymbolName::new(symbol), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
            continue;
        }

        if let Some(symbol) = rust_imported_symbol_from_part(body) {
            symbols.push((SymbolName::new(symbol), LineNumber::new(idx as i64 + 1)));
        }
    }

    symbols
}

fn rust_imported_symbol_from_part(part: &str) -> Option<String> {
    let part = part.trim();
    if part.is_empty() || part == "self" || part.starts_with('*') {
        return None;
    }

    if let Some((_, alias)) = part.split_once(" as ") {
        return Some(alias.trim().to_string());
    }

    let name = match part.split("::").last() {
        Some(n) => n.trim(),
        None => part.trim(),
    };
    if name.is_empty() || name.contains('{') || name.contains('}') {
        return None;
    }

    Some(name.to_string())
}

pub fn python_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if let Some(import_part) = trimmed
            .strip_prefix("from ")
            .and_then(|s| s.split_once(" import ").map(|(_, p)| p))
        {
            for name in import_part.split(',') {
                let name = name.trim();
                if name.is_empty() || name == "*" {
                    continue;
                }

                let used_name = match name.split_once(" as ") {
                    Some((_, alias)) => alias.trim(),
                    None => name.split_whitespace().next().unwrap_or_default(),
                };

                if !used_name.is_empty() && used_name != "*" {
                    symbols.push((SymbolName::new(used_name), LineNumber::new(idx as i64 + 1)));
                }
            }
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("import ") {
            for module in rest.split(',') {
                let module = module.trim();
                if module.is_empty() {
                    continue;
                }

                let used_name = match module.split_once(" as ") {
                    Some((_, alias)) => alias.trim(),
                    None => module.rsplit('.').next().unwrap_or(module).trim(),
                };

                if !used_name.is_empty() && used_name != "*" {
                    symbols.push((SymbolName::new(used_name), LineNumber::new(idx as i64 + 1)));
                }
            }
        }
    }

    symbols
}

pub fn js_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }

                        let name = match part.split_once(" as ") {
                            Some((_, alias)) => alias.trim(),
                            None => part.split_whitespace().next().unwrap_or_default(),
                        };

                        if !name.is_empty() && name != "type" {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") && trimmed.contains(" from ") {
            if let Some(import_part) = trimmed.split_once("import ").map(|(_, p)| p) {
                let before_from = import_part
                    .split_once(" from ")
                    .map(|(n, _)| n)
                    .unwrap_or_default()
                    .trim();

                let name = match before_from.split_once(" as ") {
                    Some((_, alias)) => alias.trim(),
                    None => before_from,
                };

                if !name.is_empty() && name != "default" {
                    symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                }
            }
            continue;
        }

        if trimmed.starts_with("const ") && trimmed.contains("require(") && trimmed.contains('{') {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }

                        let name = match part.split_once(':') {
                            Some((_, alias)) => alias.trim(),
                            None => part,
                        };

                        if !name.is_empty() {
                            symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                        }
                    }
                }
            }
        }
    }

    symbols
}

fn in_dummy_range(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| line_no >= *start && line_no <= *end)
}

fn impl_trait_name(line: &str) -> Option<String> {
    let after_impl = line.strip_prefix("impl ")?.trim();
    let (trait_part, _) = after_impl.split_once(" for ")?;
    let trait_name = match trait_part.split("::").last() {
        Some(n) => n.trim(),
        None => trait_part.trim(),
    };
    if trait_name.is_empty() {
        return None;
    }
    Some(trait_name.to_string())
}

fn impl_block<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn trait_impl_is_dummy(lines: &[&str]) -> bool {
    let mut method_count = 0usize;
    let mut dummy_count = 0usize;
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("async fn ") {
            method_count += 1;
            let (end, body) = function_body(lines, i);
            if function_body_is_dummy(&body) {
                dummy_count += 1;
            }
            i = end;
        } else {
            i += 1;
        }
    }

    method_count > 0 && dummy_count == method_count
}

fn function_body<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn function_body_is_dummy(lines: &[&str]) -> bool {
    // Collect the body lines (skip the fn signature line at index 0)
    let body_lines: Vec<&str> = lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect();

    if body_lines.is_empty() {
        return true;
    }

    // Single-line body like `{ 42 }` or `{ return x; }` — not dummy
    if body_lines.len() == 1 {
        let single = body_lines[0];
        if single.starts_with('{') && single.ends_with('}') {
            let inner = &single[1..single.len() - 1].trim();
            return inner.is_empty() || is_short_marker(inner);
        }
        return is_short_marker(single);
    }

    // Multi-line body: join and check
    let body: String = body_lines.join(" ");
    let trimmed = body.trim();
    if trimmed == "{}" || trimmed == "{ }" {
        return true;
    }

    let inner = trimmed.trim_start_matches('{').trim_end_matches('}').trim();
    if inner.is_empty() || is_short_marker(inner) {
        return true;
    }

    false
}

pub fn is_short_marker(inner: &str) -> bool {
    inner.starts_with("todo!(")
        || inner.starts_with("unimplemented!(")
        || inner.starts_with("panic!(")
        || inner.starts_with("unreachable!(")
}
