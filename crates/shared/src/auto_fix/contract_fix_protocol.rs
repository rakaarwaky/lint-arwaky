// PURPOSE: IFixProtocol — protocol trait for auto-fix operations (capabilities layer)
//
// AES402: All primitive types in this contract have been replaced with taxonomy VOs.
//   * `u32 line` → `LineNumber line` (semantic line position)
//   * `usize changes` → `Count changes` (semantic count of modifications)
//   * `&[LintResult]` → `&[LintResult]` (LintResult is already a VO aggregate)
//   * `&str file_path` → kept as `&str` (idiomatic borrow for path strings)
//   * `bool` → `FixOutcome` (reason-coded outcome per FRD: Applied / Skipped / Failed)
//   * `Vec<String>` → `Vec<LintMessage>` (lint messages, not raw strings)
//
// FRD API Contract alignment:
//   - `execute(path, dry_run)` — FR-004: per-request dry_run (not construction-time only)
//   - `rename_symbol(path, old, new)` — FR-003: public symbol rename operation
//   - `report_non_fixable(violations)` — FR-005: non-fixable violation reporting
//   - `emit_fix_event` / `is_fixable` / `fixable_codes` removed from protocol
//     (internal implementation details, not part of the public FRD API Contract)
use crate::auto_fix::taxonomy_fix_outcome_vo::FixOutcome;
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;

/// Protocol trait for auto-fix operations.
///
/// Aligned with the FRD API Contract (7 operations):
/// 1. Execute fixes (per-request dry_run)
/// 2. Apply bypass fix
/// 3. Apply unused-import fix
/// 4. Apply symbol rename (FR-003)
/// 5. Report non-fixable (FR-005)
///
/// Internal helpers (`emit_fix_event`, `is_fixable`, `fixable_codes`) are
/// implementation details — not part of the public contract.
pub trait IFixProtocol: Send + Sync {
    /// FR-001/002/003/004: Run linter, filter fixable violations, apply fixes.
    /// `dry_run` is selectable per request (FR-004 assumption §9).
    fn execute(&self, path: &FilePath, dry_run: bool) -> FixResult;

    /// FR-002: Remove or replace bypass at the specified line.
    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> FixOutcome;

    /// FR-001: Remove unused import at the specified line.
    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> FixOutcome;

    /// FR-003: Rename a symbol across the file (mechanical `renamed_` prefix).
    fn rename_symbol(&self, file_path: &str, old_name: &str, new_name: &str) -> FixOutcome;

    /// FR-005: List violations that require manual intervention.
    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage>;
}
