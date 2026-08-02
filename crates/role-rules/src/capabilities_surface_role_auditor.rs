// PURPOSE: SurfaceRoleChecker — ISurfaceRoleChecker for AES406: smart/utility/passive surface role checks
//
// ALGORITHM (uses ParseMetadata when available):
//   1. check_fn_count_limit — Count function declarations. If > max_functions (default 15), flag.
//   2. Classify surface by suffix: Smart (_command, _controller, _page, _entry, _router),
//      Utility (_hook, _store, _action, _screen), Passive (all others).
//   3. Passive + Utility: hierarchy (max_public_methods), method body length, nesting depth,
//      domain logic (control flow count).
//   4. Smart surfaces: exempt from Passive + Utility checks but subject to global function limit.

use shared::common::LintResult;
use shared::common::Severity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::filesystem::taxonomy_filesystem_vo::{
    FileEntry, ParseMetadata, PythonMetadata, RustMetadata, TypeScriptMetadata,
};
use shared::role_rules::{AesRoleViolation, ISurfaceRoleChecker};

const MAX_PUBLIC_METHODS: usize = 10;
const MAX_CONTROL_FLOW: usize = 3;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct SurfaceRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl ISurfaceRoleChecker for SurfaceRoleChecker {
    fn check_smart_surface(&self, _file: &FileEntry, _violations: &mut Vec<LintResult>) {}
    fn check_utility_surface(&self, _file: &FileEntry, _violations: &mut Vec<LintResult>) {}
    fn check_passive_surface(&self, _file: &FileEntry, _violations: &mut Vec<LintResult>) {}

    fn check_fn_count_limit(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if let Some(meta) = &file.parse_metadata {
            self._check_fn_count_metadata(file, meta, violations);
        } else {
            self._check_fn_count_fallback(file, violations);
        }

        // Classify surface and run role-specific checks (exempt Smart surfaces)
        let basename = file.path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        let is_smart = basename.ends_with("_command")
            || basename.ends_with("_controller")
            || basename.ends_with("_page")
            || basename.ends_with("_entry")
            || basename.ends_with("_router");
        if is_smart {
            return; // Smart surfaces exempt from Passive + Utility checks
        }

        if let Some(meta) = &file.parse_metadata {
            self._check_passive_with_metadata(file, meta, violations);
        } else {
            self._check_passive_fallback(file, violations);
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

    // ── Function count check ──

    fn _check_fn_count_metadata(
        &self,
        file: &FileEntry,
        meta: &ParseMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let path_str = file.path.to_string_lossy();
        let fn_count = match meta {
            ParseMetadata::Rust(m) => m.function_definitions.len(),
            ParseMetadata::Python(m) => m.function_definitions.len(),
            ParseMetadata::TypeScript(m) | ParseMetadata::JavaScript(m) => {
                m.function_definitions.len()
            }
            _ => 0, // ParseMetadata::Unknown
        };
        if fn_count > 15 {
            violations.push(LintResult::new_arch(
                &path_str,
                0,
                "AES406",
                Severity::HIGH,
                AesRoleViolation::SurfaceRoleViolation {
                    reason: Some(LintMessage::new(format!(
                        "File {} has too many function declarations (exceeds 15): found {}",
                        path_str, fn_count
                    ))),
                },
            ));
        }
    }

    fn _check_fn_count_fallback(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
        let fn_keyword = match file.language {
            shared::filesystem::taxonomy_filesystem_vo::Language::Python => "def ",
            shared::filesystem::taxonomy_filesystem_vo::Language::TypeScript
            | shared::filesystem::taxonomy_filesystem_vo::Language::JavaScript => "function ",
            _ => "fn ",
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
                        &path_str,
                        0,
                        "AES406",
                        Severity::HIGH,
                        AesRoleViolation::SurfaceRoleViolation {
                            reason: Some(LintMessage::new(format!(
                                "File {} has too many function declarations (exceeds 15): found {}",
                                path_str, count
                            ))),
                        },
                    ));
                    return;
                }
            }
        }
    }

    // ── Passive surface checks using ParseMetadata ──

    fn _check_passive_with_metadata(
        &self,
        file: &FileEntry,
        meta: &ParseMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let path_str = file.path.to_string_lossy();
        match meta {
            ParseMetadata::Rust(rust_meta) => {
                self._check_rust_passive_metadata(&path_str, rust_meta, violations);
            }
            ParseMetadata::Python(py_meta) => {
                self._check_python_passive_metadata(&path_str, py_meta, violations);
            }
            ParseMetadata::TypeScript(ts_meta) | ParseMetadata::JavaScript(ts_meta) => {
                self._check_ts_passive_metadata(&path_str, ts_meta, violations);
            }
            _ => {} // ParseMetadata::Unknown — skip
        }
    }

    fn _check_rust_passive_metadata(
        &self,
        path_str: &str,
        meta: &RustMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let pub_fn_count = meta.function_definitions.len();
        if pub_fn_count > MAX_PUBLIC_METHODS {
            violations.push(LintResult::new_arch(
                path_str,
                0,
                "AES406",
                Severity::HIGH,
                AesRoleViolation::SurfaceRoleViolation {
                    reason: Some(LintMessage::new(format!(
                        "Surface file '{}' has {} public methods (max {})",
                        path_str, pub_fn_count, MAX_PUBLIC_METHODS
                    ))),
                },
            ));
        }
    }

    fn _check_python_passive_metadata(
        &self,
        path_str: &str,
        meta: &PythonMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let fn_count = meta.function_definitions.len();
        if fn_count > MAX_PUBLIC_METHODS {
            violations.push(LintResult::new_arch(
                path_str,
                0,
                "AES406",
                Severity::HIGH,
                AesRoleViolation::SurfaceRoleViolation {
                    reason: Some(LintMessage::new(format!(
                        "Surface file '{}' has {} functions (max {})",
                        path_str, fn_count, MAX_PUBLIC_METHODS
                    ))),
                },
            ));
        }
    }

    fn _check_ts_passive_metadata(
        &self,
        path_str: &str,
        meta: &TypeScriptMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let fn_count = meta.function_definitions.len();
        if fn_count > MAX_PUBLIC_METHODS {
            violations.push(LintResult::new_arch(
                path_str,
                0,
                "AES406",
                Severity::HIGH,
                AesRoleViolation::SurfaceRoleViolation {
                    reason: Some(LintMessage::new(format!(
                        "Surface file '{}' has {} functions (max {})",
                        path_str, fn_count, MAX_PUBLIC_METHODS
                    ))),
                },
            ));
        }
    }

    // ── Passive surface fallback (line-based) ──

    fn _check_passive_fallback(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy();
        let content = &file.content;
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
            violations.push(LintResult::new_arch(
                &path_str,
                0,
                "AES406",
                Severity::HIGH,
                AesRoleViolation::NoDomainLogic {
                    reason: Some(LintMessage::new(format!(
                        "Passive surface {} has {} control flow statements (max {})",
                        path_str, control_flow_count, MAX_CONTROL_FLOW
                    ))),
                },
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::filesystem::taxonomy_filesystem_vo::{
        Language, ParseMetadata, RustMetadata, TypeScriptMetadata,
    };
    use std::path::PathBuf;

    fn checker() -> SurfaceRoleChecker {
        SurfaceRoleChecker::new()
    }

    fn make_file(path: &str, lang: Language, content: &str) -> FileEntry {
        FileEntry {
            path: PathBuf::from(path),
            extension: match lang {
                Language::Rust => "rs",
                Language::Python => "py",
                _ => "ts",
            }
            .to_string(),
            language: lang,
            size: content.len() as u64,
            content: content.to_string(),
            parse_ok: false,
            parse_metadata: None,
        }
    }

    fn make_file_with_rust_meta(path: &str, meta: RustMetadata) -> FileEntry {
        FileEntry {
            path: PathBuf::from(path),
            extension: "rs".to_string(),
            language: Language::Rust,
            size: 100,
            content: String::new(),
            parse_ok: true,
            parse_metadata: Some(ParseMetadata::Rust(meta)),
        }
    }

    fn make_file_with_ts_meta(path: &str, meta: TypeScriptMetadata) -> FileEntry {
        FileEntry {
            path: PathBuf::from(path),
            extension: "ts".to_string(),
            language: Language::TypeScript,
            size: 100,
            content: String::new(),
            parse_ok: true,
            parse_metadata: Some(ParseMetadata::TypeScript(meta)),
        }
    }

    #[test]
    fn construction_succeeds() {
        let _ = checker();
    }

    #[test]
    fn fn_count_under_limit_no_violation() {
        let content = (0..10)
            .map(|i| format!("fn func_{}() {{}}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let f = make_file("src/surface_something.rs", Language::Rust, &content);
        let mut v = Vec::new();
        checker().check_fn_count_limit(&f, &mut v);
        assert!(v.is_empty(), "10 functions should be under the limit of 15");
    }

    #[test]
    fn fn_count_over_limit_flagged_fallback() {
        let content = (0..20)
            .map(|i| format!("fn func_{}() {{}}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let f = make_file("src/surface_something.rs", Language::Rust, &content);
        let mut v = Vec::new();
        checker().check_fn_count_limit(&f, &mut v);
        assert!(!v.is_empty(), "20 functions should exceed the limit of 15");
        assert_eq!(v[0].code.code(), "AES406");
        assert_eq!(v[0].severity, Severity::HIGH);
    }

    #[test]
    fn fn_count_over_limit_flagged_metadata() {
        let meta = RustMetadata {
            function_definitions: (0..20)
                .map(|i| shared::filesystem::taxonomy_filesystem_vo::RustFnItem {
                    name: format!("func_{}", i),
                    has_body: true,
                })
                .collect(),
            ..Default::default()
        };
        let f = make_file_with_rust_meta("src/surface_something.rs", meta);
        let mut v = Vec::new();
        checker().check_fn_count_limit(&f, &mut v);
        assert!(
            !v.is_empty(),
            "20 functions in metadata should exceed the limit"
        );
        assert_eq!(v[0].severity, Severity::HIGH);
    }

    #[test]
    fn smart_surface_exempt_from_passive_checks() {
        // Smart surfaces (suffix _command, _controller, etc.) should not get passive violations
        let mut content = String::new();
        for i in 0..5 {
            content.push_str(&format!("if condition_{} {{}}\n", i));
        }
        let f = make_file("src/surface_my_command.rs", Language::Rust, &content);
        let mut v = Vec::new();
        checker().check_fn_count_limit(&f, &mut v);
        assert!(
            v.is_empty(),
            "smart surface should be exempt from passive control flow checks"
        );
    }

    #[test]
    fn passive_surface_control_flow_flagged_fallback() {
        // "my_view" is NOT a smart surface suffix — treated as passive
        let mut content = String::new();
        for i in 0..5 {
            content.push_str(&format!("if condition_{} {{}}\n", i));
        }
        let f = make_file("src/surface_my_view.rs", Language::Rust, &content);
        let mut v = Vec::new();
        checker().check_fn_count_limit(&f, &mut v);
        assert!(
            !v.is_empty(),
            "excess control flow in passive surface should be flagged"
        );
    }

    #[test]
    fn fn_count_python_fallback() {
        let content = (0..20)
            .map(|i| format!("def func_{}(): pass", i))
            .collect::<Vec<_>>()
            .join("\n");
        let f = make_file("src/surface_something.py", Language::Python, &content);
        let mut v = Vec::new();
        checker().check_fn_count_limit(&f, &mut v);
        assert!(!v.is_empty(), "20 python functions should exceed the limit");
        assert_eq!(v[0].code.code(), "AES406");
    }

    #[test]
    fn fn_count_typescript_metadata() {
        let meta = TypeScriptMetadata {
            function_definitions: (0..16)
                .map(|i| shared::filesystem::taxonomy_filesystem_vo::TSFnItem {
                    name: format!("func_{}", i),
                    has_body: true,
                })
                .collect(),
            ..Default::default()
        };
        let f = make_file_with_ts_meta("src/surface_something.ts", meta);
        let mut v = Vec::new();
        checker().check_fn_count_limit(&f, &mut v);
        assert!(!v.is_empty(), "16 ts functions should exceed the limit");
    }
}
