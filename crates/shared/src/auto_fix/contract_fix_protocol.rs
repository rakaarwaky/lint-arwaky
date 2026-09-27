// PURPOSE: auto-fix-domain capability contracts (AES102 `_protocol`).
//
// One file for the auto-fix feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs. Each FR-backed trait maps to exactly
// one `FR-AutoFix-NNN` heading in `crates/auto-fix/FRD.md`.

use crate::auto_fix::taxonomy_fix_vo::FixOutcome;
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_lint_result_vo::LintResult;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_name_vo::SymbolName;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;

/// Infrastructure seam (no FR): file reads and writes behind one adapter.
pub trait IFileAdapterProtocol: Send + Sync {
    fn read_file(&self, path: &FilePath) -> Option<ContentString>;
    fn write_file(&self, path: &FilePath, content: &ContentString) -> bool;
    fn path_exists(&self, path: &FilePath) -> bool;
}

/// FR-AutoFix-001: remove an unused import line.
pub trait IUnusedImportFixProtocol: Send + Sync {
    /// FR-001: Remove unused import at the specified line.
    fn fix_unused_import(&self, file_path: &str, line: LineNumber) -> FixOutcome;
}

/// FR-AutoFix-002: remove or replace a bypass pattern on the target line.
pub trait IBypassFixProtocol: Send + Sync {
    /// FR-002: Remove or replace bypass at the specified line.
    fn fix_bypass_comments(&self, file_path: &str, line: LineNumber) -> FixOutcome;
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
}

/// FR-AutoFix-004: run the whole fix pipeline, with per-request dry-run.
pub trait IFixPipelineProtocol: Send + Sync {
    /// FR-001/002/003/004: Run linter, filter fixable violations, apply fixes.
    /// `dry_run` is selectable per request (FR-004 assumption §9).
    fn execute(&self, path: &FilePath, dry_run: bool) -> FixResult;
}

/// FR-AutoFix-005: list violations that require manual intervention.
pub trait IManualReportProtocol: Send + Sync {
    /// FR-005: List violations that require manual intervention.
    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage>;
}
