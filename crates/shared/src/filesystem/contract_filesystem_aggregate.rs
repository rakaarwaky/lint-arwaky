// PURPOSE: Contract layer — aggregate trait for filesystem operations
// Single entry point for rule crates to access filesystem capabilities.

use super::taxonomy_filesystem_vo::*;
use std::path::PathBuf;

/// Aggregate trait — combines all filesystem capabilities into one interface.
/// Rule crates depend on this for file I/O, parsing, and dependency queries.
pub trait IFilesystemAggregate: Send + Sync {
    /// Run full scan: walk → cache → parse → extract → graph.
    fn scan(&self, root: &PathBuf, ignored: &[String]) -> FilesystemResult;

    /// Get cached file content (after scan).
    fn get_file_content(&self, path: &PathBuf) -> Option<String>;

    /// Check if a file is in the cache.
    fn has_file(&self, path: &PathBuf) -> bool;

    /// Get all discovered files.
    fn all_files(&self) -> &[FileEntry];

    /// Get imports for a specific file.
    fn imports_for(&self, path: &PathBuf) -> Vec<ImportEntry>;

    /// Get all imports.
    fn all_imports(&self) -> &[ImportEntry];

    /// Check if two files have a dependency relationship.
    fn depends_on(&self, from: &PathBuf, to: &PathBuf) -> bool;

    /// Find circular dependencies.
    fn cycles(&self) -> Vec<Vec<PathBuf>>;

    /// Find orphan files (nothing imports them).
    fn orphan_files(&self) -> Vec<PathBuf>;

    /// Get timing breakdown of last scan.
    fn timing(&self) -> &ScanTiming;
}
