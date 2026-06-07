/// MCP Tools Registry - Bridges Capabilities to the Surface Layer.
use crate::contract::service_container_aggregate::ServiceContainerAggregate;

/// Handler for registering MCP tools.
pub struct McpToolsHandler;
use std::sync::Arc;

pub fn register_tools(container: Arc<dyn ServiceContainerAggregate>) {
    // In real impl: register each tool to the MCP server
    // For now, this is a placeholder that wires the logic
    println!("Registering tools with container...");
    
    // Delegate to split modules
    crate::surfaces::mcp_execute_command::register_execute_commands(container.clone());
    crate::surfaces::mcp_command_handler::register_catalog_commands(container.clone());
    crate::surfaces::mcp_health_handler::register_health_commands(container.clone());
    crate::surfaces::mcp_client_handler::register_desktop_client(container.clone());
}
