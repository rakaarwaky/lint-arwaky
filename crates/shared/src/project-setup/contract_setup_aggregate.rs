// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use crate::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::mcp_server::taxonomy_job_vo::EnvContentVO;
use crate::mcp_server::taxonomy_job_vo::McpConfigVO;
use crate::mcp_server::taxonomy_job_vo::SuccessStatus;
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, ProjectLanguageVO, WriteConfigResult,
};

pub type SetupMgmtProtocol = Box<dyn ISetupManagementProtocol>;

#[async_trait::async_trait]
pub trait SetupManagementAggregate: Send + Sync {
    fn check_http(&self, url: &TransportUrlVO) -> SuccessStatus;
    fn generate_env(&self, transport: &TransportProtocol, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_claude(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_hermes(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_vscode(&self, transport: &TransportProtocol) -> McpConfigVO;
    async fn install_python_adapters(&self) -> SuccessStatus;
    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
    fn detect_language(&self) -> ProjectLanguageVO;
    fn get_config_template(&self, language: &str) -> &'static str;
    fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    fn create_global_config_dir(&self) -> CreateConfigDirResult;
    fn file_exists(&self, path: &str) -> bool;
}
