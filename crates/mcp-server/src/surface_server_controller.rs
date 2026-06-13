// PURPOSE: ServerController — MCP surface for server lifecycle management

use crate::surface_tools_controller::register_tools;

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

    pub fn run_server(&self) {
        register_tools();
        eprintln!("Lint Arwaky MCP server starting...");
        eprintln!("Server name: lint-arwaky");
    }

    pub fn run(&self) {
        self.run_server();
    }
}
