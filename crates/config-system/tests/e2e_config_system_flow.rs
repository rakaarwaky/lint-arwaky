// E2E tests — full config lifecycle from filesystem to validated output.
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::contract_reader_protocol::IConfigReaderProtocol;
use shared::config_system::taxonomy_config_language_vo::ConfigLanguage;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn full_config_lifecycle_rust_workspace() {
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
    fs::write(root.join("lint_arwaky.config.rust.yaml"), config_yaml).unwrap();
    let container = ConfigContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();
    let result = orch.load_project_config(&fp).await;
    assert_eq!(result.source.language, "rust");
    assert!(result.config.enabled.value);
    assert!(!result.config.layers.is_empty());
    let workspaces = orch.discover_workspaces(&fp).await;
    assert_eq!(workspaces.len(), 2);
    let ws_names: Vec<String> = workspaces.iter().map(|w| w.path.basename()).collect();
    assert!(ws_names.contains(&"core".to_string()));
    assert!(ws_names.contains(&"cli".to_string()));
    let ignored = orch.ignored_paths(root.to_str().unwrap());
    assert!(ignored.contains(&"target".to_string()));
    assert!(ignored.contains(&".git".to_string()));
}

#[tokio::test]
async fn full_config_lifecycle_typescript_fallback() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    fs::write(root.join("package.json"), r#"{"name": "my-app"}"#).unwrap();
    fs::write(
        root.join("lint_arwaky.config.javascript.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();
    let result = ConfigContainer::new()
        .orchestrator()
        .load_config_for_language(&fp, ConfigLanguage::TypeScript)
        .await;
    assert_eq!(result.source.language, "typescript");
    assert!(result.source.path.value.contains("javascript"));
}

#[tokio::test]
async fn e2e_reader_lists_multi_language_configs() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.rust.yaml"), "a: 1").unwrap();
    fs::write(tmp.path().join("lint_arwaky.config.python.yaml"), "b: 2").unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let files = ConfigContainer::new()
        .reader()
        .list_config_files(&fp)
        .await
        .unwrap();
    assert_eq!(files.len(), 2);
    let langs: Vec<ConfigLanguage> = files.iter().map(|(l, _)| *l).collect();
    assert!(langs.contains(&ConfigLanguage::Rust));
    assert!(langs.contains(&ConfigLanguage::Python));
}
