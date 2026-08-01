// PURPOSE: ITaxonomyRoleChecker — protocol trait for AES401: taxonomy role audits (VO, entity, error, event, constant)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait ITaxonomyRoleChecker: Send + Sync {
    fn check_entity(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_error(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_event(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
    fn check_constant(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}
