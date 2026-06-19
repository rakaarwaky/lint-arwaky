// PURPOSE: ArchLineChecker — ILineCheckerProtocol for AES301 (file too large) and AES302 (file too short)
use std::fs;
use std::path::Path;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::taxonomy_definition_vo::LayerDefinition;

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

        if def.code_analysis.min_lines.value > 0 && count < def.code_analysis.min_lines.value {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES302",
                Severity::HIGH,
                format!(
                    "{} (min: {}).",
                    AesCodeAnalysisViolation::FileTooShort { reason: None },
                    def.code_analysis.min_lines.value
                ),
            ));
        }

        if def.code_analysis.max_lines.value > 0 && count > def.code_analysis.max_lines.value {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES301",
                Severity::HIGH,
                format!(
                    "{} (max: {}).",
                    AesCodeAnalysisViolation::FileTooLarge { reason: None },
                    def.code_analysis.max_lines.value
                ),
            ));
        }
    }
}
