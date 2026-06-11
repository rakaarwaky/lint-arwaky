// PURPOSE: ServerWrapper — IMcpServerPort implementation wrapping the MCP server lifecycle
use cli_commands::taxonomy_transport_error::TransportError;
use mcp_server::contract_server_port::ToolHandler;
use mcp_server::IMcpServerPort;
use shared_common::taxonomy_name_vo::SymbolName;
use shared_common::taxonomy_suggestion_vo::DescriptionVO;

pub struct McpServerWrapper {
    _project_root: String,
    _server_name: String,
}

impl McpServerWrapper {
    pub fn new(project_root: &str, server_name: &str) -> Self {
        Self {
            _project_root: project_root.to_string(),
            _server_name: server_name.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl IMcpServerPort for McpServerWrapper {
    fn register_tool(&self, _name: SymbolName, _description: DescriptionVO, _handler: ToolHandler) {
    }

    async fn run_server(&self) -> Result<(), TransportError> {
        Ok(())
    }

    fn stop_server(&self) {}
}
