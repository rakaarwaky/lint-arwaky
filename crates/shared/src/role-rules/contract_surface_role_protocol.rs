// PURPOSE: ISurfaceRoleChecker — port trait for AES406: smart, utility, and passive surface role checks
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_source_vo::SourceContentVO;

pub trait ISurfaceRoleChecker: Send + Sync {
    fn check_smart_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_utility_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_passive_surface(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
    fn check_fn_count_limit(&self, source: &SourceContentVO, violations: &mut Vec<LintResult>);
}
