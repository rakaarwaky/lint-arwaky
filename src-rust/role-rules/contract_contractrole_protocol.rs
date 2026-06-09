use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IContractRoleChecker: Send + Sync {
    fn check_port(&self) -> Vec<LintResult>;
    fn check_protocol(&self) -> Vec<LintResult>;
    fn check_aggregate(&self, file: &str, content: &str, def: &LayerDefinition, violations: &mut Vec<LintResult>);
}
