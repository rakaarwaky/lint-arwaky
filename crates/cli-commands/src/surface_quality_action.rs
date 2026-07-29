use shared::common::ExitCode;
use std::sync::Arc;

use shared::cli_commands::utility_path_resolver::is_member_path;
use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::common::LayerMapVO;
use shared::config_system::IConfigOrchestratorAggregate;

use crate::surface_output_component::{output_violations, ViolationItem};

pub fn handle_scan_quality(
    path: Option<FilePath>,
    format: Format,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    filter: Option<String>,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::RUNTIME_ERROR;
    }
    let root_fp = match FilePath::new(root.clone()) {
        Ok(fp) => fp,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };

    // Load config from the target path, not from "."
    let config = config_orchestrator.load_config_sync(&root_fp);
    let layer_map = LayerMapVO::new(config.layers.clone());
    let container =
        code_analysis::root_code_analysis_container::CodeAnalysisContainer::new_with_config(
            config, layer_map,
        );
    let linter = container.code_analysis_linter();

    let results = linter.run_code_analysis_path(&root_fp);
    let mut violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    output_violations(&violations, &root, format, is_member_path(&root));
    if violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}
