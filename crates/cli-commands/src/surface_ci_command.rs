// PURPOSE: CI entry point — CLI thin wrapper
// Calls dispatcher::surface_ci_action for business logic, only adds CLI output
use shared::common::ExitCode;
use std::sync::Arc;

use shared::common::{FilePath, Threshold};
use shared::config_system::IConfigOrchestratorAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;

#[allow(clippy::too_many_arguments)]
pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    // Resolve path
    let root = match &path {
        Some(p) => p.clone(),
        None => FilePath::new(".").unwrap_or_default(),
    };

    // Delegate to dispatcher for business logic
    dispatcher::surface_ci_action::handle_ci(
        &root,
        code_analysis_linter.as_ref(),
        import_orchestrator.as_ref(),
        naming_orchestrator.as_ref(),
        orphan_orchestrator.as_ref(),
        filesystem.as_ref(),
        threshold.value() as f64,
    )
}
