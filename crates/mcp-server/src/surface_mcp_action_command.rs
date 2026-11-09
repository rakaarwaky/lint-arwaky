// PURPOSE: McpActionSurface — MCP server action: business logic + JSON building.
//
// MCP protocol surface (surface_mcp_tool_command) delegates here; this surface
// delegates to dispatcher surfaces (pure business logic) and maps results to
// JSON responses. No formatting/println — JSON is returned as serde_json::Value.
use std::path::PathBuf;
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

pub use crate::surface_mcp_action_view::{
    error_response, is_mutating_action, resolve_confined_path, violations_to_json,
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

    pub(crate) fn authorize_action(&self, action: &str) -> Result<(), serde_json::Value> {
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
<<<<<<< HEAD
=======

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
            Ok(outcome) => violations_response("naming", path, &outcome.violations),
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
                self.deps.filesystem.clone(),
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
            self.deps.config_file_reader.as_ref(),
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

    /// Dispatch execute_command actions.
    pub async fn execute_command(
        &self,
        action: &str,
        path: &str,
        threshold: u64,
        dry_run: bool,
    ) -> serde_json::Value {
        let authorization = if action == "fix" {
            self.authorize_action_allow_dry_run(action, dry_run)
        } else {
            self.authorize_action(action)
        };
        if let Err(error) = authorization {
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

    /// Run docs audit via dispatcher (delegates to the docs-scaffold action module).
    pub async fn execute_docs(&self, path: &str) -> serde_json::Value {
        crate::surface_mcp_docs_scaffold_command::McpDocsScaffoldSurface
            .execute_docs(self, path)
            .await
    }

    /// Run `docs scaffold` (delegates to the docs-scaffold action module).
    pub async fn execute_docs_scaffold(
        &self,
        path: &str,
        member: Option<&str>,
        dry_run: bool,
    ) -> serde_json::Value {
        crate::surface_mcp_docs_scaffold_command::McpDocsScaffoldSurface
            .execute_docs_scaffold(self, path, member, dry_run)
            .await
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
            .map(|(name, desc, cli_example)| {
                let supported = !matches!(*name, "watch" | "mcp-config" | "config" | "git");
                let args = match *name {
                    "ci" => serde_json::json!({"path": ".", "threshold": 80}),
                    "fix" => serde_json::json!({"path": ".", "dry_run": true}),
                    "doctor" | "adapters" | "version" | "uninstall-hook" | "init" | "install" => {
                        serde_json::Value::Null
                    }
                    _ => serde_json::json!({"path": "."}),
                };
                let example = if supported {
                    serde_json::json!({
                        "tool": "execute_command",
                        "arguments": {"action": name, "args": args}
                    })
                } else {
                    serde_json::Value::Null
                };
                serde_json::json!({
                    "name": name,
                    "description": desc,
                    "example": example,
                    "cli_example": cli_example,
                    "mcp_supported": supported,
                    "mcp_note": if supported { serde_json::Value::Null } else {
                        serde_json::json!(format!("{name} is not available via MCP"))
                    },
                })
            })
            .collect();
        serde_json::json!({ "status": "ok", "commands": commands, "total": commands.len(), "exit_code": 0 })
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
        "fix" | "install-hook" | "uninstall-hook" | "init" | "install" | "docs_scaffold"
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
>>>>>>> 9df34c75 (Improve naming-rules UX documentation and output formatting (#1378))
}
