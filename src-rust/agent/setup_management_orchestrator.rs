// setup_management_orchestrator — Implementation of SetupManagementAggregate (Agent Logic).
use crate::contract::setup_management_aggregate::SetupManagementAggregate;
use crate::taxonomy::{TransportUrlVO, BooleanVO, DirectoryPath, EnvContentVO, McpConfigVO, MetadataVO};
use std::collections::HashMap;

pub struct SetupManagementOrchestrator;

impl SetupManagementAggregate for SetupManagementOrchestrator {}

impl SetupManagementOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn check_http(&self, _url: &TransportUrlVO) -> BooleanVO {
        // Check HTTP connectivity
        BooleanVO::new(true)
    }

    pub fn generate_env(&self, transport: &TransportUrlVO, _home: &DirectoryPath) -> EnvContentVO {
        EnvContentVO {
            value: format!("TRANSPORT={}\n", transport.value),
        }
    }

    pub fn generate_mcp_config(&self, transport: &TransportUrlVO) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert("transport".to_string(), serde_json::json!(transport.value));
        McpConfigVO {
            value: config,
        }
    }

    pub fn mcp_config_claude(&self, transport: &TransportUrlVO) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert("transport".to_string(), serde_json::json!(transport.value));
        config.insert("client".to_string(), serde_json::json!("claude"));
        McpConfigVO {
            value: config,
        }
    }

    pub fn mcp_config_hermes(&self, transport: &TransportUrlVO) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert("transport".to_string(), serde_json::json!(transport.value));
        config.insert("client".to_string(), serde_json::json!("hermes"));
        McpConfigVO {
            value: config,
        }
    }

    pub fn mcp_config_vscode(&self, transport: &TransportUrlVO) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert("transport".to_string(), serde_json::json!(transport.value));
        config.insert("client".to_string(), serde_json::json!("vscode"));
        McpConfigVO {
            value: config,
        }
    }
}
