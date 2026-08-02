// PURPOSE: IRoleRunnerAggregate — contract for role-rules feature orchestrator
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait IRoleRunnerAggregate: Send + Sync {
    /// Run audit on pre-parsed file entries from the filesystem crate.
    /// This is the FRD-compliant entry point — no file I/O or parsing in role-rules.
    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
