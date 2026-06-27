use project_setup_lint_arwaky::agent_setup_orchestrator::SetupManagementOrchestrator;
use shared::cli_commands::taxonomy_protocol_vo::TransportProtocol;
use shared::cli_commands::taxonomy_protocol_vo::TransportUrlVO;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use std::sync::Arc;

struct MockProtocol;

#[async_trait::async_trait]
impl shared::project_setup::contract_setup_protocol::ISetupManagementProtocol for MockProtocol {
    fn generate_env(
        &self,
        _home: &DirectoryPath,
    ) -> shared::mcp_server::taxonomy_job_vo::EnvContentVO {
        shared::mcp_server::taxonomy_job_vo::EnvContentVO::new("MOCK=true\n")
    }
    fn generate_mcp_config(&self) -> shared::mcp_server::taxonomy_job_vo::McpConfigVO {
        let mut config = std::collections::HashMap::new();
        config.insert(
            "lint-arwaky".to_string(),
            serde_json::json!({"command": "mock"}),
        );
        shared::mcp_server::taxonomy_job_vo::McpConfigVO::new(config)
    }
    fn mcp_config_claude(&self) -> shared::mcp_server::taxonomy_job_vo::McpConfigVO {
        self.generate_mcp_config()
    }
    fn mcp_config_hermes(&self) -> shared::mcp_server::taxonomy_job_vo::McpConfigVO {
        self.generate_mcp_config()
    }
    fn mcp_config_vscode(&self) -> shared::mcp_server::taxonomy_job_vo::McpConfigVO {
        self.generate_mcp_config()
    }
    fn which_mcp_binary(
        &self,
    ) -> shared::project_setup::taxonomy_setup_contract_vo::McpBinaryNameVO {
        shared::project_setup::taxonomy_setup_contract_vo::McpBinaryNameVO::new(
            "mock-mcp".to_string(),
        )
    }
    async fn install_python_adapters(&self) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(true)
    }
    async fn install_javascript_adapters(
        &self,
        _sudo: bool,
    ) -> shared::mcp_server::taxonomy_job_vo::SuccessStatus {
        shared::mcp_server::taxonomy_job_vo::SuccessStatus::new(true)
    }
    fn detect_language(
        &self,
    ) -> shared::project_setup::taxonomy_setup_contract_vo::ProjectLanguageVO {
        shared::project_setup::taxonomy_setup_contract_vo::ProjectLanguageVO::new("rust")
    }
    fn get_config_template(&self, _language: &str) -> &'static str {
        "mock template"
    }
    fn write_config_file(
        &self,
        _filename: &str,
        _content: &str,
    ) -> shared::project_setup::WriteConfigResult {
        Ok(shared::taxonomy_suggestion_vo::DescriptionVO::new("ok"))
    }
    fn create_global_config_dir(&self) -> shared::project_setup::CreateConfigDirResult {
        Ok(std::path::PathBuf::from("/tmp/mock"))
    }
    fn file_exists(&self, _path: &str) -> bool {
        true
    }
}

fn make_orchestrator() -> SetupManagementOrchestrator {
    SetupManagementOrchestrator::new(Arc::new(MockProtocol))
}

#[test]
fn test_check_http() {
    let orch = make_orchestrator();
    let url = TransportUrlVO::new("http://localhost:3000".to_string());
    let result = orch.check_http(&url);
    assert!(result.value);
}

#[test]
fn test_generate_env() {
    let orch = make_orchestrator();
    let home = DirectoryPath::new("/home".to_string()).unwrap_or_default();
    let transport = TransportProtocol::STDAggregate;
    let env = orch.generate_env(&transport, &home);
    assert!(env.value.contains("TRANSPORT="));
}

#[test]
fn test_generate_mcp_config() {
    let orch = make_orchestrator();
    let transport = TransportProtocol::STDAggregate;
    let config = orch.generate_mcp_config(&transport);
    let val = config.value();
    assert!(val.get("transport").is_some());
}

#[test]
fn test_mcp_config_claude_has_client() {
    let orch = make_orchestrator();
    let config = orch.mcp_config_claude(&TransportProtocol::STDAggregate);
    let val = config.value();
    assert_eq!(val.get("client").unwrap(), "claude");
}

#[test]
fn test_mcp_config_hermes_has_client() {
    let orch = make_orchestrator();
    let config = orch.mcp_config_hermes(&TransportProtocol::STDAggregate);
    let val = config.value();
    assert_eq!(val.get("client").unwrap(), "hermes");
}

#[test]
fn test_mcp_config_vscode_has_client() {
    let orch = make_orchestrator();
    let config = orch.mcp_config_vscode(&TransportProtocol::STDAggregate);
    let val = config.value();
    assert_eq!(val.get("client").unwrap(), "vscode");
}

#[test]
fn test_detect_language() {
    let orch = make_orchestrator();
    let lang = orch.detect_language();
    assert_eq!(lang.value, "rust");
}

#[test]
fn test_get_config_template() {
    let orch = make_orchestrator();
    let template = orch.get_config_template("python");
    assert_eq!(template, "mock template");
}

#[test]
fn test_file_exists() {
    let orch = make_orchestrator();
    assert!(orch.file_exists("any"));
}
