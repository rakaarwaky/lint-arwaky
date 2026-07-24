// PURPOSE: McpToolArgs — typed argument structs for MCP tools with JsonSchema
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
// ─── Block 1: Struct Definition ───────────────────────────
pub struct ExecuteCommandArgs {
    /// The command action to execute (e.g. "scan", "check", "security", "doctor")
    pub action: String,
    /// Additional arguments. For scan/check: {"path": "/absolute/path/to/project"}.
    /// Path MUST be absolute — relative paths resolve from MCP server cwd, not your project.
    pub args: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListCommandsArgs {
    /// Optional domain filter
    pub domain: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReadSkillArgs {
    /// Section to read from SKILL.md
    pub section: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetConfigArgs {
    /// Optional project path to inspect config for
    pub path: Option<String>,
    /// Optional language hint (rust, python, javascript)
    pub language: Option<String>,
}
