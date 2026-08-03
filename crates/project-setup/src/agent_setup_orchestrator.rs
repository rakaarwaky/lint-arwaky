// PURPOSE: SetupOrchestrator — orchestrates project initialization and setup operations
//
// Delegates all operations to ISetupManagementProtocol (capabilities layer).
// This is a thin agent layer that passes through aggregate contract calls.
//
// Key operations:
//   - MCP config generation for different AI clients (Claude, Cursor, Windsurf, Copilot, Hermes, VS Code, All)
//   - .env file generation for JS/TS IDE integration
//   - Adapter installation (pip for Python, npm for JS)
//   - Language detection and config template loading
//   - Config file writing and XDG config dir creation
//   - Pre-flight checks for package manager availability

use shared::cli_commands::taxonomy_protocol_vo::{TransportProtocol, TransportUrlVO};
use shared::common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::contract_setup_protocol::PreFlightResult;
use shared::project_setup::{
    ISetupManagementProtocol, ProjectLanguageVO, ProjectLanguagesVO, SetupError,
    SetupManagementAggregate,
};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SetupManagementOrchestrator {
    protocol: Arc<dyn ISetupManagementProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl SetupManagementAggregate for SetupManagementOrchestrator {
    fn check_http(&self, _url: &TransportUrlVO) -> SuccessStatus {
        SuccessStatus::new(true)
    }

    /// Delegate to protocol (generate_env ignores transport, uses home only per FR-002).
    fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO {
        self.protocol.generate_env(home)
    }

    /// Delegate to protocol.
    fn generate_mcp_config(&self) -> McpConfigVO {
        self.protocol.generate_mcp_config()
    }

    /// Delegate to protocol.
    fn mcp_config_claude(&self) -> McpConfigVO {
        self.protocol.mcp_config_claude()
    }

    /// Delegate to protocol.
    fn mcp_config_cursor(&self) -> McpConfigVO {
        self.protocol.mcp_config_cursor()
    }

    /// Delegate to protocol.
    fn mcp_config_windsurf(&self) -> McpConfigVO {
        self.protocol.mcp_config_windsurf()
    }

    /// Delegate to protocol.
    fn mcp_config_copilot(&self) -> McpConfigVO {
        self.protocol.mcp_config_copilot()
    }

    /// Delegate to protocol.
    fn mcp_config_hermes(&self) -> McpConfigVO {
        self.protocol.mcp_config_hermes()
    }

    /// Delegate to protocol.
    fn mcp_config_vscode(&self) -> McpConfigVO {
        self.protocol.mcp_config_vscode()
    }

    /// Delegate to protocol.
    fn mcp_config_all(&self) -> McpConfigVO {
        self.protocol.mcp_config_all()
    }

    /// Delegate to protocol.
    fn install_python_adapters(&self) -> SuccessStatus {
        self.protocol.install_python_adapters()
    }

    /// Delegate to protocol.
    fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus {
        self.protocol.install_javascript_adapters(sudo)
    }

    /// Delegate to protocol.
    fn detect_language(&self) -> Option<ProjectLanguageVO> {
        self.protocol.detect_language()
    }

    /// Delegate to protocol.
    fn detect_languages(&self) -> ProjectLanguagesVO {
        self.protocol.detect_languages()
    }

    /// Delegate to protocol.
    fn get_config_template(&self, language: &str) -> Result<&'static str, SetupError> {
        self.protocol.get_config_template(language)
    }

    /// Delegate to protocol (FR-007).
    fn pre_flight_check(&self) -> PreFlightResult {
        self.protocol.pre_flight_check()
    }

    /// Delegate to protocol.
    fn write_config_file(
        &self,
        filename: &str,
        content: &str,
    ) -> shared::project_setup::taxonomy_setup_contract_vo::WriteConfigResult {
        self.protocol.write_config_file(filename, content)
    }

    /// Delegate to protocol.
    fn create_global_config_dir(
        &self,
    ) -> shared::project_setup::taxonomy_setup_contract_vo::CreateConfigDirResult {
        self.protocol.create_global_config_dir()
    }

    /// Delegate to protocol.
    fn file_exists(&self, path: &str) -> bool {
        self.protocol.file_exists(path)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl SetupManagementOrchestrator {
    pub fn new(protocol: Arc<dyn ISetupManagementProtocol>) -> Self {
        Self { protocol }
    }
}
