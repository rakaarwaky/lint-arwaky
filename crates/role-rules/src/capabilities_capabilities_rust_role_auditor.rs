// PURPOSE: rust capability role auditor — AES403 sub-checks for Rust files.
//
// Handles the 3-block capability shape as `impl Trait for Type` (Block 2) followed by a bare `impl Type` (Block 3).
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor by
// `file.language` and calls the trait entry point; all six AES403 sub-checks
// then run here. Type budget and implementor checks live in
// utility_capabilities_role_checker.rs so the three auditors share one copy.

use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::filesystem::taxonomy_filesystem_vo::{ExternalReferenceMap, FileEntry};
use shared::role_rules::contract_role_protocol::ICapabilitiesRoleProtocol;

use super::utility_capabilities_role_checker;

// === Block 1: Type Definition ===

pub struct CapabilitiesRustRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl ICapabilitiesRoleProtocol for CapabilitiesRustRoleAuditor {
    fn check_capability_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if !is_capabilities_layer(layer) {
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
        if !is_capabilities_layer(layer) {
            return;
        }
        utility_capabilities_role_checker::check_type_budget(file, violations);
        utility_capabilities_role_checker::check_implementor(file, violations);
        self.check_capability_block_order(file, violations);
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

    fn check_capability_block_order(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        self._block_order(
            &file.content,
            file.path.to_string_lossy().as_ref(),
            violations,
        );
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

impl Default for CapabilitiesRustRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilitiesRustRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    // ===============================================================
    // Block order — protocol impl must precede inherent impl
    // ===============================================================

    fn _block_order(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut protocol_line: Option<(usize, String)> = None;
        let mut inherent_line: Option<(usize, String)> = None;
        let mut in_cfg_test = false;

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();

            // Skip #[cfg(test)] blocks and everything they contain.
            if t.starts_with("#[cfg(test)]") {
                in_cfg_test = true;
                continue;
            }
            if in_cfg_test {
                if t == "}" || t.starts_with("} //") {
                    in_cfg_test = false;
                }
                continue;
            }

            if protocol_line.is_none() && t.starts_with("impl ") && t.contains("Protocol for") {
                // The block-order check uses the looser "Protocol for" substring
                // so it does not miss protocol implementations that use a generic
                // prefix or re-exported trait names. The implementor check uses
                // `_is_protocol_trait` instead, which is strict about the suffix.
                protocol_line = Some((i, t.trim_end_matches('{').trim().to_string()));
            }

            if inherent_line.is_none() && t.starts_with("impl ") && !t.contains("Protocol for") {
                inherent_line = Some((i, t.trim_end_matches('{').trim().to_string()));
            }
        }

        if let (Some((inh_idx, inh_src)), Some((proto_idx, proto_src))) =
            (inherent_line, protocol_line)
            && inh_idx < proto_idx
        {
            violations.push(LintResult::new_arch(
                path,
                inh_idx + 1,
                "AES403",
                Severity::HIGH,
                format!(
                    "AES403 CAPABILITY_ROLE: Block 2 (protocol impl) must precede Block 3 (inherent impl).\n\
                     WHY? `{inh_src}` is declared at line {} but `{proto_src}` follows at line {}.\n\
                     HOW TO FIX? Reorder the file so the blocks read:\n  \
                     Block 1 (type + constructor) -> Block 2 (protocol methods only) -> Block 3 (factories, std traits, helpers).\n  \
                     Move the protocol trait implementation above the inherent impl block.",
                    inh_idx + 1,
                    proto_idx + 1,
                ),
            ));
        }
    }

    // ===============================================================
    // Constant placement — policy constants live in `taxonomy_*_constant.*`
    // ===============================================================

    fn _constant_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        // Only file-level constants are in scope. A `const` nested inside an
        // `impl` block is an associated constant — it belongs to the type and
        // cannot be shared, so it is not a placement defect.
        for (i, l) in content.lines().enumerate() {
            let t = l.trim();
            // `const NAME: Type = <literal>;` belongs in taxonomy_<domain>_constant.rs.
            // `static` is excluded: lazily-initialised caches (LazyLock/OnceLock)
            // are implementation detail, not a shareable policy constant.
            let indent = l.len() - l.trim_start().len();
            if indent == 0
                && t.starts_with("const ")
                && !t.starts_with("const fn ")
                && t.contains(':')
                && t.contains('=')
            {
                let name = t
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("<unnamed>")
                    .to_string();
                violations.push(LintResult::new_arch(
                    path,
                    i + 1,
                    "AES403",
                    Severity::MEDIUM,
                    format!(
                        "AES403 CAPABILITY_ROLE: Local constant in capabilities file.\n\
                         WHY? `{name}` is declared as a file-level `const` at line {}.\n\
                         HOW TO FIX? Move `{name}` into `taxonomy_<domain>_constant.rs` so every layer shares one policy value.\n  \
                         Keep the constant in this file only when it is a private mechanical detail, not a domain policy.",
                        i + 1,
                    ),
                ));
            }
        }
    }

    // ===============================================================
    // Test placement — inline tests belong in `tests/`
    // ===============================================================

    fn _test_placement(&self, content: &str, path: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();
            // Only a real declaration counts. A `mod tests` mention inside a
            // comment, a string, or a `starts_with(...)` guard is not one.
            let is_cfg_attr = t == "#[cfg(test)]";
            let is_test_mod = (t.starts_with("mod tests")
                || t.starts_with("pub mod tests")
                || t.starts_with("#[cfg(test)] mod tests"))
                && t.ends_with('{');
            if !is_cfg_attr && !is_test_mod {
                continue;
            }
            if is_cfg_attr {
                // `#[cfg(test)]` on its own line is one finding for the block
                // that follows; a `mod tests` on the next line is the same block.
                let next = lines.get(i + 1).map(|n| n.trim()).unwrap_or("");
                if next.starts_with("mod tests") || next.starts_with("pub mod tests") {
                    continue;
                }
            }
            violations.push(LintResult::new_arch(
                path,
                i + 1,
                "AES403",
                Severity::LOW,
                format!(
                    "AES403 CAPABILITY_ROLE: Embedded test code in capabilities file.\n\
                     WHY? A test module is declared inline at line {}.\n\
                     HOW TO FIX? Move it into `crates/<crate>/tests/` as a dedicated test file \
                     (for example `tests/unit_capabilities_<name>.rs`) and keep the capability source \
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
        let mut in_cfg_test = false;
        let mut in_inherent_block = false;
        let mut block_depth: i32 = 0;

        for (i, l) in lines.iter().enumerate() {
            let t = l.trim();

            // Skip #[cfg(test)] blocks entirely.
            if t.starts_with("#[cfg(test)]") {
                in_cfg_test = true;
                continue;
            }
            if in_cfg_test && t.starts_with("}") {
                in_cfg_test = false;
                continue;
            }
            if in_cfg_test {
                continue;
            }

            // Track whether we're inside an inherent impl block.
            if t.starts_with("impl ") && !t.starts_with("impl I") && !t.contains("Protocol for") {
                in_inherent_block = true;
                block_depth = 0;
            }
            if in_inherent_block {
                block_depth += i32::from(t.contains('{')) - i32::from(t.contains('}'));
                if block_depth < 0 {
                    // Closing brace of the impl block; depth went negative so reset.
                    in_inherent_block = false;
                    block_depth = 0;
                    continue;
                }
            }
            if !in_inherent_block {
                continue;
            }

            // Constructors and factory methods are exempt.
            if t.starts_with("pub fn ")
                && !t.starts_with("pub fn new(")
                && !t.starts_with("pub fn default(")
                && !t.starts_with("pub fn with_")
            {
                let fn_name = t
                    .split_whitespace()
                    .nth(2)
                    .unwrap_or("")
                    .split('(')
                    .next()
                    .unwrap_or("");

                // A method that production code calls -- from another module
                // or from inside this file -- is a genuine public API, not a
                // leak. Skip it.
                if references.referenced_from_production(path, fn_name) {
                    continue;
                }

                // A method called only by integration tests must stay `pub`:
                // `pub(crate)` is invisible from `tests/` targets (they compile
                // as a separate crate). Demoting it would break the test suite.
                if references.referenced_only_from_tests(path, fn_name) {
                    continue;
                }

                // This `pub` helper has zero external callers — it should be
                // private or `pub(crate)`.
                violations.push(LintResult::new_arch(
                    path,
                    i + 1,
                    "AES403",
                    Severity::MEDIUM,
                    format!(
                        "AES403 CAPABILITY_ROLE: Public helper has no external caller.\n\
                         WHY? `pub fn {}` in {} is not referenced from any other module or test.\n\
                         HOW TO FIX? Change it to `fn {}` (private). \
                         If a test module in the same crate calls it, change it to `pub(crate) fn {}`.",
                        fn_name, path, fn_name, fn_name,
                    ),
                ));
            }
        }
    }
}

/// True when `layer` names the capabilities layer.
fn is_capabilities_layer(layer: &str) -> bool {
    layer == "capabilities" || layer.starts_with("capabilities(")
}
