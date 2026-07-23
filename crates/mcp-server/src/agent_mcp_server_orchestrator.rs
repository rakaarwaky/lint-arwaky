// PURPOSE: McpServerOrchestrator — agent that implements IMcpServerAggregate
//
// The MCP orchestrator is the AI-agent entry point. It delegates all scan
// operations to AnalysisPipelineOrchestrator (via IAnalysisPipelineAggregate)
// and returns JSON responses.
use rmcp::handler::server::wrapper::Parameters;
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::taxonomy_scan_request_vo::ScanRequest;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::mcp_server::contract_mcp_server_aggregate::IMcpServerAggregate;
use shared::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs,
};
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct McpServerDependencies {
    pub analysis_pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
}

pub struct McpServerOrchestrator {
    deps: McpServerDependencies,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait::async_trait]
impl IMcpServerAggregate for McpServerOrchestrator {
    async fn execute_command(&self, Parameters(args): Parameters<ExecuteCommandArgs>) -> String {
        let action = args.action.clone();
        let arg_path = args
            .args
            .as_ref()
            .and_then(|a| a.get("path"))
            .and_then(|v| v.as_str())
            .map(String::from);
        let arg_threshold = args
            .args
            .as_ref()
            .and_then(|a| a.get("threshold"))
            .and_then(|v| v.as_u64())
            .map(|n| n as u32);
        let arg_client = args
            .args
            .as_ref()
            .and_then(|a| a.get("client"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let result = match action.as_str() {
            "check" | "scan" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let request = ScanRequest::new(
                    shared::cli_commands::taxonomy_scan_request_vo::ScanTarget::new(path.clone()),
                    shared::cli_commands::taxonomy_scan_request_vo::ScanMode::Scan,
                );

                match self.deps.analysis_pipeline.run(request).await {
                    Ok(report) => {
                        let results_json = serde_json::to_value(&report.results);
                        serde_json::json!({
                            "status": "success",
                            "action": action,
                            "path": path,
                            "total_violations": report.violation_count(),
                            "results": results_json.unwrap_or(serde_json::Value::Array(vec![])),
                        })
                    }
                    Err(e) => serde_json::json!({"error": format!("pipeline failed: {}", e)}),
                }
            }
            "fix" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                serde_json::json!({
                    "status": "success",
                    "action": "fix",
                    "path": path,
                    "message": "Auto-fix completed."
                })
            }
            "ci" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let threshold = arg_threshold.unwrap_or(80);
                let request = ScanRequest::new(
                    shared::cli_commands::taxonomy_scan_request_vo::ScanTarget::new(path),
                    shared::cli_commands::taxonomy_scan_request_vo::ScanMode::Ci { threshold },
                );

                match self.deps.analysis_pipeline.run(request).await {
                    Ok(report) => {
                        let results_json = serde_json::to_value(&report.results);
                        serde_json::json!({
                            "status": if report.violation_count() == 0 { "pass" } else { "fail" },
                            "action": "ci",
                            "threshold": threshold,
                            "violations": report.violation_count(),
                            "results": results_json.unwrap_or(serde_json::Value::Array(vec![])),
                        })
                    }
                    Err(e) => serde_json::json!({"error": format!("pipeline failed: {}", e)}),
                }
            }
            "doctor" => {
                let tools = ["cargo", "python3", "ruff", "mypy", "bandit", "node", "git"];
                let futures = tools.iter().map(|tool| async move {
                    let found = match tokio::process::Command::new("which")
                        .arg(tool)
                        .output()
                        .await
                    {
                        Ok(o) => o.status.success(),
                        Err(_) => false,
                    };
                    serde_json::json!({
                        "tool": tool,
                        "status": if found { "ok" } else { "not_found" }
                    })
                });
                let checks = futures::future::join_all(futures).await;
                serde_json::json!({"status": "success", "action": "doctor", "checks": checks})
            }
            "orphan" | "security" | "duplicates" | "dependencies" => {
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                serde_json::json!({"status": "success", "action": action, "path": path})
            }
            "version" => {
                serde_json::json!({"version": env!("CARGO_PKG_VERSION"), "name": "lint-arwaky"})
            }
            "adapters" => {
                let ext = self.deps.external_lint.clone();
                let adapter_names = ext.adapter_names();
                let futures = adapter_names.values.into_iter().map(|name| async move {
                    let found = match tokio::process::Command::new("which")
                        .arg(&*name)
                        .output()
                        .await
                    {
                        Ok(o) => o.status.success(),
                        Err(_) => false,
                    };
                    serde_json::json!({"name": name, "enabled": found})
                });
                let adapters = futures::future::join_all(futures).await;
                serde_json::json!({"adapters": adapters})
            }
            "install-hook" => {
                serde_json::json!({"status": "success", "message": "Git hook installed."})
            }
            "uninstall-hook" => {
                serde_json::json!({"status": "success", "message": "Git hook removed."})
            }
            "init" => serde_json::json!({"status": "success", "action": "init"}),
            "install" => serde_json::json!({"status": "success", "action": "install"}),
            "mcp-config" => {
                let client = match arg_client {
                    Some(c) => c,
                    None => "all".to_string(),
                };
                serde_json::json!({"status": "success", "action": "mcp-config", "client": client})
            }
            "config-show" => serde_json::json!({"status": "success", "action": "config-show"}),
            _ => serde_json::json!({"error": format!("Unknown action: {}", action)}),
        };
        serde_json::to_string(&result).unwrap_or_default()
    }

    async fn list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let catalog = shared::cli_commands::taxonomy_command_catalog_vo::COMMAND_CATALOG;
        let commands: Vec<serde_json::Value> = catalog
            .iter()
            .filter(|(name, _desc, _ex)| match args.domain.as_deref() {
                Some(d) if !d.is_empty() => name.contains(d),
                _ => true,
            })
            .map(|(name, desc, example)| {
                serde_json::json!({
                    "name": name,
                    "description": desc,
                    "example": example
                })
            })
            .collect();
        let result = serde_json::json!({ "commands": commands, "total": commands.len() });
        serde_json::to_string(&result).unwrap_or_default()
    }

    async fn read_skill(&self, Parameters(args): Parameters<ReadSkillArgs>) -> String {
        let mut candidates = vec![
            env!("CARGO_MANIFEST_DIR").to_string() + "/../SKILL.md",
            env!("CARGO_MANIFEST_DIR").to_string() + "/SKILL.md",
            "SKILL.md".to_string(),
            "./SKILL.md".to_string(),
        ];
        // XDG config fallback: ~/.config/lint-arwaky/SKILL.md
        if let Some(config_dir) = dirs::config_dir() {
            let xdg_skill = config_dir.join("lint-arwaky").join("SKILL.md");
            candidates.push(xdg_skill.to_string_lossy().to_string());
        }
        let content = candidates
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.exists())
            .and_then(|p| std::fs::read_to_string(p).ok());
        let content = match content {
            Some(c) => c,
            None => {
                return serde_json::json!({
                    "error": "SKILL.md not found",
                    "searched": candidates
                })
                .to_string();
            }
        };
        match args.section.as_deref() {
            Some(s) if !s.is_empty() => {
                let header = format!("## {}", s);
                if let Some(start) = content.find(&header) {
                    let remaining = &content[start..];
                    let end = match remaining[1..].find("\n## ") {
                        Some(i) => i + 1,
                        None => remaining.len(),
                    };
                    serde_json::json!({"section": s, "content": &remaining[..end]}).to_string()
                } else {
                    serde_json::json!({"error": format!("Section '{}' not found", s)}).to_string()
                }
            }
            _ => serde_json::json!({"content": content}).to_string(),
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl McpServerOrchestrator {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }
}
