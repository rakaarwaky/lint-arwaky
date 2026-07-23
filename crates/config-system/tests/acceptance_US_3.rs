// US-3 — Config Fallback Safety
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use tempfile::TempDir;

#[tokio::test]
async fn us3_no_config_file_uses_defaults() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = ConfigContainer::new()
        .orchestrator()
        .load_project_config(&fp)
        .await;
    assert!(result.config.enabled.value);
    assert!(result
        .warnings
        .iter()
        .any(|w| w.contains("No config file found")));
    assert_eq!(result.source.path.value, "embedded");
}

#[tokio::test]
async fn us3_defaults_are_valid_and_usable() {
    let tmp = TempDir::new().unwrap();
    let orch = ConfigContainer::new().orchestrator();
    let config = orch.load_config_sync(tmp.path().to_str().unwrap());
    assert!(config.enabled.value);
    assert!(!orch.ignored_paths(tmp.path().to_str().unwrap()).is_empty());
}
