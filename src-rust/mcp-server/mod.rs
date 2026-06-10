// PURPOSE: Module: mcp-server module declarations and re-exports
pub mod capabilities_schema_checker;
pub use capabilities_schema_checker::McpSchemaChecker;
pub mod contract_server_port;
pub use contract_server_port::{IMcpServerPort, ToolHandler};
pub mod infrastructure_server_constants;
pub mod infrastructure_server_lifespan;
pub use infrastructure_server_lifespan::WrapperContext;
pub mod infrastructure_server_schemas;
pub use infrastructure_server_schemas::{build_tool_schemas, ToolSchema};
pub mod infrastructure_server_validation_util;
pub use infrastructure_server_validation_util::{
    validate_path, validate_string_input, ValidationResult,
};
pub mod infrastructure_server_wrapper;
pub use infrastructure_server_wrapper::McpServerWrapper;
pub mod surface_bootstrap_action;
pub use surface_bootstrap_action::SyspathBootstrapHandler;
pub mod surface_client_controller;
pub use surface_client_controller::{register_desktop_client, McpDesktopClientSurface};
pub mod surface_command_controller;
pub use surface_command_controller::{
    list_commands_func, register_catalog_commands, register_list_commands,
    register_read_skill_context, CommandEntry, McpCommandCatalogSurface,
};
pub mod surface_execute_command;
pub use surface_execute_command::{
    register_execute_commands, McpExecuteCommandSurface, RUNNING_JOBS,
};
pub mod surface_health_controller;
pub use surface_health_controller::{register_health_commands, McpHealthCheckSurface};
pub mod surface_job_controller;
pub use surface_job_controller::McpJobCommandsSurface;
pub mod surface_server_controller;
pub use surface_server_controller::McpServerHandlerSurface;
pub mod surface_tools_command;
pub use surface_tools_command::{
    commands_schema_tool, health_check_tool, list_commands_tool, read_skill_context_tool,
    McpToolsCommandSurface,
};
pub mod surface_tools_controller;
pub use surface_tools_controller::{register_tools, McpToolsHandler};
pub mod taxonomy_server_constant;
pub use taxonomy_server_constant::{
    AUTO_LINT_VERSION, MAX_BATCH_SIZE, MAX_PATH_DEPTH, MAX_PATH_LENGTH, MAX_STRING_LENGTH,
    MCP_SERVER_VERSION,
};
