// PURPOSE: SetupAggregate — aggregate trait for project setup orchestration
use crate::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use crate::common::taxonomy_job_vo::EnvContentVO;
use crate::common::taxonomy_job_vo::McpConfigVO;
use crate::common::taxonomy_job_vo::SuccessStatus;
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::project_setup::contract_setup_protocol::ISetupManagementProtocol;
use crate::project_setup::contract_setup_protocol::PreFlightResult;
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, ProjectLanguageVO, ProjectLanguagesVO, SetupError, WriteConfigResult,
};

pub type SetupMgmtProtocol = Box<dyn ISetupManagementProtocol>;

pub trait SetupManagementAggregate: Send + Sync {
    fn check_http(&self, url: &TransportUrlVO) -> SuccessStatus;
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self) -> McpConfigVO;
    fn mcp_config_claude(&self) -> McpConfigVO;
    fn mcp_config_cursor(&self) -> McpConfigVO;
    fn mcp_config_windsurf(&self) -> McpConfigVO;
    fn mcp_config_copilot(&self) -> McpConfigVO;
    fn mcp_config_hermes(&self) -> McpConfigVO;
    fn mcp_config_vscode(&self) -> McpConfigVO;
    fn mcp_config_all(&self) -> McpConfigVO;
    fn install_python_adapters(&self) -> SuccessStatus;
    fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
    fn detect_language(&self) -> Option<ProjectLanguageVO>;
    fn detect_languages(&self) -> ProjectLanguagesVO;
    fn get_config_template(&self, language: &str) -> Result<&'static str, SetupError>;
    fn pre_flight_check(&self) -> PreFlightResult;
    fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    fn create_global_config_dir(&self) -> CreateConfigDirResult;
    fn file_exists(&self, path: &str) -> bool;
}
