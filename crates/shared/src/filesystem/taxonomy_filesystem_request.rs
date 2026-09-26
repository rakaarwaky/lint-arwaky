// PURPOSE: FilesystemRequest — request payload for the filesystem aggregate

use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::ImportEntry;
use std::path::{Path, PathBuf};

pub enum FilesystemRequest {
    /// Return all discovered source file entries.
    FileList,
    /// Read a file from the bounded content cache.
    ReadCached { path: FilePath },
    /// Read a file's content by raw path.
    GetFileContent { path: PathBuf },
    /// Check whether a path is present in the cache.
    HasFile { path: PathBuf },
    /// Collect (path, content) pairs for lintable files matching the pattern list.
    CollectFileEntries { patterns: PatternList },
    /// Discover source files under a root, filtered by ignored patterns.
    DiscoverSourceFiles { root: PathBuf, ignored: Vec<String> },
    /// Read a file's text content by path (alias for get_file_content).
    ReadFile { path: PathBuf },
    /// Scan a directory recursively and return all file paths.
    ScanDirectory { root: PathBuf },
    /// Discover all files (source + non-source) under root.
    DiscoverFiles { root: PathBuf },
    /// Collect source-file paths under a directory, filtered by ignored patterns.
    CollectSourceFiles { dir: PathBuf, ignored: Vec<String> },
    /// Read a lintable file's text by its string path.
    ReadLintableFile { path: String },
    /// Return tree-sitter-extracted used identifiers for a cached file.
    UsedIdentifiers { path: PathBuf },
    /// Return the current built file list (snapshot).
    FileListSnapshot,
    /// Build a cross-file trait-name to implementor map from cached parse metadata.
    ImplementedTraitsMap,
    /// Build the file index from a root (discovers, reads, parses).
    BuildFileIndex { root: PathBuf },
    /// Build the file index with extra ignored patterns.
    BuildFileIndexWithIgnored { root: PathBuf, ignored: Vec<String> },
    /// Build orphan-detection graph context from a workspace root.
    BuildOrphanGraphContext { root: PathBuf, ignored: Vec<String> },
    /// Walk up from start to find the nearest workspace root.
    FindWorkspaceRoot { start: PathBuf },
    /// Return import entries with resolved paths populated.
    ResolvedImportList,
    /// Extend the import cache with entries not tied to the current file list.
    ExtendImportCache { entries: Vec<ImportEntry> },
    /// Snapshot of the import cache taken at the last build call.
    ImportListSnapshot,
    /// Returns all used identifiers across all cached parse metadata.
    UsedIdentifiersAll,
}

impl FilesystemRequest {
    pub fn file_list() -> Self {
        Self::FileList
    }
    pub fn read_cached(path: &FilePath) -> Self {
        Self::ReadCached { path: path.clone() }
    }
    pub fn get_file_content(path: &Path) -> Self {
        Self::GetFileContent {
            path: path.to_path_buf(),
        }
    }
    pub fn has_file(path: &Path) -> Self {
        Self::HasFile {
            path: path.to_path_buf(),
        }
    }
    pub fn collect_file_entries(patterns: &PatternList) -> Self {
        Self::CollectFileEntries {
            patterns: patterns.clone(),
        }
    }
    pub fn discover_source_files(root: &Path, ignored: &[String]) -> Self {
        Self::DiscoverSourceFiles {
            root: root.to_path_buf(),
            ignored: ignored.to_vec(),
        }
    }
    pub fn read_file(path: &Path) -> Self {
        Self::ReadFile {
            path: path.to_path_buf(),
        }
    }
    pub fn scan_directory(root: &Path) -> Self {
        Self::ScanDirectory {
            root: root.to_path_buf(),
        }
    }
    pub fn discover_files(root: &Path) -> Self {
        Self::DiscoverFiles {
            root: root.to_path_buf(),
        }
    }
    pub fn collect_source_files(dir: &Path, ignored: &[String]) -> Self {
        Self::CollectSourceFiles {
            dir: dir.to_path_buf(),
            ignored: ignored.to_vec(),
        }
    }
    pub fn read_lintable_file(path: &str) -> Self {
        Self::ReadLintableFile {
            path: path.to_string(),
        }
    }
    pub fn used_identifiers(path: &Path) -> Self {
        Self::UsedIdentifiers {
            path: path.to_path_buf(),
        }
    }
    pub fn file_list_snapshot() -> Self {
        Self::FileListSnapshot
    }
    pub fn implemented_traits_map() -> Self {
        Self::ImplementedTraitsMap
    }
    pub fn build_file_index(root: &Path) -> Self {
        Self::BuildFileIndex {
            root: root.to_path_buf(),
        }
    }
    pub fn build_file_index_with_ignored(root: &Path, ignored: &[String]) -> Self {
        Self::BuildFileIndexWithIgnored {
            root: root.to_path_buf(),
            ignored: ignored.to_vec(),
        }
    }
    pub fn build_orphan_graph_context(root: &Path, ignored: &[String]) -> Self {
        Self::BuildOrphanGraphContext {
            root: root.to_path_buf(),
            ignored: ignored.to_vec(),
        }
    }
    pub fn find_workspace_root(start: &Path) -> Self {
        Self::FindWorkspaceRoot {
            start: start.to_path_buf(),
        }
    }
    pub fn resolved_import_list() -> Self {
        Self::ResolvedImportList
    }
    pub fn extend_import_cache(entries: Vec<ImportEntry>) -> Self {
        Self::ExtendImportCache { entries }
    }
    pub fn import_list_snapshot() -> Self {
        Self::ImportListSnapshot
    }
    pub fn used_identifiers_all() -> Self {
        Self::UsedIdentifiersAll
    }
}
