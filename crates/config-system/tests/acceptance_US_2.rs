// US-2 — Multi-Language Support
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn us2_rust_workspace_loads_rust_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.rust.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(
        ConfigContainer::new()
            .orchestrator()
            .load_project_config(&fp)
            .await
            .source
            .language,
        "rust"
    );
}

#[tokio::test]
async fn us2_python_workspace_loads_python_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("pyproject.toml"), "[project]\nname=\"x\"\n").unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.python.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(
        ConfigContainer::new()
            .orchestrator()
            .load_project_config(&fp)
            .await
            .source
            .language,
        "python"
    );
}

#[tokio::test]
async fn us2_typescript_workspace_loads_typescript_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("package.json"), r#"{"name":"x"}"#).unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.typescript.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(
        ConfigContainer::new()
            .orchestrator()
            .load_project_config(&fp)
            .await
            .source
            .language,
        "typescript"
    );
}
