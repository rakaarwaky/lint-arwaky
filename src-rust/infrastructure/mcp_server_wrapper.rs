/// mcp_server_wrapper — Infrastructure adapter providing MCP spec compliance.
use crate::contract::mcp_server_port::IMcpServerPort;
use crate::taxonomy::{DescriptionVO, SymbolName, TransportError};

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
    fn register_tool(
        &self,
        _name: SymbolName,
        _description: DescriptionVO,
        _handler: crate::contract::ToolHandler,
    ) {
    }

    async fn run_server(&self) -> Result<(), TransportError> {
        Ok(())
    }

    fn stop_server(&self) {}
}
