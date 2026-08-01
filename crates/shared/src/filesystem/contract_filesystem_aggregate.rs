// PURPOSE: Contract layer — aggregate trait for filesystem operations
// Single entry point for rule crates to access filesystem capabilities.
// Implements FR-005 consumer access pattern with granular accessor methods.

use super::taxonomy_filesystem_vo::*;
use crate::common::taxonomy_path_vo::FilePath;
use std::path::{Path, PathBuf};

/// Aggregate trait — combines all filesystem capabilities into one interface.
/// Rule crates depend on this for file I/O, parsing, and dependency queries.
///
/// Pipeline runs once (lazy: triggered on first accessor call).
/// All accessors return references — zero-cost, no clone.
/// Result is immutable after construction (read-only queries only).
pub trait IFilesystemAggregate: Send + Sync {
    // ── Pipeline Trigger ──────────────────────────────────────

    /// Run full pipeline: walk -> cache -> parse -> extract -> graph.
    /// Lazy: pipeline runs on first accessor call. Results cached internally.
    fn run_pipeline(&self, root: &PathBuf, ignored: &[String]);

    // ── File Access (FR-001) ─────────────────────────────────

    /// All discovered source files (path, content, language, extension).
    /// Consumers: naming-rules, code-analysis, import-rules.
    fn file_list(&self) -> &[FileEntry];

    // ── Parsed File Access (FR-002) ──────────────────────────

    /// Files enriched with parse metadata and parse_ok flag.
    /// Consumers: role-rules, orphan-detector.
    fn parsed_file_list(&self) -> &[FileEntry];

    // ── Parse Warnings (FR-002) ──────────────────────────────

    /// Parse diagnostics for files that failed to parse.
    /// Consumers: all.
    fn parse_warnings(&self) -> &[ParseWarning];

    // ── Import Access (FR-003) ───────────────────────────────

    /// All extracted import entries across the workspace.
    /// Consumers: import-rules, orphan-detector.
    fn import_list(&self) -> &[ImportEntry];

    // ── Graph Access (FR-004) ────────────────────────────────

    /// Forward import graph (file -> files it imports).
    /// Consumers: import-rules, orphan-detector.
    fn dependency_graph(&self) -> &std::collections::HashMap<PathBuf, Vec<PathBuf>>;

    /// Reverse import map (file -> list of files that import it).
    /// Consumers: orphan-detector.
    fn reverse_import_map(&self) -> &std::collections::HashMap<PathBuf, Vec<PathBuf>>;

    /// Symbol definition map (trait/class/struct/interface name -> defining file).
    /// Consumers: orphan-detector.
    fn symbol_definitions(&self) -> &std::collections::HashMap<String, Vec<PathBuf>>;

    /// Trait implementation map (trait/interface name -> list of implementor files).
    /// Consumers: orphan-detector.
    fn trait_implementations(&self) -> &std::collections::HashMap<String, Vec<PathBuf>>;

    // ── Timing ───────────────────────────────────────────────

    /// Get timing breakdown of last scan.
    fn timing(&self) -> &ScanTiming;

    // ── File Reading (backward compat) ───────────────────────

    /// Read file content. Checks cache first, falls back to disk.
    fn read_file(&self, path: &Path) -> Option<String>;

    /// Read file for linting: checks cache, enforces 2MiB size limit.
    fn read_lintable_file(&self, path: &str) -> Result<Option<String>, String>;

    /// Get cached file content (after scan).
    fn get_file_content(&self, path: &PathBuf) -> Option<String>;

    /// Check if a file is in the cache.
    fn has_file(&self, path: &PathBuf) -> bool;

    // ── File Discovery (backward compat) ─────────────────────

    /// Discover all source files in directory tree.
    fn discover_files(&self, root: &Path, ignored: &[String]) -> Vec<FileEntry>;

    /// Discover source files with workspace restriction.
    fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<FilePath>;

    // ── Import/Dependency (backward compat) ──────────────────

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

    // ── Path Queries ─────────────────────────────────────────

    /// Check if path exists.
    fn path_exists(&self, path: &Path) -> bool;

    /// Check if path is a directory.
    fn is_dir(&self, path: &Path) -> bool;

    /// Check if path should be ignored based on ignore patterns.
    fn should_ignore(&self, path: &str, ignored: &[String]) -> bool;

    // ── Workspace ────────────────────────────────────────────

    /// Find workspace root by walking up from start path.
    fn workspace_root(&self, start: &str) -> Option<PathBuf>;
}
