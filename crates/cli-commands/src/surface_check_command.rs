// PURPOSE: CheckCommandsSurface — CLI surface for check/scan commands
//
/// This is the thin CLI surface that delegates all pipeline logic to the agent layer.
/// It handles path resolution, request construction, and output formatting.
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::cli_commands::taxonomy_scan_request_vo::{ScanMode, ScanRequest, ScanTarget};
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use std::process::ExitCode;
use std::sync::Arc;

/// SurfaceContext — DI container struct holding surface-level dependencies.
/// The agent layer (pipeline) is passed via the contract trait.
pub struct SurfaceContext {
    pub pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
}

pub struct CheckCommandsSurface {
    pub pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
}

impl CheckCommandsSurface {
    pub fn new(
        pipeline: Arc<dyn IAnalysisPipelineAggregate>,
        report_formatter: Arc<dyn IReportFormatterAggregate>,
        multi_project_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    ) -> Self {
        Self {
            pipeline,
            report_formatter,
            multi_project_orchestrator,
        }
    }

    /// Run the full analysis pipeline on a target path.
    ///
    /// This is a thin wrapper that delegates to the agent layer (IAnalysisPipelineAggregate).
    pub fn scan(&self, path: &str, filter: Option<&str>, format: Format) -> ExitCode {
        // Construct request and delegate to agent layer
        let request = ScanRequest {
            target: ScanTarget::new(path.to_string()),
            mode: ScanMode::Scan,
            filter: filter.map(String::from),
            member: None,
            format,
        };

        // Run pipeline via contract — use current-thread runtime to avoid nested runtime panic
        let rt = match crate::surface_common_command::create_current_thread_runtime() {
            Ok(r) => r,
            Err(_) => return ExitCode::from(2),
        };

        let report = match rt.block_on(self.pipeline.run(request)) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[error] pipeline failed: {e}");
                return ExitCode::from(2);
            }
        };

        // Filter results to target path and display
        let filtered = self.filter_results_to_path(report.results, path);
        let report = ScanReport::new(filtered, vec![]);
        let output = self.report_formatter.format(&report, format);
        println!("{output}");

        if report.violation_count() > 0 {
            ExitCode::from(1)
        } else {
            ExitCode::SUCCESS
        }
    }

    /// Run the full analysis pipeline with multi-workspace discovery.
    ///
    /// This is a thin wrapper that delegates per-member scanning to the agent layer.
    pub fn scan_with_discovery(
        &self,
        path: &str,
        filter: Option<&str>,
        member: Option<&str>,
        format: Format,
    ) -> ExitCode {
        let path_obj = match FilePath::new(path.to_string()) {
            Ok(fp) => fp,
            Err(_) => {
                eprintln!("[error] invalid path: {path}");
                return ExitCode::from(2);
            }
        };

        let orchestrator = match self.multi_project_orchestrator.as_ref() {
            Some(o) => o.clone(),
            None => {
                eprintln!("[error] multi-project orchestrator not available");
                return ExitCode::from(2);
            }
        };

        // Use current-thread runtime — multi-threaded runtime causes nested runtime panic
        let rt = match crate::surface_common_command::create_current_thread_runtime() {
            Ok(r) => r,
            Err(_) => return ExitCode::from(2),
        };

        let workspaces = rt.block_on(orchestrator.discover_workspaces(&path_obj));

        if workspaces.is_empty() {
            // No workspaces discovered — fall back to single-scan mode
            return self.scan(path, filter, format);
        }

        // Filter to specific member if requested
        let workspaces = if let Some(member_name) = member {
            let all_workspaces = workspaces.clone();
            let filtered: Vec<_> = workspaces
                .into_iter()
                .filter(|ws| {
                    let ws_file = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    ws_file == member_name || ws.path.value == member_name
                })
                .collect();
            if filtered.is_empty() {
                eprintln!("[error] no workspace member matching '{member_name}'");
                eprintln!();
                eprintln!("Available members:");
                for ws in &all_workspaces {
                    let name = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    eprintln!("  - {} ({})", name, ws.workspace_type);
                }
                eprintln!();
                eprintln!("Usage: lint-arwaky-cli scan {path} --member <name>");
                return ExitCode::from(2);
            }
            filtered
        } else {
            workspaces
        };

        let multi = workspaces.len() > 1;
        if multi && matches!(format, Format::Text) {
            println!(
                "Lint Arwaky v{} (Multi-Workspace Mode)",
                env!("CARGO_PKG_VERSION")
            );
            println!("Found {} workspaces in {path}", workspaces.len());
            println!();
        }

        let mut global_all_results = Vec::new();
        let filter_str = filter.map(String::from);

        for ws in &workspaces {
            // Run pipeline for this workspace member via agent layer
            let request = ScanRequest {
                target: ScanTarget::new(ws.path.value.clone()),
                mode: ScanMode::Scan,
                filter: filter_str.clone(),
                member: Some(ws.workspace_type.clone()),
                format: Format::Text, // Internal format, not displayed
            };

            let report = match rt.block_on(self.pipeline.run(request)) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("[warn] pipeline failed for {}: {e}", ws.path.value);
                    continue;
                }
            };

            // Filter results to this workspace member's path
            let ws_canonical = std::path::Path::new(&ws.path.value).canonicalize().ok();
            let cwd_for_ws = match std::env::current_dir() {
                Ok(d) => d,
                Err(_) => std::path::PathBuf::new(),
            };
            let ws_fallback = {
                let raw = std::path::Path::new(&ws.path.value);
                if raw.is_absolute() {
                    raw.to_path_buf()
                } else {
                    cwd_for_ws.join(raw)
                }
            };
            let ws_fallback = std::fs::canonicalize(&ws_fallback).unwrap_or(ws_fallback);

            let filtered_results: Vec<_> = if let Some(code) = filter {
                report
                    .results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        r.code.code() == code
                            && (ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(false)
                                || abs_path.starts_with(&ws_fallback))
                    })
                    .collect()
            } else {
                report
                    .results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(abs_path.starts_with(&ws_fallback))
                    })
                    .collect()
            };

            global_all_results.extend(filtered_results);
        }

        // Print per-workspace results
        if multi && matches!(format, Format::Text) {
            for ws in &workspaces {
                let ws_name = std::path::Path::new(&ws.path.value)
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                let ws_type = &ws.workspace_type;

                // Re-filter for this workspace
                let ws_canonical = std::path::Path::new(&ws.path.value).canonicalize().ok();
                let cwd_for_ws = match std::env::current_dir() {
                    Ok(d) => d,
                    Err(_) => std::path::PathBuf::new(),
                };
                let ws_fallback = {
                    let raw = std::path::Path::new(&ws.path.value);
                    if raw.is_absolute() {
                        raw.to_path_buf()
                    } else {
                        cwd_for_ws.join(raw)
                    }
                };
                let ws_fallback = std::fs::canonicalize(&ws_fallback).unwrap_or(ws_fallback);

                let member_results: Vec<_> = if let Some(code) = filter {
                    global_all_results
                        .iter()
                        .filter(|r| {
                            let abs_path = cwd_for_ws.join(&r.file.value);
                            r.code.code() == code
                                && (ws_canonical
                                    .as_ref()
                                    .map(|c| abs_path.starts_with(c))
                                    .unwrap_or(false)
                                    || abs_path.starts_with(&ws_fallback))
                        })
                        .collect()
                } else {
                    global_all_results
                        .iter()
                        .filter(|r| {
                            let abs_path = cwd_for_ws.join(&r.file.value);
                            ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(abs_path.starts_with(&ws_fallback))
                        })
                        .collect()
                };

                let total = member_results.len();
                println!("── [{ws_type}] {ws_name} — {total} violations ──");
                if !member_results.is_empty() {
                    let mut code_counts: std::collections::HashMap<String, usize> =
                        std::collections::HashMap::new();
                    for r in &member_results {
                        *code_counts.entry(r.code.to_string()).or_insert(0) += 1;
                    }
                    let mut sorted: Vec<_> = code_counts.into_iter().collect();
                    sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
                    for (code, count) in &sorted {
                        println!("       {code}: {count}");
                    }
                } else {
                    println!("   (clean)");
                }
                println!();
            }
        } else {
            // Single workspace or non-text format — delegate formatting to aggregate
            let report = ScanReport::new(global_all_results.clone(), vec![]);
            let output = self.report_formatter.format(&report, format);
            println!("{output}");
        }

        if global_all_results.is_empty() {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        }
    }

    /// Check if a single file is an orphan.
    pub fn check_orphan_single_file(&self, file_path: &str) {
        let scan_root = match crate::surface_check_action::find_workspace_root(file_path) {
            Some(r) => r,
            None => std::path::PathBuf::from("."),
        };

        // Call agent layer for orphan detection
        let file_results = match self
            .pipeline
            .check_orphan_single_file(file_path, &scan_root.to_string_lossy())
        {
            Ok(results) => results,
            Err(e) => {
                eprintln!("[error] orphan check failed: {e}");
                return;
            }
        };

        if file_results.is_empty() {
            println!(
                "  {} is NOT an orphan (reachable from entry point)",
                file_path
            );
        } else {
            println!("  {} is an ORPHAN:", file_path);
            for r in &file_results {
                println!("    [{}] {}", r.code, r.message);
            }
        }
    }

    /// Filter results to the target path.
    fn filter_results_to_path(&self, results: Vec<LintResult>, path: &str) -> Vec<LintResult> {
        let canonical_scan_path = std::path::PathBuf::from(path);
        let canonical_scan_path = canonical_scan_path
            .canonicalize()
            .unwrap_or(canonical_scan_path);
        let cwd = crate::surface_common_command::current_dir();

        results
            .into_iter()
            .filter(|r| {
                let abs_path = cwd.join(&r.file.value);
                abs_path.starts_with(&canonical_scan_path)
            })
            .collect()
    }
}
