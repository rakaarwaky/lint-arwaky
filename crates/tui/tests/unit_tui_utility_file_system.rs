// Unit tests — TUI filesystem utility tests.
use shared::common::FilePath;
use std::fs;
use tempfile::TempDir;
use tui_lint_arwaky::utility_file_system;

#[test]
fn file_size_human_bytes() {
    let result = utility_file_system::file_size_human(512);
    assert_eq!(result.value(), "512B");
}

#[test]
fn file_size_human_kilobytes() {
    let result = utility_file_system::file_size_human(1536);
    assert!(result.value().contains("K"));
}

#[test]
fn file_size_human_megabytes() {
    let result = utility_file_system::file_size_human(5 * 1024 * 1024);
    assert!(result.value().contains("M"));
}

#[test]
fn file_size_human_gigabytes() {
    let result = utility_file_system::file_size_human(2 * 1024 * 1024 * 1024);
    assert!(result.value().contains("G"));
}

#[test]
fn file_size_human_zero() {
    let result = utility_file_system::file_size_human(0);
    assert_eq!(result.value(), "0B");
}

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
fn path_components_splits_correctly() {
    let path = FilePath::new("/home/user/file.rs".to_string()).unwrap();
    let components = utility_file_system::path_components(&path);
    assert!(!components.is_empty());
    // Should have multiple components for an absolute path
    assert!(components.len() >= 3);
}

#[test]
fn copy_text_to_clipboard_returns_bool() {
    // Just verify it doesn't panic; actual clipboard access may fail in CI
    let result = utility_file_system::copy_text_to_clipboard("test");
    // Result is a bool, no assertion needed — just ensuring it doesn't panic
    let _ = result;
}
