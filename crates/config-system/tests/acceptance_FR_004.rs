// FR-004 — Multi-Workspace Analysis
use config_system_lint_arwaky::root_config_system_container::ConfigContainer;
use shared::common::taxonomy_path_vo::FilePath;
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn us4_discovers_all_workspace_member_types() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    fs::create_dir_all(root.join("crates").join("rust-lib")).unwrap();
    fs::create_dir_all(root.join("packages").join("ts-app")).unwrap();
    fs::create_dir_all(root.join("modules").join("py-mod")).unwrap();
    let fp = FilePath::new(root.to_string_lossy().to_string()).unwrap();
    let workspaces = ConfigContainer::new()
        .orchestrator()
        .discover_workspaces(&fp)
        .await;
    assert_eq!(workspaces.len(), 3);
    let names: Vec<String> = workspaces.iter().map(|w| w.path.basename()).collect();
    assert!(names.contains(&"rust-lib".to_string()));
    assert!(names.contains(&"ts-app".to_string()));
    assert!(names.contains(&"py-mod".to_string()));
}

#[tokio::test]
async fn us4_each_member_gets_own_config() {
    let tmp = TempDir::new().unwrap();
    let crate_dir = tmp.path().join("crates").join("my-crate");
    fs::create_dir_all(&crate_dir).unwrap();
    fs::write(
        crate_dir.join("Cargo.toml"),
        "[package]\nname=\"my-crate\"\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let workspaces = ConfigContainer::new()
        .orchestrator()
        .discover_workspaces(&fp)
        .await;
    assert_eq!(workspaces.len(), 1);
    assert_eq!(workspaces[0].workspace_type, "rust");
}

#[tokio::test]
async fn us4_empty_workspace_returns_empty() {
    let tmp = TempDir::new().unwrap();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(ConfigContainer::new()
        .orchestrator()
        .discover_workspaces(&fp)
        .await
        .is_empty());
}
