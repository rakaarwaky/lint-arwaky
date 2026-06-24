// PURPOSE: IMcpServerAggregate — aggregate trait for MCP server operations
use crate::taxonomy_mcp_tool_args_vo::{ExecuteCommandArgs, ListCommandsArgs, ReadSkillArgs};
use rmcp::handler::server::wrapper::Parameters;

#[async_trait::async_trait]
pub trait IMcpServerAggregate: Send + Sync {
    async fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String;
    async fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String;
    async fn read_skill(&self, args: Parameters<ReadSkillArgs>) -> String;
}
