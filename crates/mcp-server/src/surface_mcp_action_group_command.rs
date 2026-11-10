// PURPOSE (surface layer — MCP `execute_command` group actions): the
// structure-rules scan verb (`execute_structure`) and the read-only `handle_*`
// handlers.
//
// Split out of `surface_mcp_action_command.rs` under AES301. The core dispatch
// lives in the command surface; the per-group scan verbs and read-only handlers
// live here. Both files implement the same `McpActionSurface`, so callers see
// no new type.

use super::surface_mcp_action_command::McpActionSurface;
use crate::surface_mcp_action_view::violations_response;

impl McpActionSurface {
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

    // ─── Read-only MCP business logic ────────────────────────

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
