// Contract layer — aggregate trait for filesystem operations
// Composes 5 focused protocol traits + 5 cache-accessor methods
// Cache accessors live here because they use DashMap (pipeline state),
// which cannot be delegated to the child protocol traits.

use crate::common::taxonomy_display_content_vo::DisplayContent;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use crate::filesystem::contract_graph_protocol::IGraphProtocol;
use crate::filesystem::contract_parser_protocol::IParserProtocol;
use crate::filesystem::contract_tool_resolution_protocol::IToolResolutionProtocol;
use crate::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use crate::filesystem::taxonomy_filesystem_vo::FileEntry;
use crate::filesystem::taxonomy_filesystem_vo::GraphAnalysisContext;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Aggregate trait — composes all 5 focused filesystem protocol traits.
/// Cache-accessor methods live here (DashMap-backed, cannot delegate to child traits).
/// Consumer crates depending on cache data import this aggregate;
/// others import the specific protocol trait they need.
pub trait IFilesystemAggregate:
    IParserProtocol
    + IGraphProtocol
    + IWorkspaceProtocol
    + IToolResolutionProtocol
    + IFileSystemIOProtocol
{
    /// All discovered source files (after pipeline).
    fn file_list(&self) -> &[FileEntry];

    /// Read file content from bounded cache.
    fn read_cached(&self, path: &FilePath) -> ContentString;

    /// Get cached file content (after scan).
    fn get_file_content(&self, path: &Path) -> Option<String>;

    /// Check if a file is in the cache.
    fn has_file(&self, path: &Path) -> bool;

    /// Collect file entries (path, content) for each lintable file.
    fn collect_file_entries(&self, files: &[String]) -> Vec<(PathBuf, String)>;

    /// Discover source files under root, filtering by ignored patterns.
    fn discover_source_files(&self, root: &Path, ignored: &[String]) -> Vec<String>;

    /// Read file content by path (alias for get_file_content).
    fn read_file(&self, path: &Path) -> Option<String>;

    /// Scan directory recursively for all files.
    fn scan_directory(&self, root: &Path) -> Vec<String>;

    /// Discover all files (source + non-source) under root.
    fn discover_files(&self, root: &Path) -> Vec<String>;

    /// Collect source files from a directory with ignored patterns.
    fn collect_source_files(&self, dir: &Path, ignored: &[String]) -> Vec<FilePath>;

    /// Read a lintable file by path string.
    fn read_lintable_file(&self, path: &str) -> Option<String>;

    /// Get tree-sitter-extracted used identifiers for a file (from ParseMetadata).
    /// Returns empty vec if file not in cache or parse_metadata is None.
    fn used_identifiers_for(&self, path: &Path) -> Vec<String>;

    /// Build a cross-file trait implementation map from all cached ParseMetadata.
    /// Returns HashMap<trait_name, Vec<implementor_type_name>>.
    /// Used by AES203 for implicit Rust trait usage detection (method dispatch scope).
    fn implemented_traits_map(&self) -> HashMap<String, Vec<String>>;

    /// Build file index from root — discovers source files, reads content, parses imports.
    /// Populates file_list(), import_list(), and parse_metadata caches.
    /// No-op if already built. Must be called before file_list() returns useful data.
    fn build_file_index(&self, root: &Path);

    /// Build file index with additional ignored paths from config.
    /// Combines built-in defaults (target, node_modules, .git, …) with the
    /// caller-provided patterns before discovering source files.
    fn build_file_index_with_ignored(&self, root: &Path, ignored: &[String]);

    /// Build orphan-detection graph context from workspace root.
    /// Discovers source files, reads content, extracts imports, builds import graph,
    /// and returns the analysis context with forward/reverse links and inheritance.
    fn build_orphan_graph_context(
        &self,
        root_dir: &Path,
        ignored: &[String],
    ) -> GraphAnalysisContext;

    /// List directory entries, skipping hidden files (starting with '.').
    fn list_directory_filtered(&self, path: &FilePath) -> Vec<FileEntry>;

    /// Read up to `max_lines` lines of a file with line-numbered formatting.
    fn read_file_preview(&self, path: &FilePath, max_lines: usize) -> DisplayContent;
}
