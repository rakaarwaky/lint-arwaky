// Contract layer — protocol traits for filesystem operations
// Organized by FR per FRD v3.0.0

use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::{
    DefinitionEntry, FileEntry, ImplEntry, ImportEntry, Language,
};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

// ═══════════════════════════════════════════════════════════
// FR-001: File Discovery Protocol
// ═══════════════════════════════════════════════════════════

/// Protocol for walking the filesystem and discovering source files.
pub trait IFileWalkerProtocol: Send + Sync {
    /// Full mode: discover source files with content.
    fn walk(&self, root: &Path, ignored: &[String], extensions: &[&str]) -> Vec<FileEntry>;

    /// Lightweight mode: discover source file paths only.
    fn discover_paths(&self, root: &Path, ignored: &[String], extensions: &[&str])
    -> Vec<FilePath>;
}

// ═══════════════════════════════════════════════════════════
// FR-002: AST Parsing Protocol
// ═══════════════════════════════════════════════════════════

/// Protocol for AST parsing.
pub trait IASTParserProtocol: Send + Sync {
    fn parse_all(&self, files: &mut [FileEntry]);
}

// ═══════════════════════════════════════════════════════════
// FR-003: Import Extraction Protocol
// ═══════════════════════════════════════════════════════════

/// Protocol for extracting imports from source files.
pub trait IImportExtractorProtocol: Send + Sync {
    fn extract(&self, path: &Path, content: &str, language: Language) -> Vec<ImportEntry>;
}

// ═══════════════════════════════════════════════════════════
// FR-004: Graph Construction Protocol
// ═══════════════════════════════════════════════════════════

/// Protocol for the dependency graph.
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
