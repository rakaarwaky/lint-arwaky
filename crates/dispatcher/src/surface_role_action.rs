// PURPOSE: Role rules scan business logic, no formatting.
//
// All role scanning runs in-memory through `collect_role_direct`; the
// subprocess self-invocation fallback was removed (current_exe() is
// forbidden by the architecture rules).
use std::sync::Arc;

use shared_common::FilePath;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_role_rules::IRoleRunnerAggregate;
use shared_role_rules::taxonomy_role_rules_request::RoleRequest;

use shared_common::ViolationItem;

/// Direct role scan — no subprocess. Used by the CLI `role` subcommand so that
/// subprocess self-invocation from `scan` terminates (child never re-spawns).
pub fn collect_role_direct(
    role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
    root: &str,
    ignored_paths: &[String],
) -> Result<Vec<ViolationItem>, String> {
    // Build file index for target path
    let root_path = std::path::Path::new(root);
    fs_agg.execute(FilesystemRequest::build_file_index_with_ignored(
        root_path,
        ignored_paths,
    ));

    // Pass pre-fetched FileEntry data to role orchestrator
    let results = role_orchestrator
        .execute(RoleRequest::audit(
            &fs_agg.execute(FilesystemRequest::FileList).into_file_list(),
        ))
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

pub fn collect_role(
    path: Option<FilePath>,
    role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    filter: Option<String>,
    fs_agg: Arc<dyn IFilesystemAggregate>,
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

    // In-memory role scan — no subprocess self-invocation
    // (current_exe() is forbidden by the architecture rules).
    let ignored: Vec<String> = Vec::new();
    collect_role_direct(role_orchestrator, filter, fs_agg, &root, &ignored)
}
