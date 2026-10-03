// PURPOSE: McpActionSurface — MCP server action: business logic + JSON building.
//
// MCP protocol surface (surface_mcp_tool_command) delegates here; this surface
// delegates to dispatcher surfaces (pure business logic) and maps results to
// JSON responses. No formatting/println — JSON is returned as serde_json::Value.
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use dispatcher::surface_orphan_action::OrphanFactory;
use shared_auto_fix::IFixAggregate;
use shared_common::Threshold;
use shared_common::taxonomy_path_vo::FilePath;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_config_system::{
    IConfigMergeProtocol, IConfigOrchestratorAggregate, IConfigReadProtocol,
};
use shared_doc_rules::IDocRunnerAggregate;
use shared_external_lint::IExternalLintAggregate;
use shared_filesystem::FilesystemRequest;
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

use shared_common::taxonomy_violation_item_vo::ViolationItem;

#[derive(Clone)]
pub struct McpServerDependencies {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator_factory: Arc<dyn Fn(bool) -> Arc<dyn IFixAggregate> + Send + Sync>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub maintenance_orchestrator: Arc<dyn IMaintenanceAggregate>,
    pub git_hooks_aggregate: Arc<dyn IGitHooksAggregate>,
    pub setup_orchestrator: Arc<dyn ISetupAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub config_parser: Arc<dyn IConfigMergeProtocol>,
    pub config_reader: Arc<dyn IConfigReadProtocol>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub doc_orchestrator: Arc<dyn IDocRunnerAggregate>,
    pub structure_orchestrator: Arc<dyn shared_structure_rules::IStructureAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    pub filesystem_workspace: Arc<dyn shared_filesystem::IWorkspaceProtocol>,
    pub filesystem_tool_resolution: Arc<dyn shared_filesystem::IToolResolutionProtocol>,
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
    async fn run_blocking<T, F>(f: F) -> Result<T, serde_json::Value>
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

    fn to_fp(&self, path: &str) -> Result<FilePath, serde_json::Value> {
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

    /// Run check/scan — all linters combined via dispatcher.
    pub async fn execute_check(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        let opts = dispatcher::surface_check_action::ScanOptions {
            path: Some(fp),
            multi_project_orchestrator: Some(self.deps.config_orchestrator.clone()),
            filter: None,
            member: None,
            filesystem: Arc::new(self.deps.fs_seam.as_ref().clone()),
            scan_aggregates: Some(dispatcher::surface_check_action::ScanAggregates {
                quality: self.deps.code_analysis_linter.clone(),
                role: self.deps.role_orchestrator.clone(),
                import: self.deps.import_orchestrator.clone(),
                naming: self.deps.naming_orchestrator.clone(),
                external: self.deps.external_lint.clone(),
                orphan: self.deps.orphan_orchestrator.clone(),
                config: self.deps.config_orchestrator.clone(),
                structure: self.deps.structure_orchestrator.clone(),
                doc: self.deps.doc_orchestrator.clone(),
                fs_seam: self.deps.fs_seam.clone(),
            }),
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
    pub async fn execute_ci(&self, path: &str, threshold: u64) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_ci_action::collect_ci(
            dispatcher::surface_ci_action::CiScanDeps {
                code_analysis_linter: self.deps.code_analysis_linter.clone(),
                import_orchestrator: self.deps.import_orchestrator.clone(),
                naming_orchestrator: self.deps.naming_orchestrator.clone(),
                config_orchestrator: self.deps.config_orchestrator.clone(),
                orphan_orchestrator: self.deps.orphan_orchestrator.clone(),
                filesystem: self.deps.filesystem.clone(),
            },
            Some(fp),
            match u32::try_from(threshold)
                .ok()
                .and_then(|value| Threshold::try_new(value).ok())
            {
                Some(threshold) => threshold,
                None => {
                    return error_response(
                        "Invalid 'threshold': expected an integer from 0 to 100",
                    );
                }
            },
        ) {
            Ok(report) => {
                let exit_code = if report.pass { 0 } else { 1 };
                serde_json::json!({
                    "status": if report.pass { "ok" } else { "warning" },
                    "result": if report.pass { "pass" } else { "fail" },
                    "action": "ci",
                    "threshold": report.threshold,
                    "path": path,
                    "exit_code": exit_code,
                    "total_violations": report.total_violations,
                    "score": report.score,
                    "reasons": report.reasons,
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
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
            filesystem: Arc::new(self.deps.fs_seam.as_ref().clone()),
            scan_aggregates: Some(dispatcher::surface_check_action::ScanAggregates {
                quality: self.deps.code_analysis_linter.clone(),
                role: self.deps.role_orchestrator.clone(),
                import: self.deps.import_orchestrator.clone(),
                naming: self.deps.naming_orchestrator.clone(),
                external: self.deps.external_lint.clone(),
                orphan: self.deps.orphan_orchestrator.clone(),
                config: self.deps.config_orchestrator.clone(),
                structure: self.deps.structure_orchestrator.clone(),
                doc: self.deps.doc_orchestrator.clone(),
                fs_seam: self.deps.fs_seam.clone(),
            }),
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
    pub async fn execute_fix(&self, path: &str, dry_run: bool) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(error) => return error,
        };
        let linter = self.deps.code_analysis_linter.clone();
        let factory = self.deps.fix_orchestrator_factory.clone();
        let fix_result = match Self::run_blocking(move || {
            dispatcher::surface_fix_action::collect_fix(Some(fp), dry_run, linter, factory)
        })
        .await
        {
            Ok(fix_result) => fix_result,
            Err(e) => return e,
        };
        match fix_result {
            Ok(report) => {
                let exit_code = if report.has_failed {
                    2 // runtime error — any Failed(reason) outranks policy fail
                } else if report.success {
                    0
                } else if report.fixed_count > 0 {
                    1 // partial fix — violations remain
                } else {
                    0
                };
                let status = if report.has_failed {
                    "error"
                } else if report.success {
                    "success"
                } else {
                    "partial"
                };
                serde_json::json!({
                    "status": if status == "success" { "ok" } else if status == "partial" { "warning" } else { "error" },
                    "result": status,
                    "action": "fix",
                    "path": path,
                    "dry_run": report.dry_run,
                    "exit_code": exit_code,
                    "message": report.output,
                    "before_count": report.before_count,
                    "after_count": report.after_count,
                    "fixed_count": report.fixed_count,
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run quality scan via dispatcher.
    pub async fn execute_quality(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_quality_action::collect_quality(
            Some(fp),
            self.deps.code_analysis_linter.clone(),
            None,
            self.deps.filesystem.clone(),
            &[],
        ) {
            Ok(violations) => violations_response("quality", path, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run import scan via dispatcher.
    pub async fn execute_import(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_import_action::collect_import(
            Some(fp),
            self.deps.import_orchestrator.clone(),
            None,
            self.deps.filesystem.clone(),
            &[],
        ) {
            Ok(violations) => violations_response("import", path, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run naming scan via dispatcher.
    pub async fn execute_naming(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_naming_action::collect_naming(
            Some(fp),
            self.deps.naming_orchestrator.clone(),
            None,
            self.deps.filesystem.clone(),
            &[],
        ) {
            Ok(violations) => violations_response("naming", path, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run role scan via dispatcher (direct aggregate — no subprocess).
    pub async fn execute_role(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(fp) => fp,
            Err(error) => return error,
        };
        let role = self.deps.role_orchestrator.clone();
        let fs = self.deps.filesystem.clone();
        let fp_value = fp.value().to_string();
        let fp_value_outer = fp_value.clone();
        let role_result = match Self::run_blocking(move || {
            dispatcher::surface_role_action::collect_role_direct(role, None, fs, &fp_value, &[])
        })
        .await
        {
            Ok(role_result) => role_result,
            Err(e) => return e,
        };
        match role_result {
            Ok(violations) => violations_response("role", &fp_value_outer, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run orphan scan via dispatcher.
    pub async fn execute_orphan(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_orphan_action::collect_orphan(
            Some(fp),
            None,
            dispatcher::surface_orphan_action::OrphanScanDeps::new(
                self.deps.orphan_orchestrator.clone(),
                self.deps.config_orchestrator.clone(),
                self.deps.filesystem.clone(),
                self.deps.filesystem_workspace.clone(),
                self.deps.fs_factory.clone(),
                self.deps.orphan_factory.clone(),
            ),
            None,
        ) {
            Ok(violations) => {
                let exit_code = if violations.is_empty() { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "ok" } else { "warning" },
                    "result": if exit_code == 0 { "clean" } else { "violations" },
                    "action": "orphan",
                    "exit_code": exit_code,
                    "orphan_count": violations.len(),
                    "results": violations_to_json(&violations),
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run external lint via dispatcher (direct aggregate — no subprocess).
    pub async fn execute_external(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_external_action::collect_external_direct(
            Some(fp),
            self.deps.external_lint.clone(),
            self.deps.filesystem.clone(),
            self.deps.config_parser.clone(),
            None,
            &[],
        ) {
            Ok(violations) => violations_response("external", path, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run doctor diagnostics via dispatcher.
    pub async fn execute_doctor(&self) -> serde_json::Value {
        let maint = self.deps.maintenance_orchestrator.clone();
        let diag = match Self::run_blocking(move || {
            dispatcher::surface_maintenance_action::collect_doctor(maint)
        })
        .await
        {
            Ok(diag) => diag,
            Err(e) => return e,
        };
        let mut checks = Vec::new();
        for status in &diag.rust_tools {
            checks.push(serde_json::json!({"tool": status.name, "status": if status.status == "OK" { "ok" } else { "not_found" }, "version": status.version}));
        }
        for status in &diag.python_tools {
            checks.push(serde_json::json!({"tool": status.name, "status": if status.status == "OK" { "ok" } else { "not_found" }, "version": status.version}));
        }
        for status in &diag.js_tools {
            checks.push(serde_json::json!({"tool": status.name, "status": if status.status == "OK" { "ok" } else { "not_found" }, "version": status.version}));
        }
        for status in &diag.vcs_tools {
            checks.push(serde_json::json!({"tool": status.name, "status": if status.status == "OK" { "ok" } else { "not_found" }, "version": status.version}));
        }
        serde_json::json!({"status": "ok", "action": "doctor", "exit_code": 0, "checks": checks})
    }

    /// Run security scan via dispatcher.
    pub async fn execute_security(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(error) => return error,
        };
        let maint = self.deps.maintenance_orchestrator.clone();
        let security_result = match Self::run_blocking(move || {
            dispatcher::surface_maintenance_action::collect_security(maint, Some(fp))
        })
        .await
        {
            Ok(security_result) => security_result,
            Err(e) => return e,
        };
        match security_result {
            Ok(report) => {
                let exit_code = if !report.tool_installed {
                    3
                } else if report.findings.is_empty() {
                    0
                } else {
                    1
                };
                serde_json::json!({
                    "status": if exit_code == 0 { "ok" } else { "warning" },
                    "result": if exit_code == 0 { "clean" } else if exit_code == 3 { "tool_missing" } else { "findings" },
                    "action": "security",
                    "exit_code": exit_code,
                    "language": report.language,
                    "tool_name": report.tool_name,
                    "tool_installed": report.tool_installed,
                    "warning": if report.tool_installed {
                        serde_json::Value::Null
                    } else {
                        serde_json::Value::String(format!(
                            "No dependency vulnerability scanner available for {} — install {} to enable this check; no clean result was produced",
                            report.language, report.tool_name
                        ))
                    },
                    "findings_count": report.findings.len(),
                    "findings": report.findings.iter().map(|f| serde_json::json!({
                        "severity": f.severity.to_uppercase(),
                        "test_id": f.test_id,
                        "file": f.file,
                        "line": f.line,
                        "issue": f.issue,
                    })).collect::<Vec<serde_json::Value>>(),
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run dependency report via dispatcher.
    pub async fn execute_dependencies(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(error) => return error,
        };
        let maint = self.deps.maintenance_orchestrator.clone();
        let dep_result = match Self::run_blocking(move || {
            dispatcher::surface_maintenance_action::collect_dependencies(maint, Some(fp))
        })
        .await
        {
            Ok(dep_result) => dep_result,
            Err(e) => return e,
        };
        match dep_result {
            Ok(report) => serde_json::json!({
                "status": "ok",
                "result": "complete",
                "action": "dependencies",
                "exit_code": 0,
                "language": report.language,
                "dependency_count": report.dependencies.len(),
                "dependencies": report.dependencies.iter().map(|d| serde_json::json!({
                    "name": d.name, "version": d.version, "dep_type": d.dep_type,
                })).collect::<Vec<serde_json::Value>>(),
            }),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Version info.
    pub fn execute_version(&self) -> serde_json::Value {
        serde_json::json!({"status": "ok", "version": self.deps.server_version, "name": "lint-arwaky", "exit_code": 0})
    }

    /// Watch is not supported via MCP.
    pub fn execute_watch(&self) -> serde_json::Value {
        serde_json::json!({"error": "watch is not supported via MCP", "exit_code": 2})
    }

    /// Run docs audit via dispatcher.
    pub async fn execute_docs(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(fp) => fp,
            Err(error) => return error,
        };
        let doc = self.deps.doc_orchestrator.clone();
        let result = match Self::run_blocking(move || {
            dispatcher::surface_docs_action::collect_docs(fp.value(), doc)
        })
        .await
        {
            Ok(result) => result,
            Err(e) => return e,
        };
        match result {
            Ok(findings) => {
                let exit_code = if findings.is_empty() { 0 } else { 1 };
                let results: Vec<String> = findings.iter().map(|f| f.summary()).collect();
                serde_json::json!({
                    "status": if exit_code == 0 { "ok" } else { "warning" },
                    "result": if exit_code == 0 { "clean" } else { "violations" },
                    "action": "docs",
                    "exit_code": exit_code,
                    "finding_count": findings.len(),
                    "results": results,
                })
            }
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Dispatch execute_command actions.
    pub async fn execute_command(
        &self,
        action: &str,
        path: &str,
        threshold: u64,
        dry_run: bool,
    ) -> serde_json::Value {
        if let Err(error) = self.authorize_action(action) {
            return error;
        }
        let response = match action {
            "check" | "scan" => self.execute_check(path).await,
            "ci" => self.execute_ci(path, threshold).await,
            "fix" => self.execute_fix(path, dry_run).await,
            "doctor" => self.execute_doctor().await,
            "orphan" => self.execute_orphan(path).await,
            "security" => self.execute_security(path).await,
            "quality" => self.execute_quality(path).await,
            "import" => self.execute_import(path).await,
            "naming" => self.execute_naming(path).await,
            "role" => self.execute_role(path).await,
            "docs" => self.execute_docs(path).await,
            "external" => self.execute_external(path).await,
            "taxonomy" => self.execute_layer_scan(path, LAYER_TAXONOMY, action).await,
            "contract" => self.execute_layer_scan(path, LAYER_CONTRACT, action).await,
            "capabilities" => {
                self.execute_layer_scan(path, LAYER_CAPABILITIES, action)
                    .await
            }
            "utility" => self.execute_layer_scan(path, LAYER_UTILITY, action).await,
            "agents" => self.execute_layer_scan(path, LAYER_AGENT, action).await,
            "surface" => self.execute_layer_scan(path, LAYER_SURFACES, action).await,
            "dependencies" => self.execute_dependencies(path).await,
            "version" => self.execute_version(),
            "watch" => self.execute_watch(),
            "adapters" => self.handle_health_check().await,
            "install-hook" => {
                let fp = match self.to_fp(path) {
                    Ok(f) => f,
                    Err(e) => return e,
                };
                let git_hooks = self.deps.git_hooks_aggregate.clone();
                let hook_result = match Self::run_blocking(move || {
                    dispatcher::surface_git_action::collect_install_hook(git_hooks, &fp)
                })
                .await
                {
                    Ok(hook_result) => hook_result,
                    Err(e) => return e,
                };
                match hook_result {
                    Ok(report) => {
                        serde_json::json!({"status": if report.success { "ok" } else { "error" }, "result": if report.success { "installed" } else { "failed" }, "action": "install-hook", "exit_code": if report.success { 0 } else { 2 }, "message": report.message})
                    }
                    Err(e) => serde_json::json!({"error": e.to_string(), "exit_code": 2}),
                }
            }
            "uninstall-hook" => {
                let git_hooks = self.deps.git_hooks_aggregate.clone();
                let hook_result = match Self::run_blocking(move || {
                    dispatcher::surface_git_action::collect_uninstall_hook(git_hooks)
                })
                .await
                {
                    Ok(hook_result) => hook_result,
                    Err(e) => return e,
                };
                match hook_result {
                    Ok(report) => {
                        serde_json::json!({"status": if report.success { "ok" } else { "error" }, "result": if report.success { "uninstalled" } else { "failed" }, "action": "uninstall-hook", "exit_code": if report.success { 0 } else { 2 }, "message": report.message})
                    }
                    Err(e) => serde_json::json!({"error": e.to_string(), "exit_code": 2}),
                }
            }
            "init" | "install" => {
                let items = dispatcher::surface_setup_action::collect_init(
                    self.deps.setup_orchestrator.clone(),
                    self.deps.filesystem.clone(),
                );
                let any_failure = items.iter().any(|i| !i.ok);
                let exit_code = if any_failure { 2 } else { 0 };
                let messages: Vec<String> = items.iter().map(|i| i.message.clone()).collect();
                serde_json::json!({"status": if any_failure { "warning" } else { "ok" }, "result": if any_failure { "partial" } else { "complete" }, "action": action, "exit_code": exit_code, "items": messages})
            }
            "mcp-config" => {
                serde_json::json!({"error": "mcp-config requires transport configuration — use CLI for full setup", "exit_code": 2})
            }
            "config-show" => {
                let result = self.handle_get_config(path, None);
                serde_json::from_str(&result).unwrap_or_else(
                    |_| serde_json::json!({"error": "Failed to serialize config", "exit_code": 2}),
                )
            }
            _ => {
                serde_json::json!({"error": format!("Unknown action: {}", action), "exit_code": 2})
            }
        };
        normalize_response(response)
    }

    // ─── Non-dispatcher MCP business logic ────────────────────

    /// Health check: adapter availability from maintenance aggregate.
    pub async fn handle_health_check(&self) -> serde_json::Value {
        let maint = self.deps.maintenance_orchestrator.clone();
        let (health, version_report) = match Self::run_blocking(move || {
            let health = dispatcher::surface_maintenance_action::collect_health_check(maint);
            let version_report = dispatcher::surface_version_action::collect_version();
            (health, version_report)
        })
        .await
        {
            Ok(v) => v,
            Err(e) => return e,
        };
        let adapters: Vec<serde_json::Value> = health
            .adapters
            .iter()
            .map(|a| {
                serde_json::json!({"name": a.name, "language": a.language, "status": if a.available { "available" } else { "not_installed" }})
            })
            .collect();
        let available = adapters
            .iter()
            .filter(|a| a["status"] == "available")
            .count();
        serde_json::json!({
            "status": "ok",
            "version": version_report.version,
            "adapters_available": available,
            "adapters_total": adapters.len(),
            "adapters": adapters,
            "exit_code": 0,
        })
    }

    /// List CLI commands filtered by domain.
    pub fn handle_list_commands(&self, domain: Option<String>) -> serde_json::Value {
        let catalog = shared_cli_commands::taxonomy_cli_commands_vo::COMMAND_CATALOG;
        let commands: Vec<serde_json::Value> = catalog
            .iter()
            .filter(|(name, _desc, _ex)| match domain.as_deref() {
                Some(d) if !d.is_empty() => name.contains(d),
                _ => true,
            })
            .map(|(name, desc, example)| {
                serde_json::json!({"name": name, "description": desc, "example": example})
            })
            .collect();
        serde_json::json!({ "status": "ok", "commands": commands, "total": commands.len(), "exit_code": 0 })
    }

    /// Read skill documentation by section.
    pub fn handle_read_skill(&self, section: Option<String>) -> serde_json::Value {
        let skills = ["lint-arwaky"];
        let mut candidates: Vec<String> = skills
            .iter()
            .flat_map(|s| vec![format!(".agents/skills/{}/SKILL.md", s)])
            .collect();
        if let Some(config_dir) = dirs::config_dir() {
            let xdg = config_dir
                .join("lint-arwaky")
                .join(".agents")
                .join("skills");
            for s in &skills {
                candidates.push(xdg.join(s).join("SKILL.md").to_string_lossy().to_string());
            }
        }
        let content = candidates
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.exists())
            .and_then(|p| {
                self.deps
                    .filesystem
                    .execute(FilesystemRequest::read_file(p))
                    .into_content_opt()
            });
        let content = match content {
            Some(c) => c,
            None => {
                return serde_json::json!({"error": "Skill documentation not found", "searched": candidates, "exit_code": 2});
            }
        };
        match section.as_deref() {
            Some(s) if !s.is_empty() => {
                let header = format!("## {}", s);
                if let Some(start) = content.find(&header) {
                    let remaining = &content[start..];
                    let end = match remaining[1..].find("\n## ") {
                        Some(i) => i + 1,
                        None => remaining.len(),
                    };
                    serde_json::json!({"section": s, "content": &remaining[..end], "exit_code": 0})
                } else {
                    serde_json::json!({"error": format!("Section '{}' not found", s), "exit_code": 2})
                }
            }
            _ => serde_json::json!({"content": content, "exit_code": 0}),
        }
    }

    /// Effective architecture configuration for a target path/language.
    pub fn handle_get_config(&self, path: &str, language: Option<String>) -> String {
        let fp = match self.to_fp(path) {
            Ok(fp) => fp,
            Err(error) => return error.to_string(),
        };

        let config_files = match self.deps.config_reader.list_config_files(&fp) {
            Ok(files) => files,
            Err(e) => {
                return serde_json::json!({"path": path, "language": language, "error": format!("Failed to list config files: {}", e), "exit_code": 2}).to_string()
            }
        };

        let mut layers = Vec::new();
        let mut rules_enabled = Vec::new();
        let mut ignored_paths = Vec::new();
        let mut warnings = Vec::new();
        let mut score_threshold: Option<f64> = None;
        let mut adapter_toggles: Vec<serde_json::Value> = Vec::new();

        for (lang, _config_path) in &config_files {
            layers.push(lang.as_str());
            if let Ok(Some(source)) = self.deps.config_reader.read_config(&fp, *lang) {
                let arch_config = (self.deps.parse_config_yaml)(&source.raw_content);
                rules_enabled.push(lang.as_str());
                ignored_paths.extend(
                    arch_config
                        .ignored_paths
                        .values
                        .iter()
                        .map(|p| p.value.clone()),
                );
                let adapter_names = (self.deps.parse_adapter_names)(&source.raw_content);
                for name in adapter_names {
                    adapter_toggles.push(serde_json::json!({"name": name, "status": "enabled"}));
                }
                if score_threshold.is_none()
                    && let Some(t) = (self.deps.parse_score_threshold)(&source.raw_content)
                {
                    score_threshold = Some(t);
                }
            } else {
                warnings.push(format!("No config data for {}", lang.as_str()));
            }
        }

        if config_files.is_empty() {
            warnings
                .push("No config files found. Run `lint-arwaky init` to create one.".to_string());
        }

        let result = serde_json::json!({
            "path": path,
            "language": language,
            "layers": layers,
            "rules_enabled": rules_enabled,
            "score_threshold": score_threshold.unwrap_or(80.0),
            "adapter_toggles": adapter_toggles,
            "ignored_paths": ignored_paths,
            "config_files": config_files.iter().map(|(_, p)| p.value.as_str()).collect::<Vec<&str>>(),
            "warnings": warnings,
            "exit_code": 0,
        });
        serde_json::to_string_pretty(&result).unwrap_or_else(|e| {
            serde_json::json!({"error": format!("Serialization failed: {e}"), "exit_code": 2})
                .to_string()
        })
    }
}

/// Serialize violations to JSON (mirrors old execute_* shape + severity).
fn violations_to_json(violations: &[ViolationItem]) -> Vec<serde_json::Value> {
    violations
        .iter()
        .map(|v| {
            serde_json::json!({
                "file": v.file.value,
                "code": v.code.code(),
                "message": v.message.value,
                "line": v.line.value(),
                "column": v.column.value(),
                "severity": format!("{}", v.severity),
            })
        })
        .collect()
}

/// Standard scan response envelope.
fn violations_response(
    action: &str,
    path: &str,
    violations: &[ViolationItem],
) -> serde_json::Value {
    let exit_code = if violations.is_empty() { 0 } else { 1 };
    serde_json::json!({
        "status": if exit_code == 0 { "ok" } else { "warning" },
                    "result": if exit_code == 0 { "clean" } else { "violations" },
        "action": action,
        "path": path,
        "exit_code": exit_code,
        "violation_count": violations.len(),
        "results": violations_to_json(violations),
    })
}

/// Resolve a client path beneath the configured root. Lexical parent segments
/// are rejected before canonicalization, and canonicalization then protects
/// against symlink escapes.
pub fn resolve_confined_path(root: &Path, requested: &str) -> Result<PathBuf, serde_json::Value> {
    if requested.trim().is_empty() {
        return Err(error_response("Invalid path"));
    }
    let canonical_root = root
        .canonicalize()
        .map_err(|_| error_response("Workspace root does not exist"))?;
    let requested_path = Path::new(requested);
    if requested_path
        .components()
        .any(|part| matches!(part, Component::ParentDir))
    {
        return Err(error_response("Path escapes workspace root"));
    }
    let candidate = if requested_path.is_absolute() {
        requested_path.to_path_buf()
    } else {
        canonical_root.join(requested_path)
    };
    let resolved = candidate
        .canonicalize()
        .map_err(|_| error_response("Path does not exist"))?;
    if !resolved.starts_with(&canonical_root) {
        return Err(error_response("Path escapes workspace root"));
    }
    Ok(resolved)
}

pub fn is_mutating_action(action: &str) -> bool {
    matches!(
        action,
        "fix" | "install-hook" | "uninstall-hook" | "init" | "install"
    )
}

fn normalize_response(mut response: serde_json::Value) -> serde_json::Value {
    let exit_code = response
        .get("exit_code")
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(2);
    if let Some(object) = response.as_object_mut() {
        let normalized = if exit_code == 0 {
            "ok"
        } else if exit_code == 1 || exit_code == 3 {
            "warning"
        } else {
            "error"
        };
        object.insert("status".to_string(), serde_json::json!(normalized));
    }
    response
}

fn error_response(message: &str) -> serde_json::Value {
    serde_json::json!({"status": "error", "error": message, "exit_code": 2})
}
