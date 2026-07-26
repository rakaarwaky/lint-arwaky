// mcp-server — taxonomy and contract types
pub mod contract_mcp_server_aggregate;
pub mod taxonomy_mcp_tool_args_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_mcp_server_aggregate::IMcpServerAggregate;

// ── Taxonomy types ──
pub use taxonomy_mcp_tool_args_vo::ExecuteCommandArgs;
pub use taxonomy_mcp_tool_args_vo::GetConfigArgs;
pub use taxonomy_mcp_tool_args_vo::ListCommandsArgs;
pub use taxonomy_mcp_tool_args_vo::ReadSkillArgs;
