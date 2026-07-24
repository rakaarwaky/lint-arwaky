use crate::surface_common_command;
use crate::surface_output_component::{output_violations, ViolationItem};
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_scan_orphan(
    path: Option<FilePath>,
    member: Option<String>,
    format: Format,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    _report_formatter: Arc<dyn shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate>,
) -> ExitCode {
    let root = match &path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }

    let root_fp = match FilePath::new(root.clone()) {
        Ok(fp) => fp,
        Err(_) => {
            eprintln!("[error] invalid path: {root}");
            return ExitCode::from(2);
        }
    };

    let rt = match surface_common_command::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::from(2),
    };

    let workspaces = rt.block_on(config_orchestrator.discover_workspaces(&root_fp));

    if workspaces.is_empty() {
        return scan_single_root(&root, &orphan_orchestrator, &config_orchestrator);
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
            return ExitCode::from(2);
        }
        filtered
    } else {
        workspaces
    };

    let cwd = std::env::current_dir().unwrap_or_default();
    let is_specific_member = member.is_some();

    let mut all_violations: Vec<ViolationItem> = Vec::new();
    let workspace_canonicals: Vec<_> = workspaces
        .iter()
        .map(|ws| {
            let raw = std::path::Path::new(&ws.path.value);
            let canonical = raw.canonicalize().ok();
            let fallback = if raw.is_absolute() {
                raw.to_path_buf()
            } else {
                cwd.join(raw)
            };
            let fallback = std::fs::canonicalize(&fallback).unwrap_or(fallback);
            (canonical, fallback)
        })
        .collect();

    for (ws, (ws_canonical, ws_fallback)) in workspaces.iter().zip(workspace_canonicals.iter()) {
        let lang = ws
            .workspace_type
            .parse::<ConfigLanguage>()
            .unwrap_or(ConfigLanguage::Rust);
        let ignored = config_orchestrator.ignored_paths_for_language(&ws.path, lang);
        let (_, results) = orphan_orchestrator.scan_orphans(&ws.path, ignored.values());

        let filtered: Vec<_> = results
            .into_iter()
            .filter(|r| {
                let abs = cwd.join(&r.file.value);
                let canonical_result = abs.canonicalize().unwrap_or(abs);
                ws_canonical
                    .as_ref()
                    .map(|c| canonical_result.starts_with(c))
                    .unwrap_or_else(|| canonical_result.starts_with(ws_fallback))
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

    output_violations(&all_violations, &target, Format::Text, is_specific_member);

    if all_violations.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn scan_single_root(
    root: &str,
    orphan_orchestrator: &Arc<dyn IOrphanAggregate>,
    config_orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
) -> ExitCode {
    let scan_root = crate::surface_common_command::resolve_file_path(root);
    let lang = shared::cli_commands::utility_path_resolver::detect_language_from_path(root);
    let ignored = config_orchestrator.ignored_paths_for_language(&scan_root, lang);
    let (_, results) = orphan_orchestrator.scan_orphans(&scan_root, ignored.values());

    let violations: Vec<ViolationItem> = results
        .iter()
        .map(ViolationItem::from_lint_result)
        .collect();

    output_violations(&violations, root, Format::Text, false);

    if violations.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
