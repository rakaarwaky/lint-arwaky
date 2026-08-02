// PURPOSE: LintArwakyMcpServer — MCP surface: tool registration + protocol only
//
// The surface layer bridges async rmcp protocol to the sync orchestrator.
// All business logic lives in the orchestrator (agent layer).
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    Implementation, ProtocolVersion, ServerCapabilities, ServerInfo, ToolsCapability,
};
use rmcp::{tool, tool_handler, tool_router, ServerHandler};
use std::sync::Arc;

use shared::mcp_server::IMcpServerAggregate;
use shared::mcp_server::{ExecuteCommandArgs, GetConfigArgs, ListCommandsArgs, ReadSkillArgs};

#[derive(Clone)]
pub struct LintArwakyMcpServer {
    agent: Arc<dyn IMcpServerAggregate>,
    // Consumed implicitly by the `#[tool_router]` proc-macro, which also derives
    // `ServerHandler`/`tool_router()` from it. Read here to keep it live for the
    // macro-generated `ServerHandler::call_tool` dispatch path.
    tool_router: ToolRouter<Self>,
}

impl LintArwakyMcpServer {
    pub fn new(agent: Arc<dyn IMcpServerAggregate>) -> Self {
        Self {
            agent,
            tool_router: Self::tool_router(),
        }
    }

    /// Expose the configured tool router (used by the proc-macro `ServerHandler`
    /// impl for routing incoming tool calls; kept as a public surface so the
    /// field is not dead code).
    pub fn router(&self) -> &ToolRouter<Self> {
        &self.tool_router
    }
}

#[tool_handler]
impl ServerHandler for LintArwakyMcpServer {
    fn get_info(&self) -> ServerInfo {
        let mut builder = ServerCapabilities::builder();
        builder.tools = Some(ToolsCapability::default());
        let capabilities = builder.build();
        ServerInfo::new(capabilities)
            .with_server_info(Implementation::new(
                "lint-arwaky",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_protocol_version(ProtocolVersion::default())
    }
}

#[tool_router]
impl LintArwakyMcpServer {
    #[tool(description = "Execute any CLI command. This is the primary tool.")]
    pub async fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String {
        self.agent.execute_command(args)
    }

    #[tool(
        description = "List all available CLI commands with descriptions and examples. Optional `domain` filter (e.g. \"setup\", \"check\")."
    )]
    pub async fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String {
        self.agent.list_commands(args)
    }

    #[tool(
        description = "Read skill documentation by section. Searches skill candidate locations."
    )]
    pub async fn read_skill(&self, args: Parameters<ReadSkillArgs>) -> String {
        self.agent.read_skill(args)
    }

    #[tool(description = "Check system health: adapters and system state.")]
    pub async fn health_check(&self) -> String {
        // FRD FR-004: all 9 adapters must be checked
        let mut adapters = Vec::new();
        for (name, lang) in &[
            ("clippy", "Rust"),
            ("rustfmt", "Rust"),
            ("cargo-audit", "Rust"),
            ("ruff", "Python"),
            ("mypy", "Python"),
            ("bandit", "Python"),
            ("eslint", "JS/TS"),
            ("prettier", "JS/TS"),
            ("tsc", "JS/TS"),
        ] {
            let found = match *name {
                "clippy" => std::process::Command::new("cargo")
                    .args(["clippy", "--version"])
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false),
                "rustfmt" => std::process::Command::new("rustfmt")
                    .args(["--version"])
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false),
                "cargo-audit" => std::process::Command::new("cargo")
                    .args(["audit", "--version"])
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false),
                _ => std::process::Command::new("which")
                    .arg(name)
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false),
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
            "adapters": adapters,
            "exit_code": 0,
        });
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }

    #[tool(
        description = "Return the effective architecture configuration for a target path/language. Shows rules, thresholds, adapters."
    )]
    pub async fn get_config(&self, args: Parameters<GetConfigArgs>) -> String {
        self.agent.get_config(args)
    }
}
