use once_cell::sync::Lazy;
use regex::Regex;
use shared::cli_commands::LintResult;

use shared::common::SourceContentVO;
use shared::common::utility_language_detector::detect_language_info_from_source;
use shared::common::{LintMessage, Severity};
use shared::role_rules::AesRoleViolation;
use shared::role_rules::ISurfaceRoleChecker;

// PURPOSE: SurfaceRoleChecker — ISurfaceRoleChecker for AES406: smart/utility/passive surface role checks
//
// ALGORITHM:
//   1. check_fn_count_limit — Counts fn/def/function occurrences. If > 15, flags SurfaceRoleViolation.
//   2. check_smart_surface — no-op (smart surfaces exempt from hierarchy/nesting/domain checks).
//   3. check_utility_surface — Checks hierarchy (max methods), method length, nesting depth, domain logic.
//   4. check_passive_surface — Same checks as utility surface.
//
// All thresholds are configurable via LayerDefinition (from YAML config).

const MAX_PUBLIC_METHODS: usize = 10;
const MAX_FUNCTION_BODY_LINES: i64 = 80;
const MAX_IF_DEPTH: usize = 3;
const MAX_CONTROL_FLOW: usize = 3;

// Regex patterns
static PY_CLASS_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^class\s+(\w+)").ok());
static PY_METHOD_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^(?:async\s+)?def\s+(\w+)\s*\(").ok());
static JS_CLASS_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^export\s+class\s+(\w+)").ok());
static JS_METHOD_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:public|private|protected)?\s*(?:async\s+)?(\w+)\s*\(").ok());
static IF_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^\s*if\s+").ok());
static RUST_IMPL_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:pub\s+)?(?:unsafe\s+)?impl\s+").ok());
static RUST_FN_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(").ok());

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SurfaceRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ISurfaceRoleChecker for SurfaceRoleChecker {
    fn check_smart_surface(&self, _source: &SourceContentVO, _violations: &mut Vec<LintResult>) {
        // Smart surfaces are exempt from hierarchy/nesting/domain checks
    }

    fn check_utility_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        self._check_surface_impl(source, violations);
    }

    fn check_passive_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        self._check_surface_impl(source, violations);
    }

    fn check_fn_count_limit(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let content = source.content.value();
        let file = source.file_path.value();
        let li = detect_language_info_from_source(source);
        let fn_keyword = if li.is_py {
            "def "
        } else if li.is_js {
            "function "
        } else {
            "fn "
        };
        let mut count = 0;
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with("//")
                && !trimmed.starts_with('#')
                && trimmed.contains(fn_keyword)
            {
                count += 1;
                if count > 15 {
                    violations.push(LintResult::new_arch(
                        file,
                        0,
                        "AES406",
                        Severity::HIGH,
                        AesRoleViolation::SurfaceRoleViolation {
                            reason: Some(LintMessage::new(format!(
                                "File {} has too many function declarations (exceeds 15): found {}",
                                file, count
                            ))),
                        },
                    ));
                    return;
                }
            }
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for SurfaceRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfaceRoleChecker {
    pub fn new() -> Self {
        Self {}
    }

    /// Main implementation for utility and passive surface checks.
    /// Receives content via SourceContentVO (no I/O).
    fn _check_surface_impl(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let content = source.content.value();
        let file = source.file_path.value();
        let lines: Vec<&str> = content.lines().collect();
        let mut surface_violations: Vec<String> = Vec::new();

        let li = detect_language_info_from_source(source);

        if li.is_py {
            self._check_python_passive(&lines, &mut surface_violations);
        } else if li.is_js {
            self._check_javascript_passive(&lines, &mut surface_violations);
        } else {
            self._check_rust_passive(&lines, &mut surface_violations);
        }

        // Domain logic check (all languages)
        self._check_domain_logic(content, &mut surface_violations);

        if !surface_violations.is_empty() {
            self._report_aes406(file, surface_violations, violations);
        }
    }

    /// AES406: domain logic check — count control-flow statements.
    fn _check_domain_logic(&self, content: &str, violations: &mut Vec<String>) {
        let control_flow_count = content
            .lines()
            .filter(|line| {
                let t = line.trim();
                t.starts_with("if ")
                    || t.starts_with("else ")
                    || t.starts_with("for ")
                    || t.starts_with("while ")
                    || t.starts_with("match ")
                    || t.starts_with("switch ")
                    || t.starts_with("try:")
                    || t.starts_with("except")
                    || t.starts_with("catch")
            })
            .count();
        if control_flow_count > MAX_CONTROL_FLOW {
            violations.push(format!(
                "Too many control-flow statements ({}, max {})",
                control_flow_count, MAX_CONTROL_FLOW
            ));
        }
    }

    /// Rust-specific passive check: detect impl blocks and fn methods.
    fn _check_rust_passive(&self, lines: &[&str], violations: &mut Vec<String>) {
        let impl_re = match &*RUST_IMPL_RE {
            Some(r) => r,
            None => return,
        };
        let fn_re = match &*RUST_FN_RE {
            Some(r) => r,
            None => return,
        };

        let mut current_impl: Option<(String, usize)> = None;
        let mut methods: Vec<(String, usize, Option<usize>)> = Vec::new();
        let mut impl_indent: usize = 0;

        for (i, raw_line) in lines.iter().enumerate() {
            let trimmed = raw_line.trim();
            if trimmed.starts_with("use ") || trimmed.starts_with("//") || trimmed.starts_with("/*")
            {
                continue;
            }
            if trimmed.starts_with("pub mod ") || trimmed.starts_with("mod ") {
                continue;
            }

            if impl_re.captures(trimmed).is_some() {
                if let Some((_name, start)) = current_impl.take() {
                    self._add_impl_violations(&methods, "impl", start, violations);
                }
                let trait_name = if let Some(pos) = trimmed.find(" for ") {
                    trimmed[pos + 5..].trim().to_string()
                } else {
                    String::new()
                };
                current_impl = Some((trait_name, i));
                impl_indent = raw_line.len() - raw_line.trim_start().len();
                methods.clear();
                continue;
            }

            if let (Some((name, _start)), Some(cap)) = (&current_impl, fn_re.captures(trimmed)) {
                let method_name = match cap.get(1).map(|m| m.as_str()) {
                    Some(s) => s.to_string(),
                    None => String::new(),
                };
                if !method_name.starts_with('_')
                    && !name.contains("Drop")
                    && !name.contains("Clone")
                {
                    let mut end_line = lines.len();
                    for (k, line) in lines.iter().enumerate().skip(i + 1) {
                        let next = line.trim();
                        if next.starts_with("fn ") || next.starts_with("impl ") {
                            end_line = k;
                            break;
                        }
                    }
                    methods.push((method_name, i + 1, Some(end_line)));
                }
            }

            // If we exit an impl block, finalize
            if current_impl.is_some() {
                let line_indent = raw_line.len() - raw_line.trim_start().len();
                if !trimmed.is_empty()
                    && trimmed != "}"
                    && line_indent <= impl_indent
                    && let Some((_name, start)) = current_impl.take()
                {
                    self._add_impl_violations(&methods, "impl", start, violations);
                }
            }
        }
        // Finalize any remaining impl block
        if let Some((_name, start)) = current_impl.take() {
            self._add_impl_violations(&methods, "impl", start, violations);
        }
    }

    fn _add_impl_violations(
        &self,
        methods: &[(String, usize, Option<usize>)],
        impl_name: &str,
        _start: usize,
        violations: &mut Vec<String>,
    ) {
        if methods.len() > MAX_PUBLIC_METHODS {
            violations.push(format!(
                "Impl block '{}' has {} public methods (max {})",
                impl_name,
                methods.len(),
                MAX_PUBLIC_METHODS
            ));
        }
        for (method_name, s, e) in methods {
            if let Some(end_line) = e {
                let body_len = (*end_line as i64) - (*s as i64);
                if body_len > MAX_FUNCTION_BODY_LINES {
                    violations.push(format!(
                        "Method '{}' is {} lines (max {})",
                        method_name, body_len, MAX_FUNCTION_BODY_LINES
                    ));
                }
            }
        }
    }

    /// Python-specific passive check: detect classes and methods.
    fn _check_python_passive(&self, lines: &[&str], violations: &mut Vec<String>) {
        for (i, raw_line) in lines.iter().enumerate() {
            let stripped = raw_line.trim();
            let class_re = match &*PY_CLASS_RE {
                Some(r) => r,
                None => continue,
            };
            if let Some(cap) = class_re.captures(stripped) {
                let class_name = match cap.get(1).map(|m| m.as_str()) {
                    Some(s) => s,
                    None => continue,
                };
                let indent = raw_line.len() - raw_line.trim_start().len();

                let mut pub_methods: Vec<(String, usize, Option<usize>)> = Vec::new();

                for j in (i + 1)..lines.len() {
                    let method_line = lines[j];
                    if method_line.trim().is_empty() {
                        continue;
                    }
                    let m_indent = method_line.len() - method_line.trim_start().len();

                    if m_indent <= indent && !method_line.trim().is_empty() {
                        break;
                    }

                    let method_re = match &*PY_METHOD_RE {
                        Some(r) => r,
                        None => break,
                    };
                    if let Some(mcap) = method_re.captures(method_line.trim()) {
                        let method_name = match mcap.get(1).map(|m| m.as_str()) {
                            Some(s) => s,
                            None => continue,
                        };
                        if !method_name.starts_with('_') {
                            let mut end_line = lines.len();
                            for (k, next) in lines.iter().enumerate().skip(j + 1) {
                                if !next.trim().is_empty() {
                                    let n_indent = next.len() - next.trim_start().len();
                                    if n_indent <= m_indent {
                                        end_line = k;
                                        break;
                                    }
                                }
                            }
                            pub_methods.push((method_name.to_string(), j + 1, Some(end_line)));
                        }
                    }
                }

                self._check_methods_too_public(class_name, &pub_methods, violations);
                self._check_method_lengths(class_name, &pub_methods, violations);
                self._check_method_nesting(class_name, lines, &pub_methods, violations);
            }
        }
    }

    /// JavaScript/TypeScript-specific passive check: detect classes and methods.
    fn _check_javascript_passive(&self, lines: &[&str], violations: &mut Vec<String>) {
        let class_re = match &*JS_CLASS_RE {
            Some(r) => r,
            None => return,
        };
        let method_re = match &*JS_METHOD_RE {
            Some(r) => r,
            None => return,
        };

        for (i, raw_line) in lines.iter().enumerate() {
            let stripped = raw_line.trim();
            if let Some(cap) = class_re.captures(stripped) {
                let class_name = match cap.get(1).map(|m| m.as_str()) {
                    Some(s) => s,
                    None => continue,
                };
                let indent = raw_line.len() - raw_line.trim_start().len();

                let mut pub_methods: Vec<(String, usize, Option<usize>)> = Vec::new();

                for j in (i + 1)..lines.len() {
                    let method_line = lines[j];
                    if method_line.trim().is_empty() {
                        continue;
                    }
                    let m_indent = method_line.len() - method_line.trim_start().len();

                    if m_indent <= indent && !method_line.trim().is_empty() {
                        break;
                    }

                    if let Some(mcap) = method_re.captures(method_line.trim()) {
                        let method_name = match mcap.get(1).map(|m| m.as_str()) {
                            Some(s) => s,
                            None => continue,
                        };
                        if !method_name.starts_with('_') {
                            let mut end_line = lines.len();
                            for (k, next) in lines.iter().enumerate().skip(j + 1) {
                                if !next.trim().is_empty() {
                                    let n_indent = next.len() - next.trim_start().len();
                                    if n_indent <= m_indent {
                                        end_line = k;
                                        break;
                                    }
                                }
                            }
                            pub_methods.push((method_name.to_string(), j + 1, Some(end_line)));
                        }
                    }
                }

                self._check_methods_too_public(class_name, &pub_methods, violations);
                self._check_method_lengths(class_name, &pub_methods, violations);
                self._check_method_nesting(class_name, lines, &pub_methods, violations);
            }
        }
    }

    // -- AES406 sub-checks ---------------------------------------------------

    /// AES406: too many public methods in a surface class.
    fn _check_methods_too_public(
        &self,
        class_name: &str,
        pub_methods: &[(String, usize, Option<usize>)],
        violations: &mut Vec<String>,
    ) {
        if pub_methods.len() > MAX_PUBLIC_METHODS {
            violations.push(format!(
                "Class '{}' has {} public methods (max {})",
                class_name,
                pub_methods.len(),
                MAX_PUBLIC_METHODS
            ));
        }
    }

    /// AES406: method body exceeds line limit.
    fn _check_method_lengths(
        &self,
        class_name: &str,
        pub_methods: &[(String, usize, Option<usize>)],
        violations: &mut Vec<String>,
    ) {
        for (method_name, start, end) in pub_methods {
            if let Some(end_line) = end {
                let body_len = (*end_line as i64) - (*start as i64);
                if body_len > MAX_FUNCTION_BODY_LINES {
                    violations.push(format!(
                        "Method '{}.{}' is {} lines (max {})",
                        class_name, method_name, body_len, MAX_FUNCTION_BODY_LINES
                    ));
                }
            }
        }
    }

    /// AES406: method control-flow nesting exceeds limit.
    fn _check_method_nesting(
        &self,
        class_name: &str,
        lines: &[&str],
        pub_methods: &[(String, usize, Option<usize>)],
        violations: &mut Vec<String>,
    ) {
        for (method_name, start, end) in pub_methods {
            let end_line = match end {
                Some(e) => *e,
                None => lines.len(),
            };
            let mut max_depth: usize = 0;

            for i in *start..end_line {
                if i >= lines.len() {
                    break;
                }
                let line = lines[i];
                let trimmed = line.trim();

                if IF_RE.as_ref().is_some_and(|re| re.is_match(trimmed)) {
                    let indent = line.len() - line.trim_start().len();
                    let depth = indent / 4;
                    if depth > max_depth {
                        max_depth = depth;
                    }
                }
            }

            if max_depth > MAX_IF_DEPTH {
                violations.push(format!(
                    "Method '{}.{}' has deep control flow (if-nesting > {})",
                    class_name, method_name, MAX_IF_DEPTH
                ));
            }
        }
    }

    /// Append a single AES406 result to the results list.
    fn _report_aes406(&self, file: &str, violations: Vec<String>, results: &mut Vec<LintResult>) {
        let detail: String = violations
            .iter()
            .map(|v| format!("  - {}", v))
            .collect::<Vec<_>>()
            .join("\n");

        results.push(LintResult::new_arch(
            file,
            1,
            "AES406",
            Severity::HIGH,
            format!(
                "Surface file '{}' contains active domain logic:\n{}\nWHY? Surfaces must be passive I/O boundaries.\nFIX: Move logic to capabilities/agent layers.",
                file, detail
            ),
        ));
    }
}
