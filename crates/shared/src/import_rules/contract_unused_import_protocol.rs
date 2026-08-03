// PURPOSE: IUnusedImportProtocol — unified protocol for AES203 unused import detection.
// Updated: check_unused_imports now takes file_path for AST dispatch.

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::filesystem::taxonomy_filesystem_vo::ImportEntry;
use crate::import_rules::taxonomy_import_error::ImportError;

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
    fn check_unused_imports(
        &self,
        file: &str,
        content: &str,
        import_entries: &[ImportEntry],
        used_identifiers: &[String],
    ) -> Result<Vec<LintResult>, ImportError>;
}
