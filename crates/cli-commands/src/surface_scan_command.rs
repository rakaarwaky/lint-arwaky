// PURPOSE: Check command — CLI thin wrapper
// Calls dispatcher for quality scan, only adds CLI output
use shared::common::ExitCode;
use std::sync::Arc;

use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;

pub fn handle_check(
    path: Option<FilePath>,
    format: Format,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    _config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    filter: Option<String>,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };

    if !filesystem.path_exists(std::path::Path::new(&root)) {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::RUNTIME_ERROR;
    }

    let root_fp = match FilePath::new(root.clone()) {
        Ok(fp) => fp,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };

    // Delegate to dispatcher
    dispatcher::surface_quality_action::handle_scan_quality(
        Some(root_fp),
        format,
        code_analysis_linter,
        filter,
        filesystem,
    )
}
