// PURPOSE: Quality rules scan business logic, no formatting.
// Adapted: receives ICodeAnalysisAggregate via DI instead of creating it from config.
// No direct code_analysis crate dependency — only through shared contracts.
use shared::quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;

use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use shared::common::ViolationItem;

pub fn collect_quality(
    path: Option<FilePath>,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
    ignored_paths: &[String],
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !fs_agg.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }
    let root_fp = FilePath::new(root).map_err(|_| "invalid path".to_string())?;

    // Build file index first — filesystem discovers files, reads content, parses AST
    let root_path = std::path::Path::new(root_fp.value());
    fs_agg.build_file_index_with_ignored(root_path, ignored_paths);

    // Pass pre-fetched FileEntry data to quality orchestrator
    let file_list = fs_agg.file_list();
    let results = code_analysis_linter.run_analysis_with_entries(file_list);
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
