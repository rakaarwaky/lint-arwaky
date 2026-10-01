// FR-003 — Config Fallback Safety
mod common;

use shared_common::FilePath;
use shared_config_system::ConfigRequest;
use tempfile::TempDir;

#[test]
fn us3_no_config_file_uses_defaults() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = common::make_container()
        .orchestrator()
        .execute(ConfigRequest::load_project_config(&fp))
        .into_config_result();
    assert!(result.config.enabled.value);
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.contains("No config file found"))
    );
    assert_eq!(result.source.path.value, "embedded");
}

#[test]
fn us3_defaults_are_valid_and_usable() {
    let tmp = TempDir::new().unwrap();
    let orch = common::make_container().orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let config = orch
        .execute(ConfigRequest::load_sync(&fp))
        .into_sync_config();
    assert!(config.enabled.value);
    assert!(
        !orch
            .execute(ConfigRequest::ignored_paths(&fp))
            .into_patterns()
            .is_empty()
    );
}
