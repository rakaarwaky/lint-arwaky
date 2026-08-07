// PURPOSE: FixOrchestrator — orchestrates auto-fix operations via IFixProtocol (agent layer)
//
// The auto-fix feature applies safe, automatic fixes to common violations.
// Only RO removal operations are automated — no code is added or modified,
// only unused/forbidden imports are deleted, and bypass comments are removed.
//
// This orchestrator bridges the IFixProtocol (capabilities layer) to the
// LintFixOrchestratorAggregate contract (surface layer). It's intentionally
// thin — all fix logic lives in LintFixProcessor.
//
// Safety policy:
//   - AES201 (forbidden import): YES — safe to remove the import line
//   - AES203 (unused import):    YES — safe to remove the import line
//   - AES304 (bypass comment):   YES — safe to remove the bypass comment
//   - All others:               NO  — require manual review
//
// Changes from previous version:
// - BF-2: `manual_report` now on aggregate trait (not just concrete struct)
// - BF-5: Removed duplicate `run_fix` — consolidated with aggregate `execute`
// - TR-2: Aggregate trait includes `manual_report` for FR-005

use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::{FixOutcome, FixResult, IFileAdapterProtocol, IFixProtocol};
use shared::common::taxonomy_lint_result_vo::LintResult;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

/// FixOrchestrator — pure delegation to IFixProtocol.
///
/// No business logic — just wires the aggregate contract to the fix processor.
pub struct FixOrchestrator {
    fix_protocol: Arc<dyn IFixProtocol>,
    file_adapter: Arc<dyn IFileAdapterProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl LintFixOrchestratorAggregate for FixOrchestrator {
    /// Per-request dry_run via parameter (BF-1, FR-004 assumption §9).
    fn execute(&self, path: &FilePath, dry_run: bool) -> FixResult {
        self.fix_protocol.execute(path, dry_run)
    }

    /// FR-005: Report violations that require manual intervention (BF-2).
    fn manual_report(&self, violations: &[LintResult]) -> Vec<LintMessage> {
        self.fix_protocol.report_non_fixable(violations)
    }

    fn file_adapter(&self) -> Arc<dyn IFileAdapterProtocol> {
        self.file_adapter.clone()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl FixOrchestrator {
    pub fn new(
        fix_protocol: Arc<dyn IFixProtocol>,
        file_adapter: Arc<dyn IFileAdapterProtocol>,
    ) -> Self {
        Self {
            fix_protocol,
            file_adapter,
        }
    }

    /// Convenience: apply a single bypass fix at the given line.
    pub fn fix_bypass(&self, file_path: &str, line: u32) -> FixOutcome {
        self.fix_protocol
            .fix_bypass_comments(file_path, shared::common::LineNumber::new(line as i64))
    }

    /// Convenience: apply a single unused-import fix at the given line.
    pub fn fix_unused_import(&self, file_path: &str, line: u32) -> FixOutcome {
        self.fix_protocol
            .fix_unused_import(file_path, shared::common::LineNumber::new(line as i64))
    }

    /// Convenience: rename a symbol across the file (FR-003).
    pub fn rename_symbol(&self, file_path: &str, old_name: &str, new_name: &str) -> FixOutcome {
        self.fix_protocol
            .rename_symbol(file_path, old_name, new_name)
    }
}
