// PURPOSE: Git diff — CLI thin wrapper
// Calls dispatcher for git-diff business logic, only adds CLI output
use shared::common::{ExitCode, FilePath, GitBranchName};
use shared::quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;

pub fn handle_git_diff(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    base: GitBranchName,
    project_path: Option<&str>,
    filter: Option<&str>,
) -> ExitCode {
    // Delegate to dispatcher
    dispatcher::surface_git_action::handle_git_diff(code_analysis_linter, base, project_path, filter)
}
