// PURPOSE: ServerController — MCP surface for server lifecycle management

use crate::surface_tools_controller::register_tools;
use shared::common::contract_service_aggregate::ServiceContainerAggregate;

pub struct McpServerHandlerSurface {}

impl Default for McpServerHandlerSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl McpServerHandlerSurface {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_server(&self, container: std::sync::Arc<dyn ServiceContainerAggregate>) {
        register_tools(container);
        eprintln!("Lint Arwaky MCP server starting...");
        eprintln!("Server name: lint-arwaky");
    }

    pub fn run(&self, container: std::sync::Arc<dyn ServiceContainerAggregate>) {
        self.run_server(container);
    }
}
