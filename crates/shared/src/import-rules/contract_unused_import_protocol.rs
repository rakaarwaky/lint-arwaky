// PURPOSE: IUnusedImportProtocol — unified port trait for AES203: detect unused imports across Rust, Python, JavaScript
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

pub trait IUnusedImportProtocol: Send + Sync {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage>;
    fn check_unused_imports(
        &self,
        file: &FilePath,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}
