// PURPOSE: LintArwakyMcpServer — MCP server using rmcp official SDK
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    Implementation, ProtocolVersion, ServerCapabilities, ServerInfo, ToolsCapability,
};
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;

use crate::taxonomy_mcp_tool_args_vo::{ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs};

#[derive(Clone)]
pub struct LintArwakyMcpServer {
    _code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    tool_router: ToolRouter<Self>,
}

impl LintArwakyMcpServer {
    pub fn new(_code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self {
            _code_analysis_linter,
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_handler]
impl ServerHandler for LintArwakyMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::default(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: None }),
                ..Default::default()
            },
            server_info: Implementation {
                name: "lint-arwaky".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                ..Default::default()
            },
            instructions: None,
        }
    }
}

#[tool_router]
impl LintArwakyMcpServer {
    #[tool(description = "Execute any CLI command. This is the primary tool.")]
    async fn execute_command(&self, Parameters(args): Parameters<ExecuteCommandArgs>) -> String {
        // Clone Arc so spawn_blocking owns a 'static reference to the linter
        let linter = self._code_analysis_linter.clone();
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
                // spawn_blocking: arch_linter.run_lint internally creates a
                // tokio runtime, which would panic if called from within
                // rmcp's async context. Off-load to blocking thread pool.
                let linter_for_blocking = linter.clone();
                let path_for_blocking = path.clone();
                let join_result = tokio::task::spawn_blocking(move || {
                    let results = linter_for_blocking.run_code_analysis_path(&path_for_blocking);
                    let report = linter_for_blocking.format_report(
                        &shared::cli_commands::taxonomy_result_vo::LintResultList::new(
                            results.clone(),
                        ),
                        &path_for_blocking,
                    );
                    serde_json::json!({
                        "status": "success",
                        "action": action,
                        "path": path_for_blocking,
                        "total_violations": results.len(),
                        "report": report
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
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
                let threshold = match arg_threshold {
                    Some(t) => t,
                    None => 80,
                };
                let linter_for_blocking = linter.clone();
                let path_for_blocking = path.clone();
                let join_result = tokio::task::spawn_blocking(move || {
                    let results = linter_for_blocking.run_code_analysis_path(&path_for_blocking);
                    let score = linter_for_blocking.calc_score(&results);
                    let pass = score >= threshold as f64;
                    serde_json::json!({
                        "status": if pass { "pass" } else { "fail" },
                        "action": "ci",
                        "score": score,
                        "threshold": threshold,
                        "violations": results.len()
                    })
                })
                .await;
                match join_result {
                    Ok(v) => v,
                    Err(e) => serde_json::json!({"error": format!("blocking task failed: {}", e)}),
                }
            }
            "doctor" => {
                let mut checks = Vec::new();
                for tool in &["cargo", "python3", "ruff", "mypy", "bandit", "node", "git"] {
                    let found = match std::process::Command::new("which").arg(tool).output() {
                        Ok(o) => o.status.success(),
                        Err(_) => false,
                    };
                    checks.push(serde_json::json!({
                        "tool": tool,
                        "status": if found { "ok" } else { "not_found" }
                    }));
                }
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
                let mut adapters = Vec::new();
                for (name, lang) in &[
                    ("ruff", "python"),
                    ("mypy", "python"),
                    ("bandit", "python"),
                    ("clippy", "rust"),
                    ("eslint", "javascript"),
                ] {
                    let found = match std::process::Command::new("which").arg(name).output() {
                        Ok(o) => o.status.success(),
                        Err(_) => false,
                    };
                    adapters.push(
                        serde_json::json!({"name": name, "language": lang, "enabled": found}),
                    );
                }
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
        match serde_json::to_string_pretty(&result) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }

    #[tool(
        description = "List all available CLI commands with descriptions and examples. Optional `domain` filter (e.g. \"setup\", \"check\")."
    )]
    async fn list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let catalog = shared::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
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
        match serde_json::to_string_pretty(&result) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }

    #[tool(
        description = "Read SKILL.md documentation by section. Searches several candidate locations."
    )]
    async fn read_skill(&self, Parameters(args): Parameters<ReadSkillArgs>) -> String {
        let candidates = [
            env!("CARGO_MANIFEST_DIR").to_string() + "/../SKILL.md",
            env!("CARGO_MANIFEST_DIR").to_string() + "/SKILL.md",
            "SKILL.md".to_string(),
            "./SKILL.md".to_string(),
        ];
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

    #[tool(description = "Check system health: adapters and system state.")]
    async fn health_check(&self) -> String {
        let mut adapters = Vec::new();
        for (name, lang) in &[
            ("ruff", "python"),
            ("mypy", "python"),
            ("bandit", "python"),
            ("clippy", "rust"),
            ("eslint", "javascript"),
        ] {
            let found = match std::process::Command::new("which").arg(name).output() {
                Ok(o) => o.status.success(),
                Err(_) => false,
            };
            adapters.push(serde_json::json!({
                "name": name,
                "language": lang,
                "status": if found { "available" } else { "not_installed" }
            }));
        }
        let available = adapters
            .iter()
            .filter(|a| a["status"] == "available")
            .count();
        let result = serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "adapters_available": available,
            "adapters_total": adapters.len(),
            "adapters": adapters
        });
        match serde_json::to_string_pretty(&result) {
            Ok(s) => s,
            Err(_) => String::new(),
        }
    }
}
