// PURPOSE: SurfaceCheckCommand — Runs all linter subprocesses, collects JSON results,
// and delegates output formatting to surface_output_component.
use shared::common::ExitCode;
use std::sync::Arc;
use tokio::process::Command;

use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::report_formatter::IReportFormatterAggregate;

use crate::surface_output_component::{output_violations, ViolationItem};

pub struct ScanOptions {
    pub path: Option<FilePath>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

pub struct CheckCommandsSurface;

impl CheckCommandsSurface {
    pub fn new(
        _report_formatter: Arc<dyn IReportFormatterAggregate>,
        _multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    ) -> Self {
        Self
    }

    pub fn check_orphan_single_file(&self, _file_path: &str) {}
}

pub type CheckOptions = ScanOptions;

/// Run all 6 linters via subprocesses, collect JSON, output unified report.
pub fn handle_scan(opts: ScanOptions) -> ExitCode {
    let root = match &opts.path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::RUNTIME_ERROR;
    }

    let rt = match crate::surface_common_action::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };

    let format = opts.format;
    let is_specific_member =
        opts.member.is_some() || crate::utility_path_resolver::is_member_path(&root);

    // Validate member against discovered workspaces
    if let Some(ref m) = opts.member {
        if let Some(ref orchestrator) = opts.multi_project_orchestrator {
            let root_fp = match FilePath::new(root.clone()) {
                Ok(fp) => fp,
                Err(_) => return ExitCode::RUNTIME_ERROR,
            };
            let workspaces = rt.block_on(orchestrator.discover_workspaces(&root_fp));
            if !workspaces.is_empty() {
                let matched = workspaces.iter().any(|ws| {
                    let ws_file = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    ws_file.as_ref() == m.as_str() || ws.path.value == *m
                });
                if !matched {
                    eprintln!("[error] no workspace member matching '{m}'");
                    return ExitCode::RUNTIME_ERROR;
                }
            }
        }
        let target_path = {
            let member_path = std::path::Path::new(&root).join(m);
            if member_path.exists() {
                member_path.to_string_lossy().to_string()
            } else {
                root.clone()
            }
        };
        let mut all_violations = rt.block_on(run_all_linters_json(&target_path));

        // Apply filter by AES rule code (e.g. AES101, AES304)
        if let Some(ref filter_str) = opts.filter {
            let filter_upper = filter_str.to_uppercase();
            all_violations.retain(|v| v.code.code().contains(&filter_upper));
        }

        output_violations(&all_violations, &target_path, format, is_specific_member);
        if all_violations.is_empty() {
            ExitCode::OK
        } else {
            ExitCode::POLICY_FAIL
        }
    } else {
        let target_path = root.clone();
        let mut all_violations = rt.block_on(run_all_linters_json(&target_path));

        // Apply filter by AES rule code (e.g. AES101, AES304)
        if let Some(ref filter_str) = opts.filter {
            let filter_upper = filter_str.to_uppercase();
            all_violations.retain(|v| v.code.code().contains(&filter_upper));
        }

        output_violations(&all_violations, &target_path, format, is_specific_member);
        if all_violations.is_empty() {
            ExitCode::OK
        } else {
            ExitCode::POLICY_FAIL
        }
    }
}

pub use handle_scan as handle_check;

/// Run all 6 linters as subprocesses with `--format json`, collect ViolationItems.
async fn run_all_linters_json(path: &str) -> Vec<ViolationItem> {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from("lint-arwaky-cli"),
    };

    let p_quality = Command::new(&exe_path)
        .args(["quality", path, "--format", "json"])
        .output();
    let p_role = Command::new(&exe_path)
        .args(["role", path, "--format", "json"])
        .output();
    let p_import = Command::new(&exe_path)
        .args(["import", path, "--format", "json"])
        .output();
    let p_naming = Command::new(&exe_path)
        .args(["naming", path, "--format", "json"])
        .output();
    let p_orphan = Command::new(&exe_path)
        .args(["orphan", path, "--format", "json"])
        .output();
    let p_external = Command::new(&exe_path)
        .args(["external", path, "--format", "json"])
        .output();

    let (res_quality, res_role, res_import, res_naming, res_orphan, res_external) =
        tokio::join!(p_quality, p_role, p_import, p_naming, p_orphan, p_external);

    let mut all: Vec<ViolationItem> = Vec::new();
    let target_canonical = std::fs::canonicalize(path).ok();
    for out in [
        res_quality,
        res_role,
        res_import,
        res_naming,
        res_orphan,
        res_external,
    ]
    .into_iter()
    .flatten()
    {
        let stdout = String::from_utf8_lossy(&out.stdout);
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
            if let Some(results) = val.get("results").and_then(|r| r.as_array()) {
                for item in results {
                    if let Some(v) = ViolationItem::from_json_obj(item) {
                        all.push(v);
                    }
                }
            } else if let Some(items) = val.as_array() {
                for item in items {
                    if let Some(v) = ViolationItem::from_json_obj(item) {
                        all.push(v);
                    }
                }
            }
        }
    }

    // Filter: only keep violations whose file path is within the target directory.
    // File paths from subprocesses may be relative to the workspace top_root
    // (found by find_workspace_root), not relative to CWD.
    if let Some(canonical_target) = &target_canonical {
        all.retain(|v| {
            let file_path = std::path::Path::new(&v.file.value);
            // First try direct canonicalize
            if let Ok(canonical) = std::fs::canonicalize(file_path) {
                return canonical.starts_with(canonical_target);
            }
            // Try joining with CWD
            if let Ok(cwd) = std::env::current_dir() {
                let joined = cwd.join(file_path);
                let cwd_joined = std::fs::canonicalize(&joined).unwrap_or(joined);
                if cwd_joined.starts_with(canonical_target) {
                    return true;
                }
            }
            // Try joining with target directory (for paths relative to workspace top_root)
            if let Ok(target_joined) = std::fs::canonicalize(canonical_target.join(file_path)) {
                return target_joined.starts_with(canonical_target);
            }
            false
        });
    }

    all
}

/// Backward-compatible async entry point (used by MCP server).
pub async fn handle_scan_parallel_subprocesses(path: &str, format: Format) -> ExitCode {
    let all_violations = run_all_linters_json(path).await;
    output_violations(&all_violations, path, format, false);

    if all_violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}

pub fn handle_default_check(
    _project_root: &str,
    _code_analysis_linter: Arc<
        dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate,
    >,
) -> ExitCode {
    let rt = match crate::surface_common_action::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::RUNTIME_ERROR,
    };
    rt.block_on(handle_scan_parallel_subprocesses(
        _project_root,
        Format::Text,
    ))
}
