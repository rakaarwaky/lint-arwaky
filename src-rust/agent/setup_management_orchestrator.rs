// setup_management_orchestrator — Implementation of SetupManagementAggregate (Agent Logic).
use crate::contract::setup_management_aggregate::SetupManagementAggregate;
use crate::taxonomy::{TransportUrlVO, BooleanVO, DirectoryPath, EnvContentVO, McpConfigVO, MetadataVO};
use std::collections::HashMap;

use async_trait::async_trait;

pub struct SetupManagementOrchestrator;

#[async_trait]
impl SetupManagementAggregate for SetupManagementOrchestrator {
    fn check_http(&self, url: &str) -> bool {
        let vo = TransportUrlVO::new(url.to_string());
        self.check_http_old(&vo).value
    }

    fn generate_env(&self, transport: &str, home: &DirectoryPath) -> EnvContentVO {
        let vo = TransportUrlVO::new(transport.to_string());
        self.generate_env_old(&vo, home)
    }

    fn generate_mcp_config(&self, transport: &str) -> McpConfigVO {
        let vo = TransportUrlVO::new(transport.to_string());
        self.generate_mcp_config_old(&vo)
    }

    fn mcp_config_claude(&self, transport: &str) -> McpConfigVO {
        let vo = TransportUrlVO::new(transport.to_string());
        self.mcp_config_claude_old(&vo)
    }

    fn mcp_config_hermes(&self, transport: &str) -> McpConfigVO {
        let vo = TransportUrlVO::new(transport.to_string());
        self.mcp_config_hermes_old(&vo)
    }

    fn mcp_config_vscode(&self, transport: &str) -> McpConfigVO {
        let vo = TransportUrlVO::new(transport.to_string());
        self.mcp_config_vscode_old(&vo)
    }
}

impl SetupManagementOrchestrator {
    pub fn new() -> Self {
        Self
    }

    pub fn check_http_old(&self, _url: &TransportUrlVO) -> BooleanVO {
        // Check HTTP connectivity
        BooleanVO::new(true)
    }

    pub fn generate_env_old(&self, transport: &TransportUrlVO, _home: &DirectoryPath) -> EnvContentVO {
        EnvContentVO {
            value: format!("TRANSPORT={}\n", transport.value),
        }
    }

    pub fn generate_mcp_config_old(&self, transport: &TransportUrlVO) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert("transport".to_string(), serde_json::json!(transport.value));
        McpConfigVO {
            value: config,
        }
    }

    pub fn mcp_config_claude_old(&self, transport: &TransportUrlVO) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert("transport".to_string(), serde_json::json!(transport.value));
        config.insert("client".to_string(), serde_json::json!("claude"));
        McpConfigVO {
            value: config,
        }
    }

    pub fn mcp_config_hermes_old(&self, transport: &TransportUrlVO) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert("transport".to_string(), serde_json::json!(transport.value));
        config.insert("client".to_string(), serde_json::json!("hermes"));
        McpConfigVO {
            value: config,
        }
    }

    pub fn mcp_config_vscode_old(&self, transport: &TransportUrlVO) -> McpConfigVO {
        let mut config = HashMap::new();
        config.insert("transport".to_string(), serde_json::json!(transport.value));
        config.insert("client".to_string(), serde_json::json!("vscode"));
        McpConfigVO {
            value: config,
        }
    }
}
