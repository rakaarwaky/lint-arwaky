// PURPOSE: ToolsController — MCP surface for tool listing and execution

use shared::common::contract_service_aggregate::ServiceContainerAggregate;
use std::sync::Arc;

/// Handler for registering MCP tools.
pub struct McpToolsHandler {}

pub fn register_tools(container: Arc<dyn ServiceContainerAggregate>) {
    // Tools registered via sub-modules below
    crate::surface_execute_command::register_execute_commands(container.clone());
    crate::surface_command_controller::register_catalog_commands(container.clone());
    crate::surface_health_controller::register_health_commands(container.clone());
    crate::surface_client_controller::register_desktop_client(container.clone());
}
