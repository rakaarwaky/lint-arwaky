// PURPOSE: LintFixOrchestratorAggregate — aggregate trait for auto-fix orchestration
//
// FRD API Contract alignment:
//   - `execute(path, dry_run)` — per-request dry_run (FR-004 assumption §9)
//   - `manual_report(violations)` — FR-005: non-fixable violation reporting
use crate::auto_fix::contract_file_adapter_protocol::IFileAdapterProtocol;
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

/// Aggregate that drives the auto-fix pipeline for a single file.
///
/// Implementations coordinate protocol dependencies (file adapter, renamer,
/// etc.) and produce a [`FixResult`] summarising what was changed or why
/// the fix could not be applied.
pub trait LintFixOrchestratorAggregate: Send + Sync {
    /// Run linter + apply fixes. `dry_run` is selectable per request.
    fn execute(&self, path: &FilePath, dry_run: bool) -> FixResult;

    /// FR-005: Report violations that require manual intervention.
    fn manual_report(&self, violations: &[LintResult]) -> Vec<String>;

    /// Expose the file adapter so that upstream consumers (e.g. mcp-server)
    /// can access file I/O without depending on `IFileAdapterProtocol` directly.
    fn file_adapter(&self) -> Arc<dyn IFileAdapterProtocol>;
}
