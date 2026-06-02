pub const MCP_SERVER_VERSION: &str = "1.5.0";
pub const MCP_PROTOCOL_MIN: &str = "2024-11-05";
pub const MCP_PROTOCOL_MAX: &str = "2025-06-18";
pub const AUTO_LINT_VERSION: &str = "1.9.3";

pub const MAX_FILES: usize = 1000;
pub const MAX_PATH_LENGTH: usize = 1024;
pub const MAX_STRING_LENGTH: usize = 8192;
pub const MAX_PATH_DEPTH: usize = 50;
pub const MAX_BATCH_SIZE: usize = 50;

pub fn allowed_extensions() -> std::collections::HashSet<&'static str> {
    let mut set = std::collections::HashSet::new();
    for ext in &[".py", ".js", ".ts", ".jsx", ".tsx", ".mjs", ".cjs", ".css", ".html",
                  ".json", ".yaml", ".yml", ".toml", ".ini", ".cfg", ".txt", ".md", ".rs",
                  ".go", ".java", ".rb", ".php", ".sh", ".bash", ".zsh", ".dockerfile", ""] {
        set.insert(*ext);
    }
    set
}
