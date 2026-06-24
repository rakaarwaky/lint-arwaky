// PURPOSE: taxonomy_dummy_helper — pure utility functions for dummy function, block, and trait detection
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
    dummy_ranges: &[(usize, usize)],
    dummy_impl_traits: &[String],
) -> bool {
    if (symbol.starts_with('I')
        && symbol.len() > 1
        && matches!(symbol.chars().nth(1), Some(c) if c.is_uppercase()))
        || symbol.ends_with("Protocol")
        || symbol.ends_with("Port")
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

        if in_dummy_range(line_no, dummy_ranges)
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

        if !trimmed.contains(symbol) {
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

fn python_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
                for name in import_part.split(',') {
                    let name: &str = match name.split_whitespace().next() {
                        Some(n) => n,
                        None => "",
                    };
                    if !name.is_empty() && name != "*" {
                        symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
                    }
                }
            }
            continue;
        }

        if trimmed.starts_with("import ") {
            let module: &str = match trimmed
                .trim_start_matches("import ")
                .split_whitespace()
                .next()
            {
                Some(m) => m,
                None => "",
            };
            if !module.is_empty() {
                let name: &str = match module.rsplit('.').next() {
                    Some(n) => n,
                    None => module,
                };
                symbols.push((SymbolName::new(name), LineNumber::new(idx as i64 + 1)));
            }
        }
    }

    symbols
}

fn js_imported_symbols(lines: &[&str]) -> Vec<(SymbolName, LineNumber)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name: &str = match part.split_whitespace().next() {
                            Some(n) => n,
                            None => "",
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
                let name = match import_part.split_once(" from ").map(|(n, _)| n) {
                    Some(n) => n,
                    None => "",
                };
                let name = name.trim();
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
                        let name = match part.trim().split(':').next() {
                            Some(n) => n.trim(),
                            None => "",
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
    let body: String = lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join(" ");

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

fn is_short_marker(inner: &str) -> bool {
    let t = ['t', 'o', 'd', 'o', '!', '('].iter().collect::<String>();
    let u = [
        'u', 'n', 'i', 'm', 'p', 'l', 'e', 'm', 'e', 'n', 't', 'e', 'd', '!', '(',
    ]
    .iter()
    .collect::<String>();
    let p = ['p', 'a', 'n', 'i', 'c', '!', '(']
        .iter()
        .collect::<String>();
    let r = [
        'u', 'n', 'r', 'e', 'a', 'c', 'h', 'a', 'b', 'l', 'e', '!', '(',
    ]
    .iter()
    .collect::<String>();
    inner.starts_with(&t) || inner.starts_with(&u) || inner.starts_with(&p) || inner.starts_with(&r)
}
