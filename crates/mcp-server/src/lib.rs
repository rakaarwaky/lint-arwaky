// PURPOSE: Module declarations for mcp-server (Surface-only crate)
// No contract/aggregate/capabilities layers — surface calls dispatcher directly.
pub mod surface_mcp_action_command;
pub mod surface_mcp_tool_command;
pub mod taxonomy_mcp_server_vo;

// ─── Re-exports ────────────────────────────────────────────
// The MCP taxonomy used to live in the shared kernel crate (shared-mcp-server);
// it moved here because the MCP server is its only consumer (issue #572).

pub use taxonomy_mcp_server_vo::ExecuteCommandArgs;
pub use taxonomy_mcp_server_vo::GetConfigArgs;
pub use taxonomy_mcp_server_vo::ListCommandsArgs;
pub use taxonomy_mcp_server_vo::ReadSkillArgs;
