use crate::surface_common_action;
use crate::surface_output_component::{ViolationItem, output_violations};
use shared::cli_commands::Format;
use shared::common::{ExitCode, FilePath};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

use shared::config_system::{ConfigLanguage, IConfigOrchestratorAggregate};
use shared::orphan_detector::IOrphanAggregate;
use std::sync::Arc;

pub fn handle_scan_orphan(
    path: Option<FilePath>,
    member: Option<String>,
    format: Format,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::IReportFormatterAggregate>,
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
        Err(_) => {
            eprintln!("[error] invalid path: {root}");
            return ExitCode::RUNTIME_ERROR;
        }
    };

    let rt = match surface_common_action::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };

    let workspaces = rt.block_on(config_orchestrator.discover_workspaces(&root_fp));

    if workspaces.is_empty() {
        return scan_single_root(
            &root,
            &orphan_orchestrator,
            &config_orchestrator,
            format,
            &filter,
            &fs_agg,
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
            eprintln!("[error] no workspace member matching '{member_name}'");
            return ExitCode::RUNTIME_ERROR;
        }
        filtered
    } else {
        workspaces
    };

    let is_specific_member = member.is_some();

    let mut all_violations: Vec<ViolationItem> = Vec::new();

    for ws in workspaces.iter() {
        let lang = ws
            .workspace_type
            .parse::<ConfigLanguage>()
            .unwrap_or(ConfigLanguage::Rust);
        let ignored = config_orchestrator.ignored_paths_for_language(&ws.path, lang);
        // Build the orphan analyzer from THIS workspace's real config (layers + enabled)
        let orphan_analyzer =
            orphan_detector::root_orphan_detector_container::OrphanContainer::from_orchestrator(
                &config_orchestrator,
                &ws.path.value,
            )
            .analyzer();
        let (_, results) = orphan_analyzer.scan_orphans(&ws.path, ignored.values());

        // scan_orphans returns file paths relative to the workspace top_root
        // (found by find_workspace_root). We need to find the workspace member
        // prefix to filter results belonging to this specific workspace.
        // Use absolute paths for correct strip_prefix comparison.
        let cwd = std::env::current_dir().unwrap_or_default();
        let ws_abs = cwd.join(&ws.path.value);
        let ws_top_root = fs_agg.workspace_root(&ws_abs.to_string_lossy());
        let ws_prefix = ws_top_root.as_ref().and_then(|top_root| {
            ws_abs
                .strip_prefix(top_root)
                .ok()
                .map(|p| p.to_string_lossy().to_string())
        });

        let filtered: Vec<_> = results
            .into_iter()
            .filter(|r| {
                // File paths from scan_orphans are relative to workspace top_root.
                // ws_prefix is the workspace relative to top_root.
                // A file belongs to this workspace if its path starts with ws_prefix.
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

    // Use workspace root as target for proper member grouping
    let target = if is_specific_member {
        let member_path = std::path::Path::new(&root).join(member.as_deref().unwrap_or(""));
        if member_path.exists() {
            member_path.to_string_lossy().to_string()
        } else {
            root.clone()
        }
    } else {
        root.clone()
    };

    // Apply filter by AES rule code
    if let Some(filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        all_violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    output_violations(
        &all_violations,
        &target,
        format,
        is_specific_member || fs_agg.is_member_path(&target),
    );

    if all_violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}

fn scan_single_root(
    root: &str,
    _orphan_orchestrator: &Arc<dyn IOrphanAggregate>,
    config_orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
    format: Format,
    filter: &Option<String>,
    fs_agg: &Arc<dyn IFilesystemAggregate>,
) -> ExitCode {
    let scan_root = crate::surface_common_action::resolve_file_path(root);
    let lang = fs_agg.detect_language_from_path(root);
    let ignored = config_orchestrator.ignored_paths_for_language(&scan_root, lang);
    let orphan_analyzer =
        orphan_detector::root_orphan_detector_container::OrphanContainer::from_orchestrator(
            config_orchestrator,
            &scan_root.value,
        )
        .analyzer();
    let (_, results) = orphan_analyzer.scan_orphans(&scan_root, ignored.values());

    let mut violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    if let Some(filter_str) = filter {
        let filter_upper = filter_str.to_uppercase();
        violations.retain(|v| v.code.code().contains(&filter_upper));
    }

    output_violations(&violations, root, format, fs_agg.is_member_path(root));

    if violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}
