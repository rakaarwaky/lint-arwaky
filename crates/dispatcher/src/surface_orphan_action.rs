// PURPOSE: Orphan rules scan business logic, no formatting.
// Adapted: sync — all orphan calls are sync. Uses injected factories for
// filesystem and orphan aggregate creation to avoid direct root-container
// instantiation (AES201 compliance — surface layer must not bypass contracts).
use std::sync::Arc;
use tracing::debug;

use shared::common::FilePath;
use shared::config_system::{ArchitectureConfig, ConfigLanguage, IConfigOrchestratorAggregate};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::orphan_rules::IOrphanAggregate;

use crate::surface_output_component::ViolationItem;

/// Factory function type: creates a fresh filesystem aggregate (uncached pipeline).
pub type FilesystemFactory = dyn Fn() -> Arc<dyn IFilesystemAggregate> + Send + Sync;

/// Factory function type: creates an orphan aggregate from config and filesystem.
pub type OrphanFactory =
    dyn Fn(ArchitectureConfig, Arc<dyn IFilesystemAggregate>) -> Arc<dyn IOrphanAggregate>
        + Send
        + Sync;

/// DI container for all aggregates needed by orphan scanning.
pub struct OrphanScanDeps {
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub fs_agg: Arc<dyn IFilesystemAggregate>,
    /// Factory for creating fresh filesystem instances (multi-workspace / single-root).
    pub fs_factory: Arc<FilesystemFactory>,
    /// Factory for creating orphan aggregate from config + filesystem.
    pub orphan_factory: Arc<OrphanFactory>,
}

impl OrphanScanDeps {
    /// Create deps with default factories that instantiate root containers directly.
    /// Use this at entry points where root containers are available.
    pub fn with_defaults(
        orphan_orchestrator: Arc<dyn IOrphanAggregate>,
        config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
        fs_agg: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            orphan_orchestrator,
            config_orchestrator,
            fs_agg,
            fs_factory: Arc::new(|| {
                filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
            }),
            orphan_factory: Arc::new(|config, fs| {
                orphan_rules::root_orphan_detector_container::OrphanContainer::new_with_config(
                    config, fs,
                )
                .analyzer()
            }),
        }
    }
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
            &deps.fs_factory,
            &deps.orphan_factory,
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

    // Build a single unified filesystem across ALL workspace members so the orphan
    // scanner can see cross-member imports (e.g., addition importing from shared).
    // Build once from the workspace root to discover all source files.
    let unified_fs: Arc<dyn IFilesystemAggregate> = (deps.fs_factory)();
    let root_path = std::path::Path::new(&root);
    // Collect ignored paths from all members
    let mut all_ignored: Vec<String> = Vec::new();
    for ws in workspaces.iter() {
        let lang = ws
            .workspace_type
            .parse::<ConfigLanguage>()
            .unwrap_or(ConfigLanguage::Rust);
        let ignored = deps
            .config_orchestrator
            .ignored_paths_for_language(&ws.path, lang);
        all_ignored.extend(ignored.values.iter().cloned());
    }
    unified_fs.build_file_index_with_ignored(root_path, &all_ignored);

    // Use the first workspace's config for the orchestrator (configs should be similar)
    let first_ws = &workspaces[0];
    let unified_orchestrator: Arc<dyn IOrphanAggregate> =
        (deps.orphan_factory)(first_ws.config.clone(), unified_fs.clone());

    // Build unified file list from all members
    let all_file_list = unified_fs.file_list();
    let root_abs = std::env::current_dir().unwrap_or_default().join(&root);
    let ws_top_root = unified_fs
        .workspace_root(&FilePath::new(root_abs.to_string_lossy().to_string()).unwrap_or_default());
    let top_root = ws_top_root.unwrap_or_else(|| root_abs.clone());
    let top_root_str = top_root.to_string_lossy().to_string();
    let all_file_paths: Vec<String> = all_file_list
        .iter()
        .map(|f| {
            let path_str = f.path.to_string_lossy().to_string();
            if let Ok(canon) = std::fs::canonicalize(&f.path) {
                let canon_str = canon.to_string_lossy().to_string();
                if let Some(rel) = canon_str.strip_prefix(&top_root_str) {
                    rel.strip_prefix('/').unwrap_or(rel).to_string()
                } else {
                    path_str
                }
            } else if let Some(rel) = path_str.strip_prefix(&top_root_str) {
                rel.strip_prefix('/').unwrap_or(rel).to_string()
            } else {
                path_str
            }
        })
        .collect();
    let unified_orphan_files =
        shared::orphan_rules::taxonomy_orphan_contract_vo::OrphanFileListVO::new(all_file_paths);

    // Build ONE graph context from ALL files — this sees cross-member imports.
    // Use top_root (absolute workspace root) as root_dir so that path resolution
    // in _check_orphans_inner computes the correct top_root for joining relative
    // file paths. Using a member path (first_ws.path) would cause path mismatch
    // since unified_orphan_files are relative to the workspace root.
    let root_fp = FilePath::new(root.clone()).map_err(|_| "invalid path".to_string())?;
    let unified_context =
        unified_orchestrator.build_orphan_graph_context(&unified_orphan_files, &root_fp);

    // Now run orphan checks using unified file list so cross-member imports are visible.
    // Violations are filtered to each member afterward.
    let results = unified_orchestrator.check_orphans_with_context(
        &unified_orphan_files,
        &root_fp,
        &unified_context,
    );

    // Filter results per member — violation file paths are relative to workspace root
    // (e.g., "crates/shared/src/taxonomy_operation_vo.rs"), member paths are like "crates/shared"
    for ws in workspaces.iter() {
        // member_path relative to workspace root: e.g., "crates/shared"
        // BUG-FIX: ws.path.value may be relative (e.g., "workspaces-bad/crates/foo")
        // while top_root_str is absolute. Canonicalize the member path first so
        // strip_prefix works correctly in both relative and absolute input modes.
        let ws_canonical = std::fs::canonicalize(&ws.path.value)
            .unwrap_or_else(|_| top_root.join(&ws.path.value));
        let member_rel = std::path::PathBuf::from(&ws_canonical)
            .strip_prefix(&top_root_str)
            .unwrap_or_else(|_| std::path::Path::new(&ws.path.value))
            .to_path_buf();
        let member_rel_str = member_rel.to_string_lossy().to_string();
        let filtered: Vec<_> = results
            .iter()
            .filter(|r| r.file.value.starts_with(&member_rel_str))
            .cloned()
            .collect();
        debug!(
            ws = %ws.path.value,
            violations = filtered.len(),
            "orphan results for member",
        );

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
    _orphan_orchestrator: &Arc<dyn IOrphanAggregate>,
    config_orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
    filter: &Option<String>,
    _fs_agg: &Arc<dyn IFilesystemAggregate>,
    fs_factory: &Arc<FilesystemFactory>,
    orphan_factory: &Arc<OrphanFactory>,
) -> Result<Vec<ViolationItem>, String> {
    // Create a fresh filesystem instance via factory (no direct root-container)
    let ws_filesystem: Arc<dyn IFilesystemAggregate> = fs_factory();

    // Load config for this path to get ignored_paths
    let ws_config = config_orchestrator.load_config_sync(root_fp);

    // Build file index for this workspace (respects config ignored_paths)
    let root_path = std::path::Path::new(root);
    let ignored_strs: Vec<String> = ws_config
        .ignored_paths
        .values
        .iter()
        .map(|fp| fp.value().to_string())
        .collect();
    ws_filesystem.build_file_index_with_ignored(root_path, &ignored_strs);
    let ws_orchestrator: Arc<dyn IOrphanAggregate> =
        orphan_factory(ws_config, ws_filesystem.clone());

    // Build OrphanFileListVO — paths relative to workspace root (top_root)
    let file_list = ws_filesystem.file_list();
    let root_abs = std::env::current_dir().unwrap_or_default().join(root);
    let ws_top_root = ws_filesystem.workspace_root(root_fp);
    let top_root = ws_top_root.unwrap_or_else(|| root_abs.clone());
    let top_root_str = top_root.to_string_lossy().to_string();
    let file_paths: Vec<String> = file_list
        .iter()
        .map(|f| {
            let path_str = f.path.to_string_lossy().to_string();
            if let Ok(canon) = std::fs::canonicalize(&f.path) {
                let canon_str = canon.to_string_lossy().to_string();
                if let Some(rel) = canon_str.strip_prefix(&top_root_str) {
                    rel.strip_prefix('/').unwrap_or(rel).to_string()
                } else {
                    path_str
                }
            } else if let Some(rel) = path_str.strip_prefix(&top_root_str) {
                rel.strip_prefix('/').unwrap_or(rel).to_string()
            } else {
                path_str
            }
        })
        .collect();
    let orphan_files =
        shared::orphan_rules::taxonomy_orphan_contract_vo::OrphanFileListVO::new(file_paths);

    // Build graph context from filesystem's pre-built data
    let context = ws_orchestrator.build_orphan_graph_context(&orphan_files, root_fp);

    // Run orphan checks on pre-fetched data with correct root_dir
    let results = ws_orchestrator.check_orphans_with_context(&orphan_files, root_fp, &context);

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
