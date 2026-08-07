// FR-005 — Workspace Detection
// US1: Workspace root detection by walking up.
// US2: Member path detection (Cargo.toml present).
// US3: Language detection from file path.
// US4: Leaf member detection.
// US5: Source directory detection.
// US6: Container wiring check.

use filesystem_lint_arwaky::capabilities_workspace_root_finder::CapabilitiesWorkspace;
use shared::common::taxonomy_config_language_vo::ConfigLanguage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::contract_workspace_protocol::IWorkspaceProtocol;
use tempfile::TempDir;

fn make_workspace() -> CapabilitiesWorkspace {
    CapabilitiesWorkspace::new()
}

#[test]
fn us1_finds_workspace_root_from_nested() {
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
fn us1_root_via_string_path() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
    let ws = make_workspace();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let root = ws.workspace_root(&fp);
    assert!(root.is_some());
}

#[test]
fn us2_member_path_with_cargo_toml() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
    let ws = make_workspace();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(ws.is_member_path(&fp));
}

#[test]
fn us2_not_member_without_manifest() {
    let tmp = TempDir::new().unwrap();
    let ws = make_workspace();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(!ws.is_member_path(&fp));
}

#[test]
fn us3_detect_rust_language() {
    let ws = make_workspace();
    assert_eq!(
        ws.detect_language_from_path("src/main.rs"),
        ConfigLanguage::Rust
    );
    assert_eq!(ws.detect_language_from_path("lib.rs"), ConfigLanguage::Rust);
}

#[test]
fn us3_detect_python_language() {
    let ws = make_workspace();
    assert_eq!(
        ws.detect_language_from_path("modules/handler.py"),
        ConfigLanguage::Python
    );
}

#[test]
fn us3_detect_typescript_language() {
    let ws = make_workspace();
    assert_eq!(
        ws.detect_language_from_path("packages/web/index.ts"),
        ConfigLanguage::TypeScript
    );
}

#[test]
fn us3_detect_unknown_defaults_to_rust() {
    let ws = make_workspace();
    // Unknown extensions default to Rust per utility_workspace_detection
    assert_eq!(
        ws.detect_language_from_path("data.json"),
        ConfigLanguage::Rust
    );
}

#[test]
fn us4_leaf_member_path() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
    let ws = make_workspace();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    // A single member with no sub-members is a leaf
    assert!(ws.is_leaf_member_path(&fp));
}

#[test]
fn us4_non_leaf_member() {
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("Cargo.toml"), "").unwrap();
    let sub = tmp.path().join("sub-crate");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(sub.join("Cargo.toml"), "").unwrap();
    let ws = make_workspace();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(!ws.is_leaf_member_path(&fp));
}

#[test]
fn us5_detect_source_dir_prefers_crates() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("crates")).unwrap();
    let ws = make_workspace();
    assert!(ws.detect_source_dir(tmp.path()).ends_with("crates"));
}

#[test]
fn us5_detect_source_dir_prefers_packages() {
    let tmp = TempDir::new().unwrap();
    std::fs::create_dir_all(tmp.path().join("packages")).unwrap();
    let ws = make_workspace();
    assert!(ws.detect_source_dir(tmp.path()).ends_with("packages"));
}

#[test]
fn us5_detect_source_dir_falls_back_to_root() {
    let tmp = TempDir::new().unwrap();
    let ws = make_workspace();
    assert_eq!(ws.detect_source_dir(tmp.path()), tmp.path().to_path_buf());
}

#[test]
fn us6_check_wired_in_container_true() {
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
    let pl = shared::common::taxonomy_common_vo::PatternList::new(vec!["MyCrate".to_string()]);
    assert!(ws.check_wired_in_container(tmp.path(), &pl));
}

#[test]
fn us6_check_wired_in_container_false() {
    let tmp = TempDir::new().unwrap();
    let crates_dir = tmp.path().join("crates");
    std::fs::create_dir_all(&crates_dir).unwrap();
    let ws = make_workspace();
    let pl = shared::common::taxonomy_common_vo::PatternList::new(vec!["ghost-crate".to_string()]);
    assert!(!ws.check_wired_in_container(tmp.path(), &pl));
}

#[test]
fn us6_resolve_orphan_module_within_root() {
    let tmp = TempDir::new().unwrap();
    let base_dir = tmp.path().join("src");
    std::fs::create_dir_all(&base_dir).unwrap();
    let ws = make_workspace();
    let result = ws.resolve_orphan_module_path(tmp.path(), &base_dir, "new_module");
    assert!(result.is_some());
    assert!(result.unwrap().starts_with(tmp.path()));
}

#[test]
fn us6_resolve_orphan_module_rejects_escape() {
    let tmp = TempDir::new().unwrap();
    let base_dir = tmp.path().join("src");
    std::fs::create_dir_all(&base_dir).unwrap();
    let ws = make_workspace();
    let result = ws.resolve_orphan_module_path(tmp.path(), &base_dir, "../../etc/passwd");
    assert!(result.is_none());
}
