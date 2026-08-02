// PURPOSE: SetupOrchestrator — orchestrates project initialization and setup operations
//
// Delegates all operations to ISetupManagementProtocol (capabilities layer).
// This is a thin agent layer that passes through aggregate contract calls.
//
// Key operations:
//   - MCP config generation for different AI clients (Claude, Hermes, VS Code)
//   - .env file generation for JS/TS IDE integration
//   - Adapter installation (pip for Python, npm for JS)
//   - Language detection and config template loading
//   - Config file writing and XDG config dir creation

use shared::cli_commands::taxonomy_protocol_vo::{TransportProtocol, TransportUrlVO};
use shared::common::taxonomy_job_vo::{EnvContentVO, McpConfigVO, SuccessStatus};
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::{
    ISetupManagementProtocol, ProjectLanguageVO, ProjectLanguagesVO, SetupManagementAggregate,
};

use std::collections::HashMap;
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

    fn generate_env(&self, transport: &TransportProtocol, _home: &DirectoryPath) -> EnvContentVO {
        EnvContentVO::new(format!("TRANSPORT={}\n", transport))
    }

    fn generate_mcp_config(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        McpConfigVO::new(config)
    }

    fn mcp_config_claude(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        config.insert("client".to_string(), serde_json::json!("claude"));
        McpConfigVO::new(config)
    }

    fn mcp_config_hermes(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        config.insert("client".to_string(), serde_json::json!("hermes"));
        McpConfigVO::new(config)
    }

    fn mcp_config_vscode(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        config.insert("client".to_string(), serde_json::json!("vscode"));
        McpConfigVO::new(config)
    }

    fn install_python_adapters(&self) -> SuccessStatus {
        self.protocol.install_python_adapters()
    }

    fn install_javascript_adapters(&self, sudo: bool) -> SuccessStatus {
        self.protocol.install_javascript_adapters(sudo)
    }

    fn detect_language(&self) -> ProjectLanguageVO {
        self.protocol.detect_language()
    }

    fn detect_languages(&self) -> ProjectLanguagesVO {
        self.protocol.detect_languages()
    }

    fn get_config_template(&self, language: &str) -> &'static str {
        self.protocol.get_config_template(language)
    }

    fn write_config_file(
        &self,
        filename: &str,
        content: &str,
    ) -> shared::project_setup::taxonomy_setup_contract_vo::WriteConfigResult {
        self.protocol.write_config_file(filename, content)
    }

    fn create_global_config_dir(
        &self,
    ) -> shared::project_setup::taxonomy_setup_contract_vo::CreateConfigDirResult {
        self.protocol.create_global_config_dir()
    }

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
