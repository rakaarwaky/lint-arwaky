// PURPOSE: FixOrchestrator — orchestrates auto-fix operations via the FR-backed
// protocol traits (agent layer)
//
// The auto-fix feature applies safe, automatic fixes to common violations.
// Only RO removal operations are automated — no code is added or modified,
// only unused/forbidden imports are deleted, and bypass comments are removed.
//
// This orchestrator bridges the auto-fix capability protocols (capabilities
// layer) to the IFixAggregate contract (surface layer). It's intentionally
// thin — all fix logic lives in the individual capability structs.
//
// Safety policy:
//   - AES101 (naming):     YES — mechanical rename with `renamed_` prefix
//   - AES203 (unused import): YES — safe to remove the import line
//   - AES304 (bypass comment): YES — safe to remove the bypass comment
//   - All others:         NO  — require manual review
//
// Changes from previous version:
// - V3: Split LintFixProcessor into 4 focused capabilities, one per FR.
// - V3: Replaced IFixPipelineProtocol + IManualReportProtocol with
//       IViolationReportProtocol (FR-004 combines dry-run + reporting).
// - V3: Removed FixRequest::ManualReport variant — all work flows through
//       FixRequest::Execute.

use shared_auto_fix::contract_fix_aggregate::IFixAggregate;
use shared_auto_fix::taxonomy_auto_fix_request::FixRequest;
use shared_auto_fix::taxonomy_auto_fix_response::FixResponse;
use shared_auto_fix::{
    FixOutcome, IBypassFixProtocol, ISymbolRenameProtocol, IUnusedImportFixProtocol,
    IViolationReportProtocol,
};
use shared_common::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

/// FixOrchestratorDeps — one capability seam per FR-AutoFix requirement.
pub struct FixOrchestratorDeps {
    pub violation_report: Arc<dyn IViolationReportProtocol>,
    pub bypass_fix: Arc<dyn IBypassFixProtocol>,
    pub unused_import_fix: Arc<dyn IUnusedImportFixProtocol>,
    pub symbol_rename: Arc<dyn ISymbolRenameProtocol>,
}

/// FixOrchestrator — pure delegation to the auto-fix capability protocols.
///
/// No business logic — just wires the aggregate contract to the fix processor.
pub struct FixOrchestrator {
    deps: FixOrchestratorDeps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl IFixAggregate for FixOrchestrator {
    fn execute(&self, request: FixRequest) -> FixResponse {
        match request {
            FixRequest::Execute { path, dry_run } => FixResponse::Execute {
                result: self.deps.violation_report.report_violations(&path, dry_run),
            },
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl FixOrchestrator {
    pub fn new(deps: FixOrchestratorDeps) -> Self {
        Self { deps }
    }

    /// Convenience: apply a single bypass fix at the given line.
    pub fn fix_bypass(&self, file_path: &str, line: u32) -> FixOutcome {
        self.deps
            .bypass_fix
            .fix_bypass_comments(file_path, shared_common::LineNumber::new(line as i64))
    }

    /// Convenience: apply a single unused-import fix at the given line.
    pub fn fix_unused_import(&self, file_path: &str, line: u32) -> FixOutcome {
        self.deps
            .unused_import_fix
            .fix_unused_import(file_path, shared_common::LineNumber::new(line as i64))
    }

    /// Convenience: rename a symbol across the file (FR-003).
    pub fn rename_symbol(
        &self,
        file_path: &str,
        old_name: &SymbolName,
        new_name: &SymbolName,
    ) -> FixOutcome {
        self.deps
            .symbol_rename
            .rename_symbol(file_path, old_name, new_name)
    }
}
