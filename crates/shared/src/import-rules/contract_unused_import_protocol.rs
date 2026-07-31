// PURPOSE: IUnusedImportProtocol — unified protocol for AES203 unused import detection.
// Updated: check_unused_imports now takes file_path for AST dispatch.

use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::import_rules::taxonomy_import_error::ImportError;

pub trait IUnusedImportProtocol: Send + Sync {
    fn find_unused_imports(&self, path: &FilePath) -> Result<Vec<LintMessage>, ImportError>;

    /// Check unused imports given file path and content.
    /// file_path is needed for AST parser dispatch (language detection by extension).
    fn check_unused_imports(
        &self,
        file: &str,
        content: &str,
    ) -> Result<Vec<LintResult>, ImportError>;
}
