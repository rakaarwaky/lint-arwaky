// PURPOSE: SurfaceCheckCommand — Runs all linter subprocesses, collects JSON results,
// and delegates output formatting to surface_output_component.
//
// AES406 NOTE: This surface spawns the CLI binary as subprocesses (self-invocation pattern)
// to run each linter with JSON output. This is a known gap — a LintRunnerAggregate should
// be created to abstract the subprocess-based linter invocation behind a contract trait.
// Adapted: std::process::Command replaces tokio::process::Command; sequential execution
// replaces tokio::join!; filesystem aggregate injected via DI.
use shared::common::ExitCode;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::process::Command;
use std::sync::Arc;

use shared::cli_commands::Format;
use shared::common::FilePath;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::report_formatter::IReportFormatterAggregate;

use crate::surface_output_component::{ViolationItem, output_violations};

pub struct ScanOptions {
    pub path: Option<FilePath>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
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

    let format = opts.format;
    let is_specific_member = opts.member.is_some()
        || opts
            .filesystem
            .is_leaf_member_path(&FilePath::new(root.clone()).unwrap_or_default());

    // Validate member against discovered workspaces
    if let Some(ref m) = opts.member {
        if let Some(ref orchestrator) = opts.multi_project_orchestrator {
            let root_fp = match FilePath::new(root.clone()) {
                Ok(fp) => fp,
                Err(_) => return ExitCode::RUNTIME_ERROR,
            };
            let workspaces = orchestrator.discover_workspaces(&root_fp);
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
        let mut all_violations = run_all_linters_json(&target_path, opts.filesystem.as_ref());

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
        let mut all_violations = run_all_linters_json(&target_path, opts.filesystem.as_ref());

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
fn run_all_linters_json(path: &str, fs_agg: &dyn IFilesystemAggregate) -> Vec<ViolationItem> {
    let exe_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from("lint-arwaky-cli"),
    };

    let linter_names = ["quality", "role", "import", "naming", "orphan", "external"];

    let mut all: Vec<ViolationItem> = Vec::new();

    for linter_name in &linter_names {
        let output = Command::new(&exe_path)
            .args([linter_name, path, "--format", "json"])
            .output();

        if let Ok(out) = output {
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
    }

    // Normalize relative paths to absolute before filtering.
    let target_canonical = fs_agg.canonicalize(std::path::Path::new(path)).ok();
    {
        let cwd = std::env::current_dir().ok();
        let target_parent = target_canonical.as_ref().and_then(|t| t.parent());
        for v in &mut all {
            if std::path::Path::new(&v.file.value).is_absolute() {
                continue;
            }
            let rel = v.file.value.clone();
            let file_path = std::path::Path::new(&rel);
            if let Some(ref cwd) = cwd {
                if let Ok(canon) = fs_agg.canonicalize(&cwd.join(file_path)) {
                    v.file = FilePath::new(canon.to_string_lossy().to_string())
                        .unwrap_or_else(|_| v.file.clone());
                    continue;
                }
            }
            if let Some(ref target) = target_canonical {
                if let Ok(canon) = fs_agg.canonicalize(&target.join(file_path)) {
                    v.file = FilePath::new(canon.to_string_lossy().to_string())
                        .unwrap_or_else(|_| v.file.clone());
                    continue;
                }
            }
            if let Some(parent) = target_parent {
                if let Ok(canon) = fs_agg.canonicalize(&parent.join(file_path)) {
                    v.file = FilePath::new(canon.to_string_lossy().to_string())
                        .unwrap_or_else(|_| v.file.clone());
                }
            }
        }
    }

    // Filter: only keep violations whose file path is within the target directory.
    if let Some(canonical_target) = &target_canonical {
        all.retain(|v| {
            let file_path = std::path::Path::new(&v.file.value);
            if let Ok(canonical) = fs_agg.canonicalize(file_path) {
                return canonical.starts_with(canonical_target);
            }
            if let Ok(cwd) = std::env::current_dir() {
                let joined = cwd.join(file_path);
                let cwd_joined = fs_agg.canonicalize(&joined).unwrap_or(joined);
                if cwd_joined.starts_with(canonical_target) {
                    return true;
                }
            }
            if let Ok(target_joined) = fs_agg.canonicalize(&canonical_target.join(file_path)) {
                return target_joined.starts_with(canonical_target);
            }
            false
        });
    }

    all
}

/// Synchronous entry point (used by MCP server / default check).
pub fn handle_scan_parallel_subprocesses(
    path: &str,
    format: Format,
    fs_agg: &dyn IFilesystemAggregate,
) -> ExitCode {
    let all_violations = run_all_linters_json(path, fs_agg);
    output_violations(&all_violations, path, format, false);

    if all_violations.is_empty() {
        ExitCode::OK
    } else {
        ExitCode::POLICY_FAIL
    }
}

pub fn handle_default_check(
    project_root: &str,
    _code_analysis_linter: Arc<
        dyn shared::quality_rules::contract_code_analysis_aggregate::ICodeAnalysisAggregate,
    >,
    fs_agg: &dyn IFilesystemAggregate,
) -> ExitCode {
    handle_scan_parallel_subprocesses(project_root, Format::Text, fs_agg)
}
