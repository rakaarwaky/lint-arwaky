// PURPOSE: ISetupProtocol — protocol trait for project setup step definitions
// AES402: All primitive `String` / `Result<(), String>` / `Result<PathBuf, String>`
// return types in ISetupManagementProtocol are replaced with strongly-typed VOs.
//   * `String` returns → `McpBinaryNameVO` / `ProjectLanguageVO`
//   * `Result<(), String>` → `WriteConfigResult` (= `Result<DescriptionVO, SetupError>`)
//   * `Result<PathBuf, String>` → `CreateConfigDirResult` (= `Result<PathBuf, SetupError>`)
//   * `&str` parameters → kept (idiomatic borrow, AES402 allows)
//   * `bool` parameters → kept (semantic toggle, AES402 allows)
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::mcp_server::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, McpBinaryNameVO, ProjectLanguageVO, ProjectLanguagesVO, SetupError,
    WriteConfigResult,
};

#[async_trait::async_trait]
pub trait ISetupManagementProtocol: Send + Sync {
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self) -> McpConfigVO;
    fn mcp_config_claude(&self) -> McpConfigVO;
    fn mcp_config_hermes(&self) -> McpConfigVO;
    fn mcp_config_vscode(&self) -> McpConfigVO;
    /// Resolve the name of the MCP binary on the host PATH.
    fn which_mcp_binary(&self) -> McpBinaryNameVO;
    async fn install_python_adapters(&self) -> SuccessStatus;
    async fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
    /// Detect the dominant programming language of the current project.
    async fn detect_language(&self) -> ProjectLanguageVO;
    /// Detect ALL languages present in the current project.
    async fn detect_languages(&self) -> ProjectLanguagesVO;
    fn get_config_template(&self, language: &str) -> &'static str;
    /// Write a configuration file to disk. Returns a description of the
    /// operation on success, or a structured `SetupError` on failure.
    async fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    /// Create the global config directory and return its path.
    async fn create_global_config_dir(&self) -> CreateConfigDirResult;
    async fn file_exists(&self, path: &str) -> bool;
}

/// AES402: `Result<(), String>` is replaced with `Result<(), SetupError>`
/// so callers can pattern-match on specific failure modes (Io vs
/// InvalidState vs Other) instead of inspecting free-form error strings.
pub type InstallPackagesResult = Result<(), SetupError>;

#[async_trait::async_trait]
pub trait ISetupInstallerPort: Send + Sync {
    async fn install_python_packages(&self, packages: &[String]) -> InstallPackagesResult;
    async fn install_npm_packages(&self, packages: &[String], sudo: bool) -> InstallPackagesResult;
}
