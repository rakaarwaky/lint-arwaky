use cli_commands_lint_arwaky::surface_common_command::{
    canonicalize_path, current_dir, resolve_file_path,
};

#[test]
fn resolve_file_path_creates_filepath() {
    let fp = resolve_file_path("test.rs");
    assert_eq!(fp.value, "test.rs");
}

#[test]
fn resolve_file_path_empty() {
    let fp = resolve_file_path("");
    assert!(fp.value.is_empty() || fp.value == "");
}

#[test]
fn canonicalize_path_returns_path_for_relative() {
    let path = canonicalize_path(".");
    assert!(!path.is_empty(), "canonicalize of '.' should return cwd");
}

#[test]
fn canonicalize_path_returns_original_for_bad_path() {
    let path = canonicalize_path("/nonexistent_path_xyz_123");
    assert_eq!(path, "/nonexistent_path_xyz_123");
}

#[test]
fn current_dir_returns_some_path() {
    let dir = current_dir();
    assert!(
        dir.as_os_str().len() > 0,
        "current_dir should return non-empty path"
    );
}
