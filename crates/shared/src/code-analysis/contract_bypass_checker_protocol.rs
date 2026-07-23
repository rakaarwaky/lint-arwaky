// PURPOSE: IBypassCheckerProtocol — protocol trait for AES304: detect bypass comments, unwrap/expect, panic
use crate::cli_commands::taxonomy_result_vo::LintResult;

/// Protocol for detecting AES304 violations: bypass comments, unwrap/expect, panic.
///
/// Implementations scan file content and Cargo.toml manifests to find
/// patterns that suppress compiler warnings or panic at runtime, then
/// record each occurrence as a [`LintResult`].
pub trait IBypassCheckerProtocol: Send + Sync {
    fn check_bypass_comments(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
    fn check_cargo_toml(&self, content: &str, violations: &mut Vec<LintResult>);
}
