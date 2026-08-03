// PURPOSE: Import rules scan business logic, no formatting.
// Adapted: sync — IImportRunnerAggregate::run_audit is now sync. No tokio runtime.
use std::sync::Arc;

use shared::common::FilePath;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::import_rules::IImportRunnerAggregate;

use crate::surface_output_component::ViolationItem;

pub fn collect_import(
    path: Option<FilePath>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
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

    // run_audit is sync in new API — call directly
    let results =
        import_orchestrator
            .run_audit(&root_fp)
            .map_err(|e| format!("[error] import rules failed: {e}"))?;
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
