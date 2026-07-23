// Unit tests for ConfigParserProvider — YAML and TOML config parsing.
use config_system_lint_arwaky::capabilities_parser_provider::ConfigParserProvider;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use std::fs;
use tempfile::TempDir;

fn make_parser() -> ConfigParserProvider {
    ConfigParserProvider::new()
}

#[test]
fn parse_yaml_config_happy_path() {
    let tmp = TempDir::new().unwrap();
    let yaml = r#"project_name: my-project
thresholds:
  score:
    value: 85.0
  complexity:
    value: 12
  max_file_lines:
    value: 400
adapters:
  - name:
      value: ruff
    status: enabled
    weight: 1.0
"#;
    let path = tmp.path().join("config.yaml");
    fs::write(&path, yaml).unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let config = make_parser().parse_yaml_config(&fp).unwrap();
    assert_eq!(config.project_name.value, "my-project");
    assert_eq!(config.thresholds.score.value, 85.0);
    assert_eq!(config.thresholds.complexity.value, 12);
    assert_eq!(config.adapters.len(), 1);
}

#[test]
fn parse_yaml_config_file_not_found() {
    let fp = FilePath::new("/nonexistent/path/config.yaml".to_string()).unwrap();
    assert!(make_parser().parse_yaml_config(&fp).is_err());
}

#[test]
fn parse_yaml_config_invalid_yaml() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("bad.yaml");
    fs::write(&path, "{{{{invalid yaml::::").unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = make_parser().parse_yaml_config(&fp);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .message
        .value
        .contains("Failed to deserialize YAML"));
}

#[test]
fn parse_yaml_config_empty_file_uses_defaults() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("empty.yaml");
    fs::write(&path, "").unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    assert!(make_parser().parse_yaml_config(&fp).is_ok());
}

#[test]
fn parse_toml_config_with_tool_section() {
    let tmp = TempDir::new().unwrap();
    let toml_content = r#"[tool.lint-arwaky]
project_name = "my-toml-project"

[tool.lint-arwaky.thresholds]
score = { value = 90.0 }
complexity = { value = 8 }
max_file_lines = { value = 300 }
"#;
    let path = tmp.path().join("pyproject.toml");
    fs::write(&path, toml_content).unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = make_parser().parse_toml_config(&fp).unwrap();
    assert!(result.is_some());
    assert_eq!(result.unwrap().project_name.value, "my-toml-project");
}

#[test]
fn parse_toml_config_with_underscore_key() {
    let tmp = TempDir::new().unwrap();
    let toml_content = r#"[tool.lint_arwaky]
project_name = "underscore-project"
"#;
    let path = tmp.path().join("Cargo.toml");
    fs::write(&path, toml_content).unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    assert!(make_parser().parse_toml_config(&fp).unwrap().is_some());
}

#[test]
fn parse_toml_config_returns_none_without_tool_section() {
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

#[test]
fn parse_toml_config_file_not_found() {
    let fp = FilePath::new("/nonexistent/Cargo.toml".to_string()).unwrap();
    assert!(make_parser().parse_toml_config(&fp).is_err());
}

#[test]
fn parse_toml_config_invalid_toml() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("bad.toml");
    fs::write(&path, "this is [[[not valid toml").unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let result = make_parser().parse_toml_config(&fp);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .message
        .value
        .contains("Failed to parse TOML"));
}

#[test]
fn default_and_new_are_equivalent() {
    let _a = ConfigParserProvider::new();
    let _b = ConfigParserProvider::default();
}
