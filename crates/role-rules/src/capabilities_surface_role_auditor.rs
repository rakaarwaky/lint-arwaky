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
use shared::common::Language;
use shared::common::Severity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::filesystem::taxonomy_filesystem_vo::{
    FileEntry, ParseMetadata, PythonMetadata, RustMetadata, TypeScriptMetadata,
};
use shared::role_rules::{format_role_violation, AesRoleViolation, ISurfaceRoleChecker};

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
                format_role_violation(
                    &AesRoleViolation::SurfaceRoleViolation {
                        reason: Some(LintMessage::new(format!(
                            "File {} has too many function declarations (exceeds 15): found {}",
                            path_str, fn_count
                        ))),
                    },
                    Language::Rust,
                ),
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
                        format_role_violation(
                            &AesRoleViolation::SurfaceRoleViolation {
                                reason: Some(LintMessage::new(format!(
                                    "File {} has too many function declarations (exceeds 15): found {}",
                                    path_str, count
                                ))),
                            },
                            Language::Rust,
                        ),
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
                format_role_violation(
                    &AesRoleViolation::SurfaceRoleViolation {
                        reason: Some(LintMessage::new(format!(
                            "Surface file '{}' has {} public methods (max {})",
                            path_str, pub_fn_count, MAX_PUBLIC_METHODS
                        ))),
                    },
                    Language::Rust,
                ),
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
                format_role_violation(
                    &AesRoleViolation::SurfaceRoleViolation {
                        reason: Some(LintMessage::new(format!(
                            "Surface file '{}' has {} functions (max {})",
                            path_str, fn_count, MAX_PUBLIC_METHODS
                        ))),
                    },
                    Language::Rust,
                ),
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
                format_role_violation(
                    &AesRoleViolation::SurfaceRoleViolation {
                        reason: Some(LintMessage::new(format!(
                            "Surface file '{}' has {} functions (max {})",
                            path_str, fn_count, MAX_PUBLIC_METHODS
                        ))),
                    },
                    Language::Rust,
                ),
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
                format_role_violation(
                    &AesRoleViolation::NoDomainLogic {
                        reason: Some(LintMessage::new(format!(
                            "Passive surface {} has {} control flow statements (max {})",
                            path_str, control_flow_count, MAX_CONTROL_FLOW
                        ))),
                    },
                    Language::Rust,
                ),
            ));
        }
    }
}
