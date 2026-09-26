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

use shared::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use shared::common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::contract_setup_aggregate::ISetupAggregate;
use shared::project_setup::contract_setup_protocol::PreFlightResult;
use shared::project_setup::taxonomy_setup_request_vo::{SetupRequest, SetupResponse};
use shared::project_setup::{
    EmbeddedSkillVO, ISetupManagementProtocol, ProjectLanguageVO, ProjectLanguagesVO, SetupError,
};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SetupManagementOrchestrator {
    protocol: Arc<dyn ISetupManagementProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl ISetupAggregate for SetupManagementOrchestrator {
    fn execute(&self, request: SetupRequest) -> SetupResponse {
        match request {
            SetupRequest::CheckHttp { url } => SetupResponse::CheckHttp {
                status: self.check_http(&url),
            },
            SetupRequest::GenerateEnv { home } => SetupResponse::Env {
                content: self.generate_env(&home),
            },
            SetupRequest::GenerateMcpConfig => SetupResponse::Mcp {
                config: self.generate_mcp_config(),
            },
            SetupRequest::McpConfigClaude => SetupResponse::Mcp {
                config: self.mcp_config_claude(),
            },
            SetupRequest::McpConfigCursor => SetupResponse::Mcp {
                config: self.mcp_config_cursor(),
            },
            SetupRequest::McpConfigWindsurf => SetupResponse::Mcp {
                config: self.mcp_config_windsurf(),
            },
            SetupRequest::McpConfigCopilot => SetupResponse::Mcp {
                config: self.mcp_config_copilot(),
            },
            SetupRequest::McpConfigHermes => SetupResponse::Mcp {
                config: self.mcp_config_hermes(),
            },
            SetupRequest::McpConfigVscode => SetupResponse::Mcp {
                config: self.mcp_config_vscode(),
            },
            SetupRequest::McpConfigAll => SetupResponse::Mcp {
                config: self.mcp_config_all(),
            },
            SetupRequest::InstallPythonAdapters => SetupResponse::Installed {
                status: self.install_python_adapters(),
            },
            SetupRequest::InstallJavascriptAdapters { sudo } => SetupResponse::Installed {
                status: self.install_javascript_adapters(sudo),
            },
            SetupRequest::DetectLanguage => SetupResponse::Language {
                detected: self.detect_language(),
            },
            SetupRequest::DetectLanguages => SetupResponse::Languages {
                detected: self.detect_languages(),
            },
            SetupRequest::GetConfigTemplate { language } => SetupResponse::Template {
                content: self.get_config_template(&language).map(|t| t.to_string()),
            },
            SetupRequest::PreFlightCheck => SetupResponse::PreFlight {
                result: self.pre_flight_check(),
            },
            SetupRequest::GetEmbeddedSkills => SetupResponse::Skills {
                skills: self.get_embedded_skills().to_vec(),
            },
            SetupRequest::WriteConfigFile { filename, content } => SetupResponse::ConfigWritten {
                result: self.write_config_file(&filename, &content),
            },
            SetupRequest::CreateGlobalConfigDir => SetupResponse::ConfigDir {
                result: self.create_global_config_dir(),
            },
            SetupRequest::FileExists { path } => SetupResponse::Exists {
                exists: self.file_exists(&path),
            },
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl SetupManagementOrchestrator {
    pub fn new(protocol: Arc<dyn ISetupManagementProtocol>) -> Self {
        Self { protocol }
    }

    pub fn check_http(&self, _url: &TransportUrlVO) -> SuccessStatus {
        SuccessStatus::new(true)
    }

    /// Delegate to protocol (generate_env ignores transport, uses home only per FR-002).
    pub fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO {
        self.protocol.generate_env(home)
    }

    pub fn generate_mcp_config(&self) -> McpConfigVO {
        self.protocol.generate_mcp_config()
    }

    pub fn mcp_config_claude(&self) -> McpConfigVO {
        self.protocol.mcp_config_claude()
    }

    pub fn mcp_config_cursor(&self) -> McpConfigVO {
        self.protocol.mcp_config_cursor()
    }

    pub fn mcp_config_windsurf(&self) -> McpConfigVO {
        self.protocol.mcp_config_windsurf()
    }

    pub fn mcp_config_copilot(&self) -> McpConfigVO {
        self.protocol.mcp_config_copilot()
    }

    pub fn mcp_config_hermes(&self) -> McpConfigVO {
        self.protocol.mcp_config_hermes()
    }

    pub fn mcp_config_vscode(&self) -> McpConfigVO {
        self.protocol.mcp_config_vscode()
    }

    pub fn mcp_config_all(&self) -> McpConfigVO {
        self.protocol.mcp_config_all()
    }

    pub fn install_python_adapters(&self) -> SuccessStatus {
        self.protocol.install_python_adapters()
    }

    pub fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus {
        self.protocol.install_javascript_adapters(sudo)
    }

    pub fn detect_language(&self) -> Option<ProjectLanguageVO> {
        self.protocol.detect_language()
    }

    pub fn detect_languages(&self) -> ProjectLanguagesVO {
        self.protocol.detect_languages()
    }

    pub fn get_config_template(&self, language: &str) -> Result<&'static str, SetupError> {
        self.protocol.get_config_template(language)
    }

    pub fn pre_flight_check(&self) -> PreFlightResult {
        self.protocol.pre_flight_check()
    }

    pub fn get_embedded_skills(&self) -> &'static [EmbeddedSkillVO] {
        self.protocol.get_embedded_skills()
    }

    pub fn write_config_file(
        &self,
        filename: &str,
        content: &str,
    ) -> shared::project_setup::taxonomy_setup_contract_vo::WriteConfigResult {
        self.protocol.write_config_file(filename, content)
    }

    pub fn create_global_config_dir(
        &self,
    ) -> shared::project_setup::taxonomy_setup_contract_vo::CreateConfigDirResult {
        self.protocol.create_global_config_dir()
    }

    pub fn file_exists(&self, path: &str) -> bool {
        self.protocol.file_exists(path)
    }
}
