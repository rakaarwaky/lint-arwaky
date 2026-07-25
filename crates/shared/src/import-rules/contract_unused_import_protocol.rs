// PURPOSE: IUnusedImportProtocol — unified protocol trait for AES203: detect unused imports across Rust, Python, JavaScript
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `Vec<String>` returns changed to `Result<Vec<LintMessage>, ImportError>` (proper error handling)
//   * `&str file_path` params → kept as `&str` (idiomatic borrow, AES402 allows)
//   * `&mut Vec<LintResult>` → changed to `Result<Vec<LintResult>, ImportError>` (proper error handling)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::import_rules::taxonomy_import_error::ImportError;

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file by path (reads file internally).
    /// Returns a list of human-readable lint messages describing each unused
    /// import, or an ImportError if the file cannot be processed.
    fn find_unused_imports(&self, path: &FilePath) -> Result<Vec<LintMessage>, ImportError>;

    /// Check unused imports given file content directly (for inline checking).
    /// Useful when content is already available (avoids re-reading file).
    fn check_unused_imports(
        &self,
        file: &str,
        content: &str,
    ) -> Result<Vec<LintResult>, ImportError>;
}
