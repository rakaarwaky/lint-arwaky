// PURPOSE: Import rules scan business logic, no formatting.
// Adapted: sync — IImportRunnerAggregate::run_audit is now sync. No tokio runtime.
use std::sync::Arc;

use shared_common::FilePath;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_import_rules::IImportRunnerAggregate;
use shared_import_rules::taxonomy_import_rules_request::ImportRequest;

use shared_common::ViolationItem;

pub fn collect_import(
    path: Option<FilePath>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
    ignored_paths: &[String],
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !fs_agg
        .execute(FilesystemRequest::path_exists(std::path::Path::new(&root)))
        .into_path_exists()
    {
        return Err(format!("Error: path '{}' does not exist", root));
    }
    let root_fp = FilePath::new(root).map_err(|_| "invalid path".to_string())?;

    // Build file index first — filesystem discovers files, reads content, parses imports
    let root_path = std::path::Path::new(root_fp.value());
    fs_agg.execute(FilesystemRequest::build_file_index_with_ignored(
        root_path,
        ignored_paths,
    ));

    // Pass pre-fetched FileEntry data to import orchestrator
    let file_list = fs_agg.execute(FilesystemRequest::FileList).into_file_list();
    let results = import_orchestrator
        .execute(ImportRequest::audit_with_entries(&file_list))
        .into_violations();

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
