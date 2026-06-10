// PURPOSE: Checker: Line rule enforcement
use std::fs;
use std::path::Path;

use crate::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_violation_rs_constant::{
    AES020_FILE_TOO_LARGE_MSG, AES021_FILE_TOO_SHORT_MSG,
};

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
                &format!(
                    "{} (min: {}).",
                    AES021_FILE_TOO_SHORT_MSG, def.min_lines.value
                ),
            ));
        }

        if def.max_lines.value > 0 && count > def.max_lines.value {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES020",
                Severity::HIGH,
                &format!(
                    "{} (max: {}).",
                    AES020_FILE_TOO_LARGE_MSG, def.max_lines.value
                ),
            ));
        }
    }
}
