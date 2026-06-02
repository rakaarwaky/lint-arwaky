"""mcp_server_constants — Local technical constants for the MCP server infrastructure.

These are protocol-level and versioning constants that belong exclusively
to the MCP server adapter layer. They are NOT domain concepts and should
not be placed in the shared taxonomy.
"""

# ── Version constants ──────────────────────────────────────────────────────

MCP_SERVER_VERSION: str = "1.5.0"
MCP_PROTOCOL_MIN: str = "2024-11-05"
MCP_PROTOCOL_MAX: str = "2025-06-18"
AUTO_LINT_VERSION: str = "1.9.4"

# ── Input validation bounds (MCP protocol-level limits) ───────────────────

MAX_FILES: int = 1000
MAX_PATH_LENGTH: int = 1024
MAX_STRING_LENGTH: int = 8192
MAX_PATH_DEPTH: int = 50
MAX_BATCH_SIZE: int = 50
