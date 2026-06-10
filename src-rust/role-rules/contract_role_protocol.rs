// PURPOSE: IContractRoleChecker — port trait for AES0302/AES0307: contract role audits (port, protocol, aggregate checks)
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;

pub trait IContractRoleChecker: Send + Sync {
    fn check_port(&self, file: &str, content: &str, all_files: &[String]) -> Vec<LintResult>;
    fn check_protocol(&self, file: &str, content: &str, all_files: &[String]) -> Vec<LintResult>;
    fn check_aggregate(
        &self,
        file: &str,
        content: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
}
