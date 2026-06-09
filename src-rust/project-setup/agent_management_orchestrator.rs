// setup_management_orchestrator — Implementation of SetupManagementAggregate (Agent Logic).
use crate::cli_transport::taxonomy_protocol_vo::TransportProtocol;
use crate::cli_transport::taxonomy_protocol_vo::TransportUrlVO;
use crate::pipeline_jobs::taxonomy_job_vo::EnvContentVO;
use crate::pipeline_jobs::taxonomy_job_vo::McpConfigVO;
use crate::pipeline_jobs::taxonomy_job_vo::SuccessStatus;
use crate::project_setup::contract_management_aggregate::SetupManagementAggregate;
use crate::source_parsing::taxonomy_path_vo::DirectoryPath;
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

impl SetupManagementOrchestrator {
    pub fn new() -> Self {
        Self {}
    }
}
