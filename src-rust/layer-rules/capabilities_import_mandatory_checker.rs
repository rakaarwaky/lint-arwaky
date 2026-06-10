// PURPOSE: ArchImportMandatoryChecker — AES002: enforce mandatory import rules per layer definition
use crate::layer_rules::capabilities_import_utils::{
    get_basename, import_matches_scope, parse_import_lines, resolve_scope,
};
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
fn aes002_mandatory_import(required: &str) -> String {
    format!(
        "AES002 MANDATORY_IMPORT: Missing required import: '{}'.",
        required
    )
}
use std::fs;

pub struct ArchImportMandatoryChecker {}

impl Default for ArchImportMandatoryChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchImportMandatoryChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if definition.mandatory.values.is_empty() {
            return;
        }

        let basename = get_basename(file);
        if basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename) {
            return;
        }

        let Ok(content) = fs::read_to_string(file) else {
            return;
        };
        let import_lines = parse_import_lines(&content);

        for required in &definition.mandatory.values {
            let (layer, suffixes) = resolve_scope(required);
            let is_present = if suffixes.is_empty() {
                import_lines.iter().any(|(_, l)| l.contains(layer))
            } else {
                import_lines
                    .iter()
                    .any(|(_, l)| import_matches_scope(l, layer, &suffixes))
            };

            let genuinely_unreferenced = if suffixes.is_empty() {
                !import_lines.iter().any(|(_, l)| l.contains(layer))
            } else {
                !import_lines.iter().any(|(_, l)| l.contains(layer))
                    && !suffixes
                        .iter()
                        .any(|s| import_lines.iter().any(|(_, l)| l.contains(s)))
            };

            if genuinely_unreferenced {
                continue;
            }

            if !is_present {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES002",
                    Severity::HIGH,
                    &aes002_mandatory_import(required),
                ));
            }
        }
    }
}
