// PURPOSE: Controller: Tools surface controller

use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;

/// Handler for registering MCP tools.
pub struct McpToolsHandler {}
use std::sync::Arc;

pub fn register_tools(container: Arc<dyn ServiceContainerAggregate>) {
    // In real impl: register each tool to the MCP server
    // For now, this is a placeholder that wires the logic
    println!("Registering tools with container...");

    // Delegate to split modules
    crate::mcp_server::surface_execute_command::register_execute_commands(container.clone());
    crate::mcp_server::surface_command_controller::register_catalog_commands(container.clone());
    crate::mcp_server::surface_health_controller::register_health_commands(container.clone());
    crate::mcp_server::surface_client_controller::register_desktop_client(container.clone());
}
