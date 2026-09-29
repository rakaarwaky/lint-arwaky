// PURPOSE: auto-fix-domain capability contracts (AES102 `_protocol`).
//
// One file for the auto-fix feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs. Each FR-backed trait maps to exactly
// one `FR-AutoFix-NNN` heading in `crates/auto-fix/FRD.md`.
//
// FR count: exactly 4 FR-backed protocols.

use crate::auto_fix::taxonomy_auto_fix_vo::FixOutcome;
use crate::auto_fix::taxonomy_auto_fix_vo::FixResult;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
pub trait IUnusedImportFixProtocol: Send + Sync {
    /// FR-001: Remove unused import at the specified line.
    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> FixOutcome;
    /// Internal: apply the fix with a dry-run flag (used by FR-004 pipeline).
    fn fix_unused_import_dry(&self, file_path: &str, line: LineNumber, dry_run: bool)
    -> FixOutcome;
}

/// FR-AutoFix-002: remove or replace a bypass pattern on the target line.
pub trait IBypassFixProtocol: Send + Sync {
    /// FR-002: Remove or replace bypass at the specified line.
    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> FixOutcome;
    /// Internal: apply the fix with a dry-run flag (used by FR-004 pipeline).
    fn fix_bypass_dry(&self, file_path: &str, line: LineNumber, dry_run: bool) -> FixOutcome;
}

/// FR-AutoFix-003: word-boundary-aware mechanical symbol rename.
pub trait ISymbolRenameProtocol: Send + Sync {
    /// FR-003: Rename a symbol across the file (mechanical `renamed_` prefix).
    fn rename_symbol(
        &self,
        file_path: &str,
        old_name: &SymbolName,
        new_name: &SymbolName,
    ) -> FixOutcome;
    /// Internal: apply the rename with a dry-run flag (used by FR-004 pipeline).
    fn rename_symbol_dry(
        &self,
        file_path: &str,
        old_name: &str,
        new_name: &str,
        dry_run: bool,
    ) -> FixOutcome;
}

/// FR-AutoFix-004: run the fix pipeline and report non-fixable violations.
///
/// Combines the execution path (apply all fixable violations) and the
/// reporting path (list non-fixable violations) into one seam so that
/// each FR has exactly one protocol.
pub trait IViolationReportProtocol: Send + Sync {
    /// FR-004: Run linter, filter fixable violations, apply fixes, and
    /// return a result that includes both applied changes and the manual
    /// report of non-fixable violations.
    ///
    /// `dry_run` is selectable per request (FR-004 assumption §9).
    fn report_violations(&self, path: &FilePath, dry_run: bool) -> FixResult;

    /// FR-004: List violations that require manual intervention.
    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage>;
}
