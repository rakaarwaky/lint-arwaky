// PURPOSE: SurfaceCheckCommand — Runs all linter subprocesses, collects JSON results,
// and delegates output formatting to surface_output_component.
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Instant;
use tokio::process::Command;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;

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
        return ExitCode::from(2);
    }

    let rt = match crate::surface_common_command::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::from(2),
    };

    let format = opts.format;
    let is_specific_member = opts.member.is_some();
    let target_path = if let Some(ref m) = opts.member {
        let member_path = std::path::Path::new(&root).join(m);
        if member_path.exists() {
            member_path.to_string_lossy().to_string()
        } else {
            root.clone()
        }
    } else {
        root.clone()
    };

    let all_violations = rt.block_on(run_all_linters_json(&target_path));
    output_violations(&all_violations, &target_path, format, is_specific_member);

    if all_violations.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
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
        .args(["scan-quality", path, "--format", "json"])
        .output();
    let p_role = Command::new(&exe_path)
        .args(["scan-role", path, "--format", "json"])
        .output();
    let p_import = Command::new(&exe_path)
        .args(["scan-import", path, "--format", "json"])
        .output();
    let p_naming = Command::new(&exe_path)
        .args(["scan-naming", path, "--format", "json"])
        .output();
    let p_orphan = Command::new(&exe_path)
        .args(["scan-orphan", path, "--format", "json"])
        .output();
    let p_external = Command::new(&exe_path)
        .args(["scan-external", path, "--format", "json"])
        .output();

    let (res_quality, res_role, res_import, res_naming, res_orphan, res_external) =
        tokio::join!(p_quality, p_role, p_import, p_naming, p_orphan, p_external);

    let mut all: Vec<ViolationItem> = Vec::new();
    for res in [res_quality, res_role, res_import, res_naming, res_orphan, res_external] {
        if let Ok(out) = res {
            let stdout = String::from_utf8_lossy(&out.stdout);
            if let Ok(arr) = serde_json::from_str::<serde_json::Value>(stdout.trim()) {
                if let Some(items) = arr.as_array() {
                    for item in items {
                        if let Some(v) = ViolationItem::from_json_obj(item) {
                            all.push(v);
                        }
                    }
                }
            }
        }
    }
    all
}

/// Backward-compatible async entry point (used by MCP server).
pub async fn handle_scan_parallel_subprocesses(path: &str, format: Format) -> ExitCode {
    let all_violations = run_all_linters_json(path).await;
    output_violations(&all_violations, path, format, false);

    if all_violations.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

pub fn handle_default_check(
    _project_root: &str,
    _code_analysis_linter: Arc<
        dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate,
    >,
) -> ExitCode {
    let rt = match crate::surface_common_command::create_current_thread_runtime() {
        Ok(r) => r,
        Err(_) => return ExitCode::from(2),
    };
    rt.block_on(handle_scan_parallel_subprocesses(_project_root, Format::Text))
}
