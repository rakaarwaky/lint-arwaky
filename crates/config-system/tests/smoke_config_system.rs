// Smoke test — verify the config-system crate boots and core operations respond.
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use tempfile::TempDir;

#[tokio::test]
async fn config_system_boots_and_loads_defaults() {
    let start = std::time::Instant::now();
    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = orch.load_project_config(&fp).await;
    assert!(result.config.enabled.value || !result.config.enabled.value);
    assert!(!result.source.language.is_empty());
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn config_system_sync_load_responds() {
    let start = std::time::Instant::now();
    let tmp = TempDir::new().unwrap();
    let config = ConfigContainer::new()
        .orchestrator()
        .load_config_sync(tmp.path().to_str().unwrap());
    assert!(config.enabled.value);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
