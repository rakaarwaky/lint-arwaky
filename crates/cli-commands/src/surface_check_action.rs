// PURPOSE: Check/scan/CI entry points — thin wrappers around CheckCommandsSurface
//
// Three commands, distinguished by scope:
//   - check:  self-lint the lint-arwaky project itself (uses CheckCommandsSurface.scan)
//   - scan:   full analysis on external project + external adapters (uses scan_with_discovery)
//   - ci:     CI-mode with threshold comparison and critical-violation auto-fail
//
// find_workspace_root walks up from the given path looking for Cargo.toml/crates/packages/modules.
use std::collections::BTreeMap;
use std::process::ExitCode;
use std::sync::Arc;

use crate::surface_check_command::CheckCommandsSurface;
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_threshold_vo::Threshold;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;

/// Walk up from `path` to find the workspace root (parent of `crates/`, `packages/`, or `modules/`).
pub fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    shared::common::utility_file::find_workspace_root(path)
}

pub struct CheckOptions {
    pub path: Option<FilePath>,
    pub git_diff: bool,
    pub pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub filter: Option<String>,
    pub git_aggregate: Option<Arc<dyn GitHooksAggregate>>,
    pub config: ArchitectureConfig,
    pub format: Format,
}

/// check = self-lint (AES analysis on current project, same algorithm as scan)
pub fn handle_check(opts: CheckOptions) -> ExitCode {
    let root = match &opts.path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }
    let surface = CheckCommandsSurface::new(opts.pipeline, opts.report_formatter, None);
    surface.scan(&root, opts.filter.as_deref(), opts.format)
}

pub struct ScanOptions {
    pub path: Option<FilePath>,
    pub pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

/// scan = AES analysis on external project + external adapters
pub fn handle_scan(opts: ScanOptions) -> ExitCode {
    let root = match &opts.path {
        Some(p) => p.value().to_string(),
        None => ".".to_string(),
    };
    if !std::path::Path::new(&root).exists() {
        eprintln!("Error: path '{}' does not exist", root);
        return ExitCode::from(2);
    }
    let surface = CheckCommandsSurface::new(
        opts.pipeline,
        opts.report_formatter,
        opts.multi_project_orchestrator,
    );
    surface.scan_with_discovery(
        &root,
        opts.filter.as_deref(),
        opts.member.as_deref(),
        opts.format,
    )
}

pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<FilePath>,
    threshold: Threshold,
) -> ExitCode {
    crate::surface_common_command::run_ci_analysis(code_analysis_linter, path, threshold)
}

/// Default check = self-lint when no args provided (runs `lint_path(".")`)
pub fn handle_default_check(
    project_root: &str,
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
) -> ExitCode {
    let path = FilePath::new(project_root.to_string()).unwrap_or_default();
    let results = code_analysis_linter.run_code_analysis_path(&path);
    let mut lines: Vec<String> = Vec::new();
    lines.push("=".repeat(60));
    lines.push("  AES Architecture Compliance Report (Self-Lint)".to_string());
    lines.push("=".repeat(60));
    lines.push(format!("  Project: {}", project_root));
    lines.push(format!("  Files scanned: {}", results.len()));
    lines.push("=".repeat(60));
    lines.push("".to_string());
    let mut critical = Vec::new();
    let mut high = Vec::new();
    let mut medium = Vec::new();
    let mut low = Vec::new();
    for r in &results {
        match r.severity {
            Severity::CRITICAL => critical.push(r),
            Severity::HIGH => high.push(r),
            Severity::MEDIUM => medium.push(r),
            Severity::LOW => low.push(r),
            _ => medium.push(r),
        }
    }
    for (sev, items) in [
        ("CRITICAL", &critical),
        ("HIGH", &high),
        ("MEDIUM", &medium),
        ("LOW", &low),
    ] {
        if items.is_empty() {
            continue;
        }
        lines.push(format!("  [{}] {} violations", sev, items.len()));
        lines.push("-".repeat(60));
        for r in items.iter() {
            lines.push(format!("  [{}] {}", r.code, r.file.value));
            for msg_line in r.message.value.lines() {
                lines.push(format!("    {}", msg_line));
            }
        }
        lines.push("".to_string());
    }
    let total = results.len();
    let mut per_code: BTreeMap<String, usize> = BTreeMap::new();
    for r in &results {
        *per_code.entry(r.code.to_string()).or_insert(0) += 1;
    }
    lines.push("=".repeat(60));
    lines.push(format!("  Total AES Violations: {}", total));
    lines.push(format!(
        "  Total Category AES Violations: {}",
        per_code.len()
    ));
    if !per_code.is_empty() {
        lines.push("-".repeat(60));
        for (code, count) in &per_code {
            lines.push(format!("  {}: {}", code, count));
        }
    }
    lines.push("".to_string());
    if total == 0 {
        lines.push("  Status: PASS - No AES violations detected".to_string());
    } else {
        lines.push("  Status: FAIL - AES violations detected".to_string());
    }
    lines.push("=".repeat(60));
    println!("Lint Arwaky v{} (AES Self-Lint)", env!("CARGO_PKG_VERSION"));
    println!("Scanning: {}", project_root);
    println!();
    println!("{}", lines.join("\n"));
    if total > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
