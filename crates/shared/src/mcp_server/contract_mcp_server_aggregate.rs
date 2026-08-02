// PURPOSE: IMcpServerAggregate — aggregate trait for MCP server operations
use rmcp::handler::server::wrapper::Parameters;

use crate::mcp_server::taxonomy_mcp_tool_args_vo::{
    ExecuteCommandArgs, GetConfigArgs, ListCommandsArgs, ReadSkillArgs,
};

pub trait IMcpServerAggregate: Send + Sync {
    fn execute_command(&self, args: Parameters<ExecuteCommandArgs>) -> String;
    fn health_check(&self) -> String;
    fn list_commands(&self, args: Parameters<ListCommandsArgs>) -> String;
    fn read_skill(&self, args: Parameters<ReadSkillArgs>) -> String;
    fn get_config(&self, args: Parameters<GetConfigArgs>) -> String;
}
