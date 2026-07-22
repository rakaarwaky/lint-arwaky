// US-1 — Project Config Discovery
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn us1_config_in_project_root_is_found() {
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
    assert!(result
        .source
        .path
        .value
        .contains("lint_arwaky.config.rust.yaml"));
    assert!(!result
        .warnings
        .iter()
        .any(|w| w.contains("No config file found")));
}

#[tokio::test]
async fn us1_config_in_parent_directory_is_found() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let nested = tmp.path().join("src").join("deep");
    fs::create_dir_all(&nested).unwrap();
    let fp = FilePath::new(nested.to_string_lossy().to_string()).unwrap();
    let result = ConfigContainer::new()
        .orchestrator()
        .load_config_for_language(&fp, ConfigLanguage::Rust)
        .await;
    assert!(result
        .source
        .path
        .value
        .contains("lint_arwaky.config.rust.yaml"));
}

use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
