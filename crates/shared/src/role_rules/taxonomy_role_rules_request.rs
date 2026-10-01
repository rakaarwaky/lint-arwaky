// PURPOSE: RoleRequest — request payload for the role aggregate

use shared_filesystem::taxonomy_filesystem_vo::FileEntry;

pub enum RoleRequest {
    /// Run AES401-406 role audits on pre-parsed file entries.
    RunAuditWithEntries { files: Vec<FileEntry> },
    /// Report the adapter name this orchestrator implements.
    Name,
}

impl RoleRequest {
    pub fn audit(files: &[FileEntry]) -> Self {
        Self::RunAuditWithEntries {
            files: files.to_vec(),
        }
    }
}
