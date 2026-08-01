// PURPOSE: IRoleRunnerAggregate — contract for role-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;
use async_trait::async_trait;

#[async_trait]
pub trait IRoleRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    /// Run audit on pre-parsed file entries from the filesystem crate.
    /// This is the FRD-compliant entry point — no file I/O or parsing in role-rules.
    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
