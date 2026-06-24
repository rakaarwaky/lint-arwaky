// PURPOSE: LintArwakyMcpServer — MCP surface: tool registration + protocol only
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    Implementation, ProtocolVersion, ServerCapabilities, ServerInfo, ToolsCapability,
};
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use std::sync::Arc;

use crate::contract_mcp_server_aggregate::IMcpServerAggregate;
use crate::taxonomy_mcp_tool_args_vo::{ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs};

#[derive(Clone)]
pub struct LintArwakyMcpServer {
    agent: Arc<dyn IMcpServerAggregate>,
    tool_router: ToolRouter<Self>,
}

impl LintArwakyMcpServer {
    pub fn new(agent: Arc<dyn IMcpServerAggregate>) -> Self {
        Self {
            agent,
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
    async fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String {
        self.agent.execute_command(args).await
    }

    #[tool(
        description = "List all available CLI commands with descriptions and examples. Optional `domain` filter (e.g. \"setup\", \"check\")."
    )]
    async fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String {
        self.agent.list_commands(args).await
    }

    #[tool(
        description = "Read SKILL.md documentation by section. Searches several candidate locations."
    )]
    async fn read_skill(&self, args: Parameters<ReadSkillArgs>) -> String {
        self.agent.read_skill(args).await
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
