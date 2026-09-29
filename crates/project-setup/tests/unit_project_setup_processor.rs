// Unit tests — capability protocol methods for each capability separately.
use shared::common::taxonomy_path_vo::DirectoryPath;
use shared::project_setup::{
    IEnvGenerationProtocol, ILanguageDetectionProtocol, IMcpConfigGenerationProtocol,
};

fn make_mcp_generator() -> impl IMcpConfigGenerationProtocol {
    use project_setup_lint_arwaky::capabilities_mcp_config_generator::SetupMcpConfigGenerator;
    SetupMcpConfigGenerator::new()
}

fn make_env_generator() -> impl IEnvGenerationProtocol {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = c.io();
    use project_setup_lint_arwaky::capabilities_env_generator::SetupEnvGenerator;
    SetupEnvGenerator::new(fs)
}

fn make_language_detector() -> impl ILanguageDetectionProtocol {
    let c = filesystem::root_filesystem_container::FilesystemContainer::new();
    let fs = c.io();
    use project_setup_lint_arwaky::capabilities_language_detector::SetupLanguageDetector;
    SetupLanguageDetector::new(fs)
}

#[test]
fn generate_env_contains_phantom_root() {
    let proc = make_env_generator();
    let home = DirectoryPath::new("/home/test").unwrap();
    let env = proc.generate_env(&home);
    assert!(env.value().contains("PHANTOM_ROOT=/home/test/"));
}

#[test]
fn generate_env_contains_header() {
    let proc = make_env_generator();
    let home = DirectoryPath::new("/tmp").unwrap();
    let env = proc.generate_env(&home);
    assert!(
        env.value()
            .contains("Lint Arwaky Environment Configuration")
    );
}

#[test]
fn generate_mcp_config_contains_lint_arwaky_key() {
    let proc = make_mcp_generator();
    let config = proc.generate_mcp_config();
    let value = config.value();
    assert!(
        value.get("lint-arwaky").is_some(),
        "MCP config must contain 'lint-arwaky' key"
    );
}

#[test]
fn mcp_config_claude_wraps_in_mcp_servers() {
    let proc = make_mcp_generator();
    let config = proc.mcp_config_claude();
    let value = config.value();
    assert!(
        value.get("mcpServers").is_some(),
        "Claude config must contain 'mcpServers' key"
    );
}

#[test]
fn mcp_config_hermes_returns_base_config() {
    let proc = make_mcp_generator();
    let config = proc.mcp_config_hermes();
    let value = config.value();
    assert!(
        value.get("lint-arwaky").is_some(),
        "Hermes config must contain 'lint-arwaky' key"
    );
}

#[test]
fn mcp_config_vscode_wraps_in_mcp_servers() {
    let proc = make_mcp_generator();
    let config = proc.mcp_config_vscode();
    let value = config.value();
    assert!(
        value.get("mcp").is_some(),
        "VS Code config must contain 'mcp' key"
    );
}

#[test]
fn mcp_config_cursor_returns_valid() {
    let proc = make_mcp_generator();
    let config = proc.mcp_config_cursor();
    assert!(
        !config.value().is_empty(),
        "Cursor config should have entries"
    );
}

#[test]
fn mcp_config_windsurf_returns_valid() {
    let proc = make_mcp_generator();
    let config = proc.mcp_config_windsurf();
    assert!(
        !config.value().is_empty(),
        "Windsurf config should have entries"
    );
}

#[test]
fn mcp_config_copilot_returns_valid() {
    let proc = make_mcp_generator();
    let config = proc.mcp_config_copilot();
    assert!(
        !config.value().is_empty(),
        "Copilot config should have entries"
    );
}

#[test]
fn mcp_config_all_returns_valid() {
    let proc = make_mcp_generator();
    let config = proc.mcp_config_all();
    assert!(!config.value().is_empty(), "All config should have entries");
}

#[test]
fn detect_language_returns_non_empty() {
    let proc = make_language_detector();
    let lang = proc.detect_language();
    assert!(lang.is_some(), "Should detect a language");
    assert!(!lang.unwrap().value().is_empty());
}

#[test]
fn detect_languages_returns_at_least_one() {
    let proc = make_language_detector();
    let langs = proc.detect_languages();
    assert!(!langs.is_empty(), "Should detect at least one language");
}
