// PURPOSE: Contract layer — protocol traits for filesystem operations
// Rule crates depend on these traits, not concrete implementations.

use crate::taxonomy_filesystem_vo::*;
use camino::Utf8PathBuf;
use std::collections::HashSet;

/// Protocol for walking the filesystem and discovering source files.
pub trait IFileWalkerProtocol: Send + Sync {
    /// Walk directory tree, return discovered source files.
    fn walk(
        &self,
        root: &Utf8PathBuf,
        ignored: &[String],
        extensions: &[&str],
    ) -> Vec<FileEntry>;
}

/// Protocol for reading and caching file contents.
pub trait IFileCacheProtocol: Send + Sync {
    /// Populate cache from file entries (parallel read).
    fn populate(&self, files: &[FileEntry]);

    /// Get cached file content.
    fn get(&self, path: &Utf8PathBuf) -> Option<String>;

    /// Check if file is in cache.
    fn contains(&self, path: &Utf8PathBuf) -> bool;
}

/// Protocol for AST parsing.
pub trait IASTParserProtocol: Send + Sync {
    /// Parse a file's content into a tree-sitter Tree.
    fn parse(&self, path: &Utf8PathBuf, content: &str, language: Language) -> Option<()>;

    /// Check if AST is cached.
    fn has_ast(&self, path: &Utf8PathBuf) -> bool;
}

/// Protocol for extracting imports from ASTs.
pub trait IImportExtractorProtocol: Send + Sync {
    /// Extract imports from a parsed file.
    fn extract(&self, path: &Utf8PathBuf, language: Language) -> Vec<ImportEntry>;
}

/// Protocol for the dependency graph.
pub trait IDependencyGraphProtocol: Send + Sync {
    /// Build graph from extracted imports.
    fn build(&self, imports: &[ImportEntry], files: &[FileEntry]);

    /// Get files that import the given file.
    fn dependents(&self, path: &Utf8PathBuf) -> Vec<Utf8PathBuf>;

    /// Get files imported by the given file.
    fn dependencies(&self, path: &Utf8PathBuf) -> Vec<Utf8PathBuf>;

    /// Find circular dependencies.
    fn cycles(&self) -> Vec<Vec<Utf8PathBuf>>;

    /// Check if there's a path from `from` to `to`.
    fn reachable(&self, from: &Utf8PathBuf, to: &Utf8PathBuf) -> bool;

    /// Find files with no dependents (orphan candidates).
    fn orphan_files(&self) -> Vec<Utf8PathBuf>;

    /// All files in the graph.
    fn all_files(&self) -> HashSet<Utf8PathBuf>;
}

/// Protocol for the filesystem service facade.
pub trait IFilesystemServiceProtocol: Send + Sync {
    /// Run full scan pipeline: walk → cache → parse → extract → graph.
    fn scan(&self, root: &Utf8PathBuf, ignored: &[String]) -> FilesystemResult;

    /// Get the dependency graph (after scan).
    fn graph(&self) -> &dyn IDependencyGraphProtocol;

    /// Get the file cache (after scan).
    fn cache(&self) -> &dyn IFileCacheProtocol;
}
