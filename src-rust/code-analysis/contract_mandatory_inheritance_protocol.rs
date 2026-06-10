// PURPOSE: IMandatoryInheritanceProtocol — port trait for AES014: enforce contract implementation in implementer files
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IMandatoryInheritanceProtocol: Send + Sync {
    fn check_mandatory_inheritance(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    );
}
