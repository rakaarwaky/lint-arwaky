// PURPOSE: IUtilityRoleChecker — protocol trait for AES404: utility role boundary violations
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait IUtilityRoleChecker: Send + Sync {
    fn check_utility_convention(&self, file: &FileEntry, violations: &mut Vec<LintResult>);
}
