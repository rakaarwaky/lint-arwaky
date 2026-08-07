// PURPOSE: LintArwakyMcpServer — MCP tool surface: protocol only.
//
// Holds Arc<McpActionSurface> and maps rmcp protocol parameters to action
// surface methods. No business logic here — everything delegates to
// McpActionSurface (surface_mcp_action_command), which delegates to dispatcher.
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    Implementation, ProtocolVersion, ServerCapabilities, ServerInfo, ToolsCapability,
};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use std::sync::Arc;

use shared::mcp_server::{ExecuteCommandArgs, GetConfigArgs, ListCommandsArgs, ReadSkillArgs};

use crate::surface_mcp_action_command::McpActionSurface;

#[derive(Clone)]
pub struct LintArwakyMcpServer {
    action: Arc<McpActionSurface>,
    // Consumed implicitly by the `#[tool_router]` proc-macro, which also derives
    // `ServerHandler`/`tool_router()` from it. Read here to keep it live for the
    // macro-generated `ServerHandler::call_tool` dispatch path.
    tool_router: ToolRouter<Self>,
}

impl LintArwakyMcpServer {
    pub fn new(action: Arc<McpActionSurface>) -> Self {
        Self {
            action,
            tool_router: Self::tool_router(),
        }
    }

    /// Expose the configured tool router (used by the proc-macro `ServerHandler`
    /// impl for routing incoming tool calls; kept as a public surface so the
    /// field is not dead code).
    pub fn router(&self) -> &ToolRouter<Self> {
        &self.tool_router
    }

    pub fn handle_execute_command(
        &self,
        Parameters(args): Parameters<ExecuteCommandArgs>,
    ) -> String {
        let action = args.action.clone();
        let path = args
            .args
            .as_ref()
            .and_then(|a| a.get("path"))
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| ".".to_string());
        let threshold = args
            .args
            .as_ref()
            .and_then(|a| a.get("threshold"))
            .and_then(|v| v.as_u64())
            .unwrap_or(80);
        let dry_run = args
            .args
            .as_ref()
            .and_then(|a| a.get("dry_run"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let result = self
            .action
            .execute_command(&action, &path, threshold, dry_run);
        serde_json::to_string(&result).unwrap_or_default()
    }

    pub fn handle_health_check(&self) -> String {
        let result = self.action.handle_health_check();
        serde_json::to_string(&result).unwrap_or_else(|e| {
            serde_json::json!({"error": format!("Serialization failed: {e}"), "exit_code": 2})
                .to_string()
        })
    }

    pub fn handle_list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        self.action.handle_list_commands(args.domain)
    }

    pub fn handle_read_skill(&self, Parameters(args): Parameters<ReadSkillArgs>) -> String {
        self.action.handle_read_skill(args.section)
    }

    pub fn handle_get_config(&self, Parameters(args): Parameters<GetConfigArgs>) -> String {
        let path = args.path.unwrap_or_else(|| ".".to_string());
        self.action.handle_get_config(&path, args.language)
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
                &self.action.deps.server_version,
            ))
            .with_protocol_version(ProtocolVersion::default())
    }
}

#[tool_router]
impl LintArwakyMcpServer {
    #[tool(description = "Execute any CLI command. This is the primary tool.")]
    pub async fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String {
        LintArwakyMcpServer::handle_execute_command(self, args)
    }

    #[tool(
        description = "List all available CLI commands with descriptions and examples. Optional `domain` filter (e.g. \"setup\", \"check\")."
    )]
    pub async fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String {
        LintArwakyMcpServer::handle_list_commands(self, args)
    }

    #[tool(
        description = "Read skill documentation by section. Searches skill candidate locations."
    )]
    pub async fn read_skill(&self, args: Parameters<ReadSkillArgs>) -> String {
        LintArwakyMcpServer::handle_read_skill(self, args)
    }

    #[tool(description = "Check system health: adapters and system state.")]
    pub async fn health_check(&self) -> String {
        LintArwakyMcpServer::handle_health_check(self)
    }

    #[tool(
        description = "Return the effective architecture configuration for a target path/language. Shows rules, thresholds, adapters."
    )]
    pub async fn get_config(&self, args: Parameters<GetConfigArgs>) -> String {
        LintArwakyMcpServer::handle_get_config(self, args)
    }
}
