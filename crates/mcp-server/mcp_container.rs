// PURPOSE: McpContainer — wiring for mcp-server feature (root layer, wiring only)
use std::sync::Arc;
use crate::mcp_server::contract_server_port::IMcpServerPort;

pub struct McpContainer {
    server: Arc<dyn IMcpServerPort>,
}

impl McpContainer {
    pub fn new() -> Self {
        Self {
            server: Arc::new(
                crate::mcp_server::infrastructure_server_wrapper::McpServerWrapper::new(
                    ".",
                    "lint-arwaky",
                ),
            ),
        }
    }

    pub fn server(&self) -> Arc<dyn IMcpServerPort> {
        self.server.clone()
    }
}
impl Default for McpContainer {
    fn default() -> Self {
        Self::new()
    }
}

