use crate::surface_common_command;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::cli_commands::utility_path_resolver;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use std::process::ExitCode;
use std::sync::Arc;

pub fn handle_scan_orphan(
    path: Option<FilePath>,
    member: Option<String>,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    report_formatter: Arc<dyn IReportFormatterAggregate>,
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
        return scan_single_root(
            &root,
            &orphan_orchestrator,
            &config_orchestrator,
            &report_formatter,
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
            return ExitCode::from(2);
        }
        filtered
    } else {
        workspaces
    };

    let cwd = std::env::current_dir().unwrap_or_default();
    let multi = workspaces.len() > 1;
    if multi {
        println!(
            "Lint Arwaky v{} (Scan-Orphan Multi-Workspace)",
            env!("CARGO_PKG_VERSION")
        );
        println!("Found {} workspaces in {root}", workspaces.len());
        println!();
    }

    let mut global_results = Vec::new();
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

        if multi {
            let name = std::path::Path::new(&ws.path.value)
                .file_name()
                .map(|n| n.to_string_lossy())
                .unwrap_or_default();
            println!("── {} ──────────────────────", name);
            if filtered.is_empty() {
                println!("  (clean)");
            } else {
                for r in &filtered {
                    println!("  [{}] {}: {}", r.code, r.file.value, r.message.value);
                }
            }
            println!();
        }

        global_results.extend(filtered);
    }

    let report = ScanReport::new(global_results.clone(), vec![]);
    if !multi {
        let output = report_formatter.format(&report, Format::Text);
        println!("{output}");
    }

    if global_results.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn scan_single_root(
    root: &str,
    orphan_orchestrator: &Arc<dyn IOrphanAggregate>,
    config_orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
    report_formatter: &Arc<dyn IReportFormatterAggregate>,
) -> ExitCode {
    let scan_root =
        utility_path_resolver::find_workspace_root(root).unwrap_or(std::path::PathBuf::from(root));
    let scan_root_str = scan_root.to_string_lossy().to_string();
    let root_fp = match FilePath::new(scan_root_str.clone()) {
        Ok(fp) => fp,
        Err(_) => {
            eprintln!("[error] invalid path: {scan_root_str}");
            return ExitCode::from(2);
        }
    };
    let lang = utility_path_resolver::detect_language_from_path(&scan_root_str);
    let ignored = config_orchestrator.ignored_paths_for_language(&root_fp, lang);
    let (_, results) = orphan_orchestrator.scan_orphans(&root_fp, ignored.values());
    let report = ScanReport::new(results.clone(), vec![]);
    let output = report_formatter.format(&report, Format::Text);
    println!("{output}");
    if results.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}
