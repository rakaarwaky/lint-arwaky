// Unit tests for CapabilitiesWorkspace — FR-005: Workspace Detection.
use filesystem_lint_arwaky::capabilities_workspace::CapabilitiesWorkspace;
use shared::common::taxonomy_config_language_vo::ConfigLanguage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use tempfile::TempDir;

fn make_workspace() -> CapabilitiesWorkspace {
    CapabilitiesWorkspace::new()
}

#[test]
fn detect_language_from_rust_path() {
    let ws = make_workspace();
    let lang = ws.detect_language_from_path("src/main.rs");
    assert_eq!(lang, ConfigLanguage::Rust);
}

#[test]
fn detect_language_from_python_path() {
    let ws = make_workspace();
    let lang = ws.detect_language_from_path("modules/auth/handler.py");
    assert_eq!(lang, ConfigLanguage::Python);
}

#[test]
fn detect_language_from_typescript_path() {
    let ws = make_workspace();
    let lang = ws.detect_language_from_path("packages/web/index.ts");
    assert_eq!(lang, ConfigLanguage::TypeScript);
}

#[test]
fn detect_language_from_unknown_defaults_to_rust() {
    let ws = make_workspace();
    let lang = ws.detect_language_from_path("data.json");
    // Unknown extensions default to Rust per utility_workspace_detection
    assert_eq!(lang, ConfigLanguage::Rust);
}

#[test]
fn is_member_path_with_cargo_toml() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "[package]\nname=\"x\"\n").unwrap();
    let ws = make_workspace();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(ws.is_member_path(&fp));
}

#[test]
fn is_member_path_without_manifest() {
    let tmp = TempDir::new().unwrap();
    let ws = make_workspace();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(!ws.is_member_path(&fp));
}

#[test]
fn detect_source_dir_prefers_crates() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("crates")).unwrap();
    let ws = make_workspace();
    let dir = ws.detect_source_dir(tmp.path());
    assert!(dir.ends_with("crates"));
}

#[test]
fn detect_source_dir_prefers_packages() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("packages")).unwrap();
    let ws = make_workspace();
    let dir = ws.detect_source_dir(tmp.path());
    assert!(dir.ends_with("packages"));
}

#[test]
fn detect_source_dir_falls_back_to_root() {
    let tmp = TempDir::new().unwrap();
    let ws = make_workspace();
    let dir = ws.detect_source_dir(tmp.path());
    assert_eq!(dir, tmp.path().to_path_buf());
}

#[test]
fn find_workspace_root_from_path_finds_cargo() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("Cargo.toml"),
        "[workspace]\nmembers=[\"crates/*\"]\n",
    )
    .unwrap();
    let nested = tmp.path().join("crates").join("my-crate").join("src");
    std::fs::create_dir_all(&nested).unwrap();
    let ws = make_workspace();
    let root = ws.find_workspace_root_from_path(&nested).unwrap();
    assert_eq!(root, tmp.path().to_path_buf());
}

#[test]
fn find_workspace_root_from_path_errors_when_no_manifest() {
    let tmp = TempDir::new().unwrap();
    let ws = make_workspace();
    let result = ws.find_workspace_root_from_path(tmp.path());
    assert!(result.is_err());
}

#[test]
fn check_wired_in_container_true() {
    let tmp = TempDir::new().unwrap();
    let crates_dir = tmp.path().join("crates");
    let my_crate = crates_dir.join("my-crate");
    std::fs::create_dir_all(&my_crate).unwrap();
    // check_wired_in_container looks for files ending in _container.rs / _entry.rs
    std::fs::write(
        my_crate.join("root_my_crate_container.rs"),
        "pub struct MyCrate;",
    )
    .unwrap();
    let ws = make_workspace();
    assert!(ws.check_wired_in_container(tmp.path(), &["MyCrate".to_string()]));
}

#[test]
fn check_wired_in_container_false() {
    let tmp = TempDir::new().unwrap();
    let crates_dir = tmp.path().join("crates");
    std::fs::create_dir_all(&crates_dir).unwrap();
    let ws = make_workspace();
    assert!(!ws.check_wired_in_container(tmp.path(), &["nonexistent".to_string()]));
}

#[test]
fn resolve_orphan_module_path_confined_under_root() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    let base_dir = root.join("src");
    std::fs::create_dir_all(&base_dir).unwrap();
    let ws = make_workspace();
    let result = ws.resolve_orphan_module_path(root, &base_dir, "module_a");
    assert!(result.is_some());
    let path = result.unwrap();
    assert!(path.starts_with(root));
}

#[test]
fn resolve_orphan_module_path_rejects_escape() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path();
    let base_dir = root.join("src");
    std::fs::create_dir_all(&base_dir).unwrap();
    let ws = make_workspace();
    let result = ws.resolve_orphan_module_path(root, &base_dir, "../../etc/passwd");
    assert!(result.is_none());
}
