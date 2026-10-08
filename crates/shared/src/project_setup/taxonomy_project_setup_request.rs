// PURPOSE: SetupRequest — request payload for the setup aggregate

use shared_cli_commands::taxonomy_cli_commands_vo::TransportUrlVO;
use shared_common::taxonomy_path_vo::DirectoryPath;

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
