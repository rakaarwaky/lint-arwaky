use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::taxonomy_source_vo::SourceContentVO;

pub trait IUtilityRoleChecker: Send + Sync {
    fn check_utility_convention(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
