// PURPOSE: IImportRunnerAggregate — contract for import-rules feature orchestrator
use std::collections::HashMap;

use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::{FileEntry, ImportEntry};

pub trait IImportRunnerAggregate: Send + Sync {
    fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError>;
    /// Run audit on pre-parsed file entries from the filesystem crate.
    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult>;
    /// Run audit on pre-parsed file entries with a pre-built import map
    /// (used by the dispatcher to pass workspace-wide imports captured on a
    /// different filesystem instance, so AES201/202/203/205 still fire).
    fn run_audit_with_entries_and_imports(
        &self,
        files: &[FileEntry],
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
