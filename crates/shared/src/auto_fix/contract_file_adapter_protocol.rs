// PURPOSE: IFileAdapterProtocol — protocol trait for file I/O operations
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;

/// Protocol for reading, writing, and checking existence of files.
///
/// Implementations wrap filesystem (or in-memory) I/O behind this trait so
/// that consumers in auto-fix, import-rules, and other subsystems never
/// depend on std::fs directly.
///
/// NOTE: This protocol is intentionally exposed from `auto_fix` to `mcp-server`
/// because `mcp-server` needs it to construct the fix orchestrator container.
/// This is the correct ownership boundary — `auto_fix` owns the protocol and
/// `mcp-server` consumes it via composition root wiring.
pub trait IFileAdapterProtocol: Send + Sync {
    fn read_file(&self, path: &FilePath) -> Option<ContentString>;
    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool;
    fn path_exists(&self, path: &FilePath) -> bool;
}
