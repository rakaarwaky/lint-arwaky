// PURPOSE: IMandatoryInheritanceProtocol — port trait for AES014: enforce contract implementation in implementer files
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IMandatoryInheritanceProtocol: Send + Sync {
    fn check_mandatory_inheritance(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
