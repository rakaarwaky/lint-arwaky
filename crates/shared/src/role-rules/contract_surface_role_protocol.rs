// PURPOSE: ISurfaceRoleChecker — protocol trait for AES406: smart, utility, and passive surface role checks
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait ISurfaceRoleChecker: Send + Sync {
    fn check_smart_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_utility_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_passive_surface(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_fn_count_limit(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}
