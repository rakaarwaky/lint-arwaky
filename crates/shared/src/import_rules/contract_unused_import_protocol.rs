// PURPOSE: IUnusedImportProtocol — unified protocol for AES203 unused import detection.
// Updated: check_unused_imports now takes file_path for AST dispatch.
// V2: Added `implemented_traits` for cross-file trait usage analysis.

use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::ImportEntry;
use crate::import_rules::taxonomy_import_error::ImportError;
use std::collections::HashMap;

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file. `content` is pre-read file content.
    /// `used_identifiers` — pre-extracted identifiers from filesystem's tree-sitter AST
    /// (from ParseMetadata). Empty slice when no AST data is available.
    fn find_unused_imports(
        &self,
        path: &FilePath,
        content: &str,
        import_entries: &[ImportEntry],
        used_identifiers: &[String],
    ) -> Result<Vec<LintMessage>, ImportError>;

    /// Check unused imports given file path and content.
    /// file_path is needed for AST parser dispatch (language detection by extension).
    /// `used_identifiers` — pre-extracted identifiers from filesystem's tree-sitter AST.
    /// `implemented_traits` — cross-file map: trait_name → [type_names that implement it].
    /// Used for implicit Rust trait usage detection (method dispatch scope).
    fn check_unused_imports(
        &self,
        file: &str,
        content: &str,
        import_entries: &[ImportEntry],
        used_identifiers: &[String],
        implemented_traits: &HashMap<String, Vec<String>>,
    ) -> Result<Vec<LintResult>, ImportError>;
}
