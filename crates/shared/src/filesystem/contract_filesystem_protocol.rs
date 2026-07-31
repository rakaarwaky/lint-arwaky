// PURPOSE: Contract layer — protocol traits for filesystem operations
// Rule crates depend on these traits, not concrete implementations.

use super::taxonomy_filesystem_vo::*;
use std::collections::HashSet;
use std::path::PathBuf;

/// Protocol for walking the filesystem and discovering source files.
pub trait IFileWalkerProtocol: Send + Sync {
    fn walk(&self, root: &PathBuf, ignored: &[String], extensions: &[&str]) -> Vec<FileEntry>;
}

/// Protocol for reading and caching file contents.
pub trait IFileCacheProtocol: Send + Sync {
    fn populate(&self, files: &[FileEntry]);
    fn get(&self, path: &PathBuf) -> Option<String>;
    fn contains(&self, path: &PathBuf) -> bool;
}

/// Protocol for AST parsing.
pub trait IASTParserProtocol: Send + Sync {
    fn parse(&self, path: &PathBuf, content: &str, language: Language) -> Option<()>;
    fn has_ast(&self, path: &PathBuf) -> bool;
}

/// Protocol for extracting imports from source files.
pub trait IImportExtractorProtocol: Send + Sync {
    fn extract(&self, path: &PathBuf, content: &str, language: Language) -> Vec<ImportEntry>;
}

/// Protocol for the dependency graph.
pub trait IDependencyGraphProtocol: Send + Sync {
    fn build(&self, imports: &[ImportEntry], files: &[FileEntry]);
    fn dependents(&self, path: &PathBuf) -> Vec<PathBuf>;
    fn dependencies(&self, path: &PathBuf) -> Vec<PathBuf>;
    fn cycles(&self) -> Vec<Vec<PathBuf>>;
    fn reachable(&self, from: &PathBuf, to: &PathBuf) -> bool;
    fn orphan_files(&self) -> Vec<PathBuf>;
    fn all_files(&self) -> HashSet<PathBuf>;
}

/// Protocol for the filesystem service facade.
pub trait IFilesystemServiceProtocol: Send + Sync {
    fn scan(&self, root: &PathBuf, ignored: &[String]) -> FilesystemResult;
    fn graph(&self) -> &dyn IDependencyGraphProtocol;
    fn cache(&self) -> &dyn IFileCacheProtocol;
}
