use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_utility_role_protocol::IUtilityRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

pub struct UtilityRoleChecker {}

impl IUtilityRoleChecker for UtilityRoleChecker {
    fn check_utility_convention(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>) {
        let content = source.content.value();
        let file = source.file_path.value();
        if content.contains("pub struct ") || content.contains("pub enum ") {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::MEDIUM,
                AesRoleViolation::UtilityRole {
                    reason: Some("Utility files must not define structs or enums.".into()),
                }
                .to_string(),
            ));
        }
    }
}

impl Default for UtilityRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl UtilityRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
}
