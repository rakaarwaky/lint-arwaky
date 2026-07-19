// PURPOSE: McpToolArgs — typed argument structs for MCP tools with JsonSchema
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ExecuteCommandArgs {
    /// The command action to execute
    pub action: String,
    /// Additional arguments for the command
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
