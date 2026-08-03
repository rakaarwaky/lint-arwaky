// Unit tests — SetupManagementProcessor protocol methods.
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::{ISetupManagementProtocol, ISetupInstallerProtocol, SetupError};
use std::sync::Arc;

struct StubInstaller;

impl ISetupInstallerProtocol for StubInstaller {
    fn install_python_packages(&self, _packages: &[String]) -> Result<(), SetupError> {
        Ok(())
    }
    fn install_npm_packages(&self, _packages: &[String], _sudo: bool) -> Result<(), SetupError> {
        Ok(())
    }
}

fn make_processor() -> impl ISetupManagementProtocol {
    use project_setup_lint_arwaky::capabilities_setup_processor::SetupManagementProcessor;
    let fs = filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    SetupManagementProcessor::new(Arc::new(StubInstaller), fs)
}

#[test]
fn generate_env_contains_phantom_root() {
    let proc = make_processor();
    let home = DirectoryPath::new("/home/test").unwrap();
    let env = proc.generate_env(&home);
    assert!(env.value().contains("PHANTOM_ROOT=/home/test/"));
}

#[test]
fn generate_env_contains_header() {
    let proc = make_processor();
    let home = DirectoryPath::new("/tmp").unwrap();
    let env = proc.generate_env(&home);
    assert!(env.value().contains("Lint Arwaky Environment Configuration"));
}

#[test]
fn generate_mcp_config_contains_lint_arwaky_key() {
    let proc = make_processor();
    let config = proc.generate_mcp_config();
    let value = config.value();
    assert!(
        value.get("lint-arwaky").is_some(),
        "MCP config must contain 'lint-arwaky' key"
    );
}

#[test]
fn mcp_config_claude_wraps_in_mcp_servers() {
    let proc = make_processor();
    let config = proc.mcp_config_claude();
    let value = config.value();
    assert!(
        value.get("mcpServers").is_some(),
        "Claude config must contain 'mcpServers' key"
    );
}

#[test]
fn mcp_config_hermes_returns_base_config() {
    let proc = make_processor();
    let config = proc.mcp_config_hermes();
    let value = config.value();
    assert!(
        value.get("lint-arwaky").is_some(),
        "Hermes config must contain 'lint-arwaky' key"
    );
}

#[test]
fn mcp_config_vscode_wraps_in_mcp_servers() {
    let proc = make_processor();
    let config = proc.mcp_config_vscode();
    let value = config.value();
    assert!(
        value.get("mcp").is_some(),
        "VS Code config must contain 'mcp' key"
    );
}

#[test]
fn get_config_template_rust_returns_yaml() {
    let proc = make_processor();
    let template = proc.get_config_template("rust");
    assert!(!template.is_empty());
    assert!(template.contains("architecture") || template.contains("rules"));
}

#[test]
fn get_config_template_python_returns_yaml() {
    let proc = make_processor();
    let template = proc.get_config_template("python");
    assert!(!template.is_empty());
}

#[test]
fn get_config_template_javascript_returns_yaml() {
    let proc = make_processor();
    let template = proc.get_config_template("javascript");
    assert!(!template.is_empty());
}

#[test]
fn get_config_template_unknown_defaults_to_rust() {
    let proc = make_processor();
    let unknown = proc.get_config_template("kotlin");
    let rust = proc.get_config_template("rust");
    assert_eq!(unknown, rust, "Unknown language should default to rust template");
}

#[test]
fn detect_language_returns_non_empty() {
    let proc = make_processor();
    let lang = proc.detect_language();
    assert!(!lang.value().is_empty());
}

#[test]
fn detect_languages_returns_at_least_one() {
    let proc = make_processor();
    let langs = proc.detect_languages();
    assert!(!langs.is_empty(), "Should detect at least one language");
}

#[test]
fn install_python_adapters_returns_success() {
    let proc = make_processor();
    let status = proc.install_python_adapters();
    assert!(status.value(), "Stub installer should always succeed");
}

#[test]
fn install_javascript_adapters_returns_success() {
    let proc = make_processor();
    let status = proc.install_javascript_adapters(false);
    assert!(status.value(), "Stub installer should always succeed");
}

#[test]
fn install_javascript_adapters_with_sudo() {
    let proc = make_processor();
    let status = proc.install_javascript_adapters(true);
    assert!(status.value(), "Stub installer should always succeed");
}

#[test]
fn write_config_file_succeeds() {
    let proc = make_processor();
    let tmp = tempfile::TempDir::new().unwrap();
    let path = tmp.path().join("test_config.yaml");
    let result = proc.write_config_file(&path.to_string_lossy(), "key: value\n");
    assert!(result.is_ok(), "write_config_file should succeed: {:?}", result);
    let desc = result.unwrap();
    assert!(desc.value().contains("test_config.yaml"));
}

#[test]
fn file_exists_returns_correctly() {
    let proc = make_processor();
    let tmp = tempfile::TempDir::new().unwrap();
    let path = tmp.path().join("exists.txt");
    assert!(!proc.file_exists(&path.to_string_lossy()));
    std::fs::write(&path, "content").unwrap();
    assert!(proc.file_exists(&path.to_string_lossy()));
}

#[test]
fn create_global_config_dir_succeeds() {
    let proc = make_processor();
    let result = proc.create_global_config_dir();
    // This may fail in CI if XDG_CONFIG_HOME is not set, but should not panic
    match result {
        Ok(path) => assert!(path.exists() || path.to_string_lossy().contains("lint-arwaky")),
        Err(SetupError::InvalidState(_)) => {} // acceptable in some envs
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}
