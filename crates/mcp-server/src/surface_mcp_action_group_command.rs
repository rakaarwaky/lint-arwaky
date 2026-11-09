// PURPOSE (surface layer — MCP `execute_command` group actions): the per-group
// scan verbs (`execute_quality` … `execute_dependencies`) and the read-only
// `handle_*` handlers.
//
// Split out of `surface_mcp_action_command.rs` under AES301. That file passed
// 1000 lines once `ci`, `report`, `git`, `fix`, `structure` and `docs_scaffold`
// each gained a real dispatch; this half is the part that merely forwards to
// one dispatcher aggregate per rule group. Both files implement the same
// `McpActionSurface`, so callers see no new type.
use super::surface_mcp_action_command::McpActionSurface;
use crate::surface_mcp_action_view::{normalize_response, violations_response};

impl McpActionSurface {
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

    /// Run structure rules (AES701–AES706) via dispatcher.
    ///
    /// Distinct from `check`: this reports the folder/document layer alone,
    /// which is what a caller asking for one group expects.
    pub async fn execute_structure(&self, path: &str) -> serde_json::Value {
        let fp = match self.to_fp(path) {
            Ok(f) => f,
            Err(e) => return e,
        };
        let root = fp.value.clone();
        let root_owned = root.clone();
        let structure = self.deps.structure_orchestrator.clone();
        let fs_seam = self.deps.fs_seam.clone();
        let config_file_reader = self.deps.config_file_reader.clone();
        let structure_result = match Self::run_blocking(move || {
            dispatcher::surface_structure_action::collect_structure(
                &root,
                structure,
                &fs_seam,
                config_file_reader.as_ref(),
            )
        })
        .await
        {
            Ok(structure_result) => structure_result,
            Err(e) => return e,
        };
        match structure_result {
            Ok(violations) => violations_response("structure", &root_owned, &violations),
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
        base: &str,
        member: Option<&str>,
        rules: Option<&str>,
        strict: bool,
    ) -> serde_json::Value {
        // `fix` and `docs_scaffold` are the two mutating actions with a
        // preview mode; a dry run is safe in a read-only server.
        let authorization = if matches!(action, "fix" | "docs_scaffold") {
            self.authorize_action_allow_dry_run(action, dry_run)
        } else {
            self.authorize_action(action)
        };
        if let Err(error) = authorization {
            return error;
        }
        let response = match action {
            "check" | "scan" => self.execute_check(path, member, rules).await,
            "ci" => self.execute_ci(path, threshold).await,
            "fix" => self.execute_fix(path, dry_run).await,
            "doctor" => self.execute_doctor().await,
            "orphan" => self.execute_orphan(path).await,
            "security" => self.execute_security(path).await,
            "quality" => self.execute_quality(path).await,
            "import" => self.execute_import(path).await,
            "naming" => self.execute_naming(path).await,
            "role" => self.execute_role(path).await,
            "docs" => self.execute_docs(path, strict).await,
            "external" => self.execute_external(path).await,
            "structure" => self.execute_structure(path).await,
            // The catalogue name is `docs_scaffold`; the catalogue does not
            // list it, but the tool surface serves it and `is_mutating_action`
            // already classifies it, so the arm must exist too.
            "docs_scaffold" => self.execute_docs_scaffold(path, member, dry_run).await,
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
            // The catalog advertises `config`; the arm only matched the
            // internal spelling, so the advertised name answered exit 2.
            "report" => self.execute_report(path).await,
            "config" | "config-show" => {
                let result = self.handle_get_config(path, None);
                serde_json::from_str(&result).unwrap_or_else(
                    |_| serde_json::json!({"error": "Failed to serialize config", "exit_code": 2}),
                )
            }
            "git" => self.execute_git(path, base).await,
            _ => {
                serde_json::json!({"error": format!("Unknown action: {}", action), "exit_code": 2})
            }
        };
        normalize_response(response)
    }

    /// Run docs audit via dispatcher (delegates to the docs-scaffold action module).
    pub async fn execute_docs(&self, path: &str, strict: bool) -> serde_json::Value {
        crate::surface_mcp_docs_scaffold_command::McpDocsScaffoldSurface
            .execute_docs(self, path, strict)
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
