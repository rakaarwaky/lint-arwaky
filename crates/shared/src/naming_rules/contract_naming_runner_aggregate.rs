// PURPOSE: INamingRunnerAggregate — contract for naming-rules feature orchestrator
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait INamingRunnerAggregate: Send + Sync {
    /// Run audit on pre-parsed file entries from the filesystem crate.
    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
