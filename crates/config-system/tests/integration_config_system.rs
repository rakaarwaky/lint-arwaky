// Integration tests — full DI wiring via ConfigContainer.
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_parser_protocol::IConfigParserProtocol;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::contract_validator_protocol::IConfigValidatorProtocol;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use shared::config_system::taxonomy_setting_vo::ProjectConfig;
use std::fs;
use tempfile::TempDir;

#[test]
fn container_provides_orchestrator() {
    let _ = ConfigContainer::new().orchestrator();
}
#[test]
fn container_provides_reader() {
    let _ = ConfigContainer::new().reader();
}
#[test]
fn container_provides_parser() {
    let _ = ConfigContainer::new().parser();
}
#[test]
fn container_provides_validator() {
    let _ = ConfigContainer::new().validator();
}

#[test]
fn container_default_is_equivalent_to_new() {
    let _oa = ConfigContainer::new().orchestrator();
    let _ob = ConfigContainer::default().orchestrator();
}

#[tokio::test]
async fn container_orchestrator_loads_defaults_for_empty_project() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = ConfigContainer::new()
        .orchestrator()
        .load_project_config(&fp)
        .await;
    assert!(!result.warnings.is_empty());
}

#[tokio::test]
async fn container_orchestrator_loads_real_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = ConfigContainer::new()
        .orchestrator()
        .load_project_config(&fp)
        .await;
    assert_eq!(result.source.language, "rust");
}

#[tokio::test]
async fn container_reader_lists_config_files() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.rust.yaml"), "a: 1").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let files = ConfigContainer::new()
        .reader()
        .list_config_files(&fp)
        .await
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
    let config = ConfigContainer::new()
        .parser()
        .parse_yaml_config(&fp)
        .unwrap();
    assert_eq!(config.project_name.value, "integration-test");
}

#[test]
fn container_validator_validates_default_config() {
    let result = ConfigContainer::new()
        .validator()
        .validate_thresholds(&ProjectConfig::default());
    assert!(result.is_valid);
}

#[tokio::test]
async fn full_pipeline_read_parse_validate() {
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
    let config = ConfigContainer::new()
        .parser()
        .parse_yaml_config(&fp)
        .unwrap();
    let validation = ConfigContainer::new()
        .validator()
        .validate_thresholds(&config);
    assert!(validation.is_valid);
    assert_eq!(config.project_name.value, "pipeline-test");
}
