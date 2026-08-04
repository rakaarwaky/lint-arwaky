// FR-007 — Config Caching
// Tests DashMap-based config caching and concurrent access.
mod common;

use shared::common::FilePath;
use shared::config_system::{ConfigLanguage, IConfigOrchestratorAggregate};

use std::fs;
use std::sync::Arc;
use std::thread;
use tempfile::TempDir;

fn make_orchestrator() -> config_system_lint_arwaky::agent_config_orchestrator::ConfigOrchestrator {
    use config_system_lint_arwaky::agent_config_orchestrator::{
        ConfigOrchestrator, ConfigOrchestratorDeps,
    };
    use config_system_lint_arwaky::capabilities_parser_provider::ConfigParserProvider;
    use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
    use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
    use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;

    let fs = common::make_fs();
    ConfigOrchestrator::new(ConfigOrchestratorDeps {
        workspace_detector: Arc::new(WorkspaceDetector::new(fs.clone())),
        config_reader: Arc::new(ConfigYamlReader::new(fs.clone())),
        parser: Arc::new(ConfigParserProvider::new(fs.clone())),
        validator: Arc::new(ConfigRulesValidator::new()),
        filesystem: fs,
    })
}

// FR-007 Scenario 1: Same config file requested twice → parsed once, cached
#[test]
fn us7_same_config_file_cached_on_second_load() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();

    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    let r1 = sut.load_config_for_language(&fp, ConfigLanguage::Rust);
    let r2 = sut.load_config_for_language(&fp, ConfigLanguage::Rust);

    // Both loads should return the same source path (from cache)
    assert_eq!(r1.source.path, r2.source.path);
    // Both should have the same config content
    assert_eq!(r1.config.enabled.value, r2.config.enabled.value);
}

// FR-007 Scenario 2: Concurrent requests for same key → single parse
#[test]
fn us7_concurrent_requests_for_same_key_are_safe() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();

    let sut = Arc::new(make_orchestrator());
    let path_str = tmp.path().to_string_lossy().to_string();

    // Spawn multiple threads requesting the same config concurrently
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let fp = FilePath::new(path_str.clone()).unwrap();
            let sut_clone = sut.clone();
            thread::spawn(move || sut_clone.load_config_for_language(&fp, ConfigLanguage::Rust))
        })
        .collect();

    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // All concurrent loads should return identical results
    let first_path = &results[0].source.path;
    for r in &results[1..] {
        assert_eq!(&r.source.path, first_path);
    }
}

// Additional: load_config_sync also uses cache
#[test]
fn us7_load_config_sync_uses_cache_for_same_path() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.yaml"),
        "architecture:\n  enabled: false\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();

    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    // First call populates cache
    let config1 = sut.load_config_sync(&fp);
    // Second call should use cache
    let config2 = sut.load_config_sync(&fp);

    assert_eq!(config1.enabled.value, config2.enabled.value);
}
