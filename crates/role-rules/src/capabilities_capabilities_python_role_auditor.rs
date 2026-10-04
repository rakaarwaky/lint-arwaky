// PURPOSE: python capability role auditor — AES403 sub-checks for Python files.
//
// Handles the 3-block capability shape as a class inheriting a protocol ABC (Block 2) followed by `def _helper` methods (Block 3).
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

pub struct CapabilitiesPythonRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl ICapabilitiesRoleProtocol for CapabilitiesPythonRoleAuditor {
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

impl Default for CapabilitiesPythonRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesPythonRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    // ===============================================================
    // Block order — protocol impl must precede inherent impl
    // ===============================================================

    fn _block_order(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        // Python: Block 2 is the first protocol-defined method; Block 3 is a helper
        // (e.g. `def _helper`). The rule: a method that looks like a helper
        // (leading underscore or named after a common factory) must not appear
        // before the first public protocol method.
        let lines: Vec<&str> = content.lines().collect();
        let mut in_test_block = false;
        let mut proto_method_line: Option<(usize, String)> = None;
        let mut helper_line: Option<(usize, String)> = None;

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            if t.starts_with("@pytest") || t.starts_with("def test_") || t.starts_with("@unittest")
            {
                in_test_block = true;
                continue;
            }
            if in_test_block && !t.starts_with(' ') && !t.starts_with('\t') {
                in_test_block = false;
            }
            if in_test_block {
                continue;
            }
            if t.starts_with("def ") {
                let name = t
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .split('(')
                    .next()
                    .unwrap_or("")
                    .trim_end_matches(':');
                // `__init__` is Block 1 (constructor) and must be skipped
                // entirely; it is neither a helper nor a protocol method.
                if name == "__init__" {
                    continue;
                }
                let is_helper = name.starts_with('_');
                if is_helper && helper_line.is_none() {
                    helper_line = Some((i, t.trim_end_matches(':').to_string()));
                } else if proto_method_line.is_none() {
                    proto_method_line = Some((i, t.trim_end_matches(':').to_string()));
                }
            }
        }

        if let (Some((help_idx, help_src)), Some((proto_idx, proto_src))) =
            (helper_line, proto_method_line)
            && help_idx < proto_idx
        {
            violations.push(LintResult::new_arch(
                path,
                help_idx + 1,
                "AES403",
                Severity::HIGH,
                format!(
                    "AES403 CAPABILITY_ROLE: Block 2 (protocol methods) must precede Block 3 (helpers).\n\
                     WHY? `{help_src}` is declared at line {} but the first public protocol method `{proto_src}` \
                     follows at line {}.\n\
                     HOW TO FIX? Move all `def _helper` and `def with_*` methods below the public protocol methods.\n  \
                     Block 1 (class + __init__) -> Block 2 (public protocol methods) -> Block 3 (private helpers, factories).",
                    help_idx + 1,
                    proto_idx + 1,
                ),
            ));
        }
    }

    // ===============================================================
    // Constant placement — policy constants live in `taxonomy_*_constant.*`
    // ===============================================================

    fn _constant_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        // Python: a module-level `NAME = value` (or annotated `NAME: T = value`)
        // is a policy constant and belongs in `taxonomy_<domain>_constant.py`.
        // An annotated name without a value (`x: int` in a class body) is a
        // declaration, not a constant, and is skipped.
        for (i, l) in content.lines().enumerate() {
            let t = l.trim();
            let indent = l.len() - l.trim_start().len();
            if indent != 0 || t.is_empty() || t.starts_with('#') {
                continue;
            }
            let Some((lhs, _)) = t.split_once('=') else {
                continue;
            };
            let name = lhs.trim().trim_end_matches(':');
            if name.is_empty()
                || name.contains(' ')
                || !name
                    .chars()
                    .all(|c| c.is_uppercase() || c == '_' || c.is_ascii_digit())
            {
                continue;
            }
            violations.push(LintResult::new_arch(
                path,
                i + 1,
                "AES403",
                Severity::MEDIUM,
                format!(
                    "AES403 CAPABILITY_ROLE: Local constant in capabilities file.\n\
                     WHY? `{name}` is declared as a module-level constant at line {}.\n\
                     HOW TO FIX? Move `{name}` into `taxonomy_<domain>_constant.py` so every layer shares one policy value.\n  \
                     Keep the constant in this file only when it is a private mechanical detail, not a domain policy.",
                    i + 1,
                ),
            ));
        }
    }

    // ===============================================================
    // Test placement — inline tests belong in `tests/`
    // ===============================================================

    fn _test_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut reported_block = false;
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            // A `class Test*` / `Test*` class is a test suite; `def test_*` and
            // `if __name__ == "__main__"` are test entry points. Report the
            // first marker of a contiguous test region only, so a whole class
            // body produces one finding.
            let is_marker = t.starts_with("def test_")
                || t.starts_with("async def test_")
                || t.starts_with("class Test")
                || t.starts_with("@pytest")
                || t.starts_with("@unittest");
            if !is_marker {
                continue;
            }
            if reported_block {
                continue;
            }
            reported_block = true;
            violations.push(LintResult::new_arch(
                path,
                i + 1,
                "AES403",
                Severity::LOW,
                format!(
                    "AES403 CAPABILITY_ROLE: Embedded test code in capabilities file.\n\
                     WHY? Test code starts at line {} (`{t}`).\n\
                     HOW TO FIX? Move it into `tests/` as a dedicated module \
                     (for example `tests/unit_capabilities_<name>.py`) and keep the capability source \
                     free of test-only code.",
                    i + 1,
                ),
            ));
        }
    }

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
            if t.starts_with("def test_") || t.starts_with("class Test") || t.starts_with("@pytest")
            {
                in_test_block = true;
                continue;
            }
            if in_test_block && !t.starts_with(' ') && !t.starts_with('\t') {
                in_test_block = false;
            }
            if in_test_block {
                continue;
            }
            if !t.starts_with("def ") {
                continue;
            }
            let fn_name = t
                .split_whitespace()
                .nth(1)
                .map(|s| s.split('(').next().unwrap_or("").trim_end_matches(':'))
                .unwrap_or("");
            // Leading-underscore names are private by convention — not a leak.
            if fn_name.starts_with('_') || fn_name == "__init__" {
                continue;
            }
            // Only methods that sit after the first private helper are Block 3.
            // Public methods before that boundary are Block 2 protocol methods
            // and must stay visible as part of the contract.
            if i < first_private_helper_line(&lines) {
                continue;
            }
            // A public method called from another module or from inside this
            // file is genuine API; skip it.
            if references.referenced_from_production(path, fn_name) {
                continue;
            }
            // A method called only by tests must stay public to remain reachable.
            if references.referenced_only_from_tests(path, fn_name) {
                continue;
            }
            // A public method with no external caller should be private.
            violations.push(LintResult::new_arch(
                path,
                i + 1,
                "AES403",
                Severity::MEDIUM,
                format!(
                    "AES403 CAPABILITY_ROLE: Public helper has no external caller.\n\
                     WHY? `def {fn_name}` in {path} is not referenced from any other module or test.\n\
                     HOW TO FIX? Prefix it with `_` (e.g. `def _{fn_name}`) to mark it as a private helper.",
                    fn_name = fn_name,
                    path = path,
                ),
            ));
        }
    }
}

// === Free Functions ===

fn first_private_helper_line(lines: &[&str]) -> usize {
    for (i, l) in lines.iter().enumerate() {
        let t = l.trim();
        if t.starts_with("def test_") || t.starts_with("class Test") {
            return usize::MAX;
        }
        if let Some(rest) = t.strip_prefix("def ") {
            let name = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .split('(')
                .next()
                .unwrap_or("");
            if name.starts_with('_') && name != "__init__" {
                return i;
            }
        }
    }
    usize::MAX
}
