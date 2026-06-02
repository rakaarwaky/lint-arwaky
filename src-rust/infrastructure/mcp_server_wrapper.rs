/// mcp_server_wrapper — Infrastructure adapter providing MCP spec compliance.
use crate::contract::IMcpServerPort;
use crate::taxonomy::{DescriptionVO, SymbolName};
use crate::infrastructure::mcp_server_constants::{MCP_SERVER_VERSION, AUTO_LINT_VERSION, MCP_PROTOCOL_MIN, MCP_PROTOCOL_MAX};
use crate::infrastructure::mcp_server_schemas::build_tool_schemas;
use crate::infrastructure::mcp_server_resources::build_resources;

pub struct McpServerWrapper {
    project_root: String,
    server_name: String,
}

impl McpServerWrapper {
    pub fn new(project_root: &str, server_name: &str) -> Self {
        Self {
            project_root: project_root.to_string(),
            server_name: server_name.to_string(),
        }
    }

    pub fn setup(&self) {
        let _schemas = build_tool_schemas();
        let _resources = build_resources(&self.project_root);
    }
}

#[async_trait::async_trait]
impl IMcpServerPort for McpServerWrapper {
    fn register_tool(&self, _name: SymbolName, _description: DescriptionVO, _handler: crate::contract::ToolHandler) {}

    async fn run_server(&self) -> Result<(), String> {
        Ok(())
    }

    fn stop_server(&self) {}
}
