use crate::taxonomy::{DirectoryPath, EnvContentVO, McpConfigVO};
use async_trait::async_trait;

#[async_trait]
pub trait SetupManagementAggregate: Send + Sync {
    fn check_http(&self, url: &str) -> bool;
    fn generate_env(&self, transport: &str, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self, transport: &str) -> McpConfigVO;
    fn mcp_config_claude(&self, transport: &str) -> McpConfigVO;
    fn mcp_config_hermes(&self, transport: &str) -> McpConfigVO;
    fn mcp_config_vscode(&self, transport: &str) -> McpConfigVO;
}
