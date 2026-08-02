// PURPOSE: Contract layer — protocol traits for filesystem operations
// Rule crates depend on these traits, not concrete implementations.
// Updated for FR-001 through FR-004 with enriched types.

use crate::filesystem::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry, FilesystemResult, ImplEntry, ImportEntry, Language,
};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Protocol for walking the filesystem and discovering source files (FR-001).
pub trait IFileWalkerProtocol: Send + Sync {
    fn walk(&self, root: &Path, ignored: &[String], extensions: &[&str]) -> Vec<FileEntry>;
}

/// Protocol for AST parsing (FR-002).
pub trait IASTParserProtocol: Send + Sync {
    fn parse_all(&self, files: &mut [FileEntry]);
}

/// Protocol for extracting imports from source files (FR-003).
pub trait IImportExtractorProtocol: Send + Sync {
    fn extract(
        &self,
        path: &std::path::Path,
        content: &str,
        language: Language,
    ) -> Vec<ImportEntry>;
}

/// Protocol for the dependency graph (FR-004).
pub trait IDependencyGraphProtocol: Send + Sync {
    fn build(
        &mut self,
        imports: &[ImportEntry],
        files: &[FileEntry],
        definitions: &[DefinitionEntry],
        implementations: &[ImplEntry],
    );
    fn dependents(&self, path: &Path) -> Vec<PathBuf>;
    fn dependencies(&self, path: &Path) -> Vec<PathBuf>;
    fn cycles(&self) -> Vec<Vec<PathBuf>>;
    fn reachable(&self, from: &Path, to: &Path) -> bool;
    fn orphan_files(&self) -> Vec<PathBuf>;
    fn all_files(&self) -> HashSet<PathBuf>;
    fn reverse_links(&self) -> &HashMap<PathBuf, Vec<PathBuf>>;
    fn definitions(&self) -> &HashMap<String, Vec<PathBuf>>;
    fn implementations(&self) -> &HashMap<String, Vec<PathBuf>>;
}

/// Protocol for the filesystem service facade.
pub trait IFilesystemServiceProtocol: Send + Sync {
    fn scan(&self, root: &std::path::Path, ignored: &[String]) -> FilesystemResult;
    fn graph(&self) -> &dyn IDependencyGraphProtocol;
}
