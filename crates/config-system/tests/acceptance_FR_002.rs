// FR-002 — Multi-Language Support
mod common;

use shared::common::FilePath;
use std::fs;
use tempfile::TempDir;

#[test]
fn us2_rust_workspace_loads_rust_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(
        common::make_container()
            .orchestrator()
            .load_project_config(&fp)
            .source
            .language,
        "rust"
    );
}

#[test]
fn us2_python_workspace_loads_python_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("pyproject.toml"), "[project]\nname=\"x\"\n").unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(
        common::make_container()
            .orchestrator()
            .load_project_config(&fp)
            .source
            .language,
        "python"
    );
}

#[test]
fn us2_typescript_workspace_loads_typescript_config() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join("package.json"), r#"{"name":"x"}"#).unwrap();
    fs::write(
        tmp.path().join("lint_arwaky.config.yaml"),
        "architecture:\n  enabled: true\n  rules: []\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert_eq!(
        common::make_container()
            .orchestrator()
            .load_project_config(&fp)
            .source
            .language,
        "typescript"
    );
}
