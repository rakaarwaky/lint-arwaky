// PURPOSE: InfrastructureRoleChecker — IInfrastructureRoleChecker for AES404: infrastructure has no port implementation
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::role_rules::contract_infrastructure_role_protocol::IInfrastructureRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::SourceContentVO;

pub struct InfrastructureRoleChecker {}

impl Default for InfrastructureRoleChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl InfrastructureRoleChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IInfrastructureRoleChecker for InfrastructureRoleChecker {
    fn check_port_implementation(
        &self,
        source: &SourceContentVO,
        violations: &mut Vec<LintResult>,
    ) {
        let file = source.file_path.value();
        let content = source.content.value();

        // Infrastructure files should contain an `impl ... for ...` that references a port
        // Check if file imports any port/protocol but has no implementation
        let has_import = content.contains("use ")
            && (content.contains("_port::") || content.contains("_protocol::"));
        if !has_import {
            return;
        }
        let has_impl = content.contains("impl ")
            && (content.contains(" for ")
                || content.contains(" for")
                || content.contains("impl<T")
                || content.contains("impl<"));
        if !has_impl {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES404",
                Severity::HIGH,
                AesRoleViolation::InfrastructureNoPort { reason: None }.to_string(),
            ));
        }
    }
}
