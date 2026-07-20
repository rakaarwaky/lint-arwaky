// PURPOSE: IUnusedImportProtocol — unified protocol trait for AES203: detect unused imports across Rust, Python, JavaScript
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `Vec<String>` returns → `Vec<LintMessage>` (semantic messages, not raw strings)
//   * `&str file_path` params → kept as `&str` (idiomatic borrow, AES402 allows)
//   * `&mut Vec<LintResult>` → kept (`LintResult` is itself a VO)
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

pub trait IUnusedImportProtocol: Send + Sync {
    /// Find unused imports in a file by path (reads file internally).
    /// Returns a list of human-readable lint messages describing each unused
    /// import. Replaces the previous `Vec<String>` so callers can introspect,
    /// translate, or log messages without parsing free-form strings.
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage>;

    /// Check unused imports given file content directly (for inline checking).
    /// Useful when content is already available (avoids re-reading file).
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
