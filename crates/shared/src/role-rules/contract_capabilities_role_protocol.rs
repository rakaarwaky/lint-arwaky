// PURPOSE: ICapabilitiesRoleChecker — protocol trait for AES403: capability routing and role audits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait ICapabilitiesRoleChecker: Send + Sync {
    fn check_capability_routing(
        &self,
        file: &FileEntry,
        layer: &str,
        violations: &mut Vec<LintResult>,
    );
}
