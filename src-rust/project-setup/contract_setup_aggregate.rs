// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::cli_transport::taxonomy_protocol_vo::TransportProtocol;
use crate::cli_transport::taxonomy_protocol_vo::TransportUrlVO;
use crate::pipeline_jobs::taxonomy_job_vo::EnvContentVO;
use crate::pipeline_jobs::taxonomy_job_vo::McpConfigVO;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
use async_trait::async_trait;

pub type SetupMgmtProtocol = Box<dyn ISetupManagementProtocol>;

#[async_trait]
pub trait SetupManagementAggregate: Send + Sync {
    fn check_http(&self, url: &TransportUrlVO) -> SuccessStatus;
    fn generate_env(&self, transport: &TransportProtocol, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_claude(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_hermes(&self, transport: &TransportProtocol) -> McpConfigVO;
    fn mcp_config_vscode(&self, transport: &TransportProtocol) -> McpConfigVO;
}
