// Unit tests — TUI filesystem utility tests.
use shared::common::FilePath;
use std::fs;
use tempfile::TempDir;
use tui_lint_arwaky::utility_file_system;

#[test]
fn is_valid_directory_true() {
    let tmp = TempDir::new().unwrap();
    let path = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    assert!(utility_file_system::is_valid_directory(&path));
}

#[test]
fn is_valid_directory_false_for_file() {
    let tmp = TempDir::new().unwrap();
    let file_path = tmp.path().join("test.txt");
    fs::write(&file_path, "content").unwrap();
    let path = FilePath::new(file_path.to_string_lossy().to_string()).unwrap();
    assert!(!utility_file_system::is_valid_directory(&path));
}

#[test]
fn parent_directory_returns_parent() {
    let path = FilePath::new("/home/user/file.rs".to_string()).unwrap();
    let parent = utility_file_system::parent_directory(&path);
    assert!(parent.is_some());
    assert_eq!(parent.unwrap().value(), "/home/user");
}

#[test]
fn parent_directory_root_returns_none() {
    let path = FilePath::new("/".to_string()).unwrap();
    let parent = utility_file_system::parent_directory(&path);
    assert!(parent.is_none());
}

#[test]
fn copy_text_to_clipboard_returns_bool() {
    // Just verify it doesn't panic; actual clipboard access may fail in CI
    let result = utility_file_system::copy_text_to_clipboard("test");
    // Result is a bool, no assertion needed — just ensuring it doesn't panic
    let _ = result;
}
