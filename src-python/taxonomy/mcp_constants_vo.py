"""MCP-specific constraints and versioning constants."""

# ── Version constants ──────────────────────────────────────────────────────

MCP_SERVER_VERSION = "1.5.0"
MCP_PROTOCOL_MIN = "2024-11-05"
MCP_PROTOCOL_MAX = "2025-06-18"
AUTO_LINT_VERSION = "1.9.3"

# ── Input validation bounds ────────────────────────────────────────────────

MAX_FILES = 1000
MAX_PATH_LENGTH = 1024
MAX_STRING_LENGTH = 8192
MAX_PATH_DEPTH = 50
MAX_BATCH_SIZE = 50

ALLOWED_EXTENSIONS = {
    ".py",
    ".js",
    ".ts",
    ".jsx",
    ".tsx",
    ".mjs",
    ".cjs",
    ".css",
    ".html",
    ".json",
    ".yaml",
    ".yml",
    ".toml",
    ".ini",
    ".cfg",
    ".txt",
    ".md",
    ".rs",
    ".go",
    ".java",
    ".rb",
    ".php",
    ".sh",
    ".bash",
    ".zsh",
    ".dockerfile",
    "",  # allow extensionless files (e.g. Dockerfile, Makefile)
}
