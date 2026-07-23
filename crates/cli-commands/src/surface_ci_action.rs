// PURPOSE: CI entry point — thin wrapper for CI threshold validation surface action
use std::process::ExitCode;
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_threshold_vo::Threshold;

pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    crate::surface_common_command::run_ci_analysis(code_analysis_linter, path, threshold)
}
