// Contract layer — parser protocol trait
// FR-002: AST Parse Results + FR-003: Import Extraction Queries
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
}
