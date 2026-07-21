# Export: crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs

This document contains the source code for `crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs` along with its resolved dependencies and related documentation.

## File List

**Selected file:**
- [crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs)

**Dependencies & related docs:**
- [.agents/skills/create-agent-rust/SKILL.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/SKILL.md)
- [.agents/skills/create-agent-rust/references/3-block-structure.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/references/3-block-structure.md)
- [.agents/skills/create-agent-rust/references/checklist.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/references/checklist.md)
- [.agents/skills/create-agent-rust/references/commands.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/references/commands.md)
- [.agents/skills/create-agent-rust/references/computation-detection.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/references/computation-detection.md)
- [.agents/skills/create-agent-rust/references/error-handling.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/references/error-handling.md)
- [.agents/skills/create-agent-rust/references/examples.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/references/examples.md)
- [.agents/skills/create-agent-rust/references/helper-vs-utility.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/references/helper-vs-utility.md)
- [.agents/skills/create-agent-rust/references/layer-boundaries.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/references/layer-boundaries.md)
- [.agents/skills/create-agent-rust/references/primitive-vo-policy.md](file:///home/raka/mcp-arwaky/lint-arwaky/.agents/skills/create-agent-rust/references/primitive-vo-policy.md)
- [ARCHITECTURE.md](file:///home/raka/mcp-arwaky/lint-arwaky/ARCHITECTURE.md)
- [PRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/PRD.md)
- [crates/cli-commands/FRD.md](file:///home/raka/mcp-arwaky/lint-arwaky/crates/cli-commands/FRD.md)
- [crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs)
- [crates/shared/src/cli-commands/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/mod.rs)
- [crates/shared/src/cli-commands/taxonomy_format_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_format_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_result_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_result_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_scan_report_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_scan_report_vo.rs)
- [crates/shared/src/cli-commands/taxonomy_scan_request_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/cli-commands/taxonomy_scan_request_vo.rs)
- [crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs)
- [crates/shared/src/code-analysis/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/code-analysis/mod.rs)
- [crates/shared/src/common/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/mod.rs)
- [crates/shared/src/common/taxonomy_adapter_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_error.rs)
- [crates/shared/src/common/taxonomy_adapter_name_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_adapter_name_vo.rs)
- [crates/shared/src/common/taxonomy_common_error.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_error.rs)
- [crates/shared/src/common/taxonomy_common_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_common_vo.rs)
- [crates/shared/src/common/taxonomy_error_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_error_vo.rs)
- [crates/shared/src/common/taxonomy_job_id_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_job_id_vo.rs)
- [crates/shared/src/common/taxonomy_layer_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_layer_vo.rs)
- [crates/shared/src/common/taxonomy_lint_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_lint_vo.rs)
- [crates/shared/src/common/taxonomy_message_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_message_vo.rs)
- [crates/shared/src/common/taxonomy_path_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_path_vo.rs)
- [crates/shared/src/common/taxonomy_response_data_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_response_data_vo.rs)
- [crates/shared/src/common/taxonomy_severity_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_severity_vo.rs)
- [crates/shared/src/common/taxonomy_source_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_source_vo.rs)
- [crates/shared/src/common/taxonomy_suggestion_vo.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/common/taxonomy_suggestion_vo.rs)
- [crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs)
- [crates/shared/src/config-system/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/config-system/mod.rs)
- [crates/shared/src/external-lint/contract_external_lint_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/contract_external_lint_aggregate.rs)
- [crates/shared/src/external-lint/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/external-lint/mod.rs)
- [crates/shared/src/import-rules/contract_import_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/contract_import_runner_aggregate.rs)
- [crates/shared/src/import-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/import-rules/mod.rs)
- [crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs)
- [crates/shared/src/naming-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/naming-rules/mod.rs)
- [crates/shared/src/orphan-detector/contract_orphan_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/contract_orphan_aggregate.rs)
- [crates/shared/src/orphan-detector/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/orphan-detector/mod.rs)
- [crates/shared/src/role-rules/contract_role_runner_aggregate.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/contract_role_runner_aggregate.rs)
- [crates/shared/src/role-rules/mod.rs](file:///home/raka/mcp-arwaky/lint-arwaky/crates/shared/src/role-rules/mod.rs)

---

## Selected File: crates/cli-commands/src/agent_analysis_pipeline_orchestrator.rs

```rust
// PURPOSE: AnalysisPipelineOrchestrator — implements IAnalysisPipelineAggregate
//
// This is the agent layer orchestrator that wires together all 6 linter groups
// and produces a unified ScanReport. It depends only on contracts (traits),
// never on concrete implementations.
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::{
    DiagnosticSeverity, PipelineDiagnostic, PipelineError, ScanReport,
};
use shared::cli_commands::taxonomy_scan_request_vo::ScanRequest;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

/// AnalysisPipelineOrchestrator — agent layer that coordinates the full lint pipeline.
///
/// Implements IAnalysisPipelineAggregate by running all 6 linter groups in sequence:
///   1. Code analysis (AES301-305)
///   2. Naming rules (AES101-102)
///   3. Import rules (AES201-205)
///   4. External linters (Clippy, Ruff, ESLint, etc.)
///   5. Role rules (AES401-406)
///   6. Orphan detection (AES501-506)
// ─── Block 1: Struct Definition ───────────────────────────
pub struct AnalysisPipelineOrchestrator {
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    format: Format,
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
    pub fn new(
        code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
        naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
        import_orchestrator: Arc<dyn IImportRunnerAggregate>,
        external_lint: Arc<dyn IExternalLintAggregate>,
        role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
        orphan_orchestrator: Arc<dyn IOrphanAggregate>,
        config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
        format: Format,
    ) -> Self {
        Self {
            code_analysis_linter,
            naming_orchestrator,
            import_orchestrator,
            external_lint,
            role_orchestrator,
            orphan_orchestrator,
            config_orchestrator,
            format,
            filter: None,
        }
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
        let aes_results = self.code_analysis_linter.run_code_analysis(&path_obj);
        let aes_count = aes_results.len();
        all_results.extend(aes_results.values);
        diagnostics.push(PipelineDiagnostic::new(
            "code-analysis".to_string(),
            format!("AES analysis complete: {aes_count} violations"),
            DiagnosticSeverity::Info,
        ));

        // 2-5. Run async linter groups concurrently (tokio::join! works in existing async context)
        let (naming_results, import_results, external_results, role_results) = tokio::join!(
            self.naming_orchestrator.run_audit(&path_obj),
            self.import_orchestrator.run_audit(&path_obj),
            self.external_lint.scan_all(&path_obj),
            self.role_orchestrator.run_audit(&path_obj),
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
        let scan_root = crate::surface_check_action::find_workspace_root(path);
        let orphan_scan_root = scan_root.as_ref().and_then(|r| r.to_str()).unwrap_or(".");
        let dir_path = DirectoryPath::new(orphan_scan_root.to_string()).unwrap_or_default();
        let ignored = self.config_orchestrator.ignored_paths(orphan_scan_root);
        let source_files = match shared::common::utility_file::scan_directory(&dir_path, &ignored) {
            Ok(list) => list.values,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        self.orphan_orchestrator
            .check_orphans(&file_strs, orphan_scan_root)
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

        match self.format {
            Format::Text => {
                let results_list =
                    shared::cli_commands::taxonomy_result_vo::LintResultList::new(filtered_results);
                let report_path = FilePath::new(path.to_string()).unwrap_or_default();
                self.code_analysis_linter
                    .format_report(&results_list, &report_path)
            }
            Format::Json => {
                serde_json::to_string_pretty(&filtered_results).unwrap_or_else(|_| "[]".to_string())
            }
            Format::Sarif => self.format_sarif_output(&filtered_results),
            Format::Junit => self.format_junit_output(&filtered_results),
        }
    }

    /// Run the full analysis pipeline with multi-workspace discovery.
    ///
    /// Discovers workspace members (Cargo.toml, pyproject.toml, package.json workspaces),
    /// runs all 6 linter groups per member, runs cross-workspace orphan detection,
    /// filters results to each member's path, and aggregates into a single ScanReport.
    pub async fn run_pipeline_with_discovery(&self) -> Result<ScanReport, PipelineError> {
        // Discover workspaces
        let workspaces = self
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
                format: self.format,
            };
            return self.run(request).await;
        }

        let _multi = workspaces.len() > 1;
        let mut global_results = Vec::new();
        let global_diagnostics = Vec::new();

        // Collect ALL source files from workspace root for cross-workspace orphan detection
        let scan_root = crate::surface_check_action::find_workspace_root(".")
            .unwrap_or(std::path::PathBuf::from("."));
        let ignored = self
            .config_orchestrator
            .ignored_paths(scan_root.to_str().unwrap_or("."));
        let dir_path = DirectoryPath::new(scan_root.to_str().unwrap_or(".")).unwrap_or_default();
        let all_source_files: Vec<String> = {
            match shared::common::utility_file::scan_directory(&dir_path, &ignored) {
                Ok(list) => list.values.iter().map(|f| f.value.clone()).collect(),
                Err(_) => Vec::new(),
            }
        };

        // Run orphan detection once across all workspace members
        let orphan_results_all = self
            .orphan_orchestrator
            .check_orphans(&all_source_files, scan_root.to_str().unwrap_or("."));

        for ws in &workspaces {
            let mut all_results = Vec::new();

            // 1. Run AES analysis
            let aes_results = self.code_analysis_linter.run_code_analysis(&ws.path);
            all_results.extend(aes_results.values);

            // 2-5. Run async linter groups concurrently (tokio::join! works in existing async context)
            let (naming_results, import_results, external_results, role_results) = tokio::join!(
                self.naming_orchestrator.run_audit(&ws.path),
                self.import_orchestrator.run_audit(&ws.path),
                self.external_lint.scan_all(&ws.path),
                self.role_orchestrator.run_audit(&ws.path),
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

            let filtered_results: Vec<_> = match &self.filter {
                Some(code) => all_results
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
                    .collect(),
                None => all_results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(abs_path.starts_with(&ws_fallback))
                    })
                    .collect(),
            };

            // Filter orphan results to this workspace member's path
            let filtered_orphans: Vec<_> = match &self.filter {
                Some(code) => orphan_results_all
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
                    .cloned()
                    .collect(),
                None => orphan_results_all
                    .iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(abs_path.starts_with(&ws_fallback))
                    })
                    .cloned()
                    .collect(),
            };

            // Merge per-member results with filtered orphans
            let mut member_results = filtered_results;
            member_results.extend(filtered_orphans);
            global_results.extend(member_results);
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
        let ignored = self
            .config_orchestrator
            .ignored_paths(scan_root.to_str().unwrap_or("."));
        let all_files: Vec<String> = shared::common::collect_all_source_files(&scan_root, &ignored)
            .iter()
            .map(|f| f.value.clone())
            .collect();

        // Normalize the target file path
        let target_path = if path_obj.is_absolute() {
            file_path.to_string()
        } else {
            let cwd = crate::surface_common_command::current_dir();
            cwd.join(file_path).to_string_lossy().to_string()
        };

        // Run orphan detection with workspace root
        let all_results = self
            .orphan_orchestrator
            .check_orphans(&all_files, &scan_root.to_string_lossy());

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

    /// Format results as a SARIF 2.1.0 JSON string.
    fn format_sarif_output(&self, results: &[LintResult]) -> String {
        #[derive(serde::Serialize)]
        struct SarifLog {
            #[serde(rename = "$schema")]
            schema: &'static str,
            version: &'static str,
            runs: Vec<SarifRun>,
        }

        #[derive(serde::Serialize)]
        struct SarifRun {
            tool: SarifTool,
            results: Vec<SarifResult>,
        }

        #[derive(serde::Serialize)]
        struct SarifTool {
            driver: SarifDriver,
        }

        #[derive(serde::Serialize)]
        struct SarifDriver {
            name: &'static str,
            version: &'static str,
            information_uri: &'static str,
        }

        #[derive(serde::Serialize)]
        struct SarifResult {
            rule_id: String,
            level: String,
            message: SarifMessage,
            locations: Vec<SarifLocation>,
        }

        #[derive(serde::Serialize)]
        struct SarifMessage {
            text: String,
        }

        #[derive(serde::Serialize)]
        struct SarifLocation {
            physical_location: SarifPhysicalLocation,
        }

        #[derive(serde::Serialize)]
        struct SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation,
            region: SarifRegion,
        }

        #[derive(serde::Serialize)]
        struct SarifArtifactLocation {
            uri: String,
        }

        #[derive(serde::Serialize)]
        struct SarifRegion {
            start_line: i64,
        }

        // Map Severity → SARIF level
        fn severity_to_sarif_level(
            sev: &shared::cli_commands::taxonomy_severity_vo::Severity,
        ) -> &'static str {
            match sev {
                shared::cli_commands::taxonomy_severity_vo::Severity::CRITICAL
                | shared::cli_commands::taxonomy_severity_vo::Severity::HIGH => "error",
                shared::cli_commands::taxonomy_severity_vo::Severity::MEDIUM => "warning",
                shared::cli_commands::taxonomy_severity_vo::Severity::LOW
                | shared::cli_commands::taxonomy_severity_vo::Severity::INFO => "note",
            }
        }

        let sarif_results: Vec<SarifResult> = results
            .iter()
            .map(|r| SarifResult {
                rule_id: r.code.to_string(),
                level: severity_to_sarif_level(&r.severity).to_string(),
                message: SarifMessage {
                    text: r.message.value.clone(),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: r.file.value.clone(),
                        },
                        region: SarifRegion {
                            start_line: std::cmp::max(1, r.line.value()),
                        },
                    },
                }],
            })
            .collect();

        let log = SarifLog {
            schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            version: "2.1.0",
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "lint-arwaky",
                        version: env!("CARGO_PKG_VERSION"),
                        information_uri: "https://github.com/rakaarwaky/lint-arwaky",
                    },
                },
                results: sarif_results,
            }],
        };

        serde_json::to_string_pretty(&log).unwrap_or_else(|_| "{}".to_string())
    }

    /// Format results as JUnit XML.
    fn format_junit_output(&self, results: &[LintResult]) -> String {
        let total = results.len();
        let failures: Vec<_> = results
            .iter()
            .filter(|r| {
                matches!(
                    r.severity,
                    shared::cli_commands::taxonomy_severity_vo::Severity::CRITICAL
                        | shared::cli_commands::taxonomy_severity_vo::Severity::HIGH
                        | shared::cli_commands::taxonomy_severity_vo::Severity::MEDIUM
                        | shared::cli_commands::taxonomy_severity_vo::Severity::LOW
                )
            })
            .collect();
        let failure_count = failures.len();

        let mut xml = String::with_capacity(total.saturating_mul(256));
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
            "<testsuites name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));
        xml.push_str(&format!(
            "  <testsuite name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));

        for r in results {
            let classname = xml_escape(&r.code.to_string());
            let name = xml_escape(&format!("{}:{}", r.file.value, r.line.value()));
            let message = xml_escape(&r.message.value);
            let sev = r.severity.to_string();
            let is_info = r.severity == shared::cli_commands::taxonomy_severity_vo::Severity::INFO;

            xml.push_str(&format!(
                "    <testcase classname=\"{classname}\" name=\"{name}\">\n"
            ));
            if !is_info {
                xml.push_str(&format!(
                    "      <failure message=\"{sev}: {message}\" type=\"{sev}\">\n"
                ));
                xml.push_str(&format!("        {message}\n"));
                xml.push_str("      </failure>\n");
            }
            xml.push_str("    </testcase>\n");
        }

        xml.push_str("  </testsuite>\n");
        xml.push_str("</testsuites>\n");
        xml
    }
}

/// XML-escape a string for safe inclusion in JUnit XML output.
fn xml_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
}
```

---

## File: .agents/skills/create-agent-rust/SKILL.md

```markdown
---
name: create-agent-rust
description: "Create and validate Rust agent layer files following AES rules: orchestration-only, zero I/O, zero business logic, zero domain computation, 3-block structure, one impl struct per file, aggregate contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
    tags: [rust, aes, agent, aggregate, structure, 3-block-structure, di, orchestration, vo]
    triggers:
        - "create agent rust"
        - "add agent rust"
        - "fix agent structure rust"
        - "create aggregate rust"
        - "agent missing aggregate rust"
        - "validate agent logic rust"
        - "check agent rust"
        - "audit agent rust"
    dependencies: []
    related:
        - create-capabilities-rust
        - create-taxonomy-rust
        - create-contract-rust
---

# create-agent-rust

## Purpose

Create and validate Rust **agent layer** files following AES rules.

An agent file contains **orchestration / pipeline execution only**.

Agents coordinate capabilities into executable flows. They control sequence and movement, not business calculation.

Agents MUST NOT contain I/O, business logic, domain rules, domain computation, or domain data definitions.

Agents depend ONLY on Taxonomy, Contract, and Utility layers. They must be completely ignorant of Capabilities implementations.

## Definition of Done

1. ONE implementation struct per file.
2. Struct implements ONE domain aggregate trait in Block 2.
3. Block 2 contains ONLY the aggregate trait implementation.
4. Constructors, std trait impls, private helpers in Block 3.
5. Zero I/O, zero business logic, zero domain computation.
6. No locally defined domain data structures.
7. Service dependencies use DI via `Arc<dyn Trait>`.
8. Value/configuration fields use shared VOs.
9. Aggregate signatures use shared VOs for domain data.
10. `cargo check -p <crate-name>` passes.

## References

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, trait placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/computation-detection.md` | Computation detection rules, forbidden/allowed patterns |
| `references/error-handling.md` | Error handling rules |
| `references/primitive-vo-policy.md` | Primitive policy table, VO rules |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | Verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/agent_name.rs` | New agent implementation file |
| `templates/contract_name_aggregate.rs` | New aggregate trait definition |
| `templates/mod.rs` | Module registration |

## Workflow

### Step 1: Analyze File

Read the file and ask: **"Is this orchestration only?"**

If yes → keep as agent. If it contains computation → capabilities, domain data → taxonomy.

### Step 2: Check for Missing Aggregate

Does the agent struct implement an aggregate trait? If no → create one.

### Step 3: Create Aggregate File if Missing

Create `contract_<name>_aggregate.rs` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: struct definition → aggregate trait impl → constructors/std traits/helpers.

### Step 5: Verify Struct Discipline

One struct, no local data structs, DI via `Arc<dyn Trait>`, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no I/O, no business logic, no domain computation.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

``` `bash
cargo check -p <crate-name>
``` `

## Quick Commands

``` `bash
# List aggregate trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Aggregate\s+for" crates/<crate>/src/agent_*.rs

# Check computation patterns
rg "\.sum\(\)|\.len\(\)|\.map\(|\.fold\(" crates/<crate>/src/agent_*.rs

# Check forbidden imports (agent must only depend on taxonomy + contract + utility)
rg "^\s*use\s+.*(capabilities_|infrastructure_)" crates/<crate>/src/agent_*.rs
``` `

## Common Mistakes

- Putting domain computation in agents.
- Putting business logic in agents.
- Putting I/O in agents.
- Defining domain data structs in agent files.
- Using concrete service types as struct fields.
- Putting private helpers in the aggregate trait.
- Placing std trait impls before the aggregate trait.
- Mixing Block 2 and Block 3 responsibilities.
- Silent error swallowing with `unwrap_or_default()`.
- Magic constants in agent logic.
```

---

## File: .agents/skills/create-agent-rust/references/3-block-structure.md

```markdown
# The 3-Block Structure

Every implementation file MUST follow this order with mandatory block markers:

1. **Block 1 — Struct Definition**
2. **Block 2 — Aggregate Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

Each block MUST be preceded by a block marker comment:

``` `rust
// ─── Block 1: Struct Definition ───────────────────────────
``` `

``` `rust
// ─── Block 2: Aggregate Trait Implementation ──────────────
``` `

``` `rust
// ─── Block 3: Constructors, Helpers, Private Methods ──────
``` `

## Block 1 — Struct Definition

``` `rust
// ─── Block 1: Struct Definition ───────────────────────────
pub struct <NameOrchestrator> {
    analyzer: Arc<dyn I<NameAnalyzer>Protocol>,
}
``` `

## Block 2 — Public Contract

Block 2 is RESERVED for the domain aggregate trait ONLY.

``` `rust
// ─── Block 2: Aggregate Trait Implementation ──────────────
impl I<NameOrchestrator>Aggregate for <NameOrchestrator> {
    fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>> {
        // orchestration only
    }
}
``` `

Do NOT put `Default`, `Clone`, `Debug`, `Display`, `From` impls in Block 2.

## Block 3 — Constructors, Std Traits, and Helpers

``` `rust
// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl <NameOrchestrator> {
    pub fn new(analyzer: Arc<dyn I<NameAnalyzer>Protocol>) -> Self {
        Self { analyzer }
    }

    fn should_skip_file(&self, file: &FilePath) -> bool {
        self.policy.is_excluded(file)
    }
}
``` `

## Trait Placement Decision Rule

``` `text
Trait impl found in an agent file?
  │
  ├─ Is it the domain aggregate? (I<Name>Aggregate)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive/utility trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3
``` `
```

---

## File: .agents/skills/create-agent-rust/references/checklist.md

```markdown
# Verification Checklist

- [ ] File follows the 3-Block Structure.
- [ ] Block 1 contains exactly one implementation struct.
- [ ] Block 2 contains ONLY the aggregate trait implementation.
- [ ] Block 3 contains constructors, std traits, and private helpers.
- [ ] Agent struct implements an aggregate trait.
- [ ] Aggregate contains only public contract methods.
- [ ] Private helpers are not declared in the aggregate.
- [ ] Constructors are not declared in the aggregate.
- [ ] Std trait impls are in Block 3.
- [ ] Agent-specific helpers may remain in Block 3.
- [ ] Reusable, stateless, domain-agnostic functions are extracted to `*utility_.rs`.
- [ ] No reusable utility-like functions remain inside Block 3.
- [ ] Generic aggregate methods are object-safe or bounded with `where Self: Sized`.
- [ ] One file contains exactly one implementation struct.
- [ ] No domain data structures are defined locally.
- [ ] All domain data structures are imported from shared/taxonomy.
- [ ] Service dependencies use `Arc<dyn Trait>`.
- [ ] Value/configuration fields use shared VOs.
- [ ] Aggregate signatures use shared VOs for domain data.
- [ ] Agent contains zero I/O.
- [ ] Agent contains zero business logic.
- [ ] Agent contains zero domain computation.
- [ ] No forbidden imports from concrete `capabilities_*`.
- [ ] No forbidden imports from concrete `infrastructure_*`.
- [ ] No forbidden imports from concrete `surface_*`.
- [ ] Aggregate module is registered in the shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.
```

---

## File: .agents/skills/create-agent-rust/references/commands.md

```markdown
# Quick Commands

``` `bash
# List structs in agent files
rg -n "^\s*pub struct" crates/<crate>/src/agent_*.rs

# List aggregate trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Aggregate\s+for" crates/<crate>/src/agent_*.rs

# Check possible computation/transformation patterns
rg "\.sum\(\)|\.len\(\)|\.map\(|\.fold\(|\.collect\(" crates/<crate>/src/agent_*.rs

# Check possible I/O in agents
rg "std::fs|File::open|reqwest|hyper|sqlx|rusqlite" crates/<crate>/src/agent_*.rs

# Check forbidden imports
rg "^\s*use\s+.*(capabilities_|infrastructure_|surface_)" crates/<crate>/src/agent_*.rs

# Find unwrap_or_default usage
rg "unwrap_or_default\(\)" crates/<crate>/src/agent_*.rs

# Find possible magic numbers
rg "[0-9]+\.[0-9]+" crates/<crate>/src/agent_*.rs

# Check object safety issues
cargo check -p <crate-name> 2>&1 | rg "cannot be made into an object"
``` `

## Check Wrong Block Order

``` `bash
for file in crates/<crate>/src/agent_*.rs; do
  awk '
    FNR == 1 { std = 0; proto = 0 }
    /^impl (Default|Clone|Debug|Display)/ { if (!std) std = FNR }
    /^impl I[A-Z].*Aggregate/ { if (!proto) proto = FNR }
    END {
      if (std && proto && std < proto) {
        print "VIOLATION: " FILENAME " std trait (line " std ") before aggregate (line " proto ")"
      }
    }
  ' "$file"
done
``` `
```

---

## File: .agents/skills/create-agent-rust/references/computation-detection.md

```markdown
# Computation Detection Rules

Agent layer must not contain domain computation.

## Forbidden

- arithmetic,
- totals,
- averages,
- counts used as domain decisions,
- sum,
- fold,
- parsing,
- normalization,
- deriving domain meaning from data.

## Allowed

- iterating to call dependencies,
- routing results,
- pushing results into a collection,
- propagating errors,
- continuing or stopping pipeline.

## Bad

``` `rust
let total = files.len();
let average = total_score / total;
``` `

## Good

``` `rust
let summary = self.analyzer.summarize(files);
``` `

Note: Patterns like `.iter()` or `.map()` can appear in harmless technical code. Always inspect context. The real violation is **domain computation/transformation**, not merely using iterator control flow.
```

---

## File: .agents/skills/create-agent-rust/references/error-handling.md

```markdown
# Error Handling Rules

## Rule 1: Do not silently discard errors

Forbidden:

``` `rust
let result = checker.check().unwrap_or_default();
``` `

## Rule 2: Agent may return `Vec<<ResultVO>>` for analysis orchestration

``` `rust
fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>> {
    let mut results = Vec::new();
    for file in request.files() {
        match self.analyzer.analyze(file) {
            Ok(result) => results.extend(result.into_results()),
            Err(err) => results.push(<ResultVO>::from_analysis_error(file, err)),
        }
    }
    results
}
``` `

## Rule 3: Agent may return `Result` for execution orchestration

``` `rust
fn run(&self, request: &<ScanRequest>VO) -> Result<ExecutionReport, AgentExecutionError>;
``` `

## Rule 4: Agent must not perform I/O error handling directly

Bad:

``` `rust
let content = match std::fs::read_to_string(path.value()) {
    Ok(c) => c,
    Err(_) => String::new(),
};
``` `

Good:

``` `rust
match self.reader.read(file) {
    Ok(content) => { /* delegate to capability */ }
    Err(err) => { results.push(<ResultVO>::from_read_error(file, err)); }
}
``` `

The agent calls a port. The port implementation lives in infrastructure.
```

---

## File: .agents/skills/create-agent-rust/references/examples.md

```markdown
# Examples

## BAD: Computation in Agent

``` `rust
impl <NameOrchestrator> {
    fn process(&self, files: &[FilePath]) {
        let total = files.len(); // BAD: domain/technical computation
        let sum: usize = files.iter().map(|f| f.size()).sum(); // BAD
    }
}
``` `

Fix: Move computation to capabilities.

## BAD: Business Logic in Agent

``` `rust
impl <NameOrchestrator> {
    fn evaluate(&self, content: &FileContent) -> bool {
        content.value().contains("forbidden-marker") // BAD: business rule
    }
}
``` `

Fix: Move to capabilities.

## BAD: I/O in Agent

``` `rust
impl <NameOrchestrator> {
    fn execute(&self, path: &FilePath) {
        let content = std::fs::read_to_string(path.value()); // BAD
    }
}
``` `

Fix: Use an injected port.

## BAD: Dataclass Defined in Agent File

``` `rust
pub struct <Report>VO {
    results: Vec<String>,
}
``` `

Fix: Move to taxonomy.

## BAD: Concrete Service Field

``` `rust
pub struct <NameOrchestrator> {
    analyzer: <NameAnalyzer>, // BAD
}
``` `

Fix:

``` `rust
pub struct <NameOrchestrator> {
    analyzer: Arc<dyn I<NameAnalyzer>Protocol>,
}
``` `

## BAD: Std Trait in Block 2

``` `rust
impl Default for <NameOrchestrator> {
    fn default() -> Self { Self }
}

impl I<NameOrchestrator>Aggregate for <NameOrchestrator> {
    fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>> { Vec::new() }
}
``` `

Fix: Move `Default` to Block 3.

## GOOD: Correct 3-Block Order

``` `rust
use std::sync::Arc;

use shared::<name-feature>::taxonomy_file_path_vo::FilePath;
use shared::<name-feature>::taxonomy_result_vo::<ResultVO>;
use shared::<name-feature>::contract_analyzer_protocol::I<NameAnalyzer>Protocol;
use shared::<name-feature>::contract_orchestrator_aggregate::I<NameOrchestrator>Aggregate;
use shared::<name-feature>::taxonomy_scan_request_vo::<ScanRequest>VO;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct <NameOrchestrator> {
    analyzer: Arc<dyn I<NameAnalyzer>Protocol>,
}

// ─── Block 2: Public Contract (domain aggregate ONLY) ─────
impl I<NameOrchestrator>Aggregate for <NameOrchestrator> {
    fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>> {
        let mut results = Vec::new();
        for file in request.files() {
            match self.analyzer.analyze(file) {
                Ok(result) => results.extend(result.into_results()),
                Err(err) => results.push(<ResultVO>::from_analysis_error(file, err)),
            }
        }
        results
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl <NameOrchestrator> {
    pub fn new(analyzer: Arc<dyn I<NameAnalyzer>Protocol>) -> Self {
        Self { analyzer }
    }
}
``` `
```

---

## File: .agents/skills/create-agent-rust/references/helper-vs-utility.md

```markdown
# Helper vs Utility Decision

## Keep as Private Helper (Block 3)

Keep if ANY of these is true:

1. It accesses `self.field` or instance state.
2. It is tightly coupled to this orchestrator only.
3. It is a factory method such as `new()` or builder.
4. It contains agent-specific pipeline knowledge.
5. It contains domain knowledge.
6. It is stateless but only used by this one struct.

## Extract to Utility (`*_utility.rs`)

Extract ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business or orchestration rules.
5. Reusable: useful for multiple modules/layers.

## I/O Rule

A function with I/O can be a taxonomy utility if it is stateless, domain-agnostic, and reusable.

Stateless + I/O + domain-specific = infrastructure/port implementation.

## Decision Tree

``` `text
Found reusable code in agent?
  │
  ├─ Does it know agent-specific or domain-specific details?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need &self or struct state?
  │   └─ YES → keep as helper/method in Block 3
  │
  └─ Is it stateless, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility (I/O allowed)
``` `
```

---

## File: .agents/skills/create-agent-rust/references/layer-boundaries.md

```markdown
# Layer Boundaries (AES)

## Agent Layer (`agent_*.rs`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| Orchestration flow (`for`, `while`, `async for`)    | Domain computation                                    |
| Control flow (`if/else`, `elif`, `match`)           | Arithmetic or analytics calculations                  |
| Sequential pipeline statements                      | Data transformation logic                             |
| Calling injected aggregate/protocol traits           | Business rules                                        |
| Error propagation (`try/except`, `raise`)           | Domain validation                                     |
| Collecting results/violations into shared VO types  | File I/O (`std::fs`, `File::open`, `read_dir`)        |
| Async coordination (`tokio::select!`)               | Network calls (`reqwest`, `hyper`)                    |
| Aggregate trait implementation                      | Database operations (`sqlx`, `rusqlite`)              |
| Private helpers supporting orchestration            | Direct stdout/stderr printing                         |
|                                                     | Direct environment/system-clock/global-state mutation |
|                                                     | Direct import from concrete `capabilities_*` modules  |
|                                                     | Direct import from concrete `utility_*` modules       |
|                                                     | Locally defined domain data structures                |
|                                                     | Raw primitives for domain values in aggregate contracts |

## Allowed Dependencies

- `shared/*`
- taxonomy VOs, constants
- aggregate traits
- protocol traits

## Forbidden Dependencies

- concrete capabilities implementations
- concrete utility implementations
```

---

## File: .agents/skills/create-agent-rust/references/primitive-vo-policy.md

```markdown
# Primitive and VO Rules

Aggregate contracts should use shared VOs for domain data.

Bad:

``` `rust
pub trait I<NameOrchestrator>Aggregate {
    fn execute(&self, files: Vec<String>) -> Vec<String>;
}
``` `

Good:

``` `rust
pub trait I<NameOrchestrator>Aggregate {
    fn execute(&self, request: &<ScanRequest>VO) -> Vec<<ResultVO>>;
}
``` `

## Primitive Policy

| Primitive            | Rule                                                                                |
| -------------------- | ----------------------------------------------------------------------------------- |
| `String`           | Forbidden for domain fields and public contract values. Use VO.                     |
| `i32`, `i64`     | Forbidden for domain values. Use VO.                                                |
| `u32`, `u64`     | Forbidden for domain values. Use VO.                                                |
| `usize`, `isize` | Forbidden for domain values. Use VO.                                                |
| `f32`, `f64`     | Forbidden for domain values. Use VO.                                                |
| `char`             | Forbidden for domain values. Use VO.                                                |
| `bool`             | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str`             | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for: requests, reports, file paths, identifiers, execution results, results, policies, thresholds.
```

---

## File: ARCHITECTURE.md

```markdown
# Agentic Engineering System Architecture

## 1. Purpose

The Agentic Engineering System is a layered, AI-native architecture pattern. It keeps domain models stable, business logic readable, technical detail isolated, and layer boundaries explicit enough for both humans and AI agents to modify the system safely.

---

## 2. Workspace Organization

The architecture supports multi-language workspaces.

| Term               | Meaning                                                           |
| ------------------ | ----------------------------------------------------------------- |
| Project Workspaces | Project root containing all configuration and language members    |
| Workspace Member   | One self-contained crate, package, or module inside the workspace |
| Crates directory   | Rust workspace members                                            |
| Packages directory | TypeScript or JavaScript packages                                 |
| Modules directory  | Python modules or sub-projects                                    |

---

## 3. Naming Convention

File names must communicate three parts:

1. Layer as prefix
2. Concern as middle name
3. Role as suffix

The parts are joined by underscores, followed by the normal file extension for the language.

`layer_concern_role.rs/py/ts`

---

## 4. Vertical Slicing Folder Structure

The recommended folder structure follows this order:

#### Features member

_Example feature crate `crates|packages|modules/<name-features>/`_

``` `text
surface_<concern>_<role>.rs/py/ts                ← surface layer
capabilities_<concern>_<role>.rs/py/ts           ← capabilities layer
agent_<concern>_orchestrator.rs/py/ts            ← agent layer
``` `

Exceptions: `main.rs`, `lib.rs`, `mod.rs`, `__init__.py`, `index.ts`, `index.js`.

#### Shared member

`crates|packages|modules/shared/<common>or<domain-folder>`

``` `text
contract_<concern>_protocol.rs/py/ts             ← contract layer
contract_<concern>_aggregate.rs/py/ts            ← contract layer
taxonomy_<concern>_vo.rs/py/ts                   ← taxonomy layer
taxonomy_<concern>_event.rs/py/ts                ← taxonomy layer
taxonomy_<concern>_entity.rs/py/ts               ← taxonomy layer
taxonomy_<concern>_constant.rs/py/ts             ← taxonomy layer
utility_<concern>_<role>.rs/py/ts                ← utility layer
``` `

`shared` folder groups by domain. Use `shared/common/` for generic files.

---

## 5. Taxonomy Layer

### Purpose

Taxonomy is the domain foundation layer. It defines the stable language of the domain and must remain free from technical or behavioral concerns.

### Components

| Role         | Meaning                               |
| ------------ | ------------------------------------- |
| Value object | Immutable data concept                |
| Entity       | Stateful domain concept with identity |
| Event        | Immutable domain fact                 |
| Error        | Domain-level error                    |
| Constant     | Compile-time literal value            |

### Dependencies

Taxonomy depends on nothing.

### Special Rules

- Value objects and Constants may use all primitive types.
- Entities, Events, and Errors must use Value objects/Constants instead of primitive types (bool/str is an exception).
- Constants must be compile-time values.
- Taxonomy must not contain business rules, infrastructure, or imports from other layers.

---

## 6. Contract Layer

### Purpose

Contract defines the public behavior of the system without exposing implementation. It allows callers to depend on stable interfaces instead of concrete logic.

### Components

| Role      | Meaning                                                                                           |
| --------- | ------------------------------------------------------------------------------------------------- |
| Protocol  | Interface defining inbound behavior. It is implemented by Capabilities and consumed by the Agent. |
| Aggregate | Facade definition implemented by Agent, used by Surface to access feature behavior.               |

### Dependencies

Contract may depend on Taxonomy only.

### Special Rules

- Protocol defines behavior only without implementation.
- Aggregate hides Capabilities from Surface.

---

## 7. Utility Layer

### Purpose

Utility contains low-level technical mechanics. It exists so that Capabilities can remain clean and expressive.

### Role Naming

Utility role suffixes are unlimited. The role name is chosen based on demand and must describe the technical responsibility and concern of the file.

parser
splitter
trimmer
slugifier
sanitizer
normalizer
extractor
replacer
converter
counter
resolver
detector
builder
joiner
serializer
deserializer
encoder
decoder
hasher
generator
formatter
comparator
differ
matcher
checker
calculator
mapper
merger
grouper
sorter
deduplicator
printer

### Dependencies

Utility may depend only on Taxonomy.

### Technical Concern Examples

| Concern                 | Responsibility                                      |
| ----------------------- | --------------------------------------------------- |
| File discovery          | Walk directories, detect files, apply ignore        |
| External tool execution | Run linters, compilers, formatters, analyzers       |
| Parsing and matching    | Parse text, match patterns, extract structured data |
| Path normalization      | Normalize paths across platforms                    |
| System operations       | Handle process or environment mechanics             |

### Special Rules

- Utility must use stateless standalone functions only.
- Utility must not contain stateful objects, behavior definitions, or contract implementations.
- Utility must not make business decisions.
- Utility may perform technical operations if needed.
- Utility must not implement any contract.
- Utility role names may expand freely, but the layer must remain technical and standalone.
- Utility must use stateless standalone functions only.

---

## 8. Capabilities Layer

### Purpose

Capabilities contain the concrete implementation of the system's behavior. This layer encapsulates both **pure business logic** (computations, validations) and **external adaptations** (database access, third-party API calls, infrastructure mechanics). By hiding these implementations behind Contracts, the system keeps its behavior modular, swappable, and fully isolated from orchestration.

### Role Naming

#### Internal Examples

validator
assessor
calculator
resolver
classifier
selector
mapper
transformer
policy
enricher
evaluator
analyzer
scorer
grader
ranker
filter
checker
reviewer
approver
rejector

#### External Examples

repository
gateway
client
provider
fetcher
reader
writer
scanner
executor
publisher
subscriber
adapter
connector
uploader
downloader
sender
receiver
dispatcher
watcher
monitor

### Dependencies

- Capabilities may depend on Taxonomy, Contract, and Utility.
- Capabilities must not depend on or import other Capabilities.

### Concern Examples

Capabilities generally handle two types of concerns:

| Category                      | Concern        | Responsibility                                 |
| ----------------------------- | -------------- | ---------------------------------------------- |
| **Business Logic**      | Validation     | Check domain conditions or input correctness   |
|                               | Computation    | Calculate scores, totals, or derived values    |
|                               | Transformation | Map, filter, reduce, or reshape data           |
|                               | Resolution     | Apply rules and decide outcomes                |
|                               | Assessment     | Judge severity, compliance, grade, or quality  |
| **External Adaptation** | Repository     | Fetch or persist domain entities to a database |
|                               | Integration    | Communicate with third-party services or APIs  |
|                               | Provider       | Generate data from external systems            |

### Special Rules

- **No Inter-Capability Dependency:** Capabilities must never import or call other Capabilities directly. They are standalone execution units.
- **Pipeline Aggregation:** Multiple Capabilities (e.g., Capability A for data fetching, Capability B for business calculation) are designed to be composed into a sequential pipeline by the **Agent Layer**, not by themselves.
- **Shared Logic Extraction (DRY):** If multiple Capabilities require the same technical mechanics or functions, that logic must be extracted into a reusable standalone function in the **Utility Layer**. Capabilities must not duplicate technical code (Don't Repeat Yourself).
- **Contract Implementation:** Capabilities must implement the `protocol_` defined in the Contract Layer.
- **State Ownership:** Capabilities are the owners of business and technical state within their execution scope.
- **Utility Delegation:** Capabilities must call Utility standalone functions when low-level technical operations are required, passing their state/data as arguments.
- **No Orchestration:** Capabilities must not contain flow control (looping across capabilities, branching between capabilities, or error escalation policy). They execute their single responsibility and return a result.
- **No Domain Definition:** Capabilities must not define domain models (Entities, Value Objects); they only consume and produce Taxonomy.

---

## 9. Agent Layer

### Purpose

Agent coordinates multiple capabilities into executable flows. It controls sequence and movement, not business calculation.

### Allowed Role

The only Agent role is orchestrator.

### Dependencies

Agent may depend only on Taxonomy, Contract, and Utility.

### Allowed Flow Control

| Flow Type               | Purpose                                |
| ----------------------- | -------------------------------------- |
| Sequential execution    | Run steps in order                     |
| Looping                 | Process multiple items or events       |
| Branching               | Choose path based on result            |
| Error handling          | Recover, abort, continue, or escalate  |
| Timeout or cancellation | Stop long-running or asynchronous work |

### Special Rules

- Agent must depend on Contract, not concrete implementations.
- Agent must not use and must be completely ignorant of Capabilities implementations.
- Agent must not calculate business results.
- Agent must not define domain models.

---

## 10. Surface Layer

### Purpose

Surface is the outer boundary of the system. It handles user-facing or external-facing interaction and translates it into architectural actions.

### Allowed Roles

Surface roles include:

- command
- controller
- page
- view
- component
- router
- layout
- hook
- store
- action
- screen

### Surface Groups

| Group            | Roles                             | Dependencies                           | Rule                                            |
| ---------------- | --------------------------------- | -------------------------------------- | ----------------------------------------------- |
| Smart surfaces   | command, controller, page, router | Taxonomy, Contract Aggregate, Utility | May initiate feature behavior through aggregate |
| Utility surfaces | hook, store, action, screen       | Taxonomy only                          | Support smart surfaces but must not import them |
| Passive surfaces | component, view, layout           | Taxonomy only                          | Presentation-only, no logic or orchestration    |

### Special Rules

- Smart surfaces must consume Contract Aggregates.
- Surfaces must not import Capabilities, Utility, or Agent directly.
- Surfaces must not contain business calculation or orchestration.

---

## 11. Root Layer

### Purpose

Root is the composition layer. It assembles the system by connecting concrete implementations to contracts and starting the application.

### Components

| Role      | Meaning                                                                           |
| --------- | --------------------------------------------------------------------------------- |
| Container | Wires one feature by connecting Capabilities to Contract protocols and aggregates |
| Entry     | Bootstraps the application and composes feature containers                        |

### Dependencies

Root may depend on all layers.

### Special Rules

- Root may instantiate and wire components.
- Root must not contain business logic.
- Root must not contain orchestration policy.
- Root must not contain technical parsing or user interface behavior.
```

---

## File: PRD.md

```markdown
# PRD — Lint Arwaky

## Problem Statement

Software projects accumulate quality debt silently. Developers lack a single tool that audits Rust + Python + JavaScript/TypeScript together, enforces architectural rules, and works for both human developers (CLI) and AI agents (MCP tools).

## Goals & Success Metrics

- Goal 1: Multi-language linting in a single pass (Rust, Python, JS/TS)
- Goal 2: 24 AES rules enforced across 5 groups (Naming, Import, Quality, Role, Orphan)
- Goal 3: MCP server with 5 tools for autonomous AI-agent integration
- Goal 4: Zero bypass tolerance (`noqa`, `type: ignore`, `#[allow(...)]` flagged)
- Goal 5: Self-auditing — project lints itself under its own rule engine

## User Personas

- **AI Agent**: Autonomous linting, self-healing, code review via MCP tools
- **Developer**: Lint codebases, enforce architecture during local development
- **DevOps / CI**: Quality gates, trend reports, dependency scans
- **Contributor**: Extend adapters, add CLI commands
- **Reviewer**: Architecture audit, code quality analysis

## Scope

- In scope:
  - CLI binary (`lint-arwaky-cli`) for human developers
  - MCP server (`lint-arwaky-mcp`) for AI agents
  - TUI file browser (`lint-arwaky-tui`)
  - 24 AES rules across 5 groups
  - External linter adapters (Clippy, Ruff, MyPy, Bandit, ESLint, Prettier, TSC)
  - SARIF 2.1.0, JUnit XML, JSON reports
  - Git hooks integration
  - Auto-fix capabilities

- Out of scope:
  - IDE plugins (VS Code, IntelliJ)
  - Web dashboard
  - Cloud-hosted SaaS
  - Non-Rust implementation

## Feature Requirements (Prioritized)

### P0 — Must Have

- [ ] Multi-language scanning (Rust, Python, JS/TS)
- [ ] 24 AES rules enforcement
- [ ] CLI with `check`, `scan`, `fix`, `ci` commands
- [ ] MCP server with 5 tools
- [ ] Self-auditing capability

### P1 — Should Have

- [ ] External linter adapters (Clippy, Ruff, MyPy, Bandit, ESLint, Prettier, TSC)
- [ ] SARIF 2.1.0, JUnit XML, JSON reports
- [ ] Git hooks integration
- [ ] Auto-fix capabilities
- [ ] Watch mode for continuous linting

### P2 — Nice to Have

- [ ] TUI file browser
- [ ] Orphan code detection
- [ ] Duplicate code detection
- [ ] Dependency vulnerability scanning

## Non-functional Requirements (High-level)

- Performance: Scan 1000 files in < 5 seconds
- Security: No network calls required for core functionality
- Scalability: Handle monorepos with 10,000+ files
- Platform: Linux (primary), macOS (secondary)
- Binary: Static release via `cargo build --release`

## Open Questions / Risks

- Windows support timeline
- Performance optimization for very large codebases
- Integration with CI/CD platforms (GitHub Actions, GitLab CI)
```

---

## File: crates/cli-commands/FRD.md

```markdown
# FRD — cli-commands

## Feature Goal
The cli-commands crate provides a unified command-line interface (CLI) that drives the entire lint-arwaky linting pipeline. It implements thin surface handlers that delegate business logic to agent/orchestrator layers (IAnalysisPipelineAggregate, MaintenanceCommandsAggregate, etc.). Report formatting is delegated to the `report-formatter` crate via `IReportFormatterAggregate`.

## Commands & Scope

### Analysis Commands
- **check** — Run full architecture compliance analysis on a target path. Runs all 6 linter groups in sequence: code analysis (AES301-305), naming rules (AES101-102), import rules (AES201-205), external adapters (Clippy, Ruff, ESLint), role rules (AES401-406), orphan detection (AES501-506). Supports `--git-diff` for staged-only scanning.
- **scan** — Multi-workspace discovery scan that auto-detects Cargo.toml/pyproject.toml/package.json members, creates per-project DI containers, and runs the full analysis pipeline on each. Supports `--member <name>` to target a specific workspace member.
- **ci** — CI-optimized analysis with configurable threshold and exit codes. Auto-fails on CRITICAL violations regardless of score. Compares score against threshold as float comparison (not truncated integer).

### Fix Commands
- **fix** — Apply automatic fixes to files that violate rules. Supports `--dry-run` for preview mode. Only auto-fixes safe, non-destructive rule violations.

### Maintenance Commands
- **doctor** — Toolchain diagnostics: checks availability and version of cargo, python3, node, git, and other required tools. Returns exit code 0 regardless of findings (diagnostic only).
- **security** — Vulnerability scanning via cargo-audit (Rust) or bandit (Python). Returns exit code 3 when the scanning tool is missing, exit code 1 when vulnerabilities are found, exit code 0 when clean.
- **dependencies** — Dependency report from Cargo.lock / pyproject.toml / package.json. Lists all dependencies with version and type.

### Project Setup Commands
- **init** — Create default lint-arwaky configuration file in the current project directory (XDG-compliant).
- **install** — Install adapter dependencies (Clippy, Ruff, ESLint, etc.) for the detected language. Supports `--sudo` flag.
- **install-hook** — Install git pre-commit hook that runs lint-arwaky on staged files.
- **uninstall-hook** — Remove the installed git pre-commit hook.
- **mcp-config --client <name>** — Print MCP server configuration for the specified client (e.g., `claude`, `cursor`).
- **config-show** — Display active configuration files and their contents. Sensitive values (AWS keys, long base64 strings) are redacted before display.

### Utility Commands
- **adapters** — List enabled external lint adapters (Clippy, Ruff, ESLint, etc.) discovered by the external-lint layer.
- **watch** — Monitor file changes and trigger re-scans on modified files.
- **orphan <path>** — Check if a specific file is dead/unreachable code by analyzing the workspace import graph.
- **version** — Show version and build information (Git commit hash, Rustc version).

## Architecture

### Layer Delegation Pattern
All surface handlers follow strict AES406 rules:
- **Surfaces** (`surface_*.rs`) — Thin dispatch layer. Parse args, call aggregates, format output. Zero business logic.
- **Agents/Orchestrators** (`agent_*.rs`) — Orchestrate multi-linter pipelines. Depend on contracts only. No I/O, no formatting.
- **Capabilities** (`capabilities_*.rs`) — Single-responsibility implementations (report formatters, etc.). Implement contract protocols.

### Analysis Pipeline
The core analysis pipeline is defined by `IAnalysisPipelineAggregate` trait implemented by `AnalysisPipelineOrchestrator`:
1. Collect source files (ignore-aware via `collect_all_source_files`)
2. Run code analysis (AES301-305)
3. Run naming, import, external, and role audits concurrently
4. Run orphan detection across workspace
5. Merge results, apply path filtering, format output

### Formatters
Report formatting is delegated to the `report-formatter` crate via `IReportFormatterAggregate`. The surface layer never formats output directly — it calls `self.report_formatter.format(&report, format)`. Supported formats:
- **Text** — Human-readable formatted output with severity badges
- **JSON** — Machine-readable structured output for CI/CD integration
- **SARIF 2.1.0** — Standard static analysis results format (VS Code, GitHub Code Scanning)
- **JUnit XML** — Test report format for CI/CD pipelines

## Exit Codes
| Code | Meaning |
|------|---------|
| 0 | Success — no violations found |
| 1 | Violations/findings detected |
| 2 | System/operational error |
| 3 | Required tool missing (e.g., cargo-audit, bandit) |

## Non-Functional Requirements
- **Cross-platform** — File walker uses canonical paths (not inodes), works on all platforms including Windows.
- **SARIF support** — Full SARIF 2.1.0 output for IDE integration and GitHub Code Scanning.
- **Performance** — Ignore-aware scanning excludes `target/`, `node_modules/`, `.git/`, `dist/`, `build/`, `coverage/`, `.venv/`. Symlink targets outside workspace root are pruned.
- **Concurrency** — Async linter groups run concurrently via `tokio::join!`. Deferred container construction for lightweight commands (version, adapters).
- **Multi-workspace** — Scan auto-discovers workspace members and runs per-project analysis with isolated DI containers.

## Configuration Resolution Algorithm

When loading a configuration file for a given project path, lint-arwaky searches in this priority order:

1. **Project-root YAML** — Search from the project root upward (max 3 levels) for language-specific config files:
   - Rust: `lint_arwaky.config.rust.yaml` or `lint_arwaky.config.yaml`
   - Python: `lint_arwaky.config.python.yaml` or `lint_arwaky.config.yaml`
   - TypeScript: `lint_arwaky.config.javascript.yaml` or `lint_arwaky.config.yaml`

2. **Parent directory traversal** — Walk up parent directories (depth ≤ 3) looking for config files, allowing shared configs at the workspace root to apply to all members.

3. **XDG user config** — Check `<XDG_CONFIG_HOME>/lint-arwaky/<config-file>` (default: `~/.config/lint-arwaky/`).

4. **System XDG dirs** — Check each entry in `$XDG_CONFIG_DIRS` (or `/etc/xdg/lint-arwaky/` by default) for `<config-file>`. Limited to 8 directories, only absolute paths accepted.

5. **Embedded defaults** — If no config file is found at any level, use compiled-in defaults appropriate for the detected language (based on `Cargo.toml`, `pyproject.toml`, or `package.json` presence).

When multiple config files are found across levels, the deepest match wins (most specific path takes priority). The loaded config is cached by file path to avoid re-parsing.

## Dependencies
- `report-formatter` — Report formatting capabilities (text, JSON, SARIF, JUnit)
- `shared` — Taxonomy, contracts, and utility types
- `code-analysis`, `naming-rules`, `import-rules`, `role-rules`, `orphan-detector`, `external-lint` — Linter subsystems

## Success Indicators
- [ ] AES compliance — the crate passes self-lint (`cargo run --bin lint-arwaky-cli -- check`)
- [ ] Surface thinness — surface handlers contain no business logic, only dispatch
- [ ] Formatters delegated — surface uses `IReportFormatterAggregate`, no inline formatting
- [ ] Exit code correctness — all commands follow the standardized exit code convention
- [ ] Secret redaction — config-show never leaks tokens or API keys
```

---

## File: crates/shared/src/cli-commands/contract_analysis_pipeline_aggregate.rs

```rust
// PURPOSE: IAnalysisPipelineAggregate — aggregate trait for the full analysis pipeline
//
// Defines the public API for running all linter groups in sequence and returning
// a unified ScanReport. This is what the surface layer depends on to orchestrate
// code-analysis, naming, imports, external adapters, roles, and orphan detection.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_scan_report_vo::{PipelineError, ScanReport};
use crate::cli_commands::taxonomy_scan_request_vo::ScanRequest;

/// IAnalysisPipelineAggregate — aggregate port for full analysis pipeline orchestration.
///
/// Implemented by AnalysisPipelineOrchestrator (agent layer).
/// Provides methods for running the complete lint pipeline on a target,
/// with multi-workspace discovery support and single-file orphan checking.
#[async_trait::async_trait]
pub trait IAnalysisPipelineAggregate: Send + Sync {
    /// Run the full analysis pipeline on the request target.
    async fn run(&self, request: ScanRequest) -> Result<ScanReport, PipelineError>;

    /// Run the full analysis pipeline with multi-workspace discovery.
    ///
    /// Discovers workspace members (Cargo.toml, pyproject.toml, package.json workspaces),
    /// runs all 6 linter groups per member, runs cross-workspace orphan detection,
    /// filters results to each member's path, and aggregates into a single ScanReport.
    async fn run_with_discovery(&self) -> Result<ScanReport, PipelineError>;

    /// Check if a single file is an orphan.
    ///
    /// Scans ALL source files to build the import graph for reachability analysis,
    /// then filters results to only the specified file path.
    fn check_orphan_single_file(
        &self,
        file_path: &str,
        workspace_root: &str,
    ) -> Result<Vec<LintResult>, PipelineError>;
}
```

---

## File: crates/shared/src/cli-commands/mod.rs

```rust
// cli-commands — taxonomy and contract types
pub mod contract_analysis_pipeline_aggregate;
pub mod contract_report_formatter_aggregate;
pub mod contract_report_formatter_protocol;
pub mod taxonomy_catalog_constant;

pub mod taxonomy_cli_vo;
pub mod taxonomy_command_catalog_vo;
pub mod taxonomy_format_vo;
pub mod taxonomy_metadata_vo;
pub mod taxonomy_position_vo;
pub mod taxonomy_protocol_vo;
pub mod taxonomy_result_vo;
pub mod taxonomy_scan_report_vo;
pub mod taxonomy_scan_request_vo;
pub mod taxonomy_score_vo;
pub mod taxonomy_severity_vo;

pub mod utility_score_calculator;
```

---

## File: crates/shared/src/cli-commands/taxonomy_format_vo.rs

```rust
// PURPOSE: Format — output format enum for --format CLI arg (text, json, sarif, junit)
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Format {
    #[default]
    Text,
    Json,
    Sarif,
    Junit,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Format::Text => write!(f, "text"),
            Format::Json => write!(f, "json"),
            Format::Sarif => write!(f, "sarif"),
            Format::Junit => write!(f, "junit"),
        }
    }
}

impl FromStr for Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Format::Text),
            "json" => Ok(Format::Json),
            "sarif" => Ok(Format::Sarif),
            "junit" => Ok(Format::Junit),
            other => Err(format!(
                "unknown format '{other}': expected one of text, json, sarif, junit"
            )),
        }
    }
}

impl ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Format::Text, Format::Json, Format::Sarif, Format::Junit]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Format::Text => Some(clap::builder::PossibleValue::new("text")),
            Format::Json => Some(clap::builder::PossibleValue::new("json")),
            Format::Sarif => Some(clap::builder::PossibleValue::new("sarif")),
            Format::Junit => Some(clap::builder::PossibleValue::new("junit")),
        }
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_result_vo.rs

```rust
// PURPOSE: LintResult, LintResultList, FilePathSet — value objects for lint violation results
use serde::{Deserialize, Serialize};

use crate::cli_commands::taxonomy_position_vo::Position;
use crate::cli_commands::taxonomy_severity_vo::Severity;
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_layer_vo::Identity;
use crate::common::taxonomy_lint_vo::LocationList;
use crate::common::taxonomy_lint_vo::ScopeRef;
use crate::common::taxonomy_message_vo::LintMessage;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LintResult {
    pub file: FilePath,
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub code: ErrorCode,
    pub message: LintMessage,
    pub source: Option<AdapterName>,
    pub severity: Severity,
    pub enclosing_scope: Option<ScopeRef>,
    pub related_locations: LocationList,
}

impl LintResult {
    /// Convenience constructor used by architecture checkers (make_result / mk pattern).
    pub fn new_arch(
        file: &str,
        line: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self::new_arch_with_column(file, line, 0, code, sev, msg)
    }

    /// Column-aware constructor for architecture checkers.
    pub fn new_arch_with_column(
        file: &str,
        line: usize,
        column: usize,
        code: &str,
        sev: Severity,
        msg: impl Into<String>,
    ) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(column as i64),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    /// Specialized constructor for orphan detection results (no enclosing scope).
    pub fn new_orphan(file: &str, msg: impl Into<String>, sev: Severity, code: &str) -> Self {
        Self {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(0),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        }
    }

    pub fn position(&self) -> Position {
        Position {
            line: self.line.clone(),
            column: self.column.clone(),
        }
    }
    pub fn identity(&self) -> Identity {
        Identity::new(format!(
            "{}:{}:{}:{:?}",
            self.file, self.line, self.code, self.source
        ))
    }
}

/// Generate a `Vec<T>`-backed newtype with `Default`, `new`, `iter`,
/// `len`, `is_empty`, `push`, and `append`. Used for the `LintResultList`
/// wrapper below; siblings `ImportInfoList`/`PrimitiveViolationList` in
/// `taxonomy_import_source_vo.rs` carry the same surface.
macro_rules! lint_result_list_wrapper {
    ($name:ident, $item:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
        pub struct $name {
            pub values: Vec<$item>,
        }

        impl $name {
            pub fn new(value: Vec<$item>) -> Self {
                Self { values: value }
            }
            pub fn iter(&self) -> std::slice::Iter<'_, $item> {
                self.values.iter()
            }
            pub fn len(&self) -> usize {
                self.values.len()
            }
            pub fn is_empty(&self) -> bool {
                self.values.is_empty()
            }
            pub fn push(&mut self, item: $item) {
                self.values.push(item);
            }
            pub fn append(&mut self, item: $item) {
                self.values.push(item);
            }
        }
    };
}

lint_result_list_wrapper!(LintResultList, LintResult);
```

---

## File: crates/shared/src/cli-commands/taxonomy_scan_report_vo.rs

```rust
// PURPOSE: ScanReport VO — output of the analysis pipeline
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_common_vo::Score;

/// Severity level for pipeline diagnostics.
#[derive(Debug, Clone)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

/// A diagnostic message from a pipeline subsystem.
pub struct PipelineDiagnostic {
    pub source: String,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

impl PipelineDiagnostic {
    pub fn new(source: String, message: String, severity: DiagnosticSeverity) -> Self {
        Self {
            source,
            message,
            severity,
        }
    }
}

/// Error types that can occur during pipeline execution.
#[derive(Debug, Clone)]
pub enum PipelineError {
    PathNotFound(String),
    InvalidPath(String),
    WorkspaceDiscovery(String),
    Analysis(String),
    Io(String),
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::PathNotFound(p) => write!(f, "path not found: {p}"),
            PipelineError::InvalidPath(p) => write!(f, "invalid path: {p}"),
            PipelineError::WorkspaceDiscovery(e) => write!(f, "workspace discovery failed: {e}"),
            PipelineError::Analysis(e) => write!(f, "analysis failed: {e}"),
            PipelineError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for PipelineError {}

/// Results of the full analysis pipeline.
pub struct ScanReport {
    pub results: Vec<LintResult>,
    pub diagnostics: Vec<PipelineDiagnostic>,
    pub score: Option<Score>,
}

impl ScanReport {
    pub fn new(results: Vec<LintResult>, diagnostics: Vec<PipelineDiagnostic>) -> Self {
        Self {
            results,
            diagnostics,
            score: None,
        }
    }

    /// Return the number of violations (results with severity > INFO).
    pub fn violation_count(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.severity != crate::cli_commands::taxonomy_severity_vo::Severity::INFO)
            .count()
    }

    /// Attach a score to the report.
    pub fn with_score(mut self, score: Score) -> Self {
        self.score = Some(score);
        self
    }
}
```

---

## File: crates/shared/src/cli-commands/taxonomy_scan_request_vo.rs

```rust
// PURPOSE: ScanRequest VO — request payload for the analysis pipeline
use crate::cli_commands::taxonomy_format_vo::Format;

/// Target path for the scan.
pub struct ScanTarget {
    pub value: String,
}

impl ScanTarget {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Default for ScanTarget {
    fn default() -> Self {
        Self {
            value: ".".to_string(),
        }
    }
}

/// Mode of analysis to run.
#[derive(Debug, Clone, Default)]
pub enum ScanMode {
    #[default]
    Check,
    Scan,
    Ci {
        threshold: u32,
    },
}

/// Request to run the full analysis pipeline.
pub struct ScanRequest {
    pub target: ScanTarget,
    pub mode: ScanMode,
    pub filter: Option<String>,
    pub member: Option<String>,
    pub format: Format,
}

impl ScanRequest {
    pub fn new(target: ScanTarget, mode: ScanMode) -> Self {
        Self {
            target,
            mode,
            filter: None,
            member: None,
            format: Format::Text,
        }
    }
}
```

---

## File: crates/shared/src/code-analysis/contract_code_analysis_aggregate.rs

```rust
// PURPOSE: ICodeAnalysisAggregate — aggregate trait for code-analysis checks (AES301–AES305) and formatting reports
//
// Defines the public API for the code-analysis feature. This is what the
// surface layer (CLI, MCP, TUI) depends on to run quality checks, calculate
// scores, and generate reports.
//
// Unlike other aggregates (IImportRunnerAggregate, INamingRunnerAggregate),
// this one also handles report formatting and score calculation — it's both
// an orchestrator and a presentation boundary.
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use crate::common::taxonomy_common_vo::Score;
use crate::common::taxonomy_path_vo::FilePath;

/// ICodeAnalysisAggregate — aggregate port for code-analysis orchestration.
///
/// Implemented by CodeAnalysisOrchestrator (agent layer).
/// Provides methods for:
///   - Running analysis on a single project or directory
///   - Calculating quality scores from violation results
///   - Checking for CRITICAL severity violations
///   - Formatting results as human-readable reports
///   - Querying active rule configurations
pub trait ICodeAnalysisAggregate: Send + Sync {
    /// Run complete AES analysis on a project root directory.
    fn run_code_analysis(&self, project_root: &FilePath) -> LintResultList;
    /// Run AES analysis on a specific source directory (e.g., crates/, src/).
    fn run_code_analysis_dir(&self, src_dir: &FilePath) -> LintResultList;
    /// Run analysis on an arbitrary path (file or directory).
    fn run_code_analysis_path(&self, path: &FilePath) -> Vec<LintResult>;
    /// Calculate a quality score (0.0–100.0) from violation results.
    fn calc_score(&self, results: &[LintResult]) -> Score;
    /// Check if any CRITICAL violations exist in the results.
    fn check_critical(&self, results: &[LintResult]) -> bool;
    /// Format violations into a human-readable compliance report.
    fn format_report(&self, results: &LintResultList, project_root: &FilePath) -> String;
    /// Return list of currently active (enabled) rule configurations.
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO>;
}
```

---

## File: crates/shared/src/code-analysis/mod.rs

```rust
// code-analysis — taxonomy and contract types
pub mod contract_adapter_protocol;
pub mod contract_bypass_checker_protocol;
pub mod contract_class_protocol;
pub mod contract_code_analysis_aggregate;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_dead_inheritance_protocol;
pub mod contract_layer_detection_aggregate;
pub mod contract_line_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_import_source_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;
pub mod utility_bypass;
pub mod utility_column;
pub mod utility_duplication;
pub mod utility_file_reader;
pub mod utility_language_mapper;
pub mod utility_mandatory;
pub mod utility_target;
pub use taxonomy_violation_code_analysis_vo::{AesCodeAnalysisViolation, Language};
```

---

## File: crates/shared/src/common/mod.rs

```rust
// common — truly shared types used by multiple features
pub mod contract_executor_protocol;
pub mod taxonomy_action_vo;
pub mod taxonomy_adapter_list_vo;
pub use utility_file::{
    collect_all_source_files, collect_all_source_files_raw, find_workspace_root, scan_directory,
};
pub mod taxonomy_adapter_error;
pub mod taxonomy_adapter_name_vo;
pub mod taxonomy_byte_count_vo;
pub mod taxonomy_common_error;
pub mod taxonomy_common_vo;
pub mod taxonomy_definition_vo;
pub mod taxonomy_depth_vo;
pub mod taxonomy_display_content_vo;
pub mod taxonomy_duration_vo;
pub mod taxonomy_error_vo;
pub mod taxonomy_filesystem_error;
pub mod taxonomy_git_vo;
pub mod taxonomy_job_id_vo;
pub mod taxonomy_job_vo;
pub mod taxonomy_language_info_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_layer_vo;
pub mod taxonomy_line_count_vo;
pub mod taxonomy_lint_vo;
pub mod taxonomy_message_vo;
pub mod taxonomy_name_vo;
pub mod taxonomy_naming_list_vo;
pub mod taxonomy_package_name_vo;
pub mod taxonomy_parser_error;
pub mod taxonomy_path_utils_vo;
pub mod taxonomy_path_vo;
pub mod taxonomy_paths_vo;
pub mod taxonomy_percentage_vo;
pub mod taxonomy_response_data_vo;
pub mod taxonomy_severity_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_suffix_vo;
pub mod taxonomy_suggestion_vo;
pub mod taxonomy_threshold_vo;
pub mod utility_command_runner;
pub mod utility_file;
pub mod utility_language_detector;
pub mod utility_layer_detector;
pub mod utility_path_normalization;
pub mod utility_value_object_generator;
pub use utility_signature_parser::{
    extract_python_method_signatures, extract_trait_method_signatures,
    extract_typescript_method_signatures, python_signature_uses_forbidden_primitive,
    signature_uses_forbidden_primitive, typescript_signature_uses_forbidden_primitive,
};
pub mod utility_signature_parser;
```

---

## File: crates/shared/src/common/taxonomy_adapter_error.rs

```rust
// PURPOSE: AdapterError, ScanError, ValidationError — structured error types for adapter operations
use crate::common::taxonomy_adapter_name_vo::AdapterName;
use crate::common::taxonomy_common_error::Cause;
use crate::common::taxonomy_common_error::Constraint;
use crate::common::taxonomy_common_error::ErrorMessage;
use crate::common::taxonomy_common_error::ExitCode;
use crate::common::taxonomy_common_error::FieldName;
use crate::common::taxonomy_error_vo::ErrorCode;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, thiserror::Error)]
pub struct AdapterError {
    pub adapter_name: AdapterName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub command: Option<ContentString>,
    #[serde(default)]
    pub stderr: Option<ErrorMessage>,
    #[serde(default)]
    pub exit_code: Option<ExitCode>,
}

impl AdapterError {
    pub fn new(adapter_name: AdapterName, message: ErrorMessage) -> Self {
        Self {
            adapter_name,
            message,
            error_code: None,
            command: None,
            stderr: None,
            exit_code: None,
        }
    }
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self.error_code.as_ref() {
            Some(c) => format!(" [{}]", c),
            None => String::new(),
        };
        write!(f, "[{}]{} {}", self.adapter_name, code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ScanError {
    pub path: FilePath,
    pub message: ErrorMessage,
    #[serde(default)]
    pub error_code: Option<ErrorCode>,
    #[serde(default)]
    pub adapter_name: Option<AdapterName>,
    #[serde(default)]
    pub cause: Option<Cause>,
}

impl ScanError {
    pub fn new(path: FilePath, message: ErrorMessage) -> Self {
        Self {
            path,
            message,
            error_code: None,
            adapter_name: None,
            cause: None,
        }
    }
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adapter = match self.adapter_name.as_ref() {
            Some(a) => format!(" ({})", a),
            None => String::new(),
        };
        let code = match self.error_code.as_ref() {
            Some(c) => format!(" [{}]", c),
            None => String::new(),
        };
        write!(
            f,
            "Scan failed{}{}: {} — {}",
            adapter, code, self.path, self.message
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, thiserror::Error)]
pub struct ValidationError {
    pub field_name: FieldName,
    pub message: ErrorMessage,
    #[serde(default)]
    pub constraint: Option<Constraint>,
    #[serde(default)]
    pub value: Option<String>,
}

impl ValidationError {
    pub fn new(field_name: FieldName, message: ErrorMessage) -> Self {
        Self {
            field_name,
            message,
            constraint: None,
            value: None,
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Validation failed on '{}': {}",
            self.field_name, self.message
        )
    }
}
```

---

## File: crates/shared/src/common/taxonomy_adapter_name_vo.rs

```rust
// PURPOSE: AdapterName — validated newtype for adapter/linter name strings
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// adapter_name_vo — Adapter and tool identifier value objects.
///
/// Adapter/tool identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AdapterName {
    pub value: String,
}

impl AdapterName {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new AdapterName from a string.
    ///
    /// # Errors
    /// Returns an error if the adapter name is empty or only whitespace.
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err("Adapter name cannot be empty".to_string());
        }
        Ok(AdapterName {
            value: value.trim().to_string(),
        })
    }

    /// Create a raw AdapterName without error validation (for static compile-time safe inputs).
    pub fn raw<S: Into<String>>(value: S) -> Self {
        AdapterName {
            value: value.into(),
        }
    }
}

impl std::ops::Deref for AdapterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for AdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for AdapterName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_error.rs

```rust
// PURPOSE: Cause, Constraint, ExitCode, FieldName, ModuleName, PrimitiveTypeName — common error value objects
pub use crate::common::taxonomy_common_vo::ErrorMessage;
use crate::string_value_object;
use serde::Serialize;

string_value_object!(Cause);
string_value_object!(Constraint);
string_value_object!(FieldName);
string_value_object!(ModuleName);
string_value_object!(PrimitiveTypeName);

/// Strongly-typed exit code value object. Written manually because the
/// `string_value_object!` macro only supports `String` (not `i64`).
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ExitCode {
    pub value: crate::common::taxonomy_common_vo::LineNumber,
}

impl ExitCode {
    pub fn new(value: impl Into<crate::common::taxonomy_common_vo::LineNumber>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> i64 {
        self.value.value()
    }
}

impl std::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ExitCode {
    fn from(v: i64) -> Self {
        Self {
            value: crate::common::taxonomy_common_vo::LineNumber::new(v),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ExitCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct W {
            value: crate::common::taxonomy_common_vo::LineNumber,
        }
        let w = W::deserialize(deserializer)?;
        Ok(Self { value: w.value })
    }
}
```

---

## File: crates/shared/src/common/taxonomy_common_vo.rs

```rust
// PURPOSE: BooleanVO, ColumnNumber, Count, DataFlowList, LineContentList, LineNumber, PatternList, Score, Timestamp — common VOs
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_job_id_vo::JobId;
use crate::common::taxonomy_layer_vo::LineContentVO;
use crate::common::taxonomy_response_data_vo::ResponseData;
use crate::common::taxonomy_severity_vo::Severity;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct BooleanVO {
    pub value: bool,
}

impl BooleanVO {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for BooleanVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for BooleanVO {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for BooleanVO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BooleanVOVisitor {}
        impl<'de> serde::de::Visitor<'de> for BooleanVOVisitor {
            type Value = BooleanVO;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BooleanVO { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<bool>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(BooleanVO { value: val })
            }
        }
        deserializer.deserialize_any(BooleanVOVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ColumnNumber {
    pub value: i64,
}

impl ColumnNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for ColumnNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for ColumnNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for ColumnNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ColumnNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for ColumnNumberVisitor {
            type Value = ColumnNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ColumnNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(ColumnNumber { value: val })
            }
        }
        deserializer.deserialize_any(ColumnNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Count {
    pub value: i64,
}

impl Count {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for Count {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Count {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CountVisitor {}
        impl<'de> serde::de::Visitor<'de> for CountVisitor {
            type Value = Count;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Count { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Count { value: val })
            }
        }
        deserializer.deserialize_any(CountVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataFlowList {
    pub values: Vec<ErrorMessage>,
}

impl DataFlowList {
    pub fn new(value: Vec<ErrorMessage>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ErrorMessage] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ErrorMessage> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ErrorMessage) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobIdList {
    pub values: Vec<JobId>,
}

impl JobIdList {
    pub fn new(value: Vec<JobId>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[JobId] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, JobId> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: JobId) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineContentList {
    pub values: Vec<LineContentVO>,
}

impl LineContentList {
    pub fn new(value: Vec<LineContentVO>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[LineContentVO] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, LineContentVO> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: LineContentVO) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
#[derive(Default)]
pub struct LineNumber {
    pub value: i64,
}

impl LineNumber {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
}

impl std::fmt::Display for LineNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<i64> for LineNumber {
    fn from(v: i64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for LineNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LineNumberVisitor {}
        impl<'de> serde::de::Visitor<'de> for LineNumberVisitor {
            type Value = LineNumber;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LineNumber { value: v as i64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<i64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(LineNumber { value: val })
            }
        }
        deserializer.deserialize_any(LineNumberVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct PatternList {
    pub values: Vec<String>,
}

impl PatternList {
    pub fn new(value: impl IntoPatternListValues) -> Self {
        Self {
            values: value.into_pattern_list_values(),
        }
    }
    pub fn values(&self) -> &[String] {
        &self.values
    }
}

impl PatternList {
    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: String) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseDataList {
    pub values: Vec<ResponseData>,
}

impl ResponseDataList {
    pub fn new(value: Vec<ResponseData>) -> Self {
        Self { values: value }
    }
    pub fn values(&self) -> &[ResponseData] {
        &self.values
    }
    pub fn iter(&self) -> std::slice::Iter<'_, ResponseData> {
        self.values.iter()
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    pub fn push(&mut self, item: ResponseData) {
        self.values.push(item);
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Score {
    pub value: f64,
}

impl Score {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn is_perfect(&self) -> bool {
        self.value >= 100.0
    }
    pub fn is_passing(&self, threshold: &Score) -> bool {
        self.value >= threshold.value
    }
    pub fn deduct(&self, severity: &Severity) -> Score {
        Score {
            value: self.value - severity.score_impact(),
        }
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}", self.value)
    }
}

impl From<f64> for Score {
    fn from(v: f64) -> Self {
        Self { value: v }
    }
}

impl<'de> serde::Deserialize<'de> for Score {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ScoreVisitor {}
        impl<'de> serde::de::Visitor<'de> for ScoreVisitor {
            type Value = Score;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v })
            }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Score { value: v as f64 })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<f64>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Score { value: val })
            }
        }
        deserializer.deserialize_any(ScoreVisitor {})
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct Timestamp {
    pub value: String,
}

impl Timestamp {
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn now() -> Self {
        Self {
            value: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for Timestamp {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for Timestamp {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimestampVisitor {}
        impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
            type Value = Timestamp;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("primitive or map with 'value' key")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp {
                    value: v.to_string(),
                })
            }
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Timestamp { value: v })
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut value = None;
                while let Some(k) = map.next_key::<String>()? {
                    if k == "value" {
                        value = Some(map.next_value::<String>()?);
                    } else {
                        let _: serde::de::IgnoredAny = map.next_value()?;
                    }
                }
                let val = value.ok_or_else(|| serde::de::Error::missing_field("value"))?;
                Ok(Timestamp { value: val })
            }
        }
        deserializer.deserialize_any(TimestampVisitor {})
    }
}

// Custom Coercion Traits for PatternList

pub trait IntoPatternListValues {
    fn into_pattern_list_values(self) -> Vec<String>;
}

impl IntoPatternListValues for &str {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IntoPatternListValues for String {
    fn into_pattern_list_values(self) -> Vec<String> {
        vec![self]
    }
}

impl IntoPatternListValues for Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self
    }
}

impl IntoPatternListValues for Vec<&str> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.into_iter().map(|s| s.to_string()).collect()
    }
}

impl IntoPatternListValues for &Vec<String> {
    fn into_pattern_list_values(self) -> Vec<String> {
        self.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
#[derive(Default)]
pub struct ErrorMessage {
    pub value: String,
}

impl ErrorMessage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<&str> for ErrorMessage {
    fn from(s: &str) -> Self {
        Self {
            value: s.to_string(),
        }
    }
}

impl From<String> for ErrorMessage {
    fn from(s: String) -> Self {
        Self { value: s }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_error_vo.rs

```rust
// PURPOSE: ErrorCode — value object for AES error code identification
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// error_code_vo — Error code value object.
///
/// Linter error code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ErrorCode {
    code: String,
}

impl ErrorCode {
    pub fn code(&self) -> &str {
        &self.code
    }
    /// Create a new ErrorCode from a string.
    ///
    /// # Errors
    /// Returns an error if the code is empty.
    pub fn new<S: Into<String>>(code: S) -> Result<Self, String> {
        let code = code.into();
        if code.is_empty() {
            return Err("Error code cannot be empty".to_string());
        }
        Ok(ErrorCode { code })
    }

    /// Create a raw ErrorCode without error validation.
    pub fn raw<S: Into<String>>(code: S) -> Self {
        ErrorCode { code: code.into() }
    }

    /// Returns true if the code is a style error (starts with E, W, or D).
    pub fn is_style(&self) -> bool {
        self.code.starts_with('E') || self.code.starts_with('W') || self.code.starts_with('D')
    }
    pub fn is_logic(&self) -> bool {
        self.code.starts_with('F') || self.code.starts_with('I')
    }
    pub fn is_security(&self) -> bool {
        self.code.starts_with('B')
    }
    pub fn is_architecture(&self) -> bool {
        self.code.starts_with("AES")
    }
}

impl std::ops::Deref for ErrorCode {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl Hash for ErrorCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_job_id_vo.rs

```rust
// PURPOSE: JobId — value object for pipeline job identifiers
//
// `JobId` is a thin wrapper around a `String` and is generated with the
// `string_value_object!` macro. It exists in its own file so that any
// crate needing job identifiers can `use` this type without pulling in the
// rest of the common VO namespace.
use crate::string_value_object;

string_value_object!(JobId);
```

---

## File: crates/shared/src/common/taxonomy_layer_vo.rs

```rust
// PURPOSE: FileContentVO, Identity, LayerNameVO, LineContentVO — VOs for layer identity and file content
use crate::string_value_object;

string_value_object!(FileContentVO);
string_value_object!(Identity);
string_value_object!(LayerNameVO);
string_value_object!(LineContentVO);
```

---

## File: crates/shared/src/common/taxonomy_lint_vo.rs

```rust
// PURPOSE: CommandArgs, Location, LocationList, ScopeBounds, ScopeRef, ViolationConstraint — VOs for lint violations
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_common_vo::ColumnNumber;
use crate::common::taxonomy_common_vo::LineNumber;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeRef {
    pub name: DescriptionVO,
    #[serde(default)]
    pub kind: DescriptionVO,
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub start_line: Option<LineNumber>,
    #[serde(default)]
    pub end_line: Option<LineNumber>,
}

impl ScopeRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: DescriptionVO::new(name),
            kind: DescriptionVO::new("function"),
            file: None,
            start_line: None,
            end_line: None,
        }
    }
    pub fn has_range(&self) -> bool {
        self.start_line.as_ref().is_some_and(|l| l.value > 0)
            && self.end_line.as_ref().is_some_and(|l| l.value > 0)
    }
}

impl std::fmt::Display for ScopeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref file) = self.file {
            write!(f, "{} {} in {}", self.kind.value, self.name.value, file)
        } else if !self.kind.value.is_empty() {
            write!(f, "{} {}", self.kind.value, self.name.value)
        } else {
            write!(f, "{}", self.name.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    #[serde(default)]
    pub file: Option<FilePath>,
    #[serde(default)]
    pub line: Option<LineNumber>,
    #[serde(default)]
    pub column: Option<ColumnNumber>,
    #[serde(default)]
    pub description: DescriptionVO,
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Location {
    pub fn new() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            description: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(ref file) = self.file {
            parts.push(file.value.clone());
        }
        if let Some(ref line) = self.line {
            let mut s = line.value.to_string();
            if let Some(ref col) = self.column {
                if col.value > 0 {
                    s = format!("{}:{}", line.value, col.value);
                }
            }
            parts.push(s);
        }
        let result = if parts.is_empty() {
            "unknown".to_string()
        } else {
            parts.join(":")
        };
        if self.description.value.is_empty() {
            write!(f, "{}", result)
        } else {
            write!(f, "{} — {}", result, self.description.value)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LocationList {
    #[serde(default)]
    pub values: Vec<Location>,
}

impl LocationList {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}

impl LocationList {
    pub fn push(&mut self, item: Location) {
        self.values.push(item);
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl std::ops::Deref for LocationList {
    type Target = Vec<Location>;
    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViolationConstraint {
    pub rule: DescriptionVO,
    #[serde(default)]
    pub min_value: DescriptionVO,
    #[serde(default)]
    pub max_value: DescriptionVO,
}

impl ViolationConstraint {
    pub fn new(rule: impl Into<String>) -> Self {
        Self {
            rule: DescriptionVO::new(rule),
            min_value: DescriptionVO::new(String::new()),
            max_value: DescriptionVO::new(String::new()),
        }
    }
}

impl std::fmt::Display for ViolationConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rule.value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandArgs {
    #[serde(default)]
    pub args: Vec<ContentString>,
}

impl Default for CommandArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandArgs {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }
}

impl std::fmt::Display for CommandArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.args
                .iter()
                .map(|a| a.value.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeBounds {
    #[serde(default)]
    pub start: Option<LineNumber>,
    #[serde(default)]
    pub end: Option<LineNumber>,
}
```

---

## File: crates/shared/src/common/taxonomy_message_vo.rs

```rust
// PURPOSE: ComplianceStatus, LintMessage — VOs for compliance status and violation messages
use crate::string_value_object;

string_value_object!(LintMessage);

/// Boolean compliance flag. Written manually because `bool` is not supported
/// by the `string_value_object!` macro (`i64 as bool` is not a valid Rust cast).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ComplianceStatus {
    pub value: bool,
}

impl ComplianceStatus {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    pub fn value(&self) -> bool {
        self.value
    }
}

impl std::fmt::Display for ComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<bool> for ComplianceStatus {
    fn from(v: bool) -> Self {
        Self { value: v }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_path_vo.rs

```rust
// PURPOSE: FilePath, DirectoryPath — value objects for validated file and directory paths
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// file_path_vo — File and directory path value objects.
///
/// File path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilePath {
    pub value: String,
}

impl FilePath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new FilePath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("File path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, collapse repeated slashes.
        let mut normalized = String::with_capacity(value.len());
        let mut prev_slash = false;
        for c in value.chars() {
            if c == '/' || c == '\\' {
                if !prev_slash {
                    normalized.push('/');
                    prev_slash = true;
                }
            } else {
                normalized.push(c);
                prev_slash = false;
            }
        }
        value = normalized;
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        // If after normalization it's empty, then it was all slashes -> treat as root
        if value.is_empty() {
            return Ok(FilePath {
                value: "/".to_string(),
            });
        }
        Ok(FilePath { value })
    }

    /// File extension without dot.
    pub fn extension(&self) -> String {
        let special_files = [
            "Makefile",
            "Dockerfile",
            "Dockerfile.dev",
            "Dockerfile.prod",
            ".bashrc",
            ".profile",
            ".zshrc",
            ".gitignore",
            ".dockerignore",
        ];
        // Operate on the basename, not the full path — `./foo.rs` must still yield
        // `rs` as its extension, and `.bashrc` (which is fully a basename) must NOT
        // be confused with a hidden file mid-path.
        let basename = match self.value.rsplit('/').next() {
            Some(b) => b,
            None => return String::new(),
        };
        if special_files.contains(&basename) || basename.starts_with('.') {
            return String::new();
        }
        match basename.rsplit_once('.') {
            Some((_, ext)) => ext.to_string(),
            None => String::new(),
        }
    }

    /// Check if path has given extension (without dot).
    pub fn has_extension(&self, ext: &str) -> bool {
        self.extension().eq_ignore_ascii_case(ext)
    }

    /// Extract filename/basename of the path.
    pub fn basename(&self) -> String {
        match self.value.rsplit('/').next() {
            Some(f) => f.to_string(),
            None => self.value.clone(),
        }
    }

    /// Check if the path is a barrel file (module re-export aggregator).
    pub fn is_barrel_file(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    /// Check if the path is a module/layer entry point file.
    pub fn is_entry_point(&self) -> bool {
        let f = self.basename();
        matches!(
            f.as_ref(),
            "__init__.py"
                | "main.py"
                | "py.typed"
                | "app.py"
                | "lib.rs"
                | "main.rs"
                | "index.ts"
                | "index.js"
                | "index.tsx"
                | "index.jsx"
                | "main.ts"
                | "main.js"
                | "app.ts"
                | "app.js"
        )
    }
}

impl std::ops::Deref for FilePath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Hash for FilePath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Directory path identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct DirectoryPath {
    pub value: String,
}

impl DirectoryPath {
    pub fn value(&self) -> &str {
        &self.value
    }
    /// Create a new DirectoryPath from a string.
    ///
    /// # Errors
    /// Returns an error if the path is invalid (empty or only whitespace).
    pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
        let mut value = value.into();
        if value.trim().is_empty() {
            return Err("Directory path cannot be empty".to_string());
        }
        // Normalize: replace backslashes with forward slashes, and remove trailing slash.
        value = value.replace('\\', "/");
        // Remove trailing slashes
        let trimmed = value.trim_end_matches('/');
        value = if trimmed.is_empty() {
            "/".to_string()
        } else {
            trimmed.to_string()
        };
        Ok(DirectoryPath { value })
    }
}

impl std::ops::Deref for DirectoryPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::fmt::Display for DirectoryPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<'de> serde::Deserialize<'de> for DirectoryPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DirectoryPath::new(s).map_err(serde::de::Error::custom)
    }
}

impl Hash for DirectoryPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
```

---

## File: crates/shared/src/common/taxonomy_response_data_vo.rs

```rust
// PURPOSE: ResponseData — value object for pipeline job response data
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseData {
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub returncode: i64,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Default for ResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl ResponseData {
    pub fn new() -> Self {
        Self {
            value: None,
            stdout: String::new(),
            stderr: String::new(),
            returncode: 0,
            metadata: HashMap::new(),
        }
    }
    pub fn value(&self) -> Option<&serde_json::Value> {
        self.value.as_ref()
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        self.value.as_ref().and_then(|v| v.get(key))
    }
}
```

---

## File: crates/shared/src/common/taxonomy_severity_vo.rs

```rust
// PURPOSE: Severity — value object for violation severity levels (critical, high, medium, low)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Severity {
    #[serde(rename = "info")]
    #[default]
    INFO,
    #[serde(rename = "low")]
    LOW,
    #[serde(rename = "medium")]
    MEDIUM,
    #[serde(rename = "high")]
    HIGH,
    #[serde(rename = "critical")]
    CRITICAL,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::INFO => write!(f, "info"),
            Severity::LOW => write!(f, "low"),
            Severity::MEDIUM => write!(f, "medium"),
            Severity::HIGH => write!(f, "high"),
            Severity::CRITICAL => write!(f, "critical"),
        }
    }
}

impl Severity {
    pub fn score_impact(&self) -> f64 {
        match self {
            Severity::INFO => 0.0,
            Severity::LOW => 1.0,
            Severity::MEDIUM => 2.0,
            Severity::HIGH => 3.0,
            Severity::CRITICAL => 5.0,
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_source_vo.rs

```rust
// PURPOSE: ContentString, SourceContentVO — VOs for source code content representation
use crate::string_value_object;
use serde::{Deserialize, Serialize};

use crate::common::taxonomy_path_vo::FilePath;

string_value_object!(ContentString);

/// Source content value object: combines a file path, a `ContentString`
/// payload, and a language marker. Carries three fields rather than one,
/// so it does not fit the single-field `string_value_object!` macro;
/// defined manually.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceContentVO {
    pub file_path: FilePath,
    pub content: ContentString,
    pub language: String,
}

impl SourceContentVO {
    pub fn new(file_path: FilePath, content: ContentString, language: impl Into<String>) -> Self {
        Self {
            file_path,
            content,
            language: language.into(),
        }
    }
}
```

---

## File: crates/shared/src/common/taxonomy_suggestion_vo.rs

```rust
// PURPOSE: ClassPath, DescriptionVO, LogOutput, MetadataVO, StdError, StdOutput, Suggestion — domain value objects for CLI suggestion/result data
use crate::string_value_object;
use serde::{Deserialize, Serialize};

// ClassPath, DescriptionVO, LogOutput, StdError, StdOutput, and Suggestion all
// follow the standard String-wrapper VO pattern; the macro emits the
// new/value/Display/From/Hash/PartialEq/Deserialize impls they need.
string_value_object!(ClassPath);
string_value_object!(DescriptionVO);
string_value_object!(LogOutput);
string_value_object!(StdError);
string_value_object!(StdOutput);
string_value_object!(Suggestion);

/// Strongly-typed replacement for the previous
/// `HashMap<String, serde_json::Value>` return type. Each field has a real
/// domain meaning — there is no `serde_json::Value` in the contract surface.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetadataVO {
    pub values: std::collections::HashMap<String, serde_json::Value>,
}

impl MetadataVO {
    pub fn new(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self { values: value }
    }
    pub fn value(&self) -> &std::collections::HashMap<String, serde_json::Value> {
        &self.values
    }
}
```

---

## File: crates/shared/src/config-system/contract_config_orchestrator_aggregate.rs

```rust
use crate::common::taxonomy_path_vo::FilePath;
use crate::config_system::taxonomy_config_language_vo::ConfigLanguage;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo;
use crate::config_system::taxonomy_source_vo::ConfigResult;
use async_trait::async_trait;

#[async_trait]
pub trait IConfigOrchestratorAggregate: Send + Sync {
    async fn load_project_config(&self, project_root: &FilePath) -> ConfigResult;

    async fn load_config_for_language(
        &self,
        project_root: &FilePath,
        language: ConfigLanguage,
    ) -> ConfigResult;

    async fn discover_workspaces(&self, root: &FilePath) -> Vec<WorkspaceInfo>;

    /// Synchronous config loading for container initialization.
    /// Searches workspace root for config YAML, falls back to embedded defaults.
    fn load_config_sync(&self, project_root: &str) -> ArchitectureConfig;

    /// Get ignored paths from config (hardcoded defaults + config values).
    fn ignored_paths(&self, project_root: &str) -> Vec<String>;
}
```

---

## File: crates/shared/src/config-system/mod.rs

```rust
// config-system — taxonomy and contract types
pub mod contract_config_orchestrator_aggregate;
pub mod contract_parser_protocol;
pub mod contract_reader_protocol;
pub mod contract_validator_protocol;
pub mod contract_workspace_detector_protocol;
pub mod taxonomy_config_error;
pub mod taxonomy_config_language_vo;
pub mod taxonomy_config_vo;
pub mod taxonomy_identifier_vo;
pub mod taxonomy_multi_project_summary_vo;
pub mod taxonomy_multi_project_vo;
pub mod taxonomy_multi_project_workspace_info_vo;
pub mod taxonomy_setting_vo;
pub mod taxonomy_source_vo;
pub mod taxonomy_validation_vo;
pub mod utility_config_defaults;
pub mod utility_config_io;
pub mod utility_config_merger;
pub mod utility_config_parser;
```

---

## File: crates/shared/src/external-lint/contract_external_lint_aggregate.rs

```rust
// PURPOSE: IExternalLintAggregate — contract for running external linter adapters
use crate::cli_commands::taxonomy_result_vo::LintResultList;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait IExternalLintAggregate: Send + Sync {
    async fn scan_all(&self, path: &FilePath) -> LintResultList;
    fn adapter_names(&self) -> Vec<String>;
}
```

---

## File: crates/shared/src/external-lint/mod.rs

```rust
// external-lint — taxonomy types for adapter utilities
pub mod contract_external_lint_aggregate;
pub mod contract_external_lint_language_detector_protocol;
pub mod contract_external_lint_selector_protocol;
pub mod contract_external_lint_utility_protocol;
pub mod taxonomy_external_lint_helper;
pub mod utility_external_lint_io;
```

---

## File: crates/shared/src/import-rules/contract_import_runner_aggregate.rs

```rust
// PURPOSE: IImportRunnerAggregate — contract for import-rules feature orchestrator
//
// This is the primary contract that decouples the import-rules agent layer
// from its callers (CLI, MCP, TUI). Callers depend on this trait, not on
// ImportOrchestrator directly.
//
// run_audit is async because it may perform file I/O and spawn blocking
// tasks internally. The caller provides a FilePath target (file or dir).
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

/// IImportRunnerAggregate — aggregate port for import-rules orchestration.
///
/// Implemented by ImportOrchestrator (agent layer).
/// Called by surface layer (CLI, MCP, TUI) via Arc<dyn IImportRunnerAggregate>.
#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    /// Run all 5 import-related AES checks (AES201–AES205) on the target.
    /// Returns aggregated violations from mandatory, forbidden, unused, dummy, and cycle checks.
    async fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError>;
    /// Human-readable name for this orchestrator ("import-rules").
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/import-rules/mod.rs

```rust
// import-rules — taxonomy and contract types
pub mod contract_cycle_import_protocol;
pub mod contract_dummy_import_protocol;
pub mod contract_import_forbidden_protocol;
pub mod contract_import_mandatory_protocol;
pub mod contract_import_runner_aggregate;
pub mod contract_unused_import_protocol;
pub mod taxonomy_dependency_edge_vo;
pub mod taxonomy_import_constant;
pub mod taxonomy_import_rule_vo;
pub mod taxonomy_language_vo;
pub mod taxonomy_violation_import_vo;
pub mod utility_cycle_detector;
pub mod utility_dummy_detector;
pub mod utility_file_read;
pub mod utility_import_module_parser;
pub mod utility_import_resolver;
pub mod utility_import_symbol_extractor;
pub mod utility_path_normalizer;

pub use taxonomy_dependency_edge_vo::DependencyEdge;
pub use taxonomy_language_vo::LanguageVO;
pub use taxonomy_violation_import_vo::AesImportViolation;
```

---

## File: crates/shared/src/naming-rules/contract_naming_runner_aggregate.rs

```rust
// PURPOSE: INamingRunnerAggregate — contract for naming-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_adapter_error::ScanError;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

#[async_trait]
pub trait INamingRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Result<Vec<LintResult>, ScanError>;
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/naming-rules/mod.rs

```rust
pub mod contract_naming_checker_protocol;
pub mod contract_naming_runner_aggregate;
pub mod taxonomy_naming_constant;
pub mod taxonomy_naming_rule_vo;
pub mod taxonomy_naming_violation_vo;
pub mod utility_naming;
pub mod utility_naming_checker;
pub mod utility_naming_filesystem;
pub use taxonomy_naming_violation_vo::NamingViolation;
```

---

## File: crates/shared/src/orphan-detector/contract_orphan_aggregate.rs

```rust
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::code_analysis::taxonomy_analysis_vo::GraphAnalysisContext;
use std::collections::HashSet;

pub trait IOrphanAggregate: Send + Sync {
    fn build_orphan_graph_context(&self, files: &[String], root_dir: &str) -> GraphAnalysisContext;
    fn identify_orphan_entry_points(&self, files: &[String]) -> HashSet<String>;
    fn check_orphans(&self, files: &[String], root_dir: &str) -> Vec<LintResult>;
}
```

---

## File: crates/shared/src/orphan-detector/mod.rs

```rust
pub mod contract_orphan_aggregate;
pub mod contract_orphan_graph_resolver_protocol;
pub mod contract_orphan_protocol;
pub mod taxonomy_orphan_contract_vo;
pub mod taxonomy_orphan_rule_vo;
pub mod taxonomy_violation_orphan_vo;
pub mod utility_file_cache;
pub mod utility_orphan;
pub mod utility_orphan_filename;
pub mod utility_orphan_io;
pub mod utility_orphan_path;
pub mod utility_workspace;
pub use taxonomy_orphan_contract_vo::{OrphanEntryPatternListVO, OrphanFileListVO};
pub use taxonomy_violation_orphan_vo::AesOrphanViolation;
```

---

## File: crates/shared/src/role-rules/contract_role_runner_aggregate.rs

```rust
// PURPOSE: IRoleRunnerAggregate — contract for role-rules feature orchestrator
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use async_trait::async_trait;

use crate::role_rules::taxonomy_layer_names_constant::LAYER_AGENT;
use crate::role_rules::taxonomy_layer_names_vo::LayerNames;
use crate::role_rules::taxonomy_role_rule_vo::RoleRuleVO;
use crate::role_rules::taxonomy_violation_role_vo::AesRoleViolation;

pub fn anchor_taxonomy() {
    let _ = LAYER_AGENT;
}
type _LayerNamesVORef = LayerNames;
type _RoleRuleVORef = RoleRuleVO;
type _AesRoleViolationRef = AesRoleViolation;

#[async_trait]
pub trait IRoleRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

---

## File: crates/shared/src/role-rules/mod.rs

```rust
// role-rules — taxonomy and contract types
pub mod contract_agent_role_protocol;
pub mod contract_capabilities_role_protocol;
pub mod contract_role_aggregate;
pub mod contract_role_protocol;
pub mod contract_role_runner_aggregate;
pub mod contract_surface_role_protocol;
pub mod contract_taxonomy_role_protocol;
pub mod taxonomy_layer_names_constant;
pub mod taxonomy_layer_names_vo;
pub mod taxonomy_role_rule_vo;
pub mod taxonomy_violation_role_vo;
pub use taxonomy_violation_role_vo::AesRoleViolation;
```

---

