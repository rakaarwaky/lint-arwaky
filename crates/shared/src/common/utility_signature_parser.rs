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
    let mut results = Vec::new();
    let mut in_trait_depth: i32 = 0;
    let mut brace_depth: i32 = 0;

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw.trim();

        if in_trait_depth == 0 {
            let is_trait_header = (line.starts_with("pub trait ") || line.starts_with("trait "))
                && line.contains('{')
                && line.contains(')').ge(&line.contains('('));
            if is_trait_header {
                in_trait_depth = 1;
                brace_depth = line.matches('{').count() as i32 - line.matches('}').count() as i32;
                continue;
            }
            continue;
        }

        if line.starts_with("fn ") && line.contains(';') {
            results.push((line_no, raw.to_string()));
        }

        brace_depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
        if brace_depth <= 0 {
            in_trait_depth = 0;
            brace_depth = 0;
        }
    }

    results
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

        if trimmed.starts_with("class ") && trimmed.contains(':') {
            in_class = true;
            class_indent = raw.len() - raw.trim_start().len();
            continue;
        }

        if !in_class {
            continue;
        }

        let current_indent = raw.len() - raw.trim_start().len();
        if current_indent <= class_indent && !trimmed.is_empty() {
            in_class = false;
            continue;
        }

        if trimmed.starts_with("def ") && trimmed.contains("->") {
            let lower = trimmed.to_lowercase();
            let has_primitive = lower.contains(": str")
                || lower.contains(": int")
                || lower.contains(": bool")
                || lower.contains(": float")
                || lower.contains(": list")
                || lower.contains(": dict")
                || lower.contains("-> str")
                || lower.contains("-> int")
                || lower.contains("-> bool")
                || lower.contains("-> float")
                || lower.contains("-> list")
                || lower.contains("-> dict");
            if has_primitive {
                results.push((line_no, raw.to_string()));
            }
        }
    }

    results
}

/// Check if a Python method signature uses forbidden primitive types.
pub fn python_signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let mut forbidden: Vec<&'static str> = Vec::new();
    let lower = sig.to_lowercase();

    if lower.contains(": str") {
        forbidden.push("str");
    }
    if lower.contains(": int") {
        forbidden.push("int");
    }
    if lower.contains(": float") {
        forbidden.push("float");
    }
    if lower.contains(": list") {
        forbidden.push("list");
    }
    if lower.contains(": dict") {
        forbidden.push("dict");
    }

    if let Some(arrow_idx) = lower.find("->") {
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
        if ret.starts_with("list") {
            forbidden.push("list");
        }
        if ret.starts_with("dict") {
            forbidden.push("dict");
        }
    }

    forbidden.sort();
    forbidden.dedup();
    forbidden
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

        if (trimmed.starts_with("export interface ")
            || trimmed.starts_with("interface ")
            || trimmed.starts_with("export class ")
            || trimmed.starts_with("class "))
            && trimmed.contains('{')
        {
            in_block = true;
            brace_depth = trimmed.matches('{').count() as i32 - trimmed.matches('}').count() as i32;
            if brace_depth == 0 {
                if let Some(open) = trimmed.find('{') {
                    if let Some(close) = trimmed.rfind('}') {
                        let inner = &trimmed[open + 1..close];
                        if inner.contains('(') && inner.contains(':') {
                            let lower = inner.to_lowercase();
                            let has_primitive = lower.contains(": string")
                                || lower.contains(": number")
                                || lower.contains(": any")
                                || lower.contains(": string[]")
                                || lower.contains(": number[]")
                                || lower.contains("): string")
                                || lower.contains("): number")
                                || lower.contains("): any")
                                || lower.contains("): string[]")
                                || lower.contains("): number[]");
                            if has_primitive {
                                results.push((line_no, raw.to_string()));
                            }
                        }
                    }
                }
                in_block = false;
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

            if trimmed.contains('(') && trimmed.contains(':') {
                let lower = trimmed.to_lowercase();
                let has_primitive = lower.contains(": string")
                    || lower.contains(": number")
                    || lower.contains(": any")
                    || lower.contains(": string[]")
                    || lower.contains(": number[]")
                    || lower.contains("): string")
                    || lower.contains("): number")
                    || lower.contains("): any")
                    || lower.contains("): string[]")
                    || lower.contains("): number[]");
                if has_primitive {
                    results.push((line_no, raw.to_string()));
                }
            }
        }
    }

    results
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

    if let Some(paren_idx) = lower.rfind(')') {
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

    forbidden.sort();
    forbidden.dedup();
    forbidden
}

/// Decide whether a single Rust method signature uses a forbidden primitive
/// type. Returns the list of forbidden type tokens found.
pub fn signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let mut forbidden: Vec<&'static str> = Vec::new();
    let line = sig.trim();

    let ret_type: String = if let Some(arrow_idx) = line.find("->") {
        let after = &line[arrow_idx + 2..];
        let end = match after.find(';').or_else(|| after.find('{')) {
            Some(idx) => idx,
            None => after.len(),
        };
        after[..end].trim().to_string()
    } else {
        String::new()
    };

    let params_str: String = if let Some(open) = line.find('(') {
        let bytes = line.as_bytes();
        let mut depth = 0i32;
        let mut close_idx = None;
        for (i, &b) in bytes.iter().enumerate().skip(open) {
            match b {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        close_idx = Some(i);
                        break;
                    }
                }
                _ => {}
            }
        }
        if let Some(close) = close_idx {
            line[open + 1..close].to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let combined = format!("{} {}", params_str, ret_type);

    if regex_lite_match_whole_token(&combined, "String") {
        forbidden.push("String");
    }

    if combined.contains("Result<String,") || combined.contains("Result<String >") {
        forbidden.push("Result<String, _>");
    }
    if combined.contains("Result<&str,") || combined.contains("Result<&str >") {
        forbidden.push("Result<&str, _>");
    }

    for kw in &["i32", "i64", "u32", "u64", "f32", "f64", "usize", "isize"] {
        if regex_lite_match_whole_token(&combined, kw) {
            forbidden.push(kw);
        }
    }

    if regex_lite_match_whole_token(&combined, "char") {
        forbidden.push("char");
    }

    forbidden
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
    let mut i = 0;
    while i + nlen <= h.len() {
        if &h[i..i + nlen] == n {
            let before_ok = i == 0 || !is_ident_cont(h[i - 1]);
            let after_ok = i + nlen == h.len() || !is_ident_cont(h[i + nlen]);
            if before_ok && after_ok {
                return true;
            }
        }
        i += 1;
    }
    false
}
