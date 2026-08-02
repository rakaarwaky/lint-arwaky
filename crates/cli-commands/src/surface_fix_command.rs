// PURPOSE: Fix command — CLI thin wrapper
// Calls dispatcher for fix business logic, only adds CLI output
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::{ExitCode, FilePath};
use shared::quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;

pub fn handle_fix(
    path: Option<FilePath>,
    dry_run: bool,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator_factory: Arc<
        dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync,
    >,
) -> ExitCode {
    // Delegate to dispatcher
    dispatcher::surface_fix_action::handle_fix(
        path,
        dry_run,
        code_analysis_linter,
        fix_orchestrator_factory,
    )
}
