// Contract layer — single-entry aggregate for filesystem operations
// Consumers depend only on this trait; the 5 capability seams it composes
// are hidden behind the execute() envelope.
use crate::taxonomy_filesystem_request::FilesystemRequest;
use crate::taxonomy_filesystem_response::FilesystemResponse;

/// Aggregate trait — the single entry point over the filesystem feature.
pub trait IFilesystemAggregate: Send + Sync {
    /// Execute a filesystem request and return the corresponding response.
    fn execute(&self, request: FilesystemRequest) -> FilesystemResponse;
}
