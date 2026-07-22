use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::taxonomy_definition_vo::LayerDefinition;

// PURPOSE: ArchLineChecker — ILineCheckerProtocol for AES301 (file too large) and AES302 (file too short)
// ALGORITHM:
//   1. Skip barrel files (mod.rs, __init__.py)
//   2. If no LayerDefinition provided, skip
//   3. Check if filename is in exception list
//   4. Count lines in passed content string
//   5. If min_lines > 0 and count < min_lines → AES302 FILE_TOO_SHORT
//   6. If max_lines > 0 and count > max_lines → AES301 FILE_TOO_LARGE
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ArchLineChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = match Path::new(file).file_name().and_then(|f| f.to_str()) {
            Some(name) => name.to_string(),
            None => return,
        };

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

        let count = content.lines().count() as i64;

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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

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
