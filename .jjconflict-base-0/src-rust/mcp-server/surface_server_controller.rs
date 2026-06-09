/// MCP Server Handler - Server startup only
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::mcp_server::surface_tools_controller::register_tools;

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
        println!("Lint Arwaky MCP server starting...");
        println!("Server name: lint-arwaky");
        println!("Note: Full MCP server requires 'fastmcp' / 'mcp' crate integration");
    }

    pub fn run(&self, container: std::sync::Arc<dyn ServiceContainerAggregate>) {
        self.run_server(container);
    }
}
