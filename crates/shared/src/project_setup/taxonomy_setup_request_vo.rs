// PURPOSE: SetupRequest/SetupResponse — request/response VOs for the setup aggregate
use crate::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use crate::common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use crate::common::taxonomy_path_vo::DirectoryPath;
use crate::project_setup::taxonomy_setup_contract_vo::{
    CreateConfigDirResult, PreFlightResult, ProjectLanguageVO, ProjectLanguagesVO, SetupError,
    WriteConfigResult,
};
use crate::project_setup::taxonomy_skills_vo::EmbeddedSkillVO;

/// Consumer verb carried by the setup aggregate's single entry point.
pub enum SetupRequest {
    /// Verify that a transport URL is reachable.
    CheckHttp {
        url: TransportUrlVO,
    },
    /// Generate the .env file for a target home directory.
    GenerateEnv {
        home: DirectoryPath,
    },
    /// Generate the default MCP config.
    GenerateMcpConfig,
    /// Generate MCP config for a specific client.
    McpConfigClaude,
    McpConfigCursor,
    McpConfigWindsurf,
    McpConfigCopilot,
    McpConfigHermes,
    McpConfigVscode,
    McpConfigAll,
    /// Install Python external-lint adapters.
    InstallPythonAdapters,
    /// Install JavaScript external-lint adapters.
    InstallJavascriptAdapters {
        sudo: bool,
    },
    /// Detect the primary project language.
    DetectLanguage,
    /// Detect all project languages.
    DetectLanguages,
    /// Load the embedded config template for a language.
    GetConfigTemplate {
        language: String,
    },
    /// Check that required package managers are available.
    PreFlightCheck,
    /// Return the embedded skill list.
    GetEmbeddedSkills,
    /// Write a config file to disk.
    WriteConfigFile {
        filename: String,
        content: String,
    },
    /// Create the global XDG config directory.
    CreateGlobalConfigDir,
    /// Check whether a file path exists.
    FileExists {
        path: String,
    },
}

impl SetupRequest {
    pub fn check_http(url: &TransportUrlVO) -> Self {
        Self::CheckHttp { url: url.clone() }
    }
    pub fn generate_env(home: &DirectoryPath) -> Self {
        Self::GenerateEnv { home: home.clone() }
    }
    pub fn generate_mcp_config() -> Self {
        Self::GenerateMcpConfig
    }
    pub fn mcp_config_claude() -> Self {
        Self::McpConfigClaude
    }
    pub fn mcp_config_cursor() -> Self {
        Self::McpConfigCursor
    }
    pub fn mcp_config_windsurf() -> Self {
        Self::McpConfigWindsurf
    }
    pub fn mcp_config_copilot() -> Self {
        Self::McpConfigCopilot
    }
    pub fn mcp_config_hermes() -> Self {
        Self::McpConfigHermes
    }
    pub fn mcp_config_vscode() -> Self {
        Self::McpConfigVscode
    }
    pub fn mcp_config_all() -> Self {
        Self::McpConfigAll
    }
    pub fn install_python_adapters() -> Self {
        Self::InstallPythonAdapters
    }
    pub fn install_javascript_adapters(sudo: bool) -> Self {
        Self::InstallJavascriptAdapters { sudo }
    }
    pub fn detect_language() -> Self {
        Self::DetectLanguage
    }
    pub fn detect_languages() -> Self {
        Self::DetectLanguages
    }
    pub fn get_config_template(language: &str) -> Self {
        Self::GetConfigTemplate {
            language: language.to_string(),
        }
    }
    pub fn pre_flight_check() -> Self {
        Self::PreFlightCheck
    }
    pub fn get_embedded_skills() -> Self {
        Self::GetEmbeddedSkills
    }
    pub fn write_config_file(filename: &str, content: &str) -> Self {
        Self::WriteConfigFile {
            filename: filename.to_string(),
            content: content.to_string(),
        }
    }
    pub fn create_global_config_dir() -> Self {
        Self::CreateGlobalConfigDir
    }
    pub fn file_exists(path: &str) -> Self {
        Self::FileExists {
            path: path.to_string(),
        }
    }
}

/// Result of a setup aggregate request.
pub enum SetupResponse {
    CheckHttp { status: SuccessStatus },
    Env { content: EnvContentVO },
    Mcp { config: McpConfigVO },
    Installed { status: SuccessStatus },
    Language { detected: Option<ProjectLanguageVO> },
    Languages { detected: ProjectLanguagesVO },
    Template { content: Result<String, SetupError> },
    PreFlight { result: PreFlightResult },
    Skills { skills: Vec<EmbeddedSkillVO> },
    ConfigWritten { result: WriteConfigResult },
    ConfigDir { result: CreateConfigDirResult },
    Exists { exists: bool },
}

impl SetupResponse {
    pub fn into_status(self) -> SuccessStatus {
        match self {
            Self::CheckHttp { status } | Self::Installed { status } => status,
            _ => SuccessStatus::default(),
        }
    }

    pub fn into_env(self) -> EnvContentVO {
        match self {
            Self::Env { content } => content,
            _ => EnvContentVO::default(),
        }
    }

    pub fn into_mcp_config(self) -> McpConfigVO {
        match self {
            Self::Mcp { config } => config,
            _ => McpConfigVO::default(),
        }
    }

    pub fn into_language(self) -> Option<ProjectLanguageVO> {
        match self {
            Self::Language { detected } => detected,
            _ => None,
        }
    }

    pub fn into_languages(self) -> ProjectLanguagesVO {
        match self {
            Self::Languages { detected } => detected,
            _ => ProjectLanguagesVO::default(),
        }
    }

    pub fn into_template(self) -> Result<String, SetupError> {
        match self {
            Self::Template { content } => content,
            _ => Err(SetupError::other("no template available")),
        }
    }

    pub fn into_pre_flight(self) -> PreFlightResult {
        match self {
            Self::PreFlight { result } => result,
            _ => Vec::new(),
        }
    }

    pub fn into_skills(self) -> Vec<EmbeddedSkillVO> {
        match self {
            Self::Skills { skills } => skills,
            _ => Vec::new(),
        }
    }

    pub fn into_write_result(self) -> WriteConfigResult {
        match self {
            Self::ConfigWritten { result } => result,
            _ => Err(SetupError::other("no config written")),
        }
    }

    pub fn into_dir_result(self) -> CreateConfigDirResult {
        match self {
            Self::ConfigDir { result } => result,
            _ => Err(SetupError::other("no config dir created")),
        }
    }

    pub fn into_exists(self) -> bool {
        match self {
            Self::Exists { exists } => exists,
            _ => false,
        }
    }
}
