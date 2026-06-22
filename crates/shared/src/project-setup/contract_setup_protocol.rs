// PURPOSE: ISetupProtocol — protocol trait for project setup step definitions

use crate::mcp_server::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;

#[async_trait::async_trait]
pub trait ISetupManagementProtocol: Send + Sync {
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self) -> McpConfigVO;
    fn mcp_config_claude(&self) -> McpConfigVO;
    fn mcp_config_hermes(&self) -> McpConfigVO;
    fn mcp_config_vscode(&self) -> McpConfigVO;
    fn which_mcp_binary(&self) -> String;
    async fn install_python_adapters(&self) -> SuccessStatus;
    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
}

#[async_trait::async_trait]
pub trait ISetupInstallerPort: Send + Sync {
    async fn install_python_packages(&self, packages: &[String]) -> Result<(), String>;
    async fn install_npm_packages(&self, packages: &[String], sudo: bool) -> Result<(), String>;
}
