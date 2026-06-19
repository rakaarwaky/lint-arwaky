// PURPOSE: IInfrastructureRoleChecker — port trait for AES404: infrastructure has no port implementation
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait IInfrastructureRoleChecker: Send + Sync {
    fn check_port_implementation(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
