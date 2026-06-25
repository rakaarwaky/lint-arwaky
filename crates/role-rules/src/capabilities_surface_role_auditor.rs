// PURPOSE: SurfaceRoleChecker — ISurfaceRoleChecker for AES406: smart/utility/passive surface role checks
//
// ALGORITHM:
//   1. check_fn_count_limit — Counts `fn ` occurrences. If > 15, flags SurfaceRoleViolation.
//   2. check_surface_hierarchy — Iterates files, filters to surface-prefixed or surface-dir files,
//      skips smart surfaces (_command, _controller, _page, _entry) and init files, then runs
//      _check_passive on remaining (passive) surfaces.
//   3. _check_passive — Reads file content, detects language (Rust/Python/JS), dispatches to
//      language-specific passive checks:
//      - Rust: Scans impl blocks for too many public methods (>10) or methods exceeding 80 lines.
//      - Python: Scans class definitions for too many public methods, method length, if-nesting depth.
//      - JS/TS: Same as Python but uses JS-specific class/method regex.
//   4. check_surface_roles (async, IAnalyzer-dependent) — Uses analyzer.detect_layer + layer_map
//      to check no_domain_logic on non-smart surfaces (control_flow_count > 3).
//
// NOTE: check_smart_surface / check_utility_surface / check_passive_surface are no-ops because
//      the actual surface role checks run via check_surface_hierarchy (passive checks) and
//      check_surface_roles (no-domain-logic checks) which are the primary entry points.
//      These trait methods are required by ISurfaceRoleChecker but are intentionally empty.
use once_cell::sync::Lazy;
use regex::Regex;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::role_rules::taxonomy_layer_names_vo::layer_surfaces;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::source_parsing::contract_language_detector_port::Language as DetLang;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_source_vo::SourceContentVO;

const MAX_PUBLIC_METHODS: usize = 10;
const MAX_FUNCTION_BODY_LINES: i64 = 80;
const MAX_IF_DEPTH: usize = 3;

// Regex: detect Python function/method definitions inside a class
static PY_METHOD_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^(?:async\s+)?def\s+(\w+)\s*\(").ok());

// Regex: detect class definitions
static PY_CLASS_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^class\s+(\w+)").ok());

// Regex: detect JavaScript/TypeScript class definitions
static JS_CLASS_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^export\s+class\s+(\w+)").ok());

// Regex: detect JavaScript/TypeScript method definitions
static JS_METHOD_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:public|private|protected)?\s*(?:async\s+)?(\w+)\s*\(").ok());

// Regex: detect if statements for nesting depth
static IF_RE: Lazy<Option<Regex>> = Lazy::new(|| Regex::new(r"^\s*if\s+").ok());

// Regex: detect Rust impl blocks
static RUST_IMPL_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:pub\s+)?(?:unsafe\s+)?impl\s+").ok());

// Regex: detect Rust fn definitions
static RUST_FN_RE: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(").ok());

fn aes406_passive_violation_details(file: &str, details: &str) -> String {
    format!("AES406 SURFACE_ROLE: Surface file '{}' contains active domain logic:\n{}\nWHY? Surfaces must be passive I/O boundaries.\nFIX: Move logic to capabilities/agent layers.", file, details)
}

pub struct SurfaceRoleChecker {}
fn make_adapter(name: &str) -> Option<AdapterName> {
    AdapterName::new(name).ok()
}
impl Default for SurfaceRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SurfaceRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
    pub fn check_smart(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_utility(&self) -> Vec<LintResult> {
        vec![]
    }
    pub fn check_passive(&self) -> Vec<LintResult> {
        vec![]
    }

    pub fn check_fn_count_limit(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let content = source.content.value();
        let file = source.file_path.value();
        let detector =
            shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
        let lang = detector.detect(&source.file_path);
        let fn_keyword = if lang == DetLang::Python {
            "def "
        } else if lang == DetLang::JavaScript || lang == DetLang::TypeScript {
            "function "
        } else {
            "fn "
        };
        if content.matches(fn_keyword).count() > 15 {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES406",
                Severity::HIGH,
                AesRoleViolation::SurfaceRoleViolation { reason: None },
            ));
        }
    }

    // ---- moved from capabilities_role_checker.rs ----

    pub async fn check_surface_roles(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &shared::source_parsing::taxonomy_paths_vo::FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let layer_vo = match analyzer.detect_layer(f, root_dir) {
                Some(l) => l,
                None => continue,
            };

            let is_surface = layer_vo == layer_surfaces()
                || layer_vo
                    .value
                    .starts_with(&format!("{}(", layer_surfaces().value));
            if !is_surface {
                continue;
            }

            let definition = match analyzer.layer_map().values.get(&layer_vo) {
                Some(d) => d.clone(),
                None => continue,
            };

            if definition.role.no_domain_logic.value {
                let basename = std::path::Path::new(&f.value)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                let is_smart = basename.ends_with("_command")
                    || basename.ends_with("_controller")
                    || basename.ends_with("_page")
                    || basename.ends_with("_entry");
                if !is_smart {
                    self._check_no_domain_logic(f, &definition, analyzer, results, "AES406");
                }
            }
        }
    }

    fn _check_no_domain_logic(
        &self,
        f: &FilePath,
        _definition: &LayerDefinition,
        analyzer: &dyn IAnalyzer,
        results: &mut LintResultList,
        code: &str,
    ) {
        let control_flow_count = analyzer.parser().get_control_flow_count(f);
        if control_flow_count.value > 3 {
            results.push(LintResult {
                file: f.clone(),
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw(code),
                message: LintMessage::new(AesRoleViolation::NoDomainLogic { reason: None }),
                source: make_adapter("architecture"),
                severity: Severity::HIGH,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
    }

    // ---- migrated from capabilities_hierarchy_checker.rs ----

    /// Main entry point — run AES406 passive surface check.
    pub fn check_surface_hierarchy(
        &self,
        files: &[FilePath],
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in files {
            if !is_in_surfaces(f) {
                continue;
            }
            if is_init(f) {
                continue;
            }

            // AES406: check if file is passive
            self._check_passive(f, results);
        }
    }

    /// Check if a surface file is passive (thin I/O boundary).
    /// Smart surfaces (_command, _controller, _page, _entry) are exempted
    /// — they are expected to contain orchestration logic.
    fn _check_passive(&self, f: &FilePath, results: &mut LintResultList) {
        let f_str = f.to_string();
        let basename = std::path::Path::new(&f_str)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        if basename.ends_with("_command")
            || basename.ends_with("_controller")
            || basename.ends_with("_page")
            || basename.ends_with("_entry")
        {
            return;
        }

        let content = match std::fs::read_to_string(f.to_string()) {
            Ok(c) => c,
            Err(_) => return,
        };

        let lines: Vec<&str> = content.lines().collect();
        let mut violations: Vec<String> = Vec::new();
        let detector =
            shared::source_parsing::taxonomy_language_detector_helper::LanguageDetector::new();
        let lang = detector.detect(f);

        match lang {
            DetLang::Rust => self._check_rust_passive(f, &lines, &mut violations),
            DetLang::JavaScript | DetLang::TypeScript => {
                self._check_javascript_passive(f, &lines, &mut violations)
            }
            _ => self._check_python_passive(f, &lines, &mut violations),
        }

        if !violations.is_empty() {
            self._report_aes0306(f, violations, results);
        }
    }

    /// Rust-specific passive check: detect impl blocks and fn methods.
    fn _check_rust_passive(&self, _f: &FilePath, lines: &[&str], violations: &mut Vec<String>) {
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
                    let _m_indent = raw_line.len() - raw_line.trim_start().len();
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
                if !trimmed.is_empty() && trimmed != "}" && line_indent <= impl_indent {
                    if let Some((_name, start)) = current_impl.take() {
                        self._add_impl_violations(&methods, "impl", start, violations);
                    }
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
    fn _check_python_passive(&self, _f: &FilePath, lines: &[&str], violations: &mut Vec<String>) {
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
                self._check_method_lengths(class_name, lines, &pub_methods, violations);
                self._check_method_nesting(class_name, lines, &pub_methods, violations);
            }
        }
    }

    /// JavaScript/TypeScript-specific passive check: detect classes and methods.
    fn _check_javascript_passive(
        &self,
        _f: &FilePath,
        lines: &[&str],
        violations: &mut Vec<String>,
    ) {
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
                self._check_method_lengths(class_name, lines, &pub_methods, violations);
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
        _lines: &[&str],
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
    fn _report_aes0306(&self, f: &FilePath, violations: Vec<String>, results: &mut LintResultList) {
        let detail: String = violations
            .iter()
            .map(|v| format!("  - {}", v))
            .collect::<Vec<_>>()
            .join("\n");

        results.push(LintResult {
            file: f.clone(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(1),
            code: ErrorCode::raw("AES406"),
            message: LintMessage::new(aes406_passive_violation_details(&f.to_string(), &detail)),
            source: Some(AdapterName::raw("surface_hierarchy")),
            severity: Severity::HIGH,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        });
    }
}

// --- helpers -----------------------------------------------------------------

/// Check if the file is a surface file by filename prefix `surface_` or `surfaces_` or directory `surfaces/`.
fn is_in_surfaces(f: &FilePath) -> bool {
    let path_str = f.to_string();
    let basename = match path_str.rsplit('/').next() {
        Some(s) => s,
        None => &path_str,
    };
    let stem = match basename.split('.').next() {
        Some(s) => s,
        None => basename,
    };
    if stem.starts_with("surface_") || stem.starts_with("surfaces_") {
        return true;
    }
    if let Some(parent) = path_str.rsplit('/').nth(1) {
        if parent == "surfaces" || parent == "surface" || parent == "cli_commands" {
            return true;
        }
    }
    false
}

/// Check if the file is a barrel/init file.
fn is_init(f: &FilePath) -> bool {
    let path_str = f.to_string();
    path_str.ends_with("__init__.py")
        || path_str.ends_with("mod.rs")
        || path_str.ends_with("index.ts")
        || path_str.ends_with("index.js")
}

impl ISurfaceRoleChecker for SurfaceRoleChecker {
    fn check_smart_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_utility_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_passive_surface(
        &self,
        _source: &SourceContentVO,
        _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
    }
    fn check_fn_count_limit(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
    ) {
        self.check_fn_count_limit(source, violations);
    }
}

#[cfg(test)]
mod tests {
    use super::{is_in_surfaces, is_init, FilePath};

    #[test]
    fn test_is_in_surfaces() {
        let f = FilePath::new("src/surfaces/surface_handler.py")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(is_in_surfaces(&f));

        let f = FilePath::new("src/capabilities/capabilities_not_checker.py")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(!is_in_surfaces(&f));

        let f = FilePath::new("src/cli-commands/surface_check_command.rs")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(is_in_surfaces(&f));
    }

    #[test]
    fn test_is_init() {
        let f = FilePath::new("src/surfaces/__init__.py")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(is_init(&f));

        let f = FilePath::new("src/surfaces/handler.py")
            .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
        assert!(!is_init(&f));
    }
}
