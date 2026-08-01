// PURPOSE: IContractRoleChecker — contract trait for AES402: contract primitive type audits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait IContractRoleChecker: Send + Sync {
    fn check_protocol(&self, file: &FileEntry) -> Vec<LintResult>;
    fn check_aggregate(&self, file: &FileEntry) -> Vec<LintResult>;
}
