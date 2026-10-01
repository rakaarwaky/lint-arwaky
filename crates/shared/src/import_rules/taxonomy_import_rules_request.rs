// PURPOSE: ImportRequest — request payload for the import aggregate

use shared_common::taxonomy_path_vo::FilePath;
use shared_filesystem::taxonomy_filesystem_vo::{FileEntry, ImportEntry};
use std::collections::HashMap;

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
