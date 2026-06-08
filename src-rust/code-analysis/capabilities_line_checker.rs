use std::fs;
use std::path::Path;

use crate::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use crate::layer_rules::taxonomy_definition_vo::LayerDefinition;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;

pub struct ArchLineChecker {}

impl ArchLineChecker {
    pub fn new() -> Self { Self {} }
}

impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .to_string();

        if basename == "__init__.py" || basename == "mod.rs" { return; }

        let def = match definition { Some(d) => d, None => return };

        if def.exceptions.values.contains(&basename) { return; }

        let count = match fs::read_to_string(file) {
            Ok(c) => c.lines().count() as i64,
            Err(_) => return,
        };

        if def.min_lines.value > 0 && count < def.min_lines.value {
            let msg = if !def.min_lines_violation_message.value.is_empty() {
                def.min_lines_violation_message.value.clone()
            } else {
                format!("AES005 FILE_TOO_SHORT: File contains fewer than the required minimum lines.\n\
                    WHY? Excessively small files clutter the project structure.\n\
                    FIX: Expand the component or merge this logic into a related module (min: {}).",
                    def.min_lines.value)
            };
            violations.push(LintResult::new_arch(file, 0, "AES005", Severity::HIGH, &msg));
        }

        if def.max_lines.value > 0 && count > def.max_lines.value {
            let msg = if !def.max_lines_violation_message.value.is_empty() {
                def.max_lines_violation_message.value.clone()
            } else {
                format!("AES004 FILE_TOO_LARGE: File exceeds the maximum allowed line count.\n\
                    WHY? Large files violate the Single Responsibility Principle.\n\
                    FIX: Split the module into smaller, more focused files (max: {}).",
                    def.max_lines.value)
            };
            violations.push(LintResult::new_arch(file, 0, "AES004", Severity::HIGH, &msg));
        }
    }
}
