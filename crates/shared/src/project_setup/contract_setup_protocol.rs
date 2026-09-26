// PURPOSE: setup-domain capability contracts (AES102 `_protocol`).
//
// One file for the setup feature. Each trait below is one capability
// seam: a trait carries every method that capability implements, with one
// concrete return type each, so a capability implements its trait outright
// and never carries unimplemented stubs.

use crate::common::taxonomy_common_vo::PatternList;
use crate::common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::project_setup::taxonomy_setup_contract_vo::SetupError;
pub use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, McpBinaryNameVO, PackageManagerStatus, PreFlightResult,
    ProjectLanguageVO, ProjectLanguagesVO, WriteConfigResult,
};
use crate::project_setup::taxonomy_skills_vo::EmbeddedSkillVO;

pub type InstallPackagesResult = Result<(), SetupError>;

pub trait ISetupInstallerProtocol: Send + Sync {
    fn install_python_packages(&self, packages: &PatternList) -> InstallPackagesResult;
    fn install_npm_packages(&self, packages: &PatternList, sudo: bool) -> InstallPackagesResult;
}

pub trait ISetupManagementProtocol: Send + Sync {
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO;
    fn generate_mcp_config(&self) -> McpConfigVO;
    fn mcp_config_claude(&self) -> McpConfigVO;
    fn mcp_config_cursor(&self) -> McpConfigVO;
    fn mcp_config_windsurf(&self) -> McpConfigVO;
    fn mcp_config_copilot(&self) -> McpConfigVO;
    fn mcp_config_hermes(&self) -> McpConfigVO;
    fn mcp_config_vscode(&self) -> McpConfigVO;
    /// Generate MCP configs for all supported clients (FR-001).
    fn mcp_config_all(&self) -> McpConfigVO;
    /// Resolve the path to the lint-arwaky-mcp binary.
    fn which_mcp_binary(&self) -> McpBinaryNameVO;
    fn install_python_adapters(&self) -> SuccessStatus;
    fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
    /// Detect the dominant programming language of the current project.
    fn detect_language(&self) -> Option<ProjectLanguageVO>;
    /// Detect ALL languages present in the current project (FR-003).
    /// Returns empty list when no languages found — no default language.
    fn detect_languages(&self) -> ProjectLanguagesVO;
    /// Get an embedded config template for the given language (FR-005).
    /// Returns `Err(SetupError::UnknownLanguage)` for unsupported languages.
    fn get_config_template(&self, language: &str) -> Result<&'static str, SetupError>;
    /// Pre-flight check: verify package managers are available (FR-007).
    fn pre_flight_check(&self) -> PreFlightResult;
    /// Retrieve all embedded skill files compiled into the binary.
    fn get_embedded_skills(&self) -> &'static [EmbeddedSkillVO];
    /// Write a configuration file to disk. Returns a description of the
    /// operation on success, or a structured `SetupError` on failure.
    fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    /// Create the global config directory and return its path.
    fn create_global_config_dir(&self) -> CreateConfigDirResult;
    fn file_exists(&self, path: &str) -> bool;
}
