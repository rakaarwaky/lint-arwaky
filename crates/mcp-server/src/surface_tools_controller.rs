// PURPOSE: ToolsController — MCP surface for tool listing and execution

/// Handler for registering MCP tools.
pub struct McpToolsHandler {}

pub fn register_tools() {
    // Tools registered via sub-modules below
    crate::surface_execute_command::register_execute_commands();
    crate::surface_command_controller::register_catalog_commands();
    crate::surface_health_controller::register_health_commands();
    crate::surface_client_controller::register_desktop_client();
}
