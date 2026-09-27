// E2E tests — full config lifecycle from filesystem to validated output.
mod common;

use shared::common::FilePath;
use shared::config_system::ConfigLanguage;
use shared::config_system::ConfigRequest;
use std::fs;
use tempfile::TempDir;

#[test]
fn full_config_lifecycle_rust_workspace() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"crates/*\"]\n",
    )
    .unwrap();
    let crates = root.join("crates");
    fs::create_dir_all(crates.join("core")).unwrap();
    fs::write(
        crates.join("core").join("Cargo.toml"),
        "[package]\nname=\"core\"\n",
    )
    .unwrap();
    fs::create_dir_all(crates.join("cli")).unwrap();
    fs::write(
        crates.join("cli").join("Cargo.toml"),
        "[package]\nname=\"cli\"\n",
    )
    .unwrap();
    let config_yaml = r#"architecture:
  enabled: true
  layers:
    taxonomy:
      prefix: taxonomy_
      suffix:
        - strict: [vo, entity, event, error, constant]
    capabilities:
      prefix: capabilities_
      suffix:
        - strict: [validator, reader, detector, provider]
  rules: []
  ignored_paths:
    - target
    - .git
"#;
    fs::write(root.join("lint_arwaky.config.yaml"), config_yaml).unwrap();
    let container = common::make_container();
    let orch = container.orchestrator();
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();
    let result = orch
        .execute(ConfigRequest::load_project_config(&fp))
        .into_config_result();
    assert_eq!(result.source.language, "rust");
    assert!(result.config.enabled.value);
    assert!(!result.config.layers.is_empty());
    let workspaces = orch
        .execute(ConfigRequest::discover_workspaces(&fp))
        .into_workspaces();
    assert_eq!(workspaces.len(), 2);
    let ws_names: Vec<String> = workspaces.iter().map(|w| w.path.basename()).collect();
    assert!(ws_names.contains(&"core".to_string()));
    assert!(ws_names.contains(&"cli".to_string()));
    let ignored = orch
        .execute(ConfigRequest::ignored_paths(&fp))
        .into_patterns();
    assert!(ignored.values.contains(&"target".to_string()));
    assert!(ignored.values.contains(&".git".to_string()));
}

#[test]
fn full_config_lifecycle_typescript_fallback() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    fs::write(root.join("package.json"), r#"{"name": "my-app"}"#).unwrap();
    fs::write(
        root.join("lint_arwaky.config.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();
    let result = common::make_container()
        .orchestrator()
        .execute(ConfigRequest::load_for_language(
            &fp,
            ConfigLanguage::TypeScript,
        ))
        .into_config_result();
    assert_eq!(result.source.language, "typescript");
    assert!(result.source.path.value.contains("lint_arwaky.config.yaml"));
}

#[test]
fn e2e_reader_lists_multi_language_configs() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.yaml"), "a: 1").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let files = common::make_container()
        .lister()
        .list_config_files(&fp)
        .unwrap();
    // Unified config: all languages share one file, so list returns 1 entry
    assert_eq!(files.len(), 1);
}
