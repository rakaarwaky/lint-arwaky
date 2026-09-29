// PURPOSE: SetupOrchestrator — orchestrates project initialization and setup operations
//
// Delegates to the setup capability protocols (capabilities layer). The
// orchestrator holds one Arc<dyn ...> per protocol seam it calls.
// This is a thin agent layer that passes through aggregate contract calls.
//
// Key operations:
//   - MCP config generation for different AI clients (Claude, Cursor, Windsurf, Copilot, Hermes, VS Code, All)
//   - .env file generation for JS/TS IDE integration
//   - Adapter installation (pip for Python, npm for JS)
//   - Language detection and config template loading
//   - Config file writing and XDG config dir creation
//   - Pre-flight checks for package manager availability

use shared::cli_commands::taxonomy_cli_commands_vo::TransportUrlVO;
use shared::common::taxonomy_job_vo::EnvContentVO;
use shared::common::taxonomy_job_vo::McpConfigVO;
use shared::common::taxonomy_job_vo::SuccessStatus;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::contract_setup_aggregate::ISetupAggregate;
use shared::project_setup::contract_setup_protocol::PreFlightResult;
use shared::project_setup::taxonomy_project_setup_request::SetupRequest;
use shared::project_setup::taxonomy_project_setup_response::SetupResponse;
use shared::project_setup::{
    EmbeddedSkillVO, IAdapterInstallationProtocol, IConfigTemplateProtocol, IConfigWritingProtocol,
    IEnvGenerationProtocol, IFilePathExistenceProtocol, ILanguageDetectionProtocol,
    IMcpConfigGenerationProtocol, IPreFlightProtocol, ProjectLanguageVO, ProjectLanguagesVO,
    SetupError,
};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

/// Collaborators for SetupManagementOrchestrator, grouped to keep the
/// constructor below the clippy argument-count threshold.
pub struct SetupProtocols {
    pub mcp_config: Arc<dyn IMcpConfigGenerationProtocol>,
    pub env_generation: Arc<dyn IEnvGenerationProtocol>,
    pub language_detection: Arc<dyn ILanguageDetectionProtocol>,
    pub adapter_installation: Arc<dyn IAdapterInstallationProtocol>,
    pub config_template: Arc<dyn IConfigTemplateProtocol>,
    pub config_writing: Arc<dyn IConfigWritingProtocol>,
    pub pre_flight: Arc<dyn IPreFlightProtocol>,
    pub path_existence: Arc<dyn IFilePathExistenceProtocol>,
}

pub struct SetupManagementOrchestrator {
    protocols: SetupProtocols,
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
    pub fn new(protocols: SetupProtocols) -> Self {
        Self { protocols }
    }

    pub fn check_http(&self, _url: &TransportUrlVO) -> SuccessStatus {
        SuccessStatus::new(true)
    }

    /// Delegate to protocol (generate_env ignores transport, uses home only per FR-002).
    pub fn generate_env(&self, home: &DirectoryPath) -> EnvContentVO {
        self.protocols.env_generation.generate_env(home)
    }

    pub fn generate_mcp_config(&self) -> McpConfigVO {
        self.protocols.mcp_config.generate_mcp_config()
    }

    pub fn mcp_config_claude(&self) -> McpConfigVO {
        self.protocols.mcp_config.mcp_config_claude()
    }

    pub fn mcp_config_cursor(&self) -> McpConfigVO {
        self.protocols.mcp_config.mcp_config_cursor()
    }

    pub fn mcp_config_windsurf(&self) -> McpConfigVO {
        self.protocols.mcp_config.mcp_config_windsurf()
    }

    pub fn mcp_config_copilot(&self) -> McpConfigVO {
        self.protocols.mcp_config.mcp_config_copilot()
    }

    pub fn mcp_config_hermes(&self) -> McpConfigVO {
        self.protocols.mcp_config.mcp_config_hermes()
    }

    pub fn mcp_config_vscode(&self) -> McpConfigVO {
        self.protocols.mcp_config.mcp_config_vscode()
    }

    pub fn mcp_config_all(&self) -> McpConfigVO {
        self.protocols.mcp_config.mcp_config_all()
    }

    pub fn install_python_adapters(&self) -> SuccessStatus {
        self.protocols
            .adapter_installation
            .install_python_adapters()
    }

    pub fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus {
        self.protocols
            .adapter_installation
            .install_javascript_adapters(sudo)
    }

    pub fn detect_language(&self) -> Option<ProjectLanguageVO> {
        self.protocols.language_detection.detect_language()
    }

    pub fn detect_languages(&self) -> ProjectLanguagesVO {
        self.protocols.language_detection.detect_languages()
    }

    pub fn get_config_template(&self, language: &str) -> Result<&'static str, SetupError> {
        self.protocols.config_template.get_config_template(language)
    }

    pub fn pre_flight_check(&self) -> PreFlightResult {
        self.protocols.pre_flight.pre_flight_check()
    }

    pub fn get_embedded_skills(&self) -> &'static [EmbeddedSkillVO] {
        self.protocols.config_template.get_embedded_skills()
    }

    pub fn write_config_file(
        &self,
        filename: &str,
        content: &str,
    ) -> shared::project_setup::taxonomy_project_setup_vo::WriteConfigResult {
        self.protocols
            .config_writing
            .write_config_file(filename, content)
    }

    pub fn create_global_config_dir(
        &self,
    ) -> shared::project_setup::taxonomy_project_setup_vo::CreateConfigDirResult {
        self.protocols.config_writing.create_global_config_dir()
    }

    pub fn file_exists(&self, path: &str) -> bool {
        self.protocols.path_existence.file_exists(path)
    }
}
