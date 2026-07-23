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
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

/// FixOrchestrator — pure delegation to IFixProtocol.
///
/// No business logic — just wires the aggregate contract to the fix processor.
pub struct FixOrchestrator {
    fix_protocol: Arc<dyn IFixProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl LintFixOrchestratorAggregate for FixOrchestrator {
    fn execute(&self, path: &FilePath) -> FixResult {
        self.fix_protocol.execute(path)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl FixOrchestrator {
    pub fn new(fix_protocol: Arc<dyn IFixProtocol>) -> Self {
        Self { fix_protocol }
    }

    /// Execute the fix pipeline: lint → filter fixable → apply fixes.
    pub fn run_fix(&self, path: &FilePath) -> FixResult {
        self.fix_protocol.execute(path)
    }

    /// Get a report of violations that require manual intervention (not auto-fixable).
    pub fn manual_report(&self, violations: &[LintResult]) -> Vec<String> {
        self.fix_protocol
            .report_non_fixable(violations)
            .iter()
            .map(|m| m.to_string())
            .collect()
    }
}
