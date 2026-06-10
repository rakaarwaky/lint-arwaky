// PURPOSE: ISurfaceRoleChecker — port trait for AES0306: smart, utility, and passive surface role checks
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait ISurfaceRoleChecker: Send + Sync {
    fn check_smart_surface(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_utility_surface(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_passive_surface(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_fn_count_limit(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
