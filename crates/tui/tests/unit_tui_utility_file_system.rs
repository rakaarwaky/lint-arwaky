// Unit tests — TUI filesystem utility tests.
use shared::common::FilePath;
use shared::filesystem::contract_filesystem_io_protocol::IFileSystemIOProtocol;
use std::fs;
use std::sync::Arc;
use tempfile::TempDir;
use tui_lint_arwaky::utility_file_system;

fn make_fs() -> Arc<dyn IFileSystemIOProtocol> {
    filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
}

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
fn list_directory_returns_entries() {
    let tmp = TempDir::new().unwrap();
    let file_path = tmp.path().join("test_file.txt");
    fs::write(&file_path, "hello").unwrap();
    let dir_path = tmp.path().join("test_dir");
    fs::create_dir(&dir_path).unwrap();

    let path = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let entries = utility_file_system::list_directory(&path, &*make_fs());

    assert!(!entries.is_empty());
    let names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    assert!(names.contains(&"test_file.txt".to_string()));
    assert!(names.contains(&"test_dir".to_string()));
}

#[test]
fn list_directory_skips_hidden_files() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join(".hidden_file"), "hidden").unwrap();
    fs::write(tmp.path().join("visible_file"), "visible").unwrap();

    let path = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let entries = utility_file_system::list_directory(&path, &*make_fs());

    let names: Vec<String> = entries.iter().map(|e| e.name.clone()).collect();
    assert!(!names.contains(&".hidden_file".to_string()));
    assert!(names.contains(&"visible_file".to_string()));
}

#[test]
fn list_directory_nonexistent_path() {
    let path = FilePath::new("/nonexistent/path".to_string()).unwrap();
    let entries = utility_file_system::list_directory(&path, &*make_fs());
    assert!(entries.is_empty());
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
fn read_file_preview_shows_content() {
    let tmp = TempDir::new().unwrap();
    let file_path = tmp.path().join("preview.txt");
    fs::write(&file_path, "line1\nline2\nline3").unwrap();

    let path = FilePath::new(file_path.to_string_lossy().to_string()).unwrap();
    let result = utility_file_system::read_file_preview(&path, 10, &*make_fs());

    assert!(result.value.contains("line1"));
    assert!(result.value.contains("line2"));
    assert!(result.value.contains("line3"));
}

#[test]
fn read_file_preview_truncates() {
    let tmp = TempDir::new().unwrap();
    let file_path = tmp.path().join("long.txt");
    let content: String = (1..=100).map(|i| format!("line {i}\n")).collect();
    fs::write(&file_path, &content).unwrap();

    let path = FilePath::new(file_path.to_string_lossy().to_string()).unwrap();
    let result = utility_file_system::read_file_preview(&path, 5, &*make_fs());

    assert!(result.value.contains("100 more lines"));
    // Should only contain first 5 lines
    let line_count = result.value.lines().count();
    // 5 content lines + 1 blank line + truncation note
    assert!(line_count <= 8);
}

#[test]
fn read_file_preview_nonexistent_file() {
    let path = FilePath::new("/nonexistent/file.txt".to_string()).unwrap();
    let result = utility_file_system::read_file_preview(&path, 10, &*make_fs());
    assert!(result.value.contains("Cannot read file"));
}

#[test]
fn copy_text_to_clipboard_returns_bool() {
    // Just verify it doesn't panic; actual clipboard access may fail in CI
    let result = utility_file_system::copy_text_to_clipboard("test");
    // Result is a bool, no assertion needed — just ensuring it doesn't panic
    let _ = result;
}
