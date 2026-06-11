// PURPOSE: IContractRoleChecker — port trait for AES0302: contract primitive type audits
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::config_system::taxonomy_source_vo::SourceContentVO;
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IContractRoleChecker: Send + Sync {
    fn check_port(&self, source: &SourceContentVO) -> Vec<LintResult>;
    fn check_protocol(&self, source: &SourceContentVO) -> Vec<LintResult>;
    fn check_aggregate(
        &self,
        source: &SourceContentVO,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );
}
