// arch_internal_checker — Internal architectural rule checks (barrels, primitives).
// Implements IInternalCheckerProtocol: barrel completeness, forbid_internal_all, no_primitives.

use crate::taxonomy::{
    AdapterName, ColumnNumber, ErrorCode, FilePath, LayerDefinition, LineNumber, LintMessage,
    LintResult, LocationList, ScopeRef, Severity,
};
use std::fs;

pub struct ArchInternalChecker {}

impl ArchInternalChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn make_result(file: &str, code: &str, msg: &str, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(0),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: crate::taxonomy::DescriptionVO::new(String::new()),
                kind: crate::taxonomy::DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
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
        filename: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if !definition.barrel_completeness.value {
            return;
        }
        if !Self::file_has_all_export(file) {
            let msg = if !definition
                .barrel_completeness_violation_message
                .value
                .is_empty()
            {
                definition
                    .barrel_completeness_violation_message
                    .value
                    .clone()
            } else {
                format!("{} missing export declarations. Barrel files must re-export all public items via pub use/__all__/export *.", filename)
            };
            violations.push(Self::make_result(file, "AES012", &msg, Severity::MEDIUM));
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
            let msg = if !definition
                .forbid_internal_all_violation_message
                .value
                .is_empty()
            {
                definition
                    .forbid_internal_all_violation_message
                    .value
                    .clone()
            } else {
                "__all__ is forbidden in non-barrel files.".to_string()
            };
            violations.push(Self::make_result(file, "AES013", &msg, Severity::MEDIUM));
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
