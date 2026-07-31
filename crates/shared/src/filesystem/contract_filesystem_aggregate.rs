// PURPOSE: Contract layer — aggregate trait for filesystem operations
// Single entry point for rule crates to access filesystem capabilities.
//
// Rule crates MUST only use filesystem through this aggregate trait.
// Direct calls to filesystem::utility_io are allowed during migration
// but will be removed once all crates use the aggregate.

use super::taxonomy_filesystem_vo::*;
use crate::common::taxonomy_path_vo::FilePath;
use std::path::{Path, PathBuf};

/// Aggregate trait — combines all filesystem capabilities into one interface.
/// Rule crates depend on this for file I/O, parsing, and dependency queries.
///
/// Design: functions are grouped by concern (scan, read, discover, query, workspace).
/// Each rule crate only needs a subset of these — the aggregate provides all.
pub trait IFilesystemAggregate: Send + Sync {
    // ── Scan (full pipeline) ─────────────────────────────────

    /// Run full scan: walk → cache → parse → extract → graph.
    fn scan(&self, root: &PathBuf, ignored: &[String]) -> FilesystemResult;

    /// Get timing breakdown of last scan.
    fn timing(&self) -> &ScanTiming;

    // ── File Reading ──────────────────────────────────────────

    /// Read file content. Checks cache first, falls back to disk.
    /// Returns None if file is unreadable.
    fn read_file(&self, path: &Path) -> Option<String>;

    /// Read file for linting: checks cache, enforces 2MiB size limit.
    /// Returns Ok(Some(content)) if readable and within limit.
    /// Returns Ok(None) if file exceeds size limit (graceful skip).
    /// Returns Err(message) if file is unreadable.
    fn read_lintable_file(&self, path: &str) -> Result<Option<String>, String>;

    /// Get cached file content (after scan).
    fn get_file_content(&self, path: &PathBuf) -> Option<String>;

    /// Check if a file is in the cache.
    fn has_file(&self, path: &PathBuf) -> bool;

    // ── File Discovery ────────────────────────────────────────

    /// Discover all source files in directory tree.
    /// Uses ignore crate (gitignore-aware, parallel walk).
    /// Returns Vec<FileEntry> with path, extension, language, size.
    fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry>;

    /// Discover source files with workspace restriction.
    /// Only walks into workspace member directories (crates/, packages/, modules/).
    /// Returns Vec<FilePath> for backward compatibility with shared types.
    fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<FilePath>;

    /// Get all discovered files (from last scan).
    fn all_files(&self) -> &[FileEntry];

    // ── Import/Dependency ─────────────────────────────────────

    /// Get imports for a specific file.
    fn imports_for(&self, path: &PathBuf) -> Vec<ImportEntry>;

    /// Get all imports (from last scan).
    fn all_imports(&self) -> &[ImportEntry];

    /// Check if two files have a dependency relationship.
    fn depends_on(&self, from: &PathBuf, to: &PathBuf) -> bool;

    /// Find circular dependencies.
    fn cycles(&self) -> Vec<Vec<PathBuf>>;

    /// Find orphan files (nothing imports them).
    fn orphan_files(&self) -> Vec<PathBuf>;

    // ── Path Queries ──────────────────────────────────────────

    /// Check if path exists.
    fn path_exists(&self, path: &Path) -> bool;

    /// Check if path is a directory.
    fn is_dir(&self, path: &Path) -> bool;

    /// Check if path should be ignored based on ignore patterns.
    /// Combines is_path_ignored + is_ignored_dir logic.
    fn should_ignore(&self, path: &str, ignored: &[String]) -> bool;

    // ── Workspace ─────────────────────────────────────────────

    /// Find workspace root by walking up from start path.
    /// Looks for Cargo.toml, crates/, packages/, modules/ markers.
    fn workspace_root(&self, start: &str) -> Option<PathBuf>;
}
