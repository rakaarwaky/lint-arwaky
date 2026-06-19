// PURPOSE: ISetupProtocol — protocol trait for project setup step definitions

use crate::mcp_server::taxonomy_job_vo::EnvContentVO;
use crate::mcp_server::taxonomy_job_vo::McpConfigVO;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

pub trait ISetupManagementProtocol: Send + Sync {
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self) -> McpConfigVO;
    fn mcp_config_claude(&self) -> McpConfigVO;
    fn mcp_config_hermes(&self) -> McpConfigVO;
    fn mcp_config_vscode(&self) -> McpConfigVO;
    fn which_mcp_binary(&self) -> String;
}
