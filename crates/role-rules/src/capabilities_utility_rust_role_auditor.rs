// PURPOSE: rust utility role auditor — AES404 sub-checks for Rust utility files.
//
// A Rust utility file must hold free functions only. This auditor flags any
// `struct`, `enum`, `trait`, `pub type` alias, or `impl` block, whether it is
// read from the tree-sitter metadata or scanned from the source as a fallback.
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor when
// `file.language` is Rust and the file is classified to the utility layer. The
// language-independent pieces — the LintResult shape, the rule code, the
// macro-body stripper, and the forbidden-item detector — live in
// utility_utility_role_checker.rs so the three auditors share one copy.

use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};
use shared::role_rules::contract_role_protocol::IUtilityRoleProtocol;

use super::utility_utility_role_checker as utility;

// === Block 1: Type Definition ===

pub struct UtilityRustRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl IUtilityRoleProtocol for UtilityRustRoleAuditor {
    fn check_utility_convention(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if let Some(meta) = &file.parse_metadata {
            self._check_with_metadata(file, meta, violations);
        } else {
            self._check_fallback(file, violations);
        }
    }
}

// === Block 3: Constructors, Helpers, Private Methods ===

impl Default for UtilityRustRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityRustRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    /// The metadata path reads the type inventory that tree-sitter already
    /// collected and reports every forbidden item in one finding.
    fn _check_with_metadata(
        &self,
        file: &FileEntry,
        meta: &ParseMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let path_str = file.path.to_string_lossy();
        if let ParseMetadata::Rust(rust_meta) = meta {
            let mut items: Vec<String> = Vec::new();
            items.extend(
                rust_meta
                    .struct_definitions
                    .iter()
                    .map(|s| format!("struct '{s}'")),
            );
            items.extend(
                rust_meta
                    .enum_definitions
                    .iter()
                    .map(|s| format!("enum '{s}'")),
            );
            items.extend(
                rust_meta
                    .trait_definitions
                    .iter()
                    .map(|s| format!("trait '{s}'")),
            );
            items.extend(
                rust_meta
                    .type_definitions
                    .iter()
                    .map(|s| format!("type alias '{s}'")),
            );
            items.extend(
                rust_meta
                    .impl_blocks
                    .iter()
                    .map(|imp| match &imp.trait_name {
                        Some(t) => format!("impl '{t} for {}'", imp.implementor_type),
                        None => format!("inherent impl for '{}'", imp.implementor_type),
                    }),
            );
            if !items.is_empty() {
                let why = format!(
                    "Utility files must not define structs, enums, traits, type aliases, or impl blocks. Found: [{}]",
                    items.join(", ")
                );
                violations.push(utility::build_violation(
                    &path_str,
                    &why,
                    utility::type_definition_fix(),
                    utility::type_definition_detail(),
                ));
            }
        }
        // Python and TypeScript metadata are handled by their own auditors.
    }

    /// The fallback path runs when no metadata is available: it strips
    /// comments and macro bodies from the source, then looks for a forbidden
    /// item keyword at the start of a line.
    fn _check_fallback(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy().to_string();
        let stripped = utility::strip_rust_comments_and_macros(&file.content);
        if utility::rust_has_forbidden_item(&stripped) {
            let why = "Utility files must not define structs, enums, traits, type aliases, or impl blocks.";
            violations.push(utility::build_violation(
                &path_str,
                why,
                utility::type_definition_fix(),
                utility::type_definition_detail(),
            ));
        }
    }
}
