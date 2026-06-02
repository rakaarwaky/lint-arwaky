/// Main entry point for the Auto Linter MCP server.
use crate::taxonomy::*;
use crate::contract::*;
use crate::surfaces::mcp_tools_store::register_tools;

pub struct McpServerHandlerSurface;

impl McpServerHandlerSurface {
    pub fn new() -> Self {
        Self
    }

    pub fn run_server(&self, container: ServiceContainerAggregate) {
        // In real impl: create FastMCP equivalent, register tools, run
        register_tools(container);
        println!("Auto-Linter MCP server starting...");
        println!("Server name: auto-linter");
        println!("Note: Full MCP server requires 'fastmcp' / 'mcp' crate integration");
    }

    pub fn run(&self, container: ServiceContainerAggregate) {
        self.run_server(container);
    }
}
