//! Unit tests for surface_check_action — find_workspace_root, handle_default_check.

use shared::cli_commands::utility_path_resolver::find_workspace_root;

// ─── find_workspace_root ─────────────────────────────────────────────────────

#[test]
fn find_workspace_root_returns_none_for_nonexistent_path() {
    let result = find_workspace_root("/nonexistent/path/xyz");
    assert!(result.is_none());
}

#[test]
fn find_workspace_root_detects_crates_directory() {
    // Create a temp directory structure with crates/
    let tmp = std::env::temp_dir().join(format!("test_ws_root_{}", std::process::id()));
    let crates_dir = tmp.join("crates");
    std::fs::create_dir_all(&crates_dir).unwrap();

    let result = find_workspace_root(tmp.to_str().unwrap());
    assert!(result.is_some());
    assert_eq!(result.unwrap(), tmp);

    // Cleanup
    std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn find_workspace_root_detects_packages_directory() {
    let tmp = std::env::temp_dir().join(format!("test_ws_pkg_{}", std::process::id()));
    let packages_dir = tmp.join("packages");
    std::fs::create_dir_all(&packages_dir).unwrap();

    let result = find_workspace_root(tmp.to_str().unwrap());
    assert!(result.is_some());
    assert_eq!(result.unwrap(), tmp);

    std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn find_workspace_root_detects_modules_directory() {
    let tmp = std::env::temp_dir().join(format!("test_ws_mod_{}", std::process::id()));
    let modules_dir = tmp.join("modules");
    std::fs::create_dir_all(&modules_dir).unwrap();

    let result = find_workspace_root(tmp.to_str().unwrap());
    assert!(result.is_some());
    assert_eq!(result.unwrap(), tmp);

    std::fs::remove_dir_all(&tmp).ok();
}

#[test]
fn find_workspace_root_walks_up_from_child() {
    let tmp = std::env::temp_dir().join(format!("test_ws_up_{}", std::process::id()));
    let child = tmp.join("crates").join("my-crate").join("src");
    std::fs::create_dir_all(&child).unwrap();

    let result = find_workspace_root(child.to_str().unwrap());
    assert!(result.is_some());
    assert_eq!(result.unwrap(), tmp);

    std::fs::remove_dir_all(&tmp).ok();
}
