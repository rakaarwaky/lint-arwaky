// PURPOSE: Signature parsing — pure, stateless taxonomy utility.
//
// Extracts method/signature parsing logic from capabilities layer files so
// each capability file stays lean and the shared utility can be reused by
// other modules without pulling in struct definitions or trait impls.

/// Extract `(line_no, raw_signature_line)` for every `fn name(...) -> ... ;`
/// declaration that lives inside a `pub trait Name { ... }` block.
///
/// Only Rust trait declarations are tracked. Free-standing `fn` definitions
/// (impl blocks, inherent impls, free functions) are intentionally ignored
/// because the AES402 rule applies to the contract layer (protocol
/// traits) — implementation details are an adapter concern.
pub fn extract_trait_method_signatures(content: &str) -> Vec<(usize, String)> {
    let trait_starts = find_trait_brace_opening_lines(content);
    let brace_bounds = get_brace_bounds(&trait_starts, content);
    extract_signatures_from_bounds(brace_bounds, content)
}

/// Line numbers (1-based) where a Rust trait block opens on that line.
fn find_trait_brace_opening_lines(content: &str) -> Vec<usize> {
    content
        .lines()
        .enumerate()
        .filter_map(|(idx, raw)| {
            let line = raw.trim();
            if is_trait_header(line) {
                Some(idx + 1)
            } else {
                None
            }
        })
        .collect()
}

/// Get the brace boundaries for each trait start line.
fn get_brace_bounds(trait_starts: &[usize], content: &str) -> Vec<(usize, usize)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut bounds = Vec::new();

    for &start_line in trait_starts {
        if let Some((_, close)) = find_matching_brace_for_start(&lines, start_line) {
            bounds.push((start_line, close));
        }
    }

    bounds
}

/// Find the matching closing brace for a given opening brace line.
fn find_matching_brace_for_start(lines: &[&str], open_line_idx: usize) -> Option<(usize, usize)> {
    let mut depth = 0i32;
    for (idx, line) in lines.iter().enumerate().skip(open_line_idx - 1) {
        depth = advance_brace_depth(depth, line)?;
        if depth == 0 {
            return Some((open_line_idx, idx + 1));
        }
    }
    None
}

/// Apply one line's `{`/`}` delta to *depth*, returning the new depth, or
/// `None` if the line closes the current block (depth would go below zero).
fn advance_brace_depth(depth: i32, line: &str) -> Option<i32> {
    let mut next = depth;
    for ch in line.chars() {
        next = apply_brace_char(next, ch)?;
    }
    Some(next)
}

/// Apply a single `{`/`}` to *depth*. Returns `None` when a `}` would
/// close a block that is not open.
fn apply_brace_char(depth: i32, ch: char) -> Option<i32> {
    match ch {
        '{' => Some(depth + 1),
        '}' if depth > 0 => Some(depth - 1),
        '}' => None,
        _ => Some(depth),
    }
}

/// Extract `fn name(...);` signatures that live within brace boundaries.
fn extract_signatures_from_bounds(
    bounds: Vec<(usize, usize)>,
    content: &str,
) -> Vec<(usize, String)> {
    let lines: Vec<&str> = content.lines().collect();
    let mut results = Vec::new();

    for (start_line, end_line) in bounds {
        for (idx, raw) in lines.iter().enumerate().take(end_line).skip(start_line) {
            let line = raw.trim();
            if line.starts_with("fn ") && line.contains(';') {
                results.push((idx + 1, raw.to_string()));
            }
        }
    }

    results
}

/// True when `line` opens a Rust trait block on a single line.
fn is_trait_header(line: &str) -> bool {
    (line.starts_with("pub trait ") || line.starts_with("trait "))
        && line.contains('{')
        && line.contains(')').ge(&line.contains('('))
}

/// Extract `(line_no, raw_signature_line)` for every `def method_name(self, ...)` declaration
/// inside a Python class that has type annotations using primitive types.
pub fn extract_python_method_signatures(content: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();
    let mut in_class = false;
    let mut class_indent = 0;

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = raw.trim();

        if is_class_header(trimmed) {
            in_class = true;
            class_indent = raw.len() - raw.trim_start().len();
            continue;
        }

        if !in_class {
            continue;
        }

        if leaves_class_body(raw, class_indent, trimmed) {
            in_class = false;
            continue;
        }

        if is_python_signature_line(trimmed) {
            results.push((line_no, raw.to_string()));
        }
    }

    results
}

/// True when `line` opens a Python class definition.
fn is_class_header(line: &str) -> bool {
    line.starts_with("class ") && line.contains(':')
}

/// True when a line inside a Python class is a `def` with a primitive
/// annotation in its signature.
fn is_python_signature_line(trimmed: &str) -> bool {
    trimmed.starts_with("def ")
        && trimmed.contains("->")
        && python_line_has_primitive(trimmed)
}

/// True when an indented line exits the class body (dedent below the class header).
fn leaves_class_body(raw: &str, class_indent: usize, trimmed: &str) -> bool {
    let current_indent = raw.len() - raw.trim_start().len();
    current_indent <= class_indent && !trimmed.is_empty()
}

/// True when a Python `def` line annotates a parameter or return with a primitive type.
fn python_line_has_primitive(line: &str) -> bool {
    let lower = line.to_lowercase();
    const PARAM_PATTERNS: &[&str] = &[": str", ": int", ": bool", ": float", ": list", ": dict"];
    const RETURN_PATTERNS: &[&str] = &[
        "-> str", "-> int", "-> bool", "-> float", "-> list", "-> dict",
    ];
    PARAM_PATTERNS.iter().any(|p| lower.contains(p))
        || RETURN_PATTERNS.iter().any(|p| lower.contains(p))
}

/// Check if a Python method signature uses forbidden primitive types.
pub fn python_signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let mut forbidden: Vec<&'static str> = Vec::new();
    let lower = sig.to_lowercase();

    collect_python_param_primitives(&lower, &mut forbidden);
    collect_python_return_primitives(&lower, &mut forbidden);

    forbidden.sort();
    forbidden.dedup();
    forbidden
}

/// Collect forbidden primitive type tokens from the parameter section.
fn collect_python_param_primitives(lower: &str, forbidden: &mut Vec<&'static str>) {
    if lower.contains(": str") {
        forbidden.push("str");
    }
    if lower.contains(": int") {
        forbidden.push("int");
    }
    if lower.contains(": float") {
        forbidden.push("float");
    }
    // Only flag bare `list`/`dict` without type parameters (e.g., `List[ResultVO]` is OK)
    if lower.contains(": list") && !lower.contains(": list[") {
        forbidden.push("list");
    }
    if lower.contains(": dict") && !lower.contains(": dict[") {
        forbidden.push("dict");
    }
}

/// Collect forbidden primitive type tokens from the return section (after `->`).
fn collect_python_return_primitives(lower: &str, forbidden: &mut Vec<&'static str>) {
    let Some(arrow_idx) = lower.find("->") else {
        return;
    };
    let ret = lower[arrow_idx + 2..].trim();
    if ret.starts_with("str") {
        forbidden.push("str");
    }
    if ret.starts_with("int") {
        forbidden.push("int");
    }
    if ret.starts_with("float") {
        forbidden.push("float");
    }
    // Only flag bare `list`/`dict` without type parameters
    if ret.starts_with("list") && !ret.starts_with("list[") {
        forbidden.push("list");
    }
    if ret.starts_with("dict") && !ret.starts_with("dict[") {
        forbidden.push("dict");
    }
}

/// Extract `(line_no, raw_signature_line)` for every method declaration inside a TypeScript
/// `interface` or `class` that uses primitive types in parameter/return annotations.
pub fn extract_typescript_method_signatures(content: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();
    let mut in_block = false;
    let mut brace_depth = 0;

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = raw.trim();

        if is_ts_block_header(trimmed) {
            let opened = trimmed.matches('{').count() as i32 - trimmed.matches('}').count() as i32;
            if opened == 0 {
                // Single-line header with an inline body — evaluate and move on.
                push_inline_ts_signature(&mut results, line_no, raw, trimmed);
            } else {
                in_block = true;
                brace_depth = opened;
            }
            continue;
        }

        if in_block {
            brace_depth +=
                trimmed.matches('{').count() as i32 - trimmed.matches('}').count() as i32;
            if brace_depth <= 0 {
                in_block = false;
                brace_depth = 0;
                continue;
            }

            if is_ts_signature_line(trimmed) {
                results.push((line_no, raw.to_string()));
            }
        }
    }

    results
}

/// True when `line` opens a TypeScript interface/class block.
fn is_ts_block_header(line: &str) -> bool {
    (line.starts_with("export interface ")
        || line.starts_with("interface ")
        || line.starts_with("export class ")
        || line.starts_with("class "))
        && line.contains('{')
}

/// True when a line inside a TypeScript block looks like a method signature
/// with primitive annotations.
fn is_ts_signature_line(trimmed: &str) -> bool {
    trimmed.contains('(') && trimmed.contains(':') && ts_line_has_primitive(trimmed)
}

/// Push a one-line TypeScript block whose inline body uses primitives.
fn push_inline_ts_signature(
    results: &mut Vec<(usize, String)>,
    line_no: usize,
    raw: &str,
    trimmed: &str,
) {
    if let Some((open, close)) = brace_pair(trimmed)
        && ts_inline_has_primitive(&trimmed[open + 1..close])
    {
        results.push((line_no, raw.to_string()));
    }
}

/// Locate the matching `{ ... }` pair on a single line, if present.
fn brace_pair(line: &str) -> Option<(usize, usize)> {
    let open = line.find('{')?;
    let close = line.rfind('}')?;
    Some((open, close))
}

/// True when the inline body of a one-line block uses primitive annotations.
fn ts_inline_has_primitive(inner: &str) -> bool {
    inner.contains('(') && inner.contains(':') && ts_line_has_primitive(inner)
}

/// True when a TypeScript signature line uses primitive parameter/return annotations.
fn ts_line_has_primitive(line: &str) -> bool {
    let lower = line.to_lowercase();
    const PRIMITIVES: &[&str] = &["string", "number", "any"];
    PRIMITIVES.iter().any(|p| {
        let with_colon = format!(": {p}");
        let with_paren = format!("): {p}");
        let arr_param = format!(": {p}[]");
        let arr_return = format!("): {p}[]");
        lower.contains(&with_colon)
            || lower.contains(&with_paren)
            || lower.contains(&arr_param)
            || lower.contains(&arr_return)
    })
}

/// Check if a TypeScript method signature uses forbidden primitive types.
pub fn typescript_signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let mut forbidden: Vec<&'static str> = Vec::new();
    let lower = sig.to_lowercase();

    if lower.contains(": string") {
        forbidden.push("string");
    }
    if lower.contains(": number") {
        forbidden.push("number");
    }
    if lower.contains(": any") {
        forbidden.push("any");
    }

    collect_typescript_return_primitives(&lower, &mut forbidden);

    forbidden.sort();
    forbidden.dedup();
    forbidden
}

/// Collect forbidden primitive type tokens from the TypeScript return section.
fn collect_typescript_return_primitives(lower: &str, forbidden: &mut Vec<&'static str>) {
    let Some(paren_idx) = lower.rfind(')') else {
        return;
    };
    let after = lower[paren_idx + 1..].trim();
    if after.starts_with(": string") {
        forbidden.push("string");
    }
    if after.starts_with(": number") {
        forbidden.push("number");
    }
    if after.starts_with(": any") {
        forbidden.push("any");
    }
}

/// Decide whether a single Rust method signature uses a forbidden primitive
/// type. Returns the list of forbidden type tokens found.
pub fn signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let mut forbidden: Vec<&'static str> = Vec::new();
    let line = sig.trim();

    let ret_type = rust_return_type(line);
    let params_str = rust_param_list(line);
    let combined = format!("{} {}", params_str, ret_type);

    // ── String exemption: allowed in collection/error/optional contexts ──
    // HashMap<String, ...>, Vec<String>, &[String], Option<String>, Result<T, String>
    // are all valid uses of String at contract boundaries.
    if regex_lite_match_whole_token(&combined, "String") && !is_string_in_valid_context(&combined) {
        forbidden.push("String");
    }

    if combined.contains("Result<String,") || combined.contains("Result<String >") {
        forbidden.push("Result<String, _>");
    }
    if combined.contains("Result<&str,") || combined.contains("Result<&str >") {
        forbidden.push("Result<&str, _>");
    }

    collect_rust_numeric_primitives(&combined, &mut forbidden);
    collect_rust_char_primitive(&combined, &mut forbidden);

    forbidden
}

/// Extract the return type portion of a Rust signature (text after `->`).
fn rust_return_type(line: &str) -> String {
    let Some(arrow_idx) = line.find("->") else {
        return String::new();
    };
    let after = &line[arrow_idx + 2..];
    let end = match after.find(';').or_else(|| after.find('{')) {
        Some(idx) => idx,
        None => after.len(),
    };
    after[..end].trim().to_string()
}

/// Extract the parameter list text between the first balanced `(...)` pair.
fn rust_param_list(line: &str) -> String {
    let Some(open) = line.find('(') else {
        return String::new();
    };
    match find_matching_paren(line, open) {
        Some(close) => line[open + 1..close].to_string(),
        None => String::new(),
    }
}

/// Index of the `)` that balances the `(` at *open*.
fn find_matching_paren(line: &str, open: usize) -> Option<usize> {
    let mut depth = 0i32;
    for (i, b) in line.as_bytes().iter().enumerate().skip(open) {
        depth = apply_paren_char(depth, *b)?;
        if depth == 0 {
            return Some(i);
        }
    }
    None
}

/// Apply a single `(`/`)` to *depth*. Returns `None` when a `)` closes a
/// paren that is not open.
fn apply_paren_char(depth: i32, b: u8) -> Option<i32> {
    match b {
        b'(' => Some(depth + 1),
        b')' if depth > 0 => Some(depth - 1),
        b')' => None,
        _ => Some(depth),
    }
}

/// Collect forbidden numeric primitive tokens from a Rust signature.
fn collect_rust_numeric_primitives(combined: &str, forbidden: &mut Vec<&'static str>) {
    for kw in &["i32", "i64", "u32", "u64", "f32", "f64", "usize", "isize"] {
        if regex_lite_match_whole_token(combined, kw) {
            forbidden.push(kw);
        }
    }
}

/// Collect the forbidden `char` primitive token when present.
fn collect_rust_char_primitive(combined: &str, forbidden: &mut Vec<&'static str>) {
    if regex_lite_match_whole_token(combined, "char") {
        forbidden.push("char");
    }
}

/// Lightweight whole-token match: returns true if `needle` appears in
/// `haystack` as a standalone identifier.
fn regex_lite_match_whole_token(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }
    let h = haystack.as_bytes();
    let n = needle.as_bytes();
    let nlen = n.len();
    if h.len() < nlen {
        return false;
    }
    let is_ident_cont = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    (0..=h.len() - nlen).any(|i| {
        &h[i..i + nlen] == n && is_token_boundary(h, i, nlen, is_ident_cont)
    })
}

/// True when the byte range `[i, i+nlen)` is a standalone token — the
/// characters just before and after it are not identifier characters.
fn is_token_boundary(h: &[u8], i: usize, nlen: usize, is_ident_cont: impl Fn(u8) -> bool) -> bool {
    let before_ok = i == 0 || !is_ident_cont(h[i - 1]);
    let after_ok = i + nlen == h.len() || !is_ident_cont(h[i + nlen]);
    before_ok && after_ok
}

/// Check if `String` appears in a valid contract context:
/// Any `String` inside generic brackets (`<>`), slices (`&[]`), or tuples
/// is a collection element or type parameter — not a standalone domain type.
/// Only bare `String` as a top-level parameter or return type is forbidden.
fn is_string_in_valid_context(sig: &str) -> bool {
    string_in_collection(sig) || string_in_wrapper(sig) || string_in_tuple(sig)
}

/// True when `String` appears inside a collection type (HashMap, Vec, slice).
fn string_in_collection(sig: &str) -> bool {
    sig.contains("HashMap<String,")
        || sig.contains("HashMap < String,")
        || sig.contains("Vec<String>")
        || sig.contains("Vec < String >")
        // Vec<(PathBuf, String)> or similar tuple-in-Vec patterns
        || (sig.contains("Vec<(") && sig.contains("String)"))
        || sig.contains("&[String]")
        || sig.contains("& [ String ]")
}

/// True when `String` appears inside a wrapper generic type (Option, Result).
fn string_in_wrapper(sig: &str) -> bool {
    sig.contains("Option<String>")
        || sig.contains("Option < String >")
        || sig.contains("Result<String,")
        || sig.contains("Result < String ,")
        || sig.contains(", String>")
        || sig.contains(", String >")
}

/// True when `String` appears inside a tuple type.
fn string_in_tuple(sig: &str) -> bool {
    sig.contains("(String") || sig.contains(", String,") || sig.contains(", String)")
}
