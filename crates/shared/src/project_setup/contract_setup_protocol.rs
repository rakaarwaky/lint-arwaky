// PURPOSE: setup-domain capability contracts (AES102 `_protocol`).
//
// One trait per setup FR: each trait is a capability seam carrying the
// methods of a single requirement, each with one concrete return type, so a
// capability implements its trait outright and never carries stubs.

use crate::taxonomy_project_setup_vo::EmbeddedSkillVO;
use crate::taxonomy_project_setup_vo::SetupError;
pub use crate::taxonomy_project_setup_vo::{
    CreateConfigDirResult, McpBinaryNameVO, PackageManagerStatus, PreFlightResult,
    ProjectLanguageVO, ProjectLanguagesVO, WriteConfigResult,
};
use shared_common::taxonomy_common_vo::PatternList;
use shared_common::taxonomy_job_vo::EnvContentVO;
use shared_common::taxonomy_job_vo::McpConfigVO;
use shared_common::taxonomy_job_vo::SuccessStatus;
use shared_common::taxonomy_path_vo::DirectoryPath;

pub type InstallPackagesResult = Result<(), SetupError>;

// ── FR-ProjectSetup-001: MCP Configuration Generation ─────────

pub trait IMcpConfigGenerationProtocol: Send + Sync {
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
}

// ── FR-ProjectSetup-002: Environment File Generation ────────────

pub trait IEnvGenerationProtocol: Send + Sync {
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO;
}

// ── FR-ProjectSetup-003: Language Detection ────────────────────

pub trait ILanguageDetectionProtocol: Send + Sync {
    /// Detect the dominant programming language of the current project.
    fn detect_language(&self) -> Option<ProjectLanguageVO>;
    /// Detect ALL languages present in the current project (FR-003).
    /// Returns empty list when no languages found — no default language.
    fn detect_languages(&self) -> ProjectLanguagesVO;
}

// ── FR-ProjectSetup-004: Adapter Installation ──────────────────

pub trait IAdapterInstallationProtocol: Send + Sync {
    fn install_python_packages(&self, packages: &PatternList) -> InstallPackagesResult;
    fn install_npm_packages(&self, packages: &PatternList, sudo: bool) -> InstallPackagesResult;
    /// Install the Python adapter set (`ruff`, `mypy`, `bandit`).
    fn install_python_adapters(&self) -> SuccessStatus;
    /// Install the JavaScript adapter set (`eslint`, `prettier`, `typescript`).
    fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus;
}

// ── FR-ProjectSetup-005: Config Template Loading ────────────────

pub trait IConfigTemplateProtocol: Send + Sync {
    /// Get an embedded config template for the given language (FR-005).
    /// Returns `Err(SetupError::UnknownLanguage)` for unsupported languages.
    fn get_config_template(&self, language: &str) -> Result<&'static str, SetupError>;
    /// Retrieve all embedded skill files compiled into the binary.
    fn get_embedded_skills(&self) -> &'static [EmbeddedSkillVO];
}

// ── FR-ProjectSetup-006: Config File Writing and Config Directory ──

pub trait IConfigWritingProtocol: Send + Sync {
    /// Write a configuration file to disk. Returns a description of the
    /// operation on success, or a structured `SetupError` on failure.
    fn write_config_file(&self, filename: &str, content: &str) -> WriteConfigResult;
    /// Create the global config directory and return its path.
    fn create_global_config_dir(&self) -> CreateConfigDirResult;
}

// ── FR-ProjectSetup-007: Pre-flight Check ──────────────────────

pub trait IPreFlightProtocol: Send + Sync {
    /// Pre-flight check: verify package managers are available (FR-007).
    fn pre_flight_check(&self) -> PreFlightResult;
}

// ── FR-ProjectSetup-008: File Existence Check ──────────────────

pub trait IFilePathExistenceProtocol: Send + Sync {
    fn file_exists(&self, path: &str) -> bool;
}
