// PURPOSE: IFileAdapterProtocol — protocol trait for file I/O operations
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;

/// Protocol for reading, writing, and checking existence of files.
///
/// Implementations wrap filesystem (or in-memory) I/O behind this trait so
/// that consumers in auto-fix, import-rules, and other subsystems never
/// depend on std::fs directly.
///
/// Ownership: `auto_fix` crate owns this protocol. Consumers access it via
/// `LintFixOrchestratorAggregate::file_adapter()` — no direct DI needed.
pub trait IFileAdapterProtocol: Send + Sync {
    fn read_file(&self, path: &FilePath) -> Option<ContentString>;
    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool;
    fn path_exists(&self, path: &FilePath) -> bool;
}
