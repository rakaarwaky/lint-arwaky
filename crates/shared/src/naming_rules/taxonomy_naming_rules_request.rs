// PURPOSE: NamingRequest — request payload for the naming aggregate

use shared_filesystem::taxonomy_filesystem_vo::FileEntry;

pub enum NamingRequest {
    /// Run AES101/AES102 audit on pre-parsed file entries.
    RunAuditWithEntries { files: Vec<FileEntry> },
    /// Report the adapter name this orchestrator implements.
    Name,
}

impl NamingRequest {
    /// Build an audit request from a borrowed entry slice.
    pub fn audit(files: &[FileEntry]) -> Self {
        Self::RunAuditWithEntries {
            files: files.to_vec(),
        }
    }
}
