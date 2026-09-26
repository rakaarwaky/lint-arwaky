// PURPOSE: ImportRequestVO / ImportResponseVO — aggregate request/response for import-rules
use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::{FileEntry, ImportEntry};
use std::collections::HashMap;

/// Consumer verb carried by the import aggregate's single entry point.
pub enum ImportRequest {
    /// Run audit on a target path, reading files from the filesystem.
    RunAudit { target: FilePath },
    /// Run audit on pre-parsed file entries.
    RunAuditWithEntries { files: Vec<FileEntry> },
    /// Run audit on pre-parsed entries with a pre-built import map.
    RunAuditWithEntriesAndImports {
        files: Vec<FileEntry>,
        imports_map: HashMap<String, Vec<ImportEntry>>,
    },
    /// Report the adapter name.
    Name,
}

impl ImportRequest {
    pub fn audit(target: &FilePath) -> Self {
        Self::RunAudit {
            target: target.clone(),
        }
    }

    pub fn audit_with_entries(files: &[FileEntry]) -> Self {
        Self::RunAuditWithEntries {
            files: files.to_vec(),
        }
    }

    pub fn audit_with_entries_and_imports(
        files: &[FileEntry],
        imports_map: &HashMap<String, Vec<ImportEntry>>,
    ) -> Self {
        Self::RunAuditWithEntriesAndImports {
            files: files.to_vec(),
            imports_map: imports_map.clone(),
        }
    }
}

/// Result of an import aggregate request.
pub enum ImportResponse {
    /// Audit result from a path-based run (may fail if the path is missing).
    Audit { result: Result<Vec<LintResult>, ScanError> },
    /// Violations found by an entry-based audit.
    AuditEntries { violations: Vec<LintResult> },
    /// Adapter name of the orchestrator.
    Name { name: String },
}

impl ImportResponse {
    /// Take the path-based audit result, or Ok(empty) if a different verb was served.
    pub fn into_result(self) -> Result<Vec<LintResult>, ScanError> {
        match self {
            Self::Audit { result } => result,
            Self::AuditEntries { violations } => Ok(violations),
            Self::Name { .. } => Ok(Vec::new()),
        }
    }

    pub fn into_violations(self) -> Vec<LintResult> {
        match self {
            Self::AuditEntries { violations } => violations,
            Self::Audit { result } => result.unwrap_or_default(),
            Self::Name { .. } => Vec::new(),
        }
    }

    pub fn into_name(self) -> String {
        match self {
            Self::Name { name } => name,
            _ => String::new(),
        }
    }
}
