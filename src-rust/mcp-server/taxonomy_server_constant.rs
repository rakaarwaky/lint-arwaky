// PURPOSE: ServerConstant — constant definitions for MCP server configuration

pub const MCP_SERVER_VERSION: &str = "1.0.0";
pub const AUTO_LINT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAX_PATH_LENGTH: usize = 4096;
pub const MAX_PATH_DEPTH: usize = 32;
pub const MAX_BATCH_SIZE: usize = 100;
pub const MAX_STRING_LENGTH: usize = 1_000_000;
