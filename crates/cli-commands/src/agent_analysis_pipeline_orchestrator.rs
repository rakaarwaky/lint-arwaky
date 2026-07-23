// PURPOSE: AnalysisPipelineOrchestrator — implements IAnalysisPipelineAggregate
//
// This is the agent layer orchestrator that wires together all 6 linter groups
// and produces a unified ScanReport. It depends only on contracts (traits),
// never on concrete implementations.
use crate::utility_format_output::{format_junit_output, format_sarif_output};
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::{
    DiagnosticSeverity, PipelineDiagnostic, PipelineError, ScanReport,
};
use shared::cli_commands::taxonomy_scan_request_vo::ScanRequest;
use shared::cli_commands::utility_path_resolver::detect_language_from_path;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;

use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct AnalysisPipelineDeps {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub format: Format,
}

/// AnalysisPipelineOrchestrator — agent layer that coordinates the full lint pipeline.
///
/// Implements IAnalysisPipelineAggregate by running all 6 linter groups in sequence:
///   1. Code analysis (AES301-305)
///   2. Naming rules (AES101-102)
///   3. Import rules (AES201-205)
///   4. External linters (Clippy, Ruff, ESLint, etc.)
///   5. Role rules (AES401-406)
///   6. Orphan detection (AES501-506)
pub struct AnalysisPipelineOrchestrator {
    deps: AnalysisPipelineDeps,
    filter: Option<String>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait::async_trait]
impl IAnalysisPipelineAggregate for AnalysisPipelineOrchestrator {
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError> {
        self.run_pipeline(request).await
    }

    async fn run_with_discovery(&self) -> Result<ScanReport, PipelineError> {
        self.run_pipeline_with_discovery().await
    }

    fn check_orphan_single_file(
        &self,
        file_path: &str,
        workspace_root: &str,
    ) -> Result<Vec<LintResult>, PipelineError> {
        self.check_orphan_single_file_impl(file_path, workspace_root)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl AnalysisPipelineOrchestrator {
    pub fn new(deps: AnalysisPipelineDeps) -> Self {
        Self { deps, filter: None }
    }

    /// Run the full analysis pipeline on a target path.
    ///
    /// This is the core scan pipeline. It runs all 6 linter groups in the
    /// same order every time and collects results into a ScanReport.
    pub async fn run_pipeline(&self, request: ScanRequest) -> Result<ScanReport, PipelineError> {
        let target = &request.target.value;
        let path_obj = FilePath::new(target.to_string()).map_err(PipelineError::InvalidPath)?;

        let mut all_results = Vec::new();
        let mut diagnostics = Vec::new();

        // 1. Run AES analysis (AES301-305) — file lines, bypass, mandatory defs
        let aes_results = self.deps.code_analysis_linter.run_code_analysis(&path_obj);
        let aes_count = aes_results.len();
        all_results.extend(aes_results.values);
        diagnostics.push(PipelineDiagnostic::new(
            "code-analysis".to_string(),
            format!("AES analysis complete: {aes_count} violations"),
            DiagnosticSeverity::Info,
        ));

        // 2-5. Run async linter groups concurrently (tokio::join! works in existing async context)
        let (naming_results, import_results, external_results, role_results) = tokio::join!(
            self.deps.naming_orchestrator.run_audit(&path_obj),
            self.deps.import_orchestrator.run_audit(&path_obj),
            self.deps.external_lint.scan_all(&path_obj),
            self.deps.role_orchestrator.run_audit(&path_obj),
        );

        // Report audit failures instead of silently discarding them
        match naming_results {
            Ok(values) => {
                let naming_count = values.len();
                all_results.extend(values);
                diagnostics.push(PipelineDiagnostic::new(
                    "naming".to_string(),
                    format!("Naming audit complete: {naming_count} violations"),
                    DiagnosticSeverity::Info,
                ));
            }
            Err(e) => {
                eprintln!("[warn] naming audit failed: {e}");
                diagnostics.push(PipelineDiagnostic::new(
                    "naming".to_string(),
                    format!("Naming audit failed: {e}"),
                    DiagnosticSeverity::Warning,
                ));
            }
        }

        match import_results {
            Ok(values) => {
                let import_count = values.len();
                all_results.extend(values);
                diagnostics.push(PipelineDiagnostic::new(
                    "imports".to_string(),
                    format!("Import audit complete: {import_count} violations"),
                    DiagnosticSeverity::Info,
                ));
            }
            Err(e) => {
                eprintln!("[warn] import audit failed: {e}");
                diagnostics.push(PipelineDiagnostic::new(
                    "imports".to_string(),
                    format!("Import audit failed: {e}"),
                    DiagnosticSeverity::Warning,
                ));
            }
        }

        let external_count = external_results.len();
        all_results.extend(external_results.values);
        let role_count = role_results.len();
        all_results.extend(role_results);
        diagnostics.push(PipelineDiagnostic::new(
            "external".to_string(),
            format!("External lint complete: {external_count} violations"),
            DiagnosticSeverity::Info,
        ));
        diagnostics.push(PipelineDiagnostic::new(
            "roles".to_string(),
            format!("Role audit complete: {role_count} violations"),
            DiagnosticSeverity::Info,
        ));

        // 6. Run orphan detection (AES501-506) — dead code via import graph
        let orphan_results = self.run_orphan_detection(target).await;
        let orphan_count = orphan_results.len();
        all_results.extend(orphan_results);
        diagnostics.push(PipelineDiagnostic::new(
            "orphan".to_string(),
            format!("Orphan detection complete: {orphan_count} violations"),
            DiagnosticSeverity::Info,
        ));

        Ok(ScanReport::new(all_results, diagnostics))
    }

    /// Run orphan detection pass — scans workspace for cross-folder import graph.
    async fn run_orphan_detection(&self, path: &str) -> Vec<LintResult> {
        let scan_root = shared::cli_commands::utility_path_resolver::find_workspace_root(path);
        let orphan_scan_root = scan_root.as_ref().and_then(|r| r.to_str()).unwrap_or(".");
        let root_fp = FilePath::new(orphan_scan_root.to_string()).unwrap_or_default();
        let language = detect_language_from_path(orphan_scan_root);
        let ignored = self
            .deps
            .config_orchestrator
            .ignored_paths_for_language(&root_fp, language);
        let (_, results) = self
            .deps
            .orphan_orchestrator
            .scan_orphans(&root_fp, ignored.values());
        results
    }

    /// Filter results to the target path and return formatted output string.
    pub fn format_results(&self, results: Vec<LintResult>, path: &str) -> String {
        let canonical_scan_path = std::path::PathBuf::from(path);
        let canonical_scan_path = canonical_scan_path
            .canonicalize()
            .unwrap_or(canonical_scan_path);
        let cwd = crate::surface_common_command::current_dir();

        // Filter results to the target path (P2.3: use Path::starts_with)
        let filtered_results: Vec<_> = results
            .into_iter()
            .filter(|r| {
                let abs_path = cwd.join(&r.file.value);
                abs_path.starts_with(&canonical_scan_path)
            })
            .collect();

        match self.deps.format {
            Format::Text => {
                let results_list =
                    shared::cli_commands::taxonomy_result_vo::LintResultList::new(filtered_results);
                let report_path = FilePath::new(path.to_string()).unwrap_or_default();
                self.deps
                    .code_analysis_linter
                    .format_report(&results_list, &report_path)
                    .to_string()
            }
            Format::Json => {
                serde_json::to_string_pretty(&filtered_results).unwrap_or_else(|_| "[]".to_string())
            }
            Format::Sarif => format_sarif_output(&filtered_results),
            Format::Junit => format_junit_output(&filtered_results),
        }
    }

    /// Run the full analysis pipeline with multi-workspace discovery.
    ///
    /// Discovers workspace members (Cargo.toml, pyproject.toml, package.json workspaces),
    /// runs all 6 linter groups per member, runs cross-workspace orphan detection,
    /// filters results to each member's path, and aggregates into a single ScanReport.
    pub async fn run_pipeline_with_discovery(&self) -> Result<ScanReport, PipelineError> {
        // Cache cwd once for all workspace iterations
        let cwd = std::env::current_dir().unwrap_or_default();

        // Discover workspaces
        let workspaces = self
            .deps
            .config_orchestrator
            .discover_workspaces(
                &FilePath::new(".".to_string())
                    .map_err(|e| PipelineError::InvalidPath(e.to_string()))?,
            )
            .await;

        if workspaces.is_empty() {
            // No workspaces discovered — fall back to single-scan mode
            let request = ScanRequest {
                target: shared::cli_commands::taxonomy_scan_request_vo::ScanTarget::new(
                    ".".to_string(),
                ),
                mode: shared::cli_commands::taxonomy_scan_request_vo::ScanMode::default(),
                filter: self.filter.clone(),
                member: None,
                format: self.deps.format,
            };
            return self.run(request).await;
        }

        let _multi = workspaces.len() > 1;
        let mut global_results = Vec::new();
        let global_diagnostics = Vec::new();

        // Collect ALL source files from workspace root for cross-workspace orphan detection
        let scan_root = crate::surface_check_action::find_workspace_root(".")
            .unwrap_or(std::path::PathBuf::from("."));
        let language = detect_language_from_path(scan_root.to_str().unwrap_or("."));
        let root_fp =
            FilePath::new(scan_root.to_str().unwrap_or(".").to_string()).unwrap_or_default();
        let ignored = self
            .deps
            .config_orchestrator
            .ignored_paths_for_language(&root_fp, language);

        // Build graph context and run orphan detection via the orphan-detector contract
        // (all file scanning is encapsulated inside the orphan-detector crate)
        let (_orphan_context, orphan_results_all) = self
            .deps
            .orphan_orchestrator
            .scan_orphans(&root_fp, ignored.values());

        // Pre-compute canonical paths for all workspaces once
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

        // Process all workspace members in parallel using futures::future::join_all
        let filter = self.filter.clone();
        let mut workspace_futures = Vec::with_capacity(workspaces.len());
        for (ws, (ws_canonical, ws_fallback)) in workspaces.iter().zip(workspace_canonicals.iter())
        {
            let ws_path = ws.path.clone();
            let ws_canonical = ws_canonical.clone();
            let ws_fallback = ws_fallback.clone();
            let orphan_results_all = orphan_results_all.clone();
            let filter = filter.clone();
            let deps = &self.deps;

            workspace_futures.push(async move {
                let mut all_results = Vec::new();

                // 1. Run AES analysis (synchronous, fine in async context)
                let aes_results = deps.code_analysis_linter.run_code_analysis(&ws_path);
                all_results.extend(aes_results.values);

                // 2-5. Run async linter groups concurrently
                let (naming_results, import_results, external_results, role_results) = tokio::join!(
                    deps.naming_orchestrator.run_audit(&ws_path),
                    deps.import_orchestrator.run_audit(&ws_path),
                    deps.external_lint.scan_all(&ws_path),
                    deps.role_orchestrator.run_audit(&ws_path),
                );

                match naming_results {
                    Ok(values) => all_results.extend(values),
                    Err(e) => eprintln!("[warn] naming audit failed: {e}"),
                }
                match import_results {
                    Ok(values) => all_results.extend(values),
                    Err(e) => eprintln!("[warn] import audit failed: {e}"),
                }
                all_results.extend(external_results.values);
                all_results.extend(role_results);

                // Filter results to this workspace member's path
                let filtered_results: Vec<_> = match &filter {
                    Some(code) => all_results
                        .into_iter()
                        .filter(|r| {
                            let abs_path = std::env::current_dir()
                                .unwrap_or_default()
                                .join(&r.file.value);
                            r.code.code() == code
                                && (ws_canonical
                                    .as_ref()
                                    .map(|c| abs_path.starts_with(c))
                                    .unwrap_or(false)
                                    || abs_path.starts_with(&ws_fallback))
                        })
                        .collect(),
                    None => all_results
                        .into_iter()
                        .filter(|r| {
                            let abs_path = std::env::current_dir()
                                .unwrap_or_default()
                                .join(&r.file.value);
                            ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(abs_path.starts_with(&ws_fallback))
                        })
                        .collect(),
                };

                // Filter orphan results to this workspace member's path
                let filtered_orphans: Vec<_> = match &filter {
                    Some(code) => orphan_results_all
                        .iter()
                        .filter(|r| {
                            let abs_path = std::env::current_dir()
                                .unwrap_or_default()
                                .join(&r.file.value);
                            r.code.code() == code
                                && (ws_canonical
                                    .as_ref()
                                    .map(|c| abs_path.starts_with(c))
                                    .unwrap_or(false)
                                    || abs_path.starts_with(&ws_fallback))
                        })
                        .cloned()
                        .collect(),
                    None => orphan_results_all
                        .iter()
                        .filter(|r| {
                            let abs_path = std::env::current_dir()
                                .unwrap_or_default()
                                .join(&r.file.value);
                            ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(abs_path.starts_with(&ws_fallback))
                        })
                        .cloned()
                        .collect(),
                };

                let mut member_results = filtered_results;
                member_results.extend(filtered_orphans);
                member_results
            });
        }

        let all_results_sets = futures::future::join_all(workspace_futures).await;
        for results in all_results_sets {
            global_results.extend(results);
        }

        Ok(ScanReport::new(global_results, global_diagnostics))
    }

    /// Check if a single file is an orphan.
    ///
    /// Scans ALL source files to build the import graph for reachability analysis,
    /// then filters results to only the specified file path.
    pub fn check_orphan_single_file_impl(
        &self,
        file_path: &str,
        _workspace_root: &str,
    ) -> Result<Vec<LintResult>, PipelineError> {
        let path_obj = std::path::Path::new(file_path);

        // Find workspace root for cross-crate graph building
        let scan_root = match crate::surface_check_action::find_workspace_root(file_path) {
            Some(r) => r,
            None => std::path::PathBuf::from("."),
        };
        let language = detect_language_from_path(scan_root.to_str().unwrap_or("."));
        let root_fp =
            FilePath::new(scan_root.to_str().unwrap_or(".").to_string()).unwrap_or_default();
        let ignored = self
            .deps
            .config_orchestrator
            .ignored_paths_for_language(&root_fp, language);
        // Run orphan detection with workspace root — file scanning is inside orphan-detector
        let (_, all_results) = self
            .deps
            .orphan_orchestrator
            .scan_orphans(&root_fp, ignored.values());

        // Normalize the target file path
        let target_path = if path_obj.is_absolute() {
            file_path.to_string()
        } else {
            let cwd = crate::surface_common_command::current_dir();
            cwd.join(file_path).to_string_lossy().to_string()
        };

        // Filter results for the specific file — canonicalize for robust comparison
        let target_canonical = std::path::Path::new(&target_path).canonicalize().ok();
        let file_results: Vec<_> = all_results
            .into_iter()
            .filter(|r| {
                let r_canonical = std::path::Path::new(&r.file.value).canonicalize().ok();
                match (target_canonical.as_deref(), r_canonical.as_deref()) {
                    (Some(t), Some(r)) => t == r,
                    _ => r.file.value == target_path || r.file.value == file_path,
                }
            })
            .collect();

        Ok(file_results)
    }
}
