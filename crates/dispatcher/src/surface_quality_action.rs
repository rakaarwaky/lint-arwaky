// PURPOSE: Quality rules scan business logic, no formatting.
// Adapted: receives ICodeAnalysisAggregate via DI instead of creating it from config.
// No direct code_analysis crate dependency — only through shared contracts.
use shared::quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;

use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use crate::surface_output_component::ViolationItem;

pub fn collect_quality(
    path: Option<FilePath>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filter: Option<String>,
    _fs_agg: Arc<dyn IFilesystemAggregate>,
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        return Err(format!("Error: path '{}' does not exist", root));
    }
    let root_fp = FilePath::new(root).map_err(|_| "invalid path".to_string())?;

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

    Ok(violations)
}
