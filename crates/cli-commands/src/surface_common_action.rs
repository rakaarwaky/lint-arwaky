// PURPOSE: Shared utilities for CLI command surfaces
//
// Provides:
//   - create_runtime / create_current_thread_runtime: tokio runtime factories
//   - resolve_file_path / canonicalize_path / current_dir: path resolution helpers
//   - run_ci_analysis: CI pipeline that runs code analysis, computes score, compares
//     against threshold, and returns pass/fail exit code. Detects CRITICAL violations
//     as auto-fail regardless of score.
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_common_error::ExitCode;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_threshold_vo::Threshold;
use std::sync::Arc;

pub fn create_runtime() -> Result<tokio::runtime::Runtime, ExitCode> {
    match tokio::runtime::Runtime::new() {
        Ok(r) => Ok(r),
        Err(_) => {
            eprintln!("[error] failed to create tokio runtime");
            Err(ExitCode::RUNTIME_ERROR)
        }
    }
}

pub fn create_current_thread_runtime() -> Result<tokio::runtime::Runtime, ExitCode> {
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => Ok(r),
        Err(_) => {
            eprintln!("[error] failed to create tokio runtime");
            Err(ExitCode::RUNTIME_ERROR)
        }
    }
}

pub fn resolve_file_path(path: &str) -> FilePath {
    FilePath::new(path.to_string()).unwrap_or_default()
}

pub fn canonicalize_path(path: &str) -> String {
    match std::path::Path::new(path).canonicalize() {
        Ok(p) => p.to_string_lossy().to_string(),
        Err(_) => path.to_string(),
    }
}

pub fn current_dir() -> std::path::PathBuf {
    match std::env::current_dir() {
        Ok(d) => d,
        Err(_) => std::path::PathBuf::new(),
    }
}

pub fn run_ci_analysis(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    import_orchestrator: Arc<
        dyn shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate,
    >,
    naming_orchestrator: Arc<
        dyn shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate,
    >,
    role_orchestrator: Arc<
        dyn shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate,
    >,
    orphan_orchestrator: Arc<
        dyn shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate,
    >,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    crate::surface_ci_command::handle_ci(
        code_analysis_linter,
        import_orchestrator,
        naming_orchestrator,
        role_orchestrator,
        orphan_orchestrator,
        path,
        threshold,
    )
}
