// PURPOSE: IImportRunnerAggregate — contract for import-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;

pub trait IImportRunnerAggregate: Send + Sync {
    fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError>;
    /// Run audit on pre-parsed file entries from the filesystem crate.
    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
