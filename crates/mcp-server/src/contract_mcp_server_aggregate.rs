// PURPOSE: IMcpServerAggregate — aggregate trait for MCP server operations
// Includes McpToolArgs structs since they require rmcp::schemars::JsonSchema
// (contract layer is allowed to import from external crates).
use rmcp::handler::server::wrapper::Parameters;
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

#[async_trait::async_trait]
pub trait IMcpServerAggregate: Send + Sync {
    async fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String;
    async fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String;
    async fn read_skill(&self, args: Parameters<ReadSkillArgs>) -> String;
}
