// PURPOSE: IContractRoleChecker — port trait for AES0302: contract primitive type audits
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;

pub trait IContractRoleChecker: Send + Sync {
    fn check_port(&self, file: &str, content: &str) -> Vec<LintResult>;
    fn check_protocol(&self, file: &str, content: &str) -> Vec<LintResult>;
    fn check_aggregate(
        &self,
        file: &str,
        content: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
}
