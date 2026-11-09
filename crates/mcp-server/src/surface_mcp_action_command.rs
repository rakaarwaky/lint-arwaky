// PURPOSE: McpActionSurface — MCP server action: business logic + JSON building.
//
// MCP protocol surface (surface_mcp_tool_command) delegates here; this surface
// delegates to dispatcher surfaces (pure business logic) and maps results to
// JSON responses. No formatting/println — JSON is returned as serde_json::Value.
use std::sync::Arc;

use dispatcher::surface_orphan_action::OrphanFactory;
use shared_auto_fix::IFixAggregate;
use shared_common::Threshold;
use shared_common::taxonomy_path_vo::FilePath;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_config_system::{IConfigOrchestratorAggregate, IConfigReadProtocol};
use shared_doc_rules::IDocRunnerAggregate;
use shared_external_lint::IExternalLintAggregate;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_git_hooks::IGitHooksAggregate;
use shared_import_rules::IImportRunnerAggregate;
use shared_maintenance::IMaintenanceAggregate;
use shared_naming_rules::INamingRunnerAggregate;
use shared_orphan_rules::IOrphanAggregate;
use shared_project_setup::ISetupAggregate;
use shared_quality_rules::ICodeAnalysisAggregate;
use shared_role_rules::IRoleRunnerAggregate;
use shared_role_rules::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_SURFACES, LAYER_TAXONOMY, LAYER_UTILITY,
};

use crate::surface_mcp_action_view::{
    error_response, is_mutating_action, normalize_response, resolve_confined_path,
    violations_to_json,
};

#[derive(Clone)]
pub struct McpServerDependencies {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn IFixAggregate> + Send + Sync>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub maintenance_orchestrator: Arc<dyn IMaintenanceAggregate>,
    pub git_hooks_aggregate: Arc<dyn IGitHooksAggregate>,
    pub setup_orchestrator: Arc<dyn ISetupAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub config_file_reader: Arc<dyn shared_config_system::IConfigFileReaderAggregate>,
    pub config_reader: Arc<dyn IConfigReadProtocol>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub doc_orchestrator: Arc<dyn IDocRunnerAggregate>,
    pub structure_orchestrator: Arc<dyn shared_structure_rules::IStructureAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_parser: Arc<dyn shared_filesystem::IParserProtocol>,
    pub fs_seam: Arc<dispatcher::surface_check_action::FilesystemSeam>,
    pub fs_factory: Arc<dyn Fn() -> dispatcher::surface_check_action::FilesystemSeam + Send + Sync>,
    pub orphan_factory: Arc<OrphanFactory>,
    // DI: config parsing functions
    pub parse_config_yaml: fn(&str) -> ArchitectureConfig,
    pub parse_adapter_names: fn(&str) -> Vec<String>,
    pub parse_score_threshold: fn(&str) -> Option<f64>,
    pub server_version: String,
    /// Canonical confinement boundary for every client-supplied path.
    pub workspace_root: PathBuf,
    /// Mutating actions are denied unless the operator explicitly opts in.
    pub allow_mutations: bool,
}

pub struct McpActionSurface {
    pub deps: McpServerDependencies,
}

impl McpActionSurface {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }

    /// Run a CPU-bound closure on a blocking thread pool so it never blocks
    /// the async Tokio reactor. `JoinError` is mapped to a JSON error envelope
    /// with `exit_code: 2`, identical to other dispatch failures.
    pub async fn run_blocking<T, F>(f: F) -> Result<T, serde_json::Value>
    where
        T: Send + 'static,
        F: FnOnce() -> T + Send + 'static,
    {
        tokio::task::spawn_blocking(f).await.map_err(|e| {
            serde_json::json!({
                "error": format!("Blocking task failed: {e}"),
                "exit_code": 2
            })
        })
    }

    pub fn to_fp(&self, path: &str) -> Result<FilePath, serde_json::Value> {
        resolve_confined_path(&self.deps.workspace_root, path).and_then(|resolved| {
            FilePath::new(resolved.to_string_lossy().to_string())
                .map_err(|_| error_response("Invalid path"))
        })
    }

    fn authorize_action(&self, action: &str) -> Result<(), serde_json::Value> {
        if is_mutating_action(action) && !self.deps.allow_mutations {
            Err(serde_json::json!({
                "status": "error",
                "error": format!("Action '{action}' is disabled: MCP server is read-only"),
                "exit_code": 2
            }))
        } else {
            Ok(())
        }
    }

    /// Authorize a mutating action, but allow `dry_run` previews to pass
    /// through unblocked since no files are written. Delegates to
    /// `authorize_action` when `dry_run` is false; a no-op otherwise.
    pub fn authorize_action_allow_dry_run(
        &self,
        action: &str,
        dry_run: bool,
    ) -> Result<(), serde_json::Value> {
        if dry_run {
            return Ok(());
        }
        self.authorize_action(action)
    }

    /// Run check/scan — all linters combined via dispatcher.
    ///
    /// `member` and `rules` are forwarded to `ScanOptions`; dropping them made
    /// a member-scoped scan silently cover the whole workspace.
    pub async fn execute_check(
        &self,
        path: &str,
        member: Option<&str>,
        rules: Option<&str>,
    ) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        let opts = dispatcher::surface_check_action::ScanOptions {
            path: Some(fp),
            multi_project_orchestrator: Some(self.deps.config_orchestrator.clone()),
            filter: rules.map(str::to_string),
            member: member.map(str::to_string),
            // `ScanOptions.filesystem` is the seam handle; cloning the whole
            // `FilesystemSeam` here allocated a fresh copy per tool call.
            filesystem: self.deps.fs_seam.clone(),
            scan_aggregates: Some(self.scan_aggregates()),
        };
        let scan_result =
            match Self::run_blocking(move || dispatcher::surface_check_action::collect_scan(opts))
                .await
            {
                Ok(scan_result) => scan_result,
                Err(e) => return e,
            };
        match scan_result {
            Ok(violations) => {
                let total = violations.len();
                let exit_code = if total == 0 { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "ok" } else { "warning" },
                    "result": if exit_code == 0 { "clean" } else { "violations" },
                    "action": "check",
                    "path": path,
                    "exit_code": exit_code,
                    "total_violations": total,
                    "results": violations_to_json(&violations),
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run CI — scoring + threshold via dispatcher.
    ///
    /// `ci` scores the same ten rule groups as `scan`, so it gets the same
    /// aggregate bundle. A private six-field struct is how the gate silently
    /// dropped structure and doc coverage.
    pub async fn execute_ci(&self, path: &str, threshold: u64) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        let threshold = match u32::try_from(threshold)
            .ok()
            .and_then(|value| Threshold::try_new(value).ok())
        {
            Some(threshold) => threshold,
            None => {
                return error_response("Invalid 'threshold': expected an integer from 0 to 100");
            }
        };
        let aggregates = self.scan_aggregates();
        let path_owned = path.to_string();
        let ci_result = match Self::run_blocking(move || {
            dispatcher::surface_ci_action::collect_ci(aggregates, Some(fp), threshold)
        })
        .await
        {
            Ok(ci_result) => ci_result,
            Err(e) => return e,
        };
        match ci_result {
            Ok(report) => {
                let exit_code = if report.pass { 0 } else { 1 };
                serde_json::json!({
                    "status": if report.pass { "ok" } else { "warning" },
                    "result": if report.pass { "pass" } else { "fail" },
                    "action": "ci",
                    "threshold": report.threshold,
                    "path": path_owned,
                    "exit_code": exit_code,
                    "total_violations": report.total_violations,
                    "score": report.score,
                    "groups_run": report.groups_run,
                    "reasons": report.reasons,
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Per-member and per-code violation counts — the JSON half of `report`.
    ///
    /// The delta half stays CLI-only: it reads and writes the persisted
    /// snapshot store, which an MCP client should not mutate as a side effect
    /// of asking a question.
    pub async fn execute_report(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(error) => return error,
        };
        let opts = dispatcher::surface_check_action::ScanOptions {
            path: Some(fp),
            multi_project_orchestrator: Some(self.deps.config_orchestrator.clone()),
            filter: None,
            member: None,
            filesystem: self.deps.fs_seam.clone(),
            scan_aggregates: Some(self.scan_aggregates()),
        };
        let scan_result =
            match Self::run_blocking(move || dispatcher::surface_check_action::collect_scan(opts))
                .await
            {
                Ok(scan_result) => scan_result,
                Err(e) => return e,
            };
        match scan_result {
            Ok(violations) => {
                let mut members: std::collections::BTreeMap<String, usize> =
                    std::collections::BTreeMap::new();
                let mut codes: std::collections::BTreeMap<String, usize> =
                    std::collections::BTreeMap::new();
                for violation in &violations {
                    let member = violation
                        .file
                        .value
                        .split('/')
                        .take(2)
                        .collect::<Vec<_>>()
                        .join("/");
                    *members.entry(member).or_insert(0) += 1;
                    *codes.entry(violation.code.code().to_string()).or_insert(0) += 1;
                }
                serde_json::json!({
                    "status": "ok",
                    "result": "complete",
                    "action": "report",
                    "path": path,
                    "exit_code": 0,
                    "total_violations": violations.len(),
                    "members": members,
                    "codes": codes,
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run the git-diff scan — the verb the catalog advertised but never
    /// dispatched. `base` is validated by the dispatcher before it reaches
    /// `git` (see `is_valid_ref`), so a flag-shaped ref cannot be injected.
    pub async fn execute_git(&self, path: &str, base: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(error) => return error,
        };
        let linter = self.deps.code_analysis_linter.clone();
        let base = shared_common::GitBranchName::new(base.to_string());
        let path_owned = fp.value.clone();
        let git_result = match Self::run_blocking(move || {
            dispatcher::surface_git_action::collect_git_diff(
                linter,
                base,
                Some(fp.value.as_str()),
                None,
            )
        })
        .await
        {
            Ok(git_result) => git_result,
            Err(e) => return e,
        };
        match git_result {
            Ok(report) => {
                let exit_code = if report.total_violations == 0 { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "ok" } else { "warning" },
                    "result": if exit_code == 0 { "clean" } else { "violations" },
                    "action": "git",
                    "path": path_owned,
                    "base": report.base,
                    "files_changed": report.files.len(),
                    "exit_code": exit_code,
                    "total_violations": report.total_violations,
                })
            }
            // A bad base ref is a runtime error, never a clean scan: the old
            // CLI path read an empty change list as "0 violations".
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// The aggregate bundle every scan-family verb runs. One builder, so a
    /// verb can never quietly omit a rule group.
    fn scan_aggregates(&self) -> dispatcher::surface_check_action::ScanAggregates {
        dispatcher::surface_check_action::ScanAggregates {
            quality: self.deps.code_analysis_linter.clone(),
            role: self.deps.role_orchestrator.clone(),
            import: self.deps.import_orchestrator.clone(),
            naming: self.deps.naming_orchestrator.clone(),
            external: self.deps.external_lint.clone(),
            orphan: self.deps.orphan_orchestrator.clone(),
            config: self.deps.config_orchestrator.clone(),
            structure: self.deps.structure_orchestrator.clone(),
            doc: self.deps.doc_orchestrator.clone(),
            config_file_reader: self.deps.config_file_reader.clone(),
            // Hold the Arc: cloning the seam copied three `Arc<dyn …>` handles
            // into a fresh allocation on every single tool call.
            fs_seam: self.deps.fs_seam.clone(),
        }
    }

    /// Run every rule group but report only one layer's violations. `action` is the
    /// subcommand the caller asked for, echoed back verbatim — it is the layer's
    /// public name (`surface`), which the internal `LAYER_*` spelling (`surfaces`)
    /// does not match.
    pub async fn execute_layer_scan(
        &self,
        path: &str,
        layer: &str,
        action: &str,
    ) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(fp) => fp,
            Err(error) => return error,
        };
        let opts = dispatcher::surface_check_action::ScanOptions {
            path: Some(fp),
            multi_project_orchestrator: Some(self.deps.config_orchestrator.clone()),
            filter: None,
            member: None,
            // `ScanOptions.filesystem` is the seam handle; cloning the whole
            // `FilesystemSeam` here allocated a fresh copy per tool call.
            filesystem: self.deps.fs_seam.clone(),
            scan_aggregates: Some(self.scan_aggregates()),
        };
        let layer_str = layer.to_string();
        let layer_result = match Self::run_blocking(move || {
            dispatcher::surface_layer_scan_action::collect_layer_scan(opts, &layer_str)
        })
        .await
        {
            Ok(layer_result) => layer_result,
            Err(e) => return e,
        };
        match layer_result {
            Ok(violations) => {
                let total = violations.len();
                let exit_code = if total == 0 { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "ok" } else { "warning" },
                    "result": if exit_code == 0 { "clean" } else { "violations" },
                    "action": action,
                    "layer": layer,
                    "path": path,
                    "exit_code": exit_code,
                    "total_violations": total,
                    "results": violations_to_json(&violations),
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run fix — auto-fix with dry_run support via dispatcher.
    ///
    /// The exit code comes from `FixReport::verdict`, the same decision the CLI
    /// makes. This arm used to answer 0 for a partial fix while the CLI
    /// answered 1, so an agent gating on `exit_code` saw a clean tree.
    pub async fn execute_fix(&self, path: &str, dry_run: bool) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(error) => return error,
        };
        let linter = self.deps.code_analysis_linter.clone();
        let filesystem = self.deps.filesystem.clone();
        let factory = self.deps.fix_orchestrator_factory.clone();
        let path_owned = path.to_string();
        // The operator's `ignored_paths` are honoured here for the same reason
        // the CLI honours them: a fix run that rewrites an excluded tree is
        // worse than one that finds nothing to fix (#1360).
        let ignored = self
            .deps
            .config_orchestrator
            .execute(shared_config_system::ConfigRequest::ignored_paths(&fp))
            .into_patterns()
            .values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let fix_result = match Self::run_blocking(move || {
            dispatcher::surface_fix_action::collect_fix(
                Some(fp),
                dry_run,
                linter,
                filesystem,
                factory,
                &ignored,
            )
        })
        .await
        {
            Ok(fix_result) => fix_result,
            Err(e) => return e,
        };
        match fix_result {
            Ok(report) => {
                let verdict = report.verdict();
                let exit_code = verdict.value();
                let status = if report.has_failed {
                    "error"
                } else if report.success {
                    "success"
                } else {
                    "partial"
                };
                serde_json::json!({
                    "status": if verdict == shared_common::ExitCode::OK {
                        "ok"
                    } else if verdict == shared_common::ExitCode::POLICY_FAIL {
                        "warning"
                    } else {
                        "error"
                    },
                    "result": status,
                    "action": "fix",
                    "path": path_owned,
                    "dry_run": report.dry_run,
                    "exit_code": exit_code,
                    "message": report.output,
                    "before_count": report.before_count,
                    "after_count": report.after_count,
                    "fixed_count": report.fixed_count,
                    "remaining_fixable": report.remaining_fixable,
                    "resolved": report.resolved.iter().map(|r| r.code.code()).collect::<Vec<_>>(),
                    "introduced": report.introduced.iter().map(|r| serde_json::json!({
                        "code": r.code.code(),
                        "file": r.file.value,
                        "line": r.line.value(),
                        "message": r.message.value,
                    })).collect::<Vec<_>>(),
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run quality scan via dispatcher.
}
