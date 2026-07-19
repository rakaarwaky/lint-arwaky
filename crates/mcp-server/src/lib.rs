// PURPOSE: Module declarations for mcp-server
pub mod agent_mcp_server_orchestrator;
pub mod contract_mcp_server_aggregate;
pub mod root_mcp_container;
pub mod surface_mcp_command;

// Re-export shared MCP types
pub use shared::mcp_server::{ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs};
