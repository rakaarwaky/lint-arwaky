// Contract layer — parser protocol trait
// FR-001: AST Parsing & Import Extraction
// Responsibilities: parse diagnostics, import data queries

use crate::common::taxonomy_language_vo::Language;
use crate::filesystem::taxonomy_filesystem_vo::{FileEntry, ImportEntry, ParseWarning};
use std::path::Path;

/// Parser protocol — AST parse results and import extraction queries.
/// Consumers import only this trait when they need parse warnings or import data.
pub trait IParserProtocol: Send + Sync {
    fn parse_warnings(&self) -> &[ParseWarning];
    fn import_list(&self) -> &[ImportEntry];
    fn parse_all(&self, files: &mut [FileEntry]);
    fn imports_for(&self, path: &Path) -> Vec<ImportEntry>;
    fn extract(&self, path: &Path, content: &str, language: Language) -> Vec<ImportEntry>;

    /// Resolve all stored imports through barrel files (__init__.py, mod.rs, etc.).
    /// Populates `resolved_path` and `is_resolved` fields.
    /// Call after `parse_all` with the project root directory.
    fn resolve_barrel_imports(&self, root_dir: &Path);
}
