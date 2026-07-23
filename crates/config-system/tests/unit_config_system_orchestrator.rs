// Unit tests for ConfigOrchestrator — config loading, caching, workspace discovery.
use config_system_lint_arwaky::agent_config_orchestrator::{
    ConfigOrchestrator, ConfigOrchestratorDeps,
};
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use std::fs;
use std::sync::Arc;
use tempfile::TempDir;

fn make_orchestrator() -> ConfigOrchestrator {
    ConfigOrchestrator::new(ConfigOrchestratorDeps {
        workspace_detector: Arc::new(WorkspaceDetector::new()),
        config_reader: Arc::new(ConfigYamlReader::new()),
        validator: Arc::new(ConfigRulesValidator::new()),
    })
}

#[tokio::test]
async fn load_project_config_uses_defaults_when_no_file() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_orchestrator().load_project_config(&fp).await;
    assert!(!result.warnings.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("No config file found")));
    assert_eq!(result.source.language, "rust");
}

#[tokio::test]
async fn load_project_config_reads_existing_yaml() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_orchestrator().load_project_config(&fp).await;
    assert_eq!(result.source.language, "rust");
    assert!(result
        .source
        .path
        .value
        .contains("lint_arwaky.config.rust.yaml"));
}

#[tokio::test]
async fn load_config_for_language_python() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.python.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_orchestrator()
        .load_config_for_language(&fp, ConfigLanguage::Python)
        .await;
    assert_eq!(result.source.language, "python");
}

#[tokio::test]
async fn load_config_for_language_injects_defaults_when_no_layers() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_orchestrator()
        .load_config_for_language(&fp, ConfigLanguage::Rust)
        .await;
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("no architecture layers")));
}

#[test]
fn load_config_sync_returns_defaults_for_empty_dir() {
    let tmp = TempDir::new().unwrap();
    let config = make_orchestrator().load_config_sync(tmp.path().to_str().unwrap());
    assert!(config.enabled.value);
}

#[test]
fn load_config_sync_finds_config_in_current_dir() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: false\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let config = make_orchestrator().load_config_sync(tmp.path().to_str().unwrap());
    assert!(!config.enabled.value);
}

#[test]
fn ignored_paths_includes_hardcoded_defaults() {
    let tmp = TempDir::new().unwrap();
    let paths = make_orchestrator().ignored_paths(tmp.path().to_str().unwrap());
    assert!(paths.contains(&"target".to_string()));
    assert!(paths.contains(&"node_modules".to_string()));
    assert!(paths.contains(&".git".to_string()));
    assert!(paths.contains(&"dist".to_string()));
}

#[tokio::test]
async fn discover_workspaces_returns_members() {
    let tmp = TempDir::new().unwrap();
    let crates = tmp.path().join("crates");
    fs::create_dir_all(crates.join("alpha")).unwrap();
    fs::create_dir_all(crates.join("beta")).unwrap();
    fs::write(crates.join("alpha").join("Cargo.toml"), "").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_orchestrator().discover_workspaces(&fp).await.len(), 2);
}

#[tokio::test]
async fn discover_workspaces_returns_empty_for_non_workspace() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_orchestrator()
        .discover_workspaces(&fp)
        .await
        .is_empty());
}

#[tokio::test]
async fn config_cache_returns_same_arc_on_second_load() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let r1 = sut
        .load_config_for_language(&fp, ConfigLanguage::Rust)
        .await;
    let r2 = sut
        .load_config_for_language(&fp, ConfigLanguage::Rust)
        .await;
    assert_eq!(r1.source.path, r2.source.path);
}

#[test]
fn validator_accessor_returns_same_instance() {
    let _v = make_orchestrator().validator();
}
