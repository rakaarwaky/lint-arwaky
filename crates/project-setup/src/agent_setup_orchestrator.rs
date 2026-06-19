// PURPOSE: SetupOrchestrator — orchestrates project initialization and setup operations
use shared::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use shared::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use shared::mcp_server::taxonomy_job_vo::EnvContentVO;
use shared::mcp_server::taxonomy_job_vo::McpConfigVO;
use shared::mcp_server::taxonomy_job_vo::SuccessStatus;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::source_parsing::taxonomy_path_vo::DirectoryPath;
use std::collections::HashMap;

use async_trait::async_trait;

pub struct SetupManagementOrchestrator {}

#[async_trait]
impl SetupManagementAggregate for SetupManagementOrchestrator {
    fn check_http(&self, _url: &TransportUrlVO) -> SuccessStatus {
        SuccessStatus::new(true)
    }

    fn generate_env(&self, transport: &TransportProtocol, _home: &DirectoryPath) -> EnvContentVO {
        EnvContentVO {
            value: format!("TRANSPORT={}\n", transport),
        }
    }

    fn generate_mcp_config(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        McpConfigVO { value: config }
    }

    fn mcp_config_claude(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        config.insert("client".to_string(), serde_json::json!("claude"));
        McpConfigVO { value: config }
    }

    fn mcp_config_hermes(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        config.insert("client".to_string(), serde_json::json!("hermes"));
        McpConfigVO { value: config }
    }

    fn mcp_config_vscode(&self, transport: &TransportProtocol) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert(
            "transport".to_string(),
            serde_json::json!(transport.to_string()),
        );
        config.insert("client".to_string(), serde_json::json!("vscode"));
        McpConfigVO { value: config }
    }
}

impl Default for SetupManagementOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl SetupManagementOrchestrator {
    pub fn new() -> Self {
        Self {}
    }
}
