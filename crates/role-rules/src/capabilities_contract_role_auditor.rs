use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use shared::role_rules::contract_role_protocol::IContractRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_source_vo::SourceContentVO;
use shared::common::utility_language_detector::detect_language_info_from_source;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ContractRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
// (No protocol implementation found in this file)

// ─── Block 3: Constructors, Helpers, Private Methods ──────

// (No protocol implementation found in this file)

//
// These pure functions implement the trait-method-signature parser used by
// `check_contract_primitive` above. They live at module level so they can be
// unit-tested directly without needing to construct a `ContractRoleChecker`
// or feed it a full `SourceContentVO`.

/// Extract `(line_no, raw_signature_line)` for every `fn name(...) -> ... ;`
/// declaration that lives inside a `pub trait Name { ... }` block.
///
/// Only Rust trait declarations are tracked. Free-standing `fn` definitions
/// (impl blocks, inherent impls, free functions) are intentionally ignored
/// because the AES402 rule applies to the contract layer (port / protocol
/// traits) — implementation details are an adapter concern.
pub fn extract_trait_method_signatures(content: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();
    let mut in_trait_depth: i32 = 0;
    let mut brace_depth: i32 = 0;

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let line = raw.trim();

        if in_trait_depth == 0 {
            // Detect a trait header line. We accept both `pub trait Foo` and
            // `pub trait Foo: Bar` (trait inheritance). We require a `{` on the
            // same line so we don't mistake `trait Foo;` (item declaration)
            // for a real trait body.
            let is_trait_header = (line.starts_with("pub trait ") || line.starts_with("trait "))
                && line.contains('{')
                && line.contains(')').ge(&line.contains('(')); // rough sanity
            if is_trait_header {
                in_trait_depth = 1;
                brace_depth = line.matches('{').count() as i32 - line.matches('}').count() as i32;
                continue;
            }
            continue;
        }

        // We're inside a trait body. Check for method declaration.
        // Heuristic: line starts with `fn ` (allowing leading whitespace) and
        // contains a `;` somewhere — that's the canonical Rust trait method
        // declaration form.
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
///
/// Python contract files (protocol/port) should use custom types/VOs instead of
/// primitive types (str, int, bool, float, list, dict) in method signatures.
pub fn extract_python_method_signatures(content: &str) -> Vec<(usize, String)> {
    let mut results = Vec::new();
    let mut in_class = false;
    let mut class_indent = 0;

    for (idx, raw) in content.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = raw.trim();

        // Detect class definition
        if trimmed.starts_with("class ") && trimmed.contains(':') {
            in_class = true;
            class_indent = raw.len() - raw.trim_start().len();
            continue;
        }

        if !in_class {
            continue;
        }

        // Detect method definition with type annotations
        let current_indent = raw.len() - raw.trim_start().len();
        if current_indent <= class_indent && !trimmed.is_empty() {
            // We've left the class
            in_class = false;
            continue;
        }

        if trimmed.starts_with("def ") && trimmed.contains("->") {
            // Has return type annotation — check for primitive types
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

    // Check parameter annotations
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
    // Note: bool is allowed per AES402 policy (semantic toggle)

    // Check return type
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

        // Detect interface or class definition
        if (trimmed.starts_with("export interface ")
            || trimmed.starts_with("interface ")
            || trimmed.starts_with("export class ")
            || trimmed.starts_with("class "))
            && trimmed.contains('{')
        {
            in_block = true;
            brace_depth = trimmed.matches('{').count() as i32 - trimmed.matches('}').count() as i32;
            // For single-line blocks, check the content between { and }
            if brace_depth == 0 {
                // Extract content between braces and check for primitives
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

            // Look for method declarations with type annotations
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

    // Check parameter annotations
    if lower.contains(": string") {
        forbidden.push("string");
    }
    if lower.contains(": number") {
        forbidden.push("number");
    }
    if lower.contains(": any") {
        forbidden.push("any");
    }

    // Check return type
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
/// type. Returns the list of forbidden type tokens found (used for the
/// violation message). Empty list means the signature is clean.
///
/// Policy (per AES402 + project conventions):
///   * `&str` is ALLOWED — borrowed string slice, idiomatic Rust for file
///     paths, error messages, and other borrowed string data passed into
///     trait methods. Borrow lifetimes preclude replacing it with a taxonomy
///     VO without major API churn.
///   * `bool` is ALLOWED — represents a semantic toggle that is not
///     meaningfully expressible as a VO without ceremony.
///   * `String` (owned) is FORBIDDEN — must be replaced with a taxonomy VO
///     such as `LintMessage`, `ErrorMessage`, `SymbolName`, or `JobId`.
///   * `Result<String, _>` and `Result<&str, _>` are FORBIDDEN — error
///     variants must use a defined `taxonomy_*_error` type.
///   * Numeric primitives (`i32`, `i64`, `u32`, `u64`, `f32`, `f64`,
///     `usize`, `isize`) are FORBIDDEN — must be wrapped in a domain VO
///     (`Count`, `LineNumber`, `ColumnNumber`, `Duration`).
///   * `char` is FORBIDDEN — must use a domain VO if a single character is
///     ever needed (rare).
pub fn signature_uses_forbidden_primitive(sig: &str) -> Vec<&'static str> {
    let mut forbidden: Vec<&'static str> = Vec::new();

    let line = sig.trim();

    // Return type — anything after `->` up to `{` or `;` or EOL.
    let ret_type: String = if let Some(arrow_idx) = line.find("->") {
        let after = &line[arrow_idx + 2..];
        // Trim at the first `;` (single-line sig) or `{` (rare, multiline).
        let end = match after.find(';').or_else(|| after.find('{')) {
            Some(idx) => idx,
            None => after.len(),
        };
        after[..end].trim().to_string()
    } else {
        String::new()
    };

    // Parameter list — inside the FIRST top-level `(` ... `)` on the line.
    let params_str: String = if let Some(open) = line.find('(') {
        // Match the closing paren at the same nesting level.
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

    // Concatenate param + return into one searchable string.
    let combined = format!("{} {}", params_str, ret_type);

    // Owned `String` (NOT preceded by `&`).
    // Negative lookbehind on `&` to avoid matching `&String` (which is rare but
    // we still want to flag — borrow lifetimes of `String` are themselves a
    // code smell because they usually mean a borrowed temporary).
    if regex_lite_match_whole_token(&combined, "String") {
        forbidden.push("String");
    }

    // Result<String, _> / Result<&str, _> / Result<String, ErrorCode> etc.
    if combined.contains("Result<String,") || combined.contains("Result<String >") {
        forbidden.push("Result<String, _>");
    }
    if combined.contains("Result<&str,") || combined.contains("Result<&str >") {
        forbidden.push("Result<&str, _>");
    }

    // Numeric primitives (and `usize`/`isize`).
    for kw in &["i32", "i64", "u32", "u64", "f32", "f64", "usize", "isize"] {
        if regex_lite_match_whole_token(&combined, kw) {
            forbidden.push(kw);
        }
    }

    // `char` — only single-token usage, not inside identifiers.
    if regex_lite_match_whole_token(&combined, "char") {
        forbidden.push("char");
    }

    forbidden
}

/// Lightweight whole-token match: returns true if `needle` appears in
/// `haystack` as a standalone identifier (not preceded or followed by
/// alphanumeric or `_`). Avoids pulling in the `regex` crate for a check this
/// small. Uses ASCII byte-level comparisons.
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

impl ContractRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_port(&self, source: &SourceContentVO) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
    }

    pub fn check_protocol(&self, source: &SourceContentVO) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.check_contract_primitive(source, &mut violations);
        violations
    }

    pub fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if def.role.forbidden_inheritance.values.is_empty() {
            return;
        }
        let content = source.content.value();
        let file = source.file_path.value();
        let mut forbidden_traits: Vec<String> = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            let is_import = t.starts_with("use ")
                || (t.starts_with("from ") && t.contains(" import "))
                || (t.starts_with("import ") && t.contains(" from "));
            if !is_import {
                continue;
            }
            for pattern in &def.role.forbidden_inheritance.values {
                let (layer, suffixes) = Self::resolve_scope(pattern);
                let lower = t.to_lowercase();
                let layer_match = lower.contains(&format!("{}::", layer))
                    || lower.contains(&format!("::{}::", layer))
                    || lower.contains(&format!("{}.", layer))
                    || lower.contains(&format!(".{}.", layer))
                    || lower.contains(&format!("{}/", layer))
                    || lower.contains(&format!("/{}/", layer));
                if !layer_match {
                    continue;
                }
                if !suffixes.is_empty()
                    && !suffixes.iter().any(|s| {
                        lower.contains(&format!("_{}", s)) || lower.contains(&format!("::{}", s))
                    })
                {
                    continue;
                }
                if let Some(name) = t.split("::").last() {
                    let tn = match name
                        .trim_end_matches(';')
                        .trim()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .split(',')
                        .next()
                    {
                        Some(s) => s.trim().to_string(),
                        None => String::new(),
                    };
                    if !tn.is_empty() {
                        forbidden_traits.push(tn);
                    }
                }
            }
        }
        for trait_name in &forbidden_traits {
            let rust_pattern = format!("impl {} for ", trait_name);
            let py_pattern = format!("({}", trait_name);
            let js_extends = format!("extends {}", trait_name);
            let js_implements = format!("implements {}", trait_name);
            if content.contains(&rust_pattern)
                || content.contains(&py_pattern)
                || content.contains(&js_extends)
                || content.contains(&js_implements)
            {
                let msg = aes013_forbidden_inheritance(trait_name);
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES013",
                    Severity::HIGH,
                    &msg,
                ));
            }
        }
    }

    /// Detect primitive type usage in contract method signatures (AES402).
    ///
    /// Scans ONLY method signatures inside `pub trait Name { ... }` blocks, NOT
    /// the whole file. This prevents false positives from:
    ///   * doc-comments mentioning "String" or "str" in prose
    ///   * identifier names that contain primitive type names
    ///     (e.g. `StringBuilder`, `MyFloat`)
    ///   * language words in English comments ("Float values are rounded")
    ///
    /// Rules:
    ///   * `&str` (borrowed string slice) is allowed — borrow lifetimes preclude
    ///     replacement with a taxonomy VO without major API changes. It is the
    ///     idiomatic Rust type for file paths, error messages, and other borrowed
    ///     string data passed into trait methods.
    ///   * `bool` is allowed — represents a semantic toggle that is not meaningfully
    ///     expressible as a VO without ceremony.
    ///   * `String` (owned) is FORBIDDEN — must be replaced with a taxonomy VO
    ///     (`LintMessage`, `ErrorMessage`, `SymbolName`, `JobId`, etc.).
    ///   * `Result<String, _>` / `Result<&str, _>` are FORBIDDEN — error variants
    ///     must use a defined `taxonomy_*_error` type, not a raw `String`.
    ///   * Numeric primitives `i32/i64/u32/u64/f32/f64` and `char` are FORBIDDEN —
    ///     must be wrapped in domain VOs (`Count`, `LineNumber`, `ColumnNumber`,
    ///     `Duration`, etc.) or new domain-specific VOs.
    ///
    /// Only the parameter types and return type of each trait method signature
    /// are inspected — implementation bodies are out of scope (the contract
    /// layer is the public interface; internal representations are an adapter
    /// concern).
    fn check_contract_primitive(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let file = source.file_path.value();
        let content = source.content.value();
        let li = detect_language_info_from_source(source);
        let is_rs = li.is_rs;
        let is_py = li.is_py;
        let is_js = li.is_js;
        if !is_rs && !is_py && !is_js {
            return;
        }

        let lang = if is_rs {
            Language::Rust
        } else if is_py {
            Language::Python
        } else {
            Language::JavaScript
        };

        if is_py {
            // Python-specific: look for class methods with primitive type annotations
            for (line_no, sig) in extract_python_method_signatures(content) {
                let forbidden = python_signature_uses_forbidden_primitive(&sig);
                if forbidden.is_empty() {
                    continue;
                }
                let msg = AesRoleViolation::ContractPrimitive { reason: None }
                    .with_language(lang)
                    .to_string();
                violations.push(LintResult::new_arch(
                    file,
                    line_no,
                    "AES402",
                    Severity::HIGH,
                    msg,
                ));
            }
            return;
        }

        if is_js {
            // JS/TS-specific: look for interface/class methods with primitive type annotations
            for (line_no, sig) in extract_typescript_method_signatures(content) {
                let forbidden = typescript_signature_uses_forbidden_primitive(&sig);
                if forbidden.is_empty() {
                    continue;
                }
                let msg = AesRoleViolation::ContractPrimitive { reason: None }
                    .with_language(lang)
                    .to_string();
                violations.push(LintResult::new_arch(
                    file,
                    line_no,
                    "AES402",
                    Severity::HIGH,
                    msg,
                ));
            }
            return;
        }

        // Rust: use trait-method-signature parser
        for (line_no, sig) in extract_trait_method_signatures(content) {
            let forbidden = signature_uses_forbidden_primitive(&sig);
            if forbidden.is_empty() {
                continue;
            }
            let msg = AesRoleViolation::ContractPrimitive { reason: None }
                .with_language(lang)
                .to_string();
            violations.push(LintResult::new_arch(
                file,
                line_no,
                "AES402",
                Severity::HIGH,
                msg,
            ));
        }
    }

    fn resolve_scope(scope: &str) -> (&str, Vec<&str>) {
        if let Some(paren) = scope.find('(') {
            let layer = scope[..paren].trim();
            let inner = scope[paren + 1..].trim_end_matches(')').trim();
            let suffixes: Vec<&str> = if inner.contains('|') {
                inner
                    .split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                inner
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            };
            (layer, suffixes)
        } else {
            (scope.trim(), vec![])
        }
    }
}

impl IContractRoleChecker for ContractRoleChecker {
    fn check_port(
        &self,
        source: &SourceContentVO,
    ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        self.check_port(source)
    }
    fn check_protocol(
        &self,
        source: &SourceContentVO,
    ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        self.check_protocol(source)
    }
    fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &shared::taxonomy_definition_vo::LayerDefinition,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_aggregate(source, def, violations);
    }
}

// (No protocol implementation found in this file)

// PURPOSE: ContractRoleChecker — IContractRoleChecker for AES402: contract primitive type audits
//
// ALGORITHM:
//   1. check_aggregate — Scans import lines for blocked trait patterns (layer + suffix)
//      defined in LayerDefinition.role.forbidden_inheritance. Flags any `impl Trait for X`
//      or equivalent that uses a disallowed trait by name.
//   2. scan_contract_primitive (port/protocol dispatch) — Detects primitive type employment
//      in contract interfaces (port/protocol files). Uses LanguageDetector to determine
//      language, then checks for known primitive keywords per language.
//
// NOTE: check_contract_primitive is called for all files (not just test projects)
//      since AES402 applies universally — removed test-project guard per DX audit.
use shared::cli_commands::taxonomy_result_vo::LintResult;

fn aes013_forbidden_inheritance(trait_name: &str) -> String {
    format!(
        "AES013 FORBIDDEN_INHERITANCE: '{}' implemented from forbidden source.\n\
         WHY? Contracts must not inherit from forbidden source layers.\n\
         FIX: Remove the inheritance or use a valid contract port/protocol instead.",
        trait_name
    )
}

impl Default for ContractRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}
