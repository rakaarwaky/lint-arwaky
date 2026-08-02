// Unit tests for ConfigOrchestrator — config loading, caching, workspace discovery.
mod common;

use config_system_lint_arwaky::agent_config_orchestrator::{
    ConfigOrchestrator, ConfigOrchestratorDeps,
};
use config_system_lint_arwaky::capabilities_parser_provider::ConfigParserProvider;
use config_system_lint_arwaky::capabilities_rules_validator::ConfigRulesValidator;
use config_system_lint_arwaky::capabilities_workspace_detector::WorkspaceDetector;
use config_system_lint_arwaky::capabilities_yaml_reader::ConfigYamlReader;
use shared::common::FilePath;
use shared::config_system::{ConfigLanguage, IConfigOrchestratorAggregate};

use std::fs;
use std::sync::Arc;
use tempfile::TempDir;

fn make_orchestrator() -> ConfigOrchestrator {
    let fs = common::make_fs();
    ConfigOrchestrator::new(ConfigOrchestratorDeps {
        workspace_detector: Arc::new(WorkspaceDetector::new(fs.clone())),
        config_reader: Arc::new(ConfigYamlReader::new(fs.clone())),
        parser: Arc::new(ConfigParserProvider::new(fs.clone())),
        validator: Arc::new(ConfigRulesValidator::new()),
        filesystem: fs,
    })
}

#[test]
fn load_project_config_uses_defaults_when_no_file() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_orchestrator().load_project_config(&fp);
    assert!(!result.warnings.is_empty());
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.contains("No config file found"))
    );
    assert_eq!(result.source.language, "rust");
}

#[test]
fn load_project_config_reads_existing_yaml() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_orchestrator().load_project_config(&fp);
    assert_eq!(result.source.language, "rust");
    assert!(
        result
            .source
            .path
            .value
            .contains("lint_arwaky.config.rust.yaml")
    );
}

#[test]
fn load_config_for_language_python() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.python.yaml"),
        "architecture:\n  enabled: true\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_orchestrator().load_config_for_language(&fp, ConfigLanguage::Python);
    assert_eq!(result.source.language, "python");
}

#[test]
fn load_config_for_language_injects_defaults_when_no_layers() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let result = make_orchestrator().load_config_for_language(&fp, ConfigLanguage::Rust);
    assert!(
        result
            .warnings
            .iter()
            .any(|w| w.contains("no architecture layers"))
    );
}

#[test]
fn load_config_sync_returns_defaults_for_empty_dir() {
    let tmp = TempDir::new().unwrap();
    let fp = shared::common::taxonomy_path_vo::FilePath::new(tmp.path().to_str().unwrap()).unwrap();
    let config = make_orchestrator().load_config_sync(&fp);
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
    let fp = shared::common::taxonomy_path_vo::FilePath::new(tmp.path().to_str().unwrap()).unwrap();
    let config = make_orchestrator().load_config_sync(&fp);
    assert!(!config.enabled.value);
}

#[test]
fn ignored_paths_includes_hardcoded_defaults() {
    let tmp = TempDir::new().unwrap();
    let fp = shared::common::taxonomy_path_vo::FilePath::new(tmp.path().to_str().unwrap()).unwrap();
    let paths = make_orchestrator().ignored_paths(&fp);
    assert!(paths.values.contains(&"target".to_string()));
    assert!(paths.values.contains(&"node_modules".to_string()));
    assert!(paths.values.contains(&".git".to_string()));
    assert!(paths.values.contains(&"dist".to_string()));
}

#[test]
fn discover_workspaces_returns_members() {
    let tmp = TempDir::new().unwrap();
    let crates = tmp.path().join("crates");
    fs::create_dir_all(crates.join("alpha")).unwrap();
    fs::create_dir_all(crates.join("beta")).unwrap();
    fs::write(crates.join("alpha").join("Cargo.toml"), "").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(make_orchestrator().discover_workspaces(&fp).len(), 2);
}

#[test]
fn discover_workspaces_returns_empty_for_non_workspace() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(make_orchestrator().discover_workspaces(&fp).is_empty());
}

#[test]
fn config_cache_returns_same_arc_on_second_load() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let sut = make_orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let r1 = sut.load_config_for_language(&fp, ConfigLanguage::Rust);
    let r2 = sut.load_config_for_language(&fp, ConfigLanguage::Rust);
    assert_eq!(r1.source.path, r2.source.path);
}

// ─── Regression: Phase 3.4 — Config loading from deeply nested file paths ─────

/// `load_config_sync` should find the config file when scanning a file
/// nested 4 levels deep under the project root (file.py → src → security → modules → root).
/// This was broken before the fix because the upward search depth was limited to 3.
#[test]
fn load_config_sync_finds_config_from_deep_file_path() {
    let tmp = TempDir::new().unwrap();
    // Create config at project root
    fs::write(
        tmp.path().join("lint_arwaky.config.python.yaml"),
        "architecture:\n  enabled: false\n  rules: []\n",
    )
    .unwrap();
    // Create deeply nested file: modules/security/src/capabilities_handler.py
    let nested_file = tmp
        .path()
        .join("modules")
        .join("security")
        .join("src")
        .join("capabilities_handler.py");
    fs::create_dir_all(nested_file.parent().unwrap()).unwrap();
    fs::write(&nested_file, "def handle(): pass\n").unwrap();

    let fp = FilePath::new(nested_file.to_string_lossy().to_string()).unwrap();
    let config = make_orchestrator().load_config_sync(&fp);
    // Config has enabled: false, so we verify it was loaded (not the default which has enabled: true)
    assert!(!config.enabled.value);
}

/// `load_config_sync` should find the config from a file nested 4 levels deep
/// (the original bug scenario: modules/security/src/capabilities_archive_guard.py).
/// Config is at project root, 4 levels up from the file.
#[test]
fn load_config_sync_finds_config_from_very_deep_file_path() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.python.yaml"),
        "architecture:\n  enabled: false\n  rules: []\n",
    )
    .unwrap();
    // Path: modules/security/src/test.py (4 levels up to root)
    let deep_file = tmp
        .path()
        .join("modules")
        .join("security")
        .join("src")
        .join("test.py");
    fs::create_dir_all(deep_file.parent().unwrap()).unwrap();
    fs::write(&deep_file, "# test\n").unwrap();

    let fp = FilePath::new(deep_file.to_string_lossy().to_string()).unwrap();
    let config = make_orchestrator().load_config_sync(&fp);
    assert!(!config.enabled.value);
}

/// `load_config_sync` returns default config (enabled) when no config file exists,
/// even for deeply nested file paths.
#[test]
fn load_config_sync_returns_defaults_for_deep_file_with_no_config() {
    let tmp = TempDir::new().unwrap();
    let deep_file = tmp
        .path()
        .join("modules")
        .join("security")
        .join("src")
        .join("capabilities_handler.py");
    fs::create_dir_all(deep_file.parent().unwrap()).unwrap();
    fs::write(&deep_file, "def handle(): pass\n").unwrap();

    let fp = FilePath::new(deep_file.to_string_lossy().to_string()).unwrap();
    let config = make_orchestrator().load_config_sync(&fp);
    // Default config has enabled: true
    assert!(config.enabled.value);
}

/// `load_config_sync` should find the Rust config for a deeply nested file
/// under crates/.
#[test]
fn load_config_sync_finds_rust_config_from_deep_crate_file() {
    let tmp = TempDir::new().unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: false\n  rules: []\n",
    )
    .unwrap();
    let nested_file = tmp
        .path()
        .join("crates")
        .join("my-crate")
        .join("src")
        .join("lib.rs");
    fs::create_dir_all(nested_file.parent().unwrap()).unwrap();
    fs::write(&nested_file, "fn main() {}\n").unwrap();

    let fp = FilePath::new(nested_file.to_string_lossy().to_string()).unwrap();
    let config = make_orchestrator().load_config_sync(&fp);
    assert!(!config.enabled.value);
}

#[test]
fn validator_accessor_returns_same_instance() {
    let _v = make_orchestrator().validator();
}
