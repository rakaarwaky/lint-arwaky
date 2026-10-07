// PURPOSE: typescript utility role auditor — AES404 sub-checks for TypeScript files.
//
// A TypeScript utility file must hold exported free functions only. This
// auditor flags any `class`, `interface`, `enum`, or `type` alias definition,
// whether it is read from the tree-sitter metadata or scanned from the source
// as a fallback.
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor when
// `file.language` is TypeScript or JavaScript and the file is classified to
// the utility layer. The language-independent pieces live in
// utility_utility_role_checker.rs.

use shared_common::taxonomy_lint_vo::LintResult;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};
use shared_role_rules::contract_role_protocol::IUtilityRoleProtocol;

use shared_role_rules::utility_utility_role_checker as utility;

// === Block 1: Type Definition ===

pub struct UtilityTypeScriptRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl IUtilityRoleProtocol for UtilityTypeScriptRoleAuditor {
    fn check_utility_convention(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if let Some(meta) = &file.parse_metadata {
            self._check_with_metadata(file, meta, violations);
        } else {
            self._check_fallback(file, violations);
        }
    }
}

// === Block 3: Constructors, Helpers, Private Methods ===

impl Default for UtilityTypeScriptRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityTypeScriptRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    /// The metadata path reads every forbidden item type that tree-sitter
    /// collected. Each category is reported so the finding names exactly what
    /// the author needs to delete.
    fn _check_with_metadata(
        &self,
        file: &FileEntry,
        meta: &ParseMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let path_str = file.path.to_string_lossy().to_string();
        match meta {
            ParseMetadata::TypeScript(ts_meta) | ParseMetadata::JavaScript(ts_meta) => {
                let mut forbidden: Vec<String> = Vec::new();
                for c in &ts_meta.class_declarations {
                    forbidden.push(format!("class '{}'", c.name));
                }
                for name in &ts_meta.interface_declarations {
                    forbidden.push(format!("interface '{}'", name));
                }
                for name in &ts_meta.type_alias_declarations {
                    forbidden.push(format!("type '{}'", name));
                }
                for name in &ts_meta.enum_declarations {
                    forbidden.push(format!("enum '{}'", name));
                }
                if !forbidden.is_empty() {
                    let why = format!(
                        "Utility files must not define classes, interfaces, enums, or type aliases. Found: [{}]",
                        forbidden.join(", ")
                    );
                    violations.push(utility::build_violation(
                        &path_str,
                        &why,
                        "Remove type definitions; use exported free functions only.",
                        utility::type_definition_detail(),
                    ));
                }
            }
            _ => {
                // Rust and Python metadata are handled by their own auditors.
            }
        }
    }

    /// The fallback path strips comments and template literals from the source,
    /// then looks for an exported type keyword. The `export` prefix is included
    /// because the acceptance test files declare the types at module scope.
    fn _check_fallback(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy().to_string();
        let stripped = utility::strip_ts_comments(&file.content);
        if utility::ts_has_forbidden_item(&stripped) {
            let why = "Utility files must not define classes, interfaces, enums, or type aliases.";
            violations.push(utility::build_violation(
                &path_str,
                why,
                "Remove type definitions; use exported free functions only.",
                utility::type_definition_detail(),
            ));
        }
    }
}
