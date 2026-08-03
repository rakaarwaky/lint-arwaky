// PURPOSE: Orphan rules scan business logic, no formatting.
// Adapted: sync — all orphan calls are sync. Removed orphan_detector crate dependency;
// uses injected IOrphanAggregate via DI. Simplified workspace handling.
use std::sync::Arc;

use shared::common::FilePath;
use shared::config_system::{ConfigLanguage, IConfigOrchestratorAggregate};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_rules::IOrphanAggregate;

use crate::surface_output_component::ViolationItem;

/// DI container for all aggregates needed by orphan scanning.
pub struct OrphanScanDeps {
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub fs_agg: Arc<dyn IFilesystemAggregate>,
}

pub fn collect_orphan(
    path: Option<FilePath>,
    member: Option<String>,
    deps: OrphanScanDeps,
    filter: Option<String>,
) -> Result<Vec<ViolationItem>, String> {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !deps.fs_agg.path_exists(std::path::Path::new(&root)) {
        return Err(format!("Error: path '{}' does not exist", root));
    }

    let root_fp = FilePath::new(root.clone()).map_err(|_| "invalid path".to_string())?;

    // discover_workspaces is sync in new API
    let workspaces = deps.config_orchestrator.discover_workspaces(&root_fp);

    if workspaces.is_empty() {
        return scan_single_root(
            &root,
            &root_fp,
            &deps.orphan_orchestrator,
            &deps.config_orchestrator,
            &filter,
            &deps.fs_agg,
        );
    }

    let workspaces = if let Some(ref member_name) = member {
        let all_workspaces = workspaces;
        let filtered: Vec<_> = all_workspaces
            .into_iter()
            .filter(|ws| {
                let ws_file = std::path::Path::new(&ws.path.value)
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                ws_file.as_ref() == member_name || ws.path.value == *member_name
            })
            .collect();
        if filtered.is_empty() {
            return Err(format!(
                "[error] no workspace member matching '{member_name}'"
            ));
        }
        filtered
    } else {
        workspaces
    };

    let mut all_violations: Vec<ViolationItem> = Vec::new();

    for ws in workspaces.iter() {
        let lang = ws
            .workspace_type
            .parse::<ConfigLanguage>()
            .unwrap_or(ConfigLanguage::Rust);
        let _ignored = deps
            .config_orchestrator
            .ignored_paths_for_language(&ws.path, lang);

        // Build file index for this workspace
        let ws_path = std::path::Path::new(ws.path.value.as_str());
        deps.fs_agg.build_file_index(ws_path);

        // Build OrphanFileListVO from pre-fetched FileEntry data
        let file_list = deps.fs_agg.file_list();
        let file_paths: Vec<String> = file_list
            .iter()
            .map(|f| f.path.to_string_lossy().to_string())
            .collect();
        let orphan_files =
            shared::orphan_rules::taxonomy_orphan_contract_vo::OrphanFileListVO::new(file_paths);

        // Build graph context from filesystem's pre-built data
        let context = deps
            .orphan_orchestrator
            .build_orphan_graph_context(&orphan_files, &ws.path);

        // Run orphan checks on pre-fetched data with correct root_dir
        let results =
            deps.orphan_orchestrator
                .check_orphans_with_context(&orphan_files, &ws.path, &context);

        // Filter results belonging to this workspace
        let ws_abs = std::env::current_dir()
            .unwrap_or_default()
            .join(&ws.path.value);
        let ws_top_root = deps.fs_agg.workspace_root(
            &FilePath::new(ws_abs.to_string_lossy().to_string()).unwrap_or_default(),
        );
        let ws_prefix = ws_top_root.as_ref().and_then(|top_root| {
            ws_abs
                .strip_prefix(top_root)
                .ok()
                .map(|p| p.to_string_lossy().to_string())
        });

        let filtered: Vec<_> = results
            .into_iter()
            .filter(|r| {
                if let Some(ref prefix) = ws_prefix {
                    r.file.value.starts_with(prefix.as_str())
                        && (r.file.value.len() == prefix.len()
                            || r.file.value[prefix.len()..].starts_with('/'))
                } else {
                    true
                }
            })
            .collect();

        for r in &filtered {
            all_violations.push(ViolationItem::from_lint_result(r));
        }
    }

    if let Some(filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        all_violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    Ok(all_violations)
}

fn scan_single_root(
    root: &str,
    root_fp: &FilePath,
    orphan_orchestrator: &Arc<dyn IOrphanAggregate>,
    _config_orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
    filter: &Option<String>,
    fs_agg: &Arc<dyn IFilesystemAggregate>,
) -> Result<Vec<ViolationItem>, String> {
    // Build file index first — filesystem discovers files, reads content, parses AST
    let root_path = std::path::Path::new(root);
    fs_agg.build_file_index(root_path);

    // Build OrphanFileListVO from pre-fetched FileEntry data
    let file_list = fs_agg.file_list();
    let file_paths: Vec<String> = file_list
        .iter()
        .map(|f| f.path.to_string_lossy().to_string())
        .collect();
    let orphan_files =
        shared::orphan_rules::taxonomy_orphan_contract_vo::OrphanFileListVO::new(file_paths);

    // Build graph context from filesystem's pre-built data
    let context = orphan_orchestrator.build_orphan_graph_context(&orphan_files, root_fp);

    // Run orphan checks on pre-fetched data with correct root_dir
    let results = orphan_orchestrator.check_orphans_with_context(&orphan_files, root_fp, &context);

    let mut violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    if let Some(filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    Ok(violations)
}
