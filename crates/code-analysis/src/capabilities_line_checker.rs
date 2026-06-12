// PURPOSE: ArchLineChecker — ILineCheckerProtocol for AES020 (file too large) and AES021 (file too short)
use std::fs;
use std::path::Path;

use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::output_report::taxonomy_result_vo::LintResult;
use shared::output_report::taxonomy_severity_vo::Severity;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_violation_message_rs_error::AesViolation;

pub struct ArchLineChecker {}

impl Default for ArchLineChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchLineChecker {
    pub fn new() -> Self {
        Self {}
    }
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

        if basename == "__init__.py" || basename == "mod.rs" {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if def.exceptions.values.contains(&basename) {
            return;
        }

        let count = match fs::read_to_string(file) {
            Ok(c) => c.lines().count() as i64,
            Err(_) => return,
        };

        if def.min_lines.value > 0 && count < def.min_lines.value {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES021",
                Severity::HIGH,
                format!(
                    "{} (min: {}).",
                    AesViolation::FileTooShort { reason: None },
                    def.min_lines.value
                ),
            ));
        }

        if def.max_lines.value > 0 && count > def.max_lines.value {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES020",
                Severity::HIGH,
                format!(
                    "{} (max: {}).",
                    AesViolation::FileTooLarge { reason: None },
                    def.max_lines.value
                ),
            ));
        }
    }
}
