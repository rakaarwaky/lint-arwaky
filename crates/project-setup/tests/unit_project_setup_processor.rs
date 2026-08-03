// Unit tests — SetupManagementProcessor protocol methods.
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::{ISetupInstallerProtocol, ISetupManagementProtocol, SetupError};
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
    assert!(
        env.value()
            .contains("Lint Arwaky Environment Configuration")
    );
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
fn mcp_config_cursor_returns_valid() {
    let proc = make_processor();
    let config = proc.mcp_config_cursor();
    assert!(
        !config.value().is_empty(),
        "Cursor config should have entries"
    );
}

#[test]
fn mcp_config_windsurf_returns_valid() {
    let proc = make_processor();
    let config = proc.mcp_config_windsurf();
    assert!(
        !config.value().is_empty(),
        "Windsurf config should have entries"
    );
}

#[test]
fn mcp_config_copilot_returns_valid() {
    let proc = make_processor();
    let config = proc.mcp_config_copilot();
    assert!(
        !config.value().is_empty(),
        "Copilot config should have entries"
    );
}

#[test]
fn mcp_config_all_returns_valid() {
    let proc = make_processor();
    let config = proc.mcp_config_all();
    assert!(!config.value().is_empty(), "All config should have entries");
}

#[test]
fn get_config_template_rust_returns_yaml() {
    let proc = make_processor();
    let result = proc.get_config_template("rust");
    let template = result.unwrap();
    assert!(!template.is_empty());
    assert!(template.contains("architecture") || template.contains("rules"));
}

#[test]
fn get_config_template_python_returns_yaml() {
    let proc = make_processor();
    let template = proc.get_config_template("python").unwrap();
    assert!(!template.is_empty());
}

#[test]
fn get_config_template_javascript_returns_yaml() {
    let proc = make_processor();
    let template = proc.get_config_template("javascript").unwrap();
    assert!(!template.is_empty());
}

#[test]
fn get_config_template_unknown_returns_error() {
    let proc = make_processor();
    let result = proc.get_config_template("kotlin");
    assert!(result.is_err(), "Unknown language should return Err");
    match result.unwrap_err() {
        SetupError::UnknownLanguage(_) => {}
        e => panic!("Expected UnknownLanguage error, got: {:?}", e),
    }
}

#[test]
fn get_config_template_typescript_returns_yaml() {
    let proc = make_processor();
    let result = proc.get_config_template("typescript");
    assert!(
        result.is_ok(),
        "FR-005: 'typescript' should have a template"
    );
    let template = result.unwrap();
    assert!(!template.is_empty());
}

#[test]
fn detect_language_returns_non_empty() {
    let proc = make_processor();
    let lang = proc.detect_language();
    assert!(lang.is_some(), "Should detect a language");
    assert!(!lang.unwrap().value().is_empty());
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
    assert!(
        result.is_ok(),
        "write_config_file should succeed: {:?}",
        result
    );
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
    match result {
        Ok(path) => assert!(path.exists() || path.to_string_lossy().contains("lint-arwaky")),
        Err(SetupError::InvalidState(_)) => {}
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn pre_flight_check_returns_results() {
    let proc = make_processor();
    let results = proc.pre_flight_check();
    assert!(
        !results.is_empty(),
        "Pre-flight check should return at least one entry"
    );
}
