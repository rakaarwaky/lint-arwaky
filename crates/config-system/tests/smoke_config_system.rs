// Smoke test — verify the config-system crate boots and core operations respond.
mod common;

use shared::common::FilePath;
use tempfile::TempDir;

#[test]
fn config_system_boots_and_loads_defaults() {
    let start = std::time::Instant::now();
    let container = common::make_container();
    let orch = container.orchestrator();
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = orch.load_project_config(&fp);
    let _ = result.config.enabled; // validated by successful load
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
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let config = common::make_container()
        .orchestrator()
        .load_config_sync(&fp);
    assert!(config.enabled.value);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
