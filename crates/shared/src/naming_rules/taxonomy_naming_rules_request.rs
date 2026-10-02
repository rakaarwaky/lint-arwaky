// PURPOSE: NamingRequest — request payload for the naming aggregate

use shared_filesystem::taxonomy_filesystem_vo::FileEntry;

pub enum NamingRequest {
    /// Run AES101/AES102 audit on pre-parsed file entries.
    RunAuditWithEntries { files: Vec<FileEntry> },
    /// Run AES101–AES103 over two file sets: production source, and the test /
    /// bench files the default discovery walk prunes.
    ///
    /// AES101 and AES102 read only `source_files` — they judge production
    /// source. AES103 reads `test_files`, because its subject is exactly the
    /// files `source_files` never contains. Passing both keeps one request
    /// shape instead of a second verb, and keeps the two rule groups from
    /// seeing each other's files.
    RunAuditWithTestEntries {
        source_files: Vec<FileEntry>,
        test_files: Vec<FileEntry>,
    },
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

    /// Build an audit request that also judges the test and bench files.
    pub fn audit_with_tests(source_files: &[FileEntry], test_files: &[FileEntry]) -> Self {
        Self::RunAuditWithTestEntries {
            source_files: source_files.to_vec(),
            test_files: test_files.to_vec(),
        }
    }
}
