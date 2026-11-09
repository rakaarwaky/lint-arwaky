// PURPOSE: MCP surface helpers — JSON response builders and path guards.
//
// Extracted from `surface_mcp_action_command.rs` (AES301: file too large).
// These pure helpers build JSON envelopes, normalize responses, and guard
// client paths against workspace escapes. No dispatcher logic lives here.
use crate::surface_mcp_action_command::McpActionSurface;
use serde_json::Value;
use serde_json::json;
use shared_common::taxonomy_violation_item_vo::ViolationItem;
use std::path::{Component, Path, PathBuf};

/// Serialize violations to JSON (mirrors old execute_* shape + severity).
pub(crate) fn violations_to_json(violations: &[ViolationItem]) -> Vec<Value> {
    violations
        .iter()
        .map(|v| {
            json!({
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
pub(crate) fn violations_response(action: &str, path: &str, violations: &[ViolationItem]) -> Value {
    let exit_code = if violations.is_empty() { 0 } else { 1 };
    json!({
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
pub(crate) fn resolve_confined_path(root: &Path, requested: &str) -> Result<PathBuf, Value> {
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

pub(crate) fn is_mutating_action(action: &str) -> bool {
    matches!(
        action,
        "fix" | "install-hook" | "uninstall-hook" | "init" | "install" | "docs_scaffold"
    )
}

pub(crate) fn normalize_response(mut response: Value) -> Value {
    let exit_code = response
        .get("exit_code")
        .and_then(Value::as_i64)
        .unwrap_or(2);
    if let Some(object) = response.as_object_mut() {
        let normalized = if exit_code == 0 {
            "ok"
        } else if exit_code == 1 || exit_code == 3 {
            "warning"
        } else {
            "error"
        };
        object.insert("status".to_string(), json!(normalized));
    }
    response
}

pub(crate) fn error_response(message: &str) -> Value {
    json!({"status": "error", "error": message, "exit_code": 2})
}

// ─── McpActionSurface surface helpers (moved from surface_mcp_action_command) ──

impl McpActionSurface {
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
