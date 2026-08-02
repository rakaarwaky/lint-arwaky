// Integration tests — full DI wiring via ConfigContainer.
mod common;

use shared::common::FilePath;
use shared::config_system::{ConfigLanguage, ProjectConfig};

use std::fs;
use tempfile::TempDir;

#[test]
fn container_provides_orchestrator() {
    let _ = common::make_container().orchestrator();
}
#[test]
fn container_provides_reader() {
    let _ = common::make_container().reader();
}
#[test]
fn container_provides_parser() {
    let _ = common::make_container().parser();
}
#[test]
fn container_provides_validator() {
    let _ = common::make_container().validator();
}

#[test]
fn container_orchestrator_loads_defaults_for_empty_project() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = common::make_container()
        .orchestrator()
        .load_project_config(&fp);
    assert!(!result.warnings.is_empty());
}

#[test]
fn container_orchestrator_loads_real_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = common::make_container()
        .orchestrator()
        .load_project_config(&fp);
    assert_eq!(result.source.language, "rust");
}

#[test]
fn container_reader_lists_config_files() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.rust.yaml"), "a: 1").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let files = common::make_container()
        .reader()
        .list_config_files(&fp)
        .unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].0, ConfigLanguage::Rust);
}

#[test]
fn container_parser_parses_yaml() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("config.yaml");
    fs::write(&path, "project_name: integration-test\n").unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let config = common::make_container()
        .parser()
        .parse_yaml_config(&fp)
        .unwrap();
    assert_eq!(config.project_name.value, "integration-test");
}

#[test]
fn container_validator_validates_default_config() {
    let result = common::make_container()
        .validator()
        .validate_thresholds(&ProjectConfig::default());
    assert!(result.is_valid);
}

#[test]
fn full_pipeline_read_parse_validate() {
    let tmp = TempDir::new().unwrap();
    let yaml = r#"project_name: pipeline-test
thresholds:
  score:
    value: 75.0
  complexity:
    value: 15
  max_file_lines:
    value: 600
"#;
    let path = tmp.path().join("config.yaml");
    fs::write(&path, yaml).unwrap();
    let fp = FilePath::new(path.to_string_lossy().to_string()).unwrap();
    let config = common::make_container()
        .parser()
        .parse_yaml_config(&fp)
        .unwrap();
    let validation = common::make_container()
        .validator()
        .validate_thresholds(&config);
    assert!(validation.is_valid);
    assert_eq!(config.project_name.value, "pipeline-test");
}
