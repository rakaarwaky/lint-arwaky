// FR-009 — TOML Config Parsing
// Tests [tool.lint-arwaky] section parsing per FR-009 spec.
mod common;

use config_system_lint_arwaky::capabilities_parser_provider::ConfigParserProvider;
use shared::common::FilePath;
use shared::config_system::IConfigParserProtocol;
use std::fs;
use tempfile::TempDir;

fn make_parser() -> ConfigParserProvider {
    ConfigParserProvider::new(common::make_fs())
}

// FR-009 Scenario 1: Cargo.toml with [tool.lint-arwaky] → parsed correctly
#[test]
fn us9_toml_with_tool_section_parses_correctly() {
    let tmp = TempDir::new().unwrap();
    let toml_content = r#"[tool.lint-arwaky]
project_name = "my-project"

[tool.lint-arwaky.thresholds]
score = { value = 90.0 }
complexity = { value = 8 }
max_file_lines = { value = 300 }
"#;
    let path = tmp.path().join("Cargo.toml");
    fs::write(&path, toml_content).unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = make_parser().parse_toml_config(&fp).unwrap();
    assert!(result.is_some());
    let config = result.unwrap();
    assert_eq!(config.project_name.value, "my-project");
    assert_eq!(config.thresholds.score.value, 90.0);
}

// FR-009 Scenario 2: Cargo.toml without [tool] section → returns None
#[test]
fn us9_toml_without_tool_section_returns_none() {
    let tmp = TempDir::new().unwrap();
    let toml_content = r#"[package]
name = "my-crate"
version = "0.1.0"
"#;
    let path = tmp.path().join("Cargo.toml");
    fs::write(&path, toml_content).unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    assert!(make_parser().parse_toml_config(&fp).unwrap().is_none());
}

// FR-009 Scenario 3: Invalid TOML syntax → ConfigError returned
#[test]
fn us9_invalid_toml_returns_error() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("bad.toml");
    fs::write(&path, "this is [[[not valid toml").unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = make_parser().parse_toml_config(&fp);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .message
            .value
            .contains("Failed to parse TOML")
    );
}

// Additional: TOML with underscore key variant
#[test]
fn us9_toml_with_underscore_key_parses_correctly() {
    let tmp = TempDir::new().unwrap();
    let toml_content = r#"[tool.lint_arwaky]
project_name = "underscore-project"
"#;
    let path = tmp.path().join("pyproject.toml");
    fs::write(&path, toml_content).unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = make_parser().parse_toml_config(&fp).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().project_name.value, "underscore-project");
}

// Additional: TOML file not found
#[test]
fn us9_toml_file_not_found_returns_error() {
    let fp = FilePath::new("/nonexistent/Cargo.toml".to_string()).unwrap();
    assert!(make_parser().parse_toml_config(&fp).is_err());
}
