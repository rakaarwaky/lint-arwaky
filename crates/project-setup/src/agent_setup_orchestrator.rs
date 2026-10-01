// PURPOSE: SetupOrchestrator — orchestrates project initialization and setup operations
//
// Delegates to the setup capability protocols (capabilities layer). The
// orchestrator holds one Arc<dyn ...> per protocol seam it calls.
// Auxiliary operations delegate to the utility layer.
//
// Key operations:
//   - MCP config generation for different AI clients (Claude, Cursor,
//     Windsurf, Copilot, Hermes, VS Code, All)
//   - .env file generation for JS/TS IDE integration
//   - Adapter installation (pip for Python, npm for JS)
//   - Language detection

use shared_common::taxonomy_job_vo::SuccessStatus;
use shared_project_setup::contract_setup_aggregate::ISetupAggregate;
use shared_project_setup::taxonomy_project_setup_request::SetupRequest;
use shared_project_setup::taxonomy_project_setup_response::SetupResponse;
use shared_project_setup::{
    IAdapterInstallationProtocol, IEnvGenerationProtocol, ILanguageDetectionProtocol,
    IMcpConfigGenerationProtocol,
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
}

pub struct SetupManagementOrchestrator {
    protocols: SetupProtocols,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl ISetupAggregate for SetupManagementOrchestrator {
    fn execute(&self, request: SetupRequest) -> SetupResponse {
        match request {
            SetupRequest::CheckHttp { .. } => SetupResponse::CheckHttp {
                status: SuccessStatus::new(true),
            },
            SetupRequest::GenerateEnv { home } => SetupResponse::Env {
                content: self.protocols.env_generation.generate_env(&home),
            },
            SetupRequest::GenerateMcpConfig => SetupResponse::Mcp {
                config: self.protocols.mcp_config.generate_mcp_config(),
            },
            SetupRequest::McpConfigClaude => SetupResponse::Mcp {
                config: self.protocols.mcp_config.mcp_config_claude(),
            },
            SetupRequest::McpConfigCursor => SetupResponse::Mcp {
                config: self.protocols.mcp_config.mcp_config_cursor(),
            },
            SetupRequest::McpConfigWindsurf => SetupResponse::Mcp {
                config: self.protocols.mcp_config.mcp_config_windsurf(),
            },
            SetupRequest::McpConfigCopilot => SetupResponse::Mcp {
                config: self.protocols.mcp_config.mcp_config_copilot(),
            },
            SetupRequest::McpConfigHermes => SetupResponse::Mcp {
                config: self.protocols.mcp_config.mcp_config_hermes(),
            },
            SetupRequest::McpConfigVscode => SetupResponse::Mcp {
                config: self.protocols.mcp_config.mcp_config_vscode(),
            },
            SetupRequest::McpConfigAll => SetupResponse::Mcp {
                config: self.protocols.mcp_config.mcp_config_all(),
            },
            SetupRequest::InstallPythonAdapters => SetupResponse::Installed {
                status: self
                    .protocols
                    .adapter_installation
                    .install_python_adapters(),
            },
            SetupRequest::InstallJavascriptAdapters { sudo } => SetupResponse::Installed {
                status: self
                    .protocols
                    .adapter_installation
                    .install_javascript_adapters(sudo),
            },
            SetupRequest::DetectLanguage => SetupResponse::Language {
                detected: self.protocols.language_detection.detect_language(),
            },
            SetupRequest::DetectLanguages => SetupResponse::Languages {
                detected: self.protocols.language_detection.detect_languages(),
            },
            SetupRequest::GetConfigTemplate { language } => {
                let result =
                    shared_project_setup::utility_project_setup_helpers::get_config_template(
                        &language,
                    );
                SetupResponse::Template {
                    content: result.map(|t| t.to_string()),
                }
            }
            SetupRequest::PreFlightCheck => SetupResponse::PreFlight {
                result: shared_project_setup::utility_project_setup_helpers::pre_flight_check(),
            },
            SetupRequest::GetEmbeddedSkills => SetupResponse::Skills {
                skills: shared_project_setup::utility_project_setup_helpers::get_embedded_skills()
                    .to_vec(),
            },
            SetupRequest::WriteConfigFile { filename, content } => SetupResponse::ConfigWritten {
                result: shared_project_setup::utility_project_setup_helpers::write_config_file(
                    &filename, &content,
                ),
            },
            SetupRequest::CreateGlobalConfigDir => SetupResponse::ConfigDir {
                result:
                    shared_project_setup::utility_project_setup_helpers::create_global_config_dir(),
            },
            SetupRequest::FileExists { ref path } => SetupResponse::Exists {
                exists: shared_project_setup::utility_project_setup_helpers::file_exists(path),
            },
        }
    }
}

// ─── Block 3: Constructors ────────────────────────────────

impl SetupManagementOrchestrator {
    pub fn new(protocols: SetupProtocols) -> Self {
        Self { protocols }
    }
}
