// PURPOSE: IFixProtocol — protocol trait for auto-fix operations (capabilities layer)
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `u32 line` → `LineNumber line` (semantic line position)
//   * `usize changes` → `Count changes` (semantic count of modifications)
//   * `&[LintResult]` → `&[LintResult]` (LintResult is already a VO aggregate)
//   * `&str file_path` → kept as `&str` (idiomatic borrow for path strings)
//   * `bool` → `FixOutcome` (reason-coded outcome per FRD: Applied / Skipped / Failed)
//   * `Vec<String>` → `Vec<LintMessage>` (lint messages, not raw strings)
//   * `&[&str]` → kept (read-only list of error code strings — no VO replacement
//     without changing the entire taxonomy; could be `&[ErrorCode]` but that
//     would require wrapping at every call site).
use crate::auto_fix::taxonomy_fix_applied_event::FixApplied;
use crate::auto_fix::taxonomy_fix_outcome_vo::FixOutcome;
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::Count;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

pub trait IFixProtocol: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> FixOutcome;
    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> FixOutcome;
    fn emit_fix_event(&self, path: &FilePath, error_code: ErrorCode, changes: Count) -> FixApplied;
    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage>;
    fn is_fixable(&self, violation: &LintResult) -> bool;
    fn fixable_codes(&self) -> &[ErrorCode];
}
