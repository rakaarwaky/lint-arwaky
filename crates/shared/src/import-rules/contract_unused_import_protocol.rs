// PURPOSE: IUnusedImportProtocol — unified port trait for AES203: detect unused imports across Rust, Python, JavaScript
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file by path (reads file internally)
    fn find_unused_imports(&self, path: &FilePath) -> Vec<String>;

    /// Check unused imports given file content directly (for inline checking)
    /// Useful when content is already available (avoids re-reading file)
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
