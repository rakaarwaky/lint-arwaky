// PURPOSE: LintFixOrchestratorAggregate — aggregate trait for auto-fix orchestration
use crate::auto_fix::contract_file_adapter_protocol::IFileAdapterProtocol;
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

/// Aggregate that drives the auto-fix pipeline for a single file.
///
/// Implementations coordinate protocol dependencies (file adapter, renamer,
/// etc.) and produce a [`FixResult`] summarising what was changed or why
/// the fix could not be applied.
pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;

    /// Expose the file adapter so that upstream consumers (e.g. mcp-server)
    /// can access file I/O without depending on `IFileAdapterProtocol` directly.
    fn file_adapter(&self) -> Arc<dyn IFileAdapterProtocol>;
}
