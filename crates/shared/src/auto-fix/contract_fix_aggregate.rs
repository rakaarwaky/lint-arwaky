// PURPOSE: LintFixOrchestratorAggregate — aggregate trait for auto-fix orchestration
use crate::auto_fix::taxonomy_fix_vo::FixResult;
use crate::common::taxonomy_path_vo::FilePath;

/// Aggregate that drives the auto-fix pipeline for a single file.
///
/// Implementations coordinate protocol dependencies (file adapter, renamer,
/// etc.) and produce a [`FixResult`] summarising what was changed or why
/// the fix could not be applied.
pub trait LintFixOrchestratorAggregate: Send + Sync {
    fn execute(&self, path: &FilePath) -> FixResult;
}
