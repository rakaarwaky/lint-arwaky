// PURPOSE: Module declarations for mcp-server (controllers, actions, schemas)
pub use shared::mcp_server::contract_server_port::{IMcpServerPort, ToolHandler};
pub use shared::mcp_server::taxonomy_server_constant::{
    AUTO_LINT_VERSION, MAX_BATCH_SIZE, MAX_PATH_DEPTH, MAX_PATH_LENGTH, MAX_STRING_LENGTH,
    MCP_SERVER_VERSION,
};
pub use shared::mcp_server::taxonomy_server_validation_utility::{
    validate_path, validate_string_input, ValidationResult,
};
pub mod infrastructure_server_wrapper;
pub use infrastructure_server_wrapper::McpServerWrapper;
pub mod surface_command_controller;
pub use surface_command_controller::{
    list_commands_func, register_catalog_commands, register_list_commands,
    register_read_skill_context, CommandEntry, McpCommandCatalogSurface,
};
pub mod surface_health_controller;
pub use surface_health_controller::{register_health_commands, McpHealthCheckSurface};
pub mod root_mcp_server_container;
