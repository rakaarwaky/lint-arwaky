// PURPOSE: typescript capability role auditor — AES403 sub-checks for TypeScript and JavaScript files.
//
// Handles the 3-block capability shape as a class constructor (Block 1), public methods (Block 2), then private and static helpers (Block 3).
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor by
// `file.language` and calls the trait entry point; all six AES403 sub-checks
// then run here. Type budget and implementor checks live in
// utility_capabilities_role_checker.rs so the three auditors share one copy.

use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_common::taxonomy_severity_vo::Severity;
use shared_filesystem::taxonomy_filesystem_vo::{ExternalReferenceMap, FileEntry};
use shared_role_rules::contract_role_protocol::ICapabilitiesRoleProtocol;

use shared_role_rules::utility_capabilities_role_checker;

// === Block 1: Type Definition ===

pub struct CapabilitiesTypeScriptRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl ICapabilitiesRoleProtocol for CapabilitiesTypeScriptRoleAuditor {
    fn check_capability_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if !utility_capabilities_role_checker::is_capabilities_layer(layer) {
            return;
        }
        let references = ExternalReferenceMap::default();
        self.check_capability_routing_with_references(file, layer, &references, violations);
    }

    fn check_capability_routing_with_references(
        &self,
        file: &FileEntry,
        layer: &str,
        references: &ExternalReferenceMap,
        violations: &mut Vec<LintResult>,
    ) {
        if !utility_capabilities_role_checker::is_capabilities_layer(layer) {
            return;
        }
        utility_capabilities_role_checker::check_type_budget(file, violations);
        utility_capabilities_role_checker::check_implementor(file, violations);
        utility_capabilities_role_checker::check_single_protocol(file, violations);
        self.check_capability_block_order(file, violations);
        utility_capabilities_role_checker::check_block_markers(file, violations);
        self.check_capability_constant_placement(file, violations);
        self.check_capability_test_placement(file, violations);
        self.check_capability_helper_visibility(file, references, violations);
    }

    fn check_capability_type_budget(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_type_budget(file, violations);
    }

    fn check_capability_implementor(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_implementor(file, violations);
    }

    fn check_capability_single_protocol(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_single_protocol(file, violations);
    }

    fn check_capability_block_order(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._block_order(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_capability_block_markers(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        utility_capabilities_role_checker::check_block_markers(file, violations);
    }

    fn check_capability_constant_placement(
        &self,
        file: &FileEntry,
        violations: &mut Vec<LintResult>,
    ) {
        self._constant_placement(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_capability_test_placement(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._test_placement(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
    }

    fn check_capability_helper_visibility(
        &self,
        file: &FileEntry,
        references: &ExternalReferenceMap,
        violations: &mut Vec<LintResult>,
    ) {
        self._helper_visibility(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            references,
            violations,
        );
    }
}

// === Block 3: Helpers ===

impl Default for CapabilitiesTypeScriptRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesTypeScriptRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    // ===============================================================
    // Block order — protocol impl must precede inherent impl
    // ===============================================================

    fn _block_order(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        // TypeScript/JS: `constructor` is Block 1. A helper method (leading
        // underscore or a factory `static` method) appearing before the first
        // public protocol method is a Block 2/3 order violation.
        let lines: Vec<&str> = content.lines().collect();
        let mut in_test_block = false;
        let mut proto_method_line: Option<(usize, String)> = None;
        let mut helper_line: Option<(usize, String)> = None;

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if t.starts_with("// @ts-") || t.starts_with("/* @jest") {
                in_test_block = true;
                continue;
            }
            if in_test_block && !t.starts_with(' ') && !t.starts_with('\t') {
                in_test_block = false;
            }
            if in_test_block {
                continue;
            }
            // Method lines look like: `fnName() {`, `private fnName() {`,
            // `public fnName(args) {`, `static fnName() {`, `async fnName() {`
            let stripped = t
                .strip_prefix("public ")
                .or_else(|| t.strip_prefix("private "))
                .or_else(|| t.strip_prefix("protected "))
                .or_else(|| t.strip_prefix("readonly "))
                .or_else(|| t.strip_prefix("static "))
                .or_else(|| t.strip_prefix("async "))
                .unwrap_or(t);
            let is_constructor = stripped.starts_with("constructor");
            if is_constructor {
                continue; // Block 1
            }
            // Check if it is a method (has identifier followed by `(`)
            let is_method = stripped
                .chars()
                .any(|c: char| c.is_alphabetic() || c == '_')
                && stripped.contains('(');
            if !is_method {
                continue;
            }
            let name = stripped.split('(').next().unwrap_or("").trim();
            let is_helper = name.starts_with('_')
                || stripped.starts_with("static ")
                || name.starts_with("with_")
                || name.starts_with("build");
            if is_helper && helper_line.is_none() {
                helper_line = Some((i, t.to_string()));
            } else if !is_helper && proto_method_line.is_none() {
                proto_method_line = Some((i, t.to_string()));
            }
        }

        if let (Some((help_idx, help_src)), Some((proto_idx, proto_src))) =
            (helper_line, proto_method_line)
            && help_idx < proto_idx
        {
            violations.push(LintResult::new_arch_with_name(
                path,
                help_idx + 1,
                "AES403",
                Severity::HIGH,
                format!(
                    "AES403 CAPABILITY_ROLE: Block 2 (protocol methods) must precede Block 3 (helpers).\n\
                     WHY: `{help_src}` is declared at line {} but the first public protocol method `{proto_src}` \
                     follows at line {}.\n\
                     FIX: Move all helper methods below the public protocol methods.\n  \
                     Block 1 (class + constructor) -> Block 2 (public protocol methods) -> Block 3 (helpers, statics, factories).",
                    help_idx + 1,
                    proto_idx + 1,
                ),
                "CAPABILITY_ROLE",
                format!(
                    "`{help_src}` is declared at line {} but the first public protocol method `{proto_src}` \
                     follows at line {}.",
                    help_idx + 1,
                    proto_idx + 1
                ),
                "Move all helper methods below the public protocol methods.\n  \
                 Block 1 (class + constructor) -> Block 2 (public protocol methods) -> Block 3 (helpers, statics, factories).",
            ));
        }
    }

    // ────────────────────────────────────────────────────────
    // Constant placement — local constants belong in taxonomy_*_constant.rs
    // ────────────────────────────────────────────────────────

    // ===============================================================
    // Constant placement — policy constants live in `taxonomy_*_constant.*`
    // ===============================================================

    fn _constant_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        // TypeScript/JS: a module-level `const NAME = value` belongs in
        // `taxonomy_<domain>_constant.ts`. A `const` nested inside a class or
        // function body is a local binding, not a shared policy value.
        for (i, l) in content.lines().enumerate() {
            let t = l.trim();
            let indent = l.len() - l.trim_start().len();
            if indent != 0 || t.is_empty() || t.starts_with("//") || t.starts_with('*') {
                continue;
            }
            let Some(rest) = t.strip_prefix("const ") else {
                continue;
            };
            let Some((lhs, _)) = rest.split_once('=') else {
                continue;
            };
            let name = lhs.trim().trim_end_matches(':');
            if name.is_empty() || name.contains(' ') {
                continue;
            }
            violations.push(LintResult::new_arch_with_name(
                path,
                i + 1,
                "AES403",
                Severity::MEDIUM,
                format!(
                    "AES403 CAPABILITY_ROLE: Local constant in capabilities file.\n\
                     WHY: `{name}` is declared as a module-level `const` at line {}.\n\
                     FIX: Move `{name}` into `taxonomy_<domain>_constant.ts` so every layer shares one policy value.\n  \
                     Keep the constant in this file only when it is a private mechanical detail, not a domain policy.",
                    i + 1,
                ),
                "CAPABILITY_ROLE",
                format!(
                    "`{name}` is declared as a module-level `const` at line {}.",
                    i + 1
                ),
                format!(
                    "Move `{name}` into `taxonomy_<domain>_constant.ts` so every layer shares one policy value.\n  \
                     Keep the constant in this file only when it is a private mechanical detail, not a domain policy."
                ),
            ));
        }
    }

    // ────────────────────────────────────────────────────────
    // Test placement — inline tests belong in tests/
    // ────────────────────────────────────────────────────────

    // ===============================================================
    // Test placement — inline tests belong in `tests/`
    // ===============================================================

    fn _test_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut reported_block = false;
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            // `describe(...)`, `it(...)`, `test(...)`, and `class Test*` are
            // test declarations. One finding per contiguous test region.
            let is_marker = t.starts_with("describe(")
                || t.starts_with("it(")
                || t.starts_with("test(")
                || t.starts_with("class Test")
                || t.starts_with("suite(")
                || t.starts_with("beforeEach(")
                || t.starts_with("@test");
            if !is_marker {
                continue;
            }
            if reported_block {
                continue;
            }
            reported_block = true;
            violations.push(LintResult::new_arch_with_name(
                path,
                i + 1,
                "AES403",
                Severity::LOW,
                format!(
                    "AES403 CAPABILITY_ROLE: Embedded test code in capabilities file.\n\
                     WHY: Test code starts at line {} (`{t}`).\n\
                     FIX: Move it into `tests/` as a dedicated test file \
                     (for example `tests/unit_capabilities_<name>.test.ts`) and keep the capability source \
                     free of test-only code.",
                    i + 1,
                ),
                "CAPABILITY_ROLE",
                format!("Test code starts at line {} (`{t}`).", i + 1),
                "Move it into `tests/` as a dedicated test file \
                 (for example `tests/unit_capabilities_<name>.test.ts`) and keep the capability source \
                 free of test-only code.",
            ));
        }
    }

    // ────────────────────────────────────────────────────────
    // Helper visibility — Block 3 helpers must not be fully `pub`
    // ────────────────────────────────────────────────────────
    //
    // We only look at methods that appear inside an inherent impl block
    // (i.e. `impl SomeStruct { ... }`), not inside a protocol impl block.
    // Protocol methods are part of the contract and must be visible.
    // Constructors (`new`, `default`, `with_*`) are exempt.

    // ===============================================================
    // Helper visibility — a Block 3 helper is not `pub` without a caller
    // ===============================================================

    fn _helper_visibility(
        &self,
        content: &str,
        path: &str,
        references: &ExternalReferenceMap,
        violations: &mut Vec<LintResult>,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let mut in_test_block = false;
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if t.starts_with("describe(") || t.starts_with("it(") || t.starts_with("test(") {
                in_test_block = true;
                continue;
            }
            if in_test_block && !t.starts_with(' ') && !t.starts_with('\t') {
                in_test_block = false;
            }
            if in_test_block {
                continue;
            }
            // Only `public` methods are in scope; `private`/`protected` are fine.
            let Some(rest) = t.strip_prefix("public ") else {
                continue;
            };
            let fn_name = rest.split('(').next().unwrap_or("").trim();
            if fn_name.is_empty() || fn_name == "constructor" {
                continue;
            }
            if references.referenced_from_production(path, fn_name) {
                continue;
            }
            if references.referenced_only_from_tests(path, fn_name) {
                continue;
            }
            violations.push(LintResult::new_arch_with_name(
                path,
                i + 1,
                "AES403",
                Severity::MEDIUM,
                format!(
                    "AES403 CAPABILITY_ROLE: Public helper has no external caller.\n\
                     WHY: `public {fn_name}()` is not referenced from any other module or test.\n\
                     FIX: Remove the `public` keyword or mark it `private` if it is an internal helper.",
                    fn_name = fn_name,
                ),
                "CAPABILITY_ROLE",
                format!(
                    "`public {fn_name}()` is not referenced from any other module or test.",
                    fn_name = fn_name,
                ),
                "Remove the `public` keyword or mark it `private` if it is an internal helper.",
            ));
        }
    }
}
