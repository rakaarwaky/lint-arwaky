// PURPOSE: SetupMcpConfigGenerator — capability for MCP configuration generation
//
// Implements IMcpConfigGenerationProtocol. Generates MCP server configurations
// for 7 client formats (Claude, Cursor, Windsurf, Copilot, Hermes, VS Code, All)
// and resolves the lint-arwaky-mcp binary path.

use shared_common::taxonomy_job_vo::McpConfigVO;
use shared_project_setup::contract_setup_protocol::IMcpConfigGenerationProtocol;
use shared_project_setup::taxonomy_project_setup_vo::McpBinaryNameVO;

use std::collections::HashMap;

// ─── Block 1: Struct Definition ───────────────────────────

/// Business logic for MCP config generation. Delegates auxiliary file operations
/// to the utility layer (`shared_project_setup::utility_project_setup_helpers`).
pub struct SetupMcpConfigGenerator;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IMcpConfigGenerationProtocol for SetupMcpConfigGenerator {
    /// Generate the base mcp.json entry for lint-arwaky.
    /// Uses the resolved binary path and FRD-aligned alwaysAllow list.
    fn generate_mcp_config(&self) -> McpConfigVO {
        let bin = self.which_mcp_binary();
        let mut config = HashMap::new();
        let server_entry = serde_json::json!({
            "command": bin.value(),
            "args": [],
            "alwaysAllow": ["execute_command", "list_commands", "read_skill",
                            "health_check", "get_config"]
        });
        config.insert("lint-arwaky".to_string(), server_entry);
        McpConfigVO::new(config)
    }

    /// Generate Claude Desktop MCP config format.
    fn mcp_config_claude(&self) -> McpConfigVO {
        let base = self.generate_mcp_config();
        let mut config = HashMap::new();
        config.insert("mcpServers".to_string(), serde_json::json!(base.value()));
        McpConfigVO::new(config)
    }

    /// Generate Cursor MCP config format.
    fn mcp_config_cursor(&self) -> McpConfigVO {
        let base = self.generate_mcp_config();
        let mut config = HashMap::new();
        config.insert("mcpServers".to_string(), serde_json::json!(base.value()));
        McpConfigVO::new(config)
    }

    /// Generate Windsurf MCP config format.
    fn mcp_config_windsurf(&self) -> McpConfigVO {
        let base = self.generate_mcp_config();
        let mut config = HashMap::new();
        config.insert("mcpServers".to_string(), serde_json::json!(base.value()));
        McpConfigVO::new(config)
    }

    /// Generate Copilot MCP config format.
    fn mcp_config_copilot(&self) -> McpConfigVO {
        let base = self.generate_mcp_config();
        let mut config = HashMap::new();
        config.insert("mcpServers".to_string(), serde_json::json!(base.value()));
        McpConfigVO::new(config)
    }

    /// Generate Hermes/Antigravity MCP config format (base config directly).
    fn mcp_config_hermes(&self) -> McpConfigVO {
        self.generate_mcp_config()
    }

    /// Generate VS Code MCP config format.
    fn mcp_config_vscode(&self) -> McpConfigVO {
        let base = self.generate_mcp_config();
        let mut config = HashMap::new();
        config.insert(
            "mcp".to_string(),
            serde_json::json!({"servers": base.value()}),
        );
        McpConfigVO::new(config)
    }

    /// Generate MCP configs for all supported clients in one object (FR-001).
    fn mcp_config_all(&self) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "claude-code".to_string(),
            serde_json::json!(self.mcp_config_claude().value()),
        );
        config.insert(
            "cursor".to_string(),
            serde_json::json!(self.mcp_config_cursor().value()),
        );
        config.insert(
            "windsurf".to_string(),
            serde_json::json!(self.mcp_config_windsurf().value()),
        );
        config.insert(
            "copilot".to_string(),
            serde_json::json!(self.mcp_config_copilot().value()),
        );
        config.insert(
            "hermes".to_string(),
            serde_json::json!(self.mcp_config_hermes().value()),
        );
        config.insert(
            "vscode".to_string(),
            serde_json::json!(self.mcp_config_vscode().value()),
        );
        McpConfigVO::new(config)
    }

    /// Resolve the path to the lint-arwaky-mcp binary (FR-001 priority order).
    fn which_mcp_binary(&self) -> McpBinaryNameVO {
        // Priority 1: LINT_ARWAKY_MCP_BIN env var (must point to existing file)
        if let Ok(env_bin) = std::env::var("LINT_ARWAKY_MCP_BIN") {
            if !env_bin.is_empty() && std::path::Path::new(&env_bin).is_file() {
                return McpBinaryNameVO::new(env_bin);
            }
            // Non-file path or empty → fall through to priority 2
        }

        // Priority 2: CARGO_HOME/bin/lint-arwaky-mcp
        let cargo_home = std::env::var("CARGO_HOME").unwrap_or_else(|_| "~/.cargo".to_string());
        let cargo_home_candidate = format!("{}/bin/lint-arwaky-mcp", cargo_home);

        // Priority 3: Bare name (relies on OS PATH resolution at runtime)
        let bare_name = "lint-arwaky-mcp".to_string();

        let candidates = [cargo_home_candidate, bare_name];
        for c in &candidates {
            if !c.is_empty() && std::path::Path::new(c).exists() {
                return McpBinaryNameVO::new(c.clone());
            }
        }

        // Fall back to bare name for runtime PATH resolution
        McpBinaryNameVO::new("lint-arwaky-mcp".to_string())
    }
}

// ─── Block 3: Constructors ────────────────────────────────

impl SetupMcpConfigGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SetupMcpConfigGenerator {
    fn default() -> Self {
        Self::new()
    }
}
