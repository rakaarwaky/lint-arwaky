// PURPOSE: McpActionSurface — MCP server action: business logic + JSON building.
//
// MCP protocol surface (surface_mcp_tool_command) delegates here; this surface
// delegates to dispatcher surfaces (pure business logic) and maps results to
// JSON responses. No formatting/println — JSON is returned as serde_json::Value.
use std::sync::Arc;

use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::Threshold;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::project_setup::SetupManagementAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::role_rules::IRoleRunnerAggregate;

use dispatcher::surface_output_component::ViolationItem;

#[derive(Clone)]
pub struct McpServerDependencies {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator_factory:
        Arc<dyn Fn(bool) -> Arc<dyn LintFixOrchestratorAggregate> + Send + Sync>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    pub git_hooks_aggregate: Arc<dyn GitHooksAggregate>,
    pub setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
    // DI: config parsing functions
    pub parse_config_yaml: fn(&str) -> ArchitectureConfig,
    pub parse_adapter_names: fn(&str) -> Vec<String>,
    pub parse_score_threshold: fn(&str) -> Option<f64>,
}

pub struct McpActionSurface {
    pub deps: McpServerDependencies,
}

impl McpActionSurface {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }

    fn to_fp(path: &str) -> Result<FilePath, serde_json::Value> {
        FilePath::new(path.to_string())
            .map_err(|_| serde_json::json!({"error": "Invalid path", "exit_code": 2}))
    }

    /// Run check/scan — all linters combined via dispatcher.
    pub fn execute_check(&self, path: &str) -> serde_json::Value {
        let fp = match Self::to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        let opts = dispatcher::surface_check_action::ScanOptions {
            path: Some(fp),
            multi_project_orchestrator: Some(self.deps.config_orchestrator.clone()),
            filter: None,
            member: None,
            filesystem: self.deps.filesystem.clone(),
        };
        match dispatcher::surface_check_action::collect_scan(opts) {
            Ok(violations) => {
                let total = violations.len();
                let exit_code = if total == 0 { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "success" } else { "failure" },
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
    pub fn execute_ci(&self, path: &str, threshold: u64) -> serde_json::Value {
        let fp = match Self::to_fp(path) {
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
            Threshold::new(threshold as u32),
        ) {
            Ok(report) => {
                let exit_code = if report.pass { 0 } else { 1 };
                serde_json::json!({
                    "status": if report.pass { "pass" } else { "fail" },
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

    /// Run fix — auto-fix with dry_run support via dispatcher.
    pub fn execute_fix(&self, path: &str, dry_run: bool) -> serde_json::Value {
        let fp = match FilePath::new(path.to_string()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "exit_code": 2});
            }
        };
        match dispatcher::surface_fix_action::collect_fix(
            Some(fp),
            dry_run,
            self.deps.code_analysis_linter.clone(),
            self.deps.fix_orchestrator_factory.clone(),
        ) {
            Ok(report) => {
                let exit_code = if report.success {
                    0
                } else if report.fixed_count > 0 {
                    1
                } else {
                    2
                };
                serde_json::json!({
                    "status": if report.success { "success" } else { "partial" },
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
    pub fn execute_quality(&self, path: &str) -> serde_json::Value {
        let fp = match Self::to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_quality_action::collect_quality(
            Some(fp),
            self.deps.code_analysis_linter.clone(),
            None,
            self.deps.filesystem.clone(),
            &Vec::new(),
        ) {
            Ok(violations) => violations_response("quality", path, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run import scan via dispatcher.
    pub fn execute_import(&self, path: &str) -> serde_json::Value {
        let fp = match Self::to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_import_action::collect_import(
            Some(fp),
            self.deps.import_orchestrator.clone(),
            None,
            self.deps.filesystem.clone(),
            &Vec::new(),
        ) {
            Ok(violations) => violations_response("import", path, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run naming scan via dispatcher.
    pub fn execute_naming(&self, path: &str) -> serde_json::Value {
        let fp = match Self::to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_naming_action::collect_naming(
            Some(fp),
            self.deps.naming_orchestrator.clone(),
            None,
            self.deps.filesystem.clone(),
            &Vec::new(),
        ) {
            Ok(violations) => violations_response("naming", path, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run role scan via dispatcher (direct aggregate — no subprocess).
    pub fn execute_role(&self, path: &str) -> serde_json::Value {
        match dispatcher::surface_role_action::collect_role_direct(
            self.deps.role_orchestrator.clone(),
            None,
            self.deps.filesystem.clone(),
            path,
            &Vec::new(),
        ) {
            Ok(violations) => violations_response("role", path, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run orphan scan via dispatcher.
    pub fn execute_orphan(&self, path: &str) -> serde_json::Value {
        let fp = match Self::to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_orphan_action::collect_orphan(
            Some(fp),
            None,
            dispatcher::surface_orphan_action::OrphanScanDeps {
                orphan_orchestrator: self.deps.orphan_orchestrator.clone(),
                config_orchestrator: self.deps.config_orchestrator.clone(),
                fs_agg: self.deps.filesystem.clone(),
            },
            None,
        ) {
            Ok(violations) => {
                let exit_code = if violations.is_empty() { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "success" } else { "violations" },
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
    pub fn execute_external(&self, path: &str) -> serde_json::Value {
        let fp = match Self::to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        match dispatcher::surface_external_action::collect_external_direct(
            Some(fp),
            self.deps.external_lint.clone(),
            self.deps.filesystem.clone(),
            None,
            &[],
        ) {
            Ok(violations) => violations_response("external", path, &violations),
            Err(e) => serde_json::json!({"error": e, "exit_code": 2}),
        }
    }

    /// Run doctor diagnostics via dispatcher.
    pub fn execute_doctor(&self) -> serde_json::Value {
        let diag = dispatcher::surface_maintenance_action::collect_doctor(
            self.deps.maintenance_orchestrator.clone(),
        );
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
        serde_json::json!({"status": "success", "action": "doctor", "exit_code": 0, "checks": checks})
    }

    /// Run security scan via dispatcher.
    pub fn execute_security(&self, path: &str) -> serde_json::Value {
        let fp = match FilePath::new(path.to_string()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "exit_code": 2});
            }
        };
        match dispatcher::surface_maintenance_action::collect_security(
            self.deps.maintenance_orchestrator.clone(),
            Some(fp),
        ) {
            Ok(report) => {
                let exit_code = if !report.tool_installed {
                    3
                } else if report.findings.is_empty() {
                    0
                } else {
                    1
                };
                serde_json::json!({
                    "status": if exit_code == 0 { "clean" } else if exit_code == 3 { "tool_missing" } else { "findings" },
                    "action": "security",
                    "exit_code": exit_code,
                    "language": report.language,
                    "tool_name": report.tool_name,
                    "tool_installed": report.tool_installed,
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
    pub fn execute_dependencies(&self, path: &str) -> serde_json::Value {
        let fp = match FilePath::new(path.to_string()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "exit_code": 2});
            }
        };
        match dispatcher::surface_maintenance_action::collect_dependencies(
            self.deps.maintenance_orchestrator.clone(),
            Some(fp),
        ) {
            Ok(report) => serde_json::json!({
                "status": "success",
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
        let report = dispatcher::surface_version_action::collect_version();
        serde_json::json!({"version": report.version, "name": "lint-arwaky", "exit_code": 0})
    }

    /// Watch is not supported via MCP.
    pub fn execute_watch(&self) -> serde_json::Value {
        serde_json::json!({"error": "watch is not supported via MCP", "exit_code": 2})
    }

    /// Dispatch execute_command actions.
    pub fn execute_command(
        &self,
        action: &str,
        path: &str,
        threshold: u64,
        dry_run: bool,
    ) -> serde_json::Value {
        match action {
            "check" | "scan" => self.execute_check(path),
            "ci" => self.execute_ci(path, threshold),
            "fix" => self.execute_fix(path, dry_run),
            "doctor" => self.execute_doctor(),
            "orphan" => self.execute_orphan(path),
            "security" => self.execute_security(path),
            "quality" => self.execute_quality(path),
            "import" => self.execute_import(path),
            "naming" => self.execute_naming(path),
            "role" => self.execute_role(path),
            "external" => self.execute_external(path),
            "dependencies" => self.execute_dependencies(path),
            "version" => self.execute_version(),
            "watch" => self.execute_watch(),
            "adapters" => {
                let result = self.handle_health_check();
                serde_json::from_str(&result).unwrap_or_else(|_| {
                    serde_json::json!({"error": "Failed to serialize health check", "exit_code": 2})
                })
            }
            "install-hook" => {
                let fp = match Self::to_fp(path) {
                    Ok(f) => f,
                    Err(e) => return e,
                };
                match dispatcher::surface_git_action::collect_install_hook(
                    self.deps.git_hooks_aggregate.clone(),
                    &fp,
                ) {
                    Ok(report) => {
                        serde_json::json!({"status": if report.success { "success" } else { "error" }, "action": "install-hook", "exit_code": if report.success { 0 } else { 2 }, "message": report.message})
                    }
                    Err(e) => serde_json::json!({"error": format!("{e}"), "exit_code": 2}),
                }
            }
            "uninstall-hook" => {
                match dispatcher::surface_git_action::collect_uninstall_hook(
                    self.deps.git_hooks_aggregate.clone(),
                ) {
                    Ok(report) => {
                        serde_json::json!({"status": if report.success { "success" } else { "error" }, "action": "uninstall-hook", "exit_code": if report.success { 0 } else { 2 }, "message": report.message})
                    }
                    Err(e) => serde_json::json!({"error": format!("{e}"), "exit_code": 2}),
                }
            }
            "init" | "install" => {
                let items = dispatcher::surface_setup_action::collect_init(
                    self.deps.setup_orchestrator.clone(),
                );
                let messages: Vec<String> = items.iter().map(|i| i.message.clone()).collect();
                serde_json::json!({"status": "success", "action": action, "exit_code": 0, "items": messages})
            }
            "mcp-config" => {
                serde_json::json!({"error": "mcp-config requires transport configuration — use CLI for full setup", "exit_code": 1})
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
        }
    }

    // ─── Non-dispatcher MCP business logic ────────────────────

    /// Health check: adapter availability from maintenance aggregate.
    pub fn handle_health_check(&self) -> String {
        let health = dispatcher::surface_maintenance_action::collect_health_check(
            self.deps.maintenance_orchestrator.clone(),
        );
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
        let version_report = dispatcher::surface_version_action::collect_version();
        let result = serde_json::json!({
            "version": version_report.version,
            "adapters_available": available,
            "adapters_total": adapters.len(),
            "adapters": adapters,
            "exit_code": 0,
        });
        serde_json::to_string_pretty(&result).unwrap_or_else(|e| {
            serde_json::json!({"error": format!("Serialization failed: {e}"), "exit_code": 2})
                .to_string()
        })
    }

    /// List CLI commands filtered by domain.
    pub fn handle_list_commands(&self, domain: Option<String>) -> String {
        let catalog = shared::cli_commands::taxonomy_command_catalog_vo::COMMAND_CATALOG;
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
        let result =
            serde_json::json!({ "commands": commands, "total": commands.len(), "exit_code": 0 });
        serde_json::to_string(&result).unwrap_or_else(|e| {
            serde_json::json!({"error": format!("Serialization failed: {e}"), "exit_code": 2})
                .to_string()
        })
    }

    /// Read skill documentation by section.
    pub fn handle_read_skill(&self, section: Option<String>) -> String {
        let skills = [
            "lint-arwaky-rust",
            "lint-arwaky-python",
            "lint-arwaky-typescript",
        ];
        let base = env!("CARGO_MANIFEST_DIR");
        let mut candidates: Vec<String> = skills
            .iter()
            .flat_map(|s| {
                vec![
                    format!("{}/../.agents/skills/{}/SKILL.md", base, s),
                    format!(".agents/skills/{}/SKILL.md", s),
                ]
            })
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
            .and_then(|p| std::fs::read_to_string(p).ok());
        let content = match content {
            Some(c) => c,
            None => {
                return serde_json::json!({"error": "Skill documentation not found", "searched": candidates, "exit_code": 2})
                    .to_string()
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
                        .to_string()
                } else {
                    serde_json::json!({"error": format!("Section '{}' not found", s), "exit_code": 2}).to_string()
                }
            }
            _ => serde_json::json!({"content": content, "exit_code": 0}).to_string(),
        }
    }

    /// Effective architecture configuration for a target path/language.
    pub fn handle_get_config(&self, path: &str, language: Option<String>) -> String {
        let fp = match FilePath::new(path.to_string()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "exit_code": 2}).to_string();
            }
        };

        let config_files = match self.deps.config_orchestrator.list_config_files(&fp) {
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
            if let Ok(Some(source)) = self.deps.config_orchestrator.read_config(&fp, *lang) {
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
        "status": if exit_code == 0 { "success" } else { "violations" },
        "action": action,
        "path": path,
        "exit_code": exit_code,
        "violation_count": violations.len(),
        "results": violations_to_json(violations),
    })
}
