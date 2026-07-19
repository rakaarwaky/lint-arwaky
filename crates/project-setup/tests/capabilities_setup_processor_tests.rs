use async_trait::async_trait;
use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::contract_setup_protocol::{
    ISetupInstallerPort, ISetupManagementProtocol,
};
use shared::project_setup::taxonomy_setup_contract_vo::SetupError;
use std::sync::Arc;

struct MockInstaller;

#[async_trait]
impl ISetupInstallerPort for MockInstaller {
    async fn install_python_packages(&self, _packages: &[String]) -> Result<(), SetupError> {
        Ok(())
    }
    async fn install_npm_packages(
        &self,
        _packages: &[String],
        _sudo: bool,
    ) -> Result<(), SetupError> {
        Ok(())
    }
}

fn make_processor() -> SetupManagementProcessor {
    SetupManagementProcessor::new(Arc::new(MockInstaller))
}

#[test]
fn test_generate_env_contains_phantom_root() {
    let proc = make_processor();
    let home = DirectoryPath::new("/home/user".to_string()).unwrap_or_default();
    let env = proc.generate_env(&home);
    assert!(env.value.contains("PHANTOM_ROOT=/home/user/"));
    assert!(env.value.contains("# Lint Arwaky"));
}

#[test]
fn test_generate_mcp_config_has_command() {
    let proc = make_processor();
    let config = proc.generate_mcp_config();
    let val = config.value();
    assert!(val.contains_key("lint-arwaky"));
    let entry = val.get("lint-arwaky").unwrap();
    assert_eq!(entry["command"], "lint-arwaky");
}

#[test]
fn test_mcp_config_claude_wraps_in_mcp_servers() {
    let proc = make_processor();
    let config = proc.mcp_config_claude();
    let val = config.value();
    assert!(val.contains_key("mcpServers"));
    let inner = val.get("mcpServers").unwrap();
    assert!(inner.get("lint-arwaky").is_some());
}

#[test]
fn test_mcp_config_hermes_returns_base() {
    let proc = make_processor();
    let config = proc.mcp_config_hermes();
    let val = config.value();
    assert!(val.contains_key("lint-arwaky"));
}

#[test]
fn test_mcp_config_vscode_wraps_in_mcp_servers() {
    let proc = make_processor();
    let config = proc.mcp_config_vscode();
    let val = config.value();
    assert!(val.contains_key("mcp"));
    let outer = val.get("mcp").unwrap();
    let servers = outer.get("servers").unwrap();
    assert!(servers.get("lint-arwaky").is_some());
}

#[test]
fn test_which_mcp_binary_returns_non_empty() {
    let proc = make_processor();
    let binary = proc.which_mcp_binary();
    assert!(!binary.value.is_empty());
}

#[test]
fn test_write_config_file() {
    let proc = make_processor();
    let dir = std::env::temp_dir().join("lint_arwaky_test_setup");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("test_config.yaml");
    let result = proc.write_config_file(&path.to_string_lossy(), "rules:\n  aes001: enabled\n");
    assert!(result.is_ok());
    let desc = result.unwrap();
    assert!(desc.value.contains("wrote"));
    assert!(desc.value.contains("bytes"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn test_detect_language_rust() {
    let proc = make_processor();
    if std::path::Path::new("crates").exists() {
        let lang = proc.detect_language();
        assert_eq!(lang.value, "rust");
    }
}

#[test]
fn test_file_exists() {
    let proc = make_processor();
    assert!(proc.file_exists("Cargo.toml"));
    assert!(!proc.file_exists("nonexistent_file_xyz.txt"));
}

#[test]
fn test_get_config_template_rust() {
    let proc = make_processor();
    let template = proc.get_config_template("rust");
    assert!(!template.is_empty());
}

#[test]
fn test_get_config_template_python() {
    let proc = make_processor();
    let template = proc.get_config_template("python");
    assert!(!template.is_empty());
}
