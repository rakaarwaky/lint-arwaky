/// mcp_server_constants — Local technical constants for the MCP server infrastructure.
///
/// These are protocol-level and versioning constants that belong exclusively
/// to the MCP server adapter layer. They are NOT domain concepts and should
/// not be placed in the shared taxonomy.

/// ── Version constants ──────────────────────────────────────────────────────

pub const MCP_SERVER_VERSION: &str = "1.5.0";
pub const MCP_PROTOCOL_MIN: &str = "2024-11-05";
pub const MCP_PROTOCOL_MAX: &str = "2025-06-18";
pub const AUTO_LINT_VERSION: &str = "1.9.4";

/// ── Input validation bounds (MCP protocol-level limits) ───────────────────

pub const MAX_FILES: usize = 1000;
pub const MAX_PATH_LENGTH: usize = 1024;
pub const MAX_STRING_LENGTH: usize = 8192;
pub const MAX_PATH_DEPTH: usize = 50;
pub const MAX_BATCH_SIZE: usize = 50;
