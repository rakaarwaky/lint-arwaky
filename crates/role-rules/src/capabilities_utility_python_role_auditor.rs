// PURPOSE: python utility role auditor — AES404 sub-checks for Python utility files.
//
// A Python utility file must hold free functions only. This auditor flags any
// `class` definition, whether it is read from the tree-sitter metadata or
// scanned from the source as a fallback.
//
// The orchestrator (agent_role_orchestrator.rs) selects this auditor when
// `file.language` is Python and the file is classified to the utility layer.
// The language-independent pieces live in utility_utility_role_checker.rs.

use shared_common::taxonomy_lint_result_vo::LintResult;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, ParseMetadata};
use shared_role_rules::contract_role_protocol::IUtilityRoleProtocol;

use shared_role_rules::utility_utility_role_checker as utility;

// === Block 1: Type Definition ===

pub struct UtilityPythonRoleAuditor {}

// === Block 2: Protocol Implementation ===

impl IUtilityRoleProtocol for UtilityPythonRoleAuditor {
    fn check_utility_convention(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        if let Some(meta) = &file.parse_metadata {
            self._check_with_metadata(file, meta, violations);
        } else {
            self._check_fallback(file, violations);
        }
    }
}

// === Block 3: Constructors, Helpers, Private Methods ===

impl Default for UtilityPythonRoleAuditor {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityPythonRoleAuditor {
    pub fn new() -> Self {
        Self {}
    }

    /// The metadata path reads class names that tree-sitter already collected.
    fn _check_with_metadata(
        &self,
        file: &FileEntry,
        meta: &ParseMetadata,
        violations: &mut Vec<LintResult>,
    ) {
        let path_str = file.path.to_string_lossy().to_string();
        if let ParseMetadata::Python(py_meta) = meta {
            if !py_meta.class_declarations.is_empty() {
                let names: Vec<&str> = py_meta
                    .class_declarations
                    .iter()
                    .map(|c| c.name.as_str())
                    .collect();
                let why = format!(
                    "Utility files must not define classes. Found: [{}]",
                    names.join(", ")
                );
                violations.push(utility::build_violation(
                    &path_str,
                    &why,
                    "Remove class definitions; use module-level functions only.",
                    utility::type_definition_detail(),
                ));
            }
        }
        // Rust and TypeScript metadata are handled by their own auditors.
    }

    /// The fallback path strips comments and docstrings from the source, then
    /// looks for a class declaration at the start of a line.
    fn _check_fallback(&self, file: &FileEntry, violations: &mut Vec<LintResult>) {
        let path_str = file.path.to_string_lossy().to_string();
        let stripped = utility::strip_python_comments_and_docstrings(&file.content);
        if utility::python_has_forbidden_item(&stripped) {
            let why = "Utility files must not define classes.";
            violations.push(utility::build_violation(
                &path_str,
                why,
                "Remove class definitions; use module-level functions only.",
                utility::type_definition_detail(),
            ));
        }
    }
}
