// arch_internal_checker — Internal architectural rule checks (barrels, primitives).
// Implements IInternalCheckerProtocol: barrel completeness, forbid_internal_all, no_primitives.

use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_violationrs_constant::{AES012_BARREL_COMPLETENESS, AES013_INTERNAL_ALL_FORBIDDEN};
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use std::fs;

pub struct ArchInternalChecker {}

impl ArchInternalChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn file_has_all_export(file: &str) -> bool {
        if let Ok(content) = fs::read_to_string(file) {
            let is_rs = file.ends_with(".rs");
            // Rust: pub use (must be followed by space, not field name like use_retry)
            let has_rust_export = content.contains("pub use ")
                || content.contains("pub use{")
                || content.contains("pub use(");
            // Python: __all__ = [...] (only in .py files)
            let has_python_export = !is_rs
                && content.contains("__all__")
                && !content.contains("// __all__")
                && !content.contains("# __all__");
            // JS/TS: export * or export { (only in .js/.ts files, not in .rs comments)
            let has_js_export = content.contains("export *");
            return has_rust_export || has_python_export || has_js_export;
        }
        false
    }

    fn is_barrel_file(filename: &str) -> bool {
        matches!(filename, "__init__.py" | "mod.rs" | "index.ts" | "index.js")
    }

    /// Check barrel completeness (AES012): barrel files must have __all__ / pub use.
    pub fn check_barrel_completeness(
        &self,
        file: &str,
        _filename: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if !definition.barrel_completeness.value {
            return;
        }
        if !Self::file_has_all_export(file) {
            violations.push(LintResult::new_arch(file, 0, "AES012", Severity::MEDIUM, AES012_BARREL_COMPLETENESS));
        }
    }

    /// Check forbid_internal_all (AES013): non-barrel files must NOT have __all__.
    /// JS/TS is exempted because `export` in every file is standard module practice,
    /// not a bypass — unlike Python __all__ or Rust pub use which centralize public API.
    pub fn check_forbid_internal_all(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if !definition.forbid_internal_all.value {
            return;
        }
        let lower = file.to_lowercase();
        if lower.ends_with(".js")
            || lower.ends_with(".ts")
            || lower.ends_with(".jsx")
            || lower.ends_with(".tsx")
        {
            return;
        }
        if Self::file_has_all_export(file) {
            violations.push(LintResult::new_arch(file, 0, "AES013", Severity::MEDIUM, AES013_INTERNAL_ALL_FORBIDDEN));
        }
    }

    /// Check internal rules for a single file (barrel completeness or forbid_internal_all + no_primitives).
    pub fn check_internal_rules(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if Self::is_barrel_file(filename) {
            self.check_barrel_completeness(file, filename, def, violations);
            return;
        }

        self.check_forbid_internal_all(file, def, violations);
        // Note: no_primitives check (AES006) requires AST parsing of class attributes.
        // That is delegated to the main ArchitectureRulesEvaluator which has AST access.
    }
}
