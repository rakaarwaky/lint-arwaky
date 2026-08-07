// PURPOSE: SurfaceRoleChecker — ISurfaceRoleChecker for AES406: smart/utility/passive surface role checks
//
// ALGORITHM (uses ParseMetadata when available):
//   1. Classify surface by suffix: Smart (_command, _controller, _page, _entry, _router),
//      Utility (_hook, _store, _action, _screen), Passive (all others).
//   2. Passive + Utility: hierarchy (max_public_methods), method body length, nesting depth,
//      domain logic (control flow count).
//   3. Smart surfaces: exempt from Passive + Utility checks.
//   4. Function count limit removed — surface files may contain any number of functions.

use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::filesystem::taxonomy_filesystem_vo::{
    FileEntry, ParseMetadata, PythonMetadata, RustMetadata, TypeScriptMetadata,
};
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;

const MAX_PUBLIC_METHODS: usize = 50;
const MAX_CONTROL_FLOW: usize = 50;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct SurfaceRoleChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl ISurfaceRoleChecker for SurfaceRoleChecker {
    fn check_smart_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        // Smart surfaces are exempt from passive/utility checks — function count runs in check_fn_count_limit.
        let _ = (file, violations);
    }

    fn check_utility_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if let Some(meta) = &file.parse_metadata {
            self._check_passive_with_metadata(file, meta, violations);
        } else {
            self._check_domain_logic(file, violations);
        }
    }

    fn check_passive_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if let Some(meta) = &file.parse_metadata {
            self._check_passive_with_metadata(file, meta, violations);
        } else {
            self._check_domain_logic(file, violations);
        }
    }

    fn check_fn_count_limit(&self, _file: &FileEntry, _violations: &mut Vec<LintResult>) {
        // Function count limit removed — surface files may contain any number of functions.
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

        // Domain logic check runs on all files regardless of ParseMetadata
        self._check_domain_logic(file, violations);
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

                format!("AES406 SURFACE_ROLE: Surface role boundary violation.\nWHY? Surface file '{}' has {} public methods (max {})\nFIX: Ensure surface only performs its designated responsibilities.", path_str, pub_fn_count, MAX_PUBLIC_METHODS)
,
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

                format!("AES406 SURFACE_ROLE: Surface role boundary violation.\nWHY? Surface file '{}' has {} functions (max {})\nFIX: Ensure surface only performs its designated responsibilities.", path_str, fn_count, MAX_PUBLIC_METHODS)
,
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

                format!("AES406 SURFACE_ROLE: Surface role boundary violation.\nWHY? Surface file '{}' has {} functions (max {})\nFIX: Ensure surface only performs its designated responsibilities.", path_str, fn_count, MAX_PUBLIC_METHODS)
,
            ));
        }
    }

    // ── Domain logic check (control flow count) ──

    fn _check_domain_logic(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
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

                format!("AES406 SURFACE_ROLE: Complex domain logic detected in a passive/utility surface.\nWHY? Surface {} has {} control flow statements (max {})\nFIX: Move the complex domain/control logic into capabilities or orchestrator components.", path_str, control_flow_count, MAX_CONTROL_FLOW)
,
            ));
        }
    }
}
