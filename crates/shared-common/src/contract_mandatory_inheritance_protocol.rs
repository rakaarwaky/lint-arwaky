// PURPOSE: IMandatoryInheritanceProtocol — port trait for AES014: enforce contract implementation (bidirectional)
use config_system::taxonomy_config_vo::ArchitectureConfig;
use output_report::taxonomy_result_vo::LintResult;

pub trait IMandatoryInheritanceProtocol: Send + Sync {
    /// One-way: file that imports contract must implement it.
    fn check_mandatory_inheritance(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    );

    /// Bidirectional: contract file must be implemented by expected layer.
    /// - _port → infrastructure_*
    /// - _protocol → capabilities_*
    /// - _aggregate → agent_*
    fn check_contract_implementation(
        &self,
        file: &str,
        content: &str,
        all_files: &[String],
        violations: &mut Vec<LintResult>,
    );
}
