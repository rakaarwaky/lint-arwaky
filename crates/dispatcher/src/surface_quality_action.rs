// PURPOSE: Quality rules scan surface action
// Adapted: receives ICodeAnalysisAggregate via DI instead of creating it from config.
// No direct code_analysis crate dependency — only through shared contracts.
use shared::common::ExitCode;
use shared::quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;

use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use crate::surface_output_component::{ViolationItem, output_violations};

pub fn handle_scan_quality(
    path: Option<FilePath>,
    format: Format,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
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

    // Use the injected linter directly — per-config loading handled at binary level
    let results = code_analysis_linter.run_code_analysis_path(&root_fp);
    let mut violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    if let Some(ref filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    output_violations(&violations, &root, format, fs_agg.is_member_path(&root_fp));
    if violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}
