use std::fs;
use std::sync::Arc;

use import_rules_lint_arwaky::infrastructure_filesystem_adapter::OSFileSystemAdapter;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::contract_system_port::IFileSystemPort;
use shared::common::taxonomy_common_vo::PatternList;
use shared::taxonomy_layer_vo::Identity;
use shared::taxonomy_source_vo::ContentString;

fn make_fp(s: &str) -> FilePath {
    FilePath::new(s.to_string()).unwrap_or_default()
}

fn setup_temp_dir(name: &str) -> (tempfile::TempDir, FilePath) {
    let dir = tempfile::tempdir().expect("failed to create temp dir");
    let fp = make_fp(dir.path().to_string_lossy().as_ref());
    (dir, fp)
}

// ---------------------------------------------------------------------------
// exists
// ---------------------------------------------------------------------------

#[tokio::test]
async fn exists_returns_true_for_existing_file() {
    let adapter = OSFileSystemAdapter::new();
    let (dir, _) = setup_temp_dir("exists_test");
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "hello").unwrap();
    let fp = make_fp(&file_path.to_string_lossy());
    let result = adapter.exists(&fp).await;
    assert!(result.value());
}

#[tokio::test]
async fn exists_returns_false_for_nonexistent() {
    let adapter = OSFileSystemAdapter::new();
    let fp = make_fp("/nonexistent_path_xyz_12345");
    let result = adapter.exists(&fp).await;
    assert!(!result.value());
}

// ---------------------------------------------------------------------------
// is_file / is_directory
// ---------------------------------------------------------------------------

#[tokio::test]
async fn is_file_true_for_file() {
    let adapter = OSFileSystemAdapter::new();
    let (dir, _) = setup_temp_dir("is_file_test");
    let file_path = dir.path().join("file.txt");
    fs::write(&file_path, "data").unwrap();
    let fp = make_fp(&file_path.to_string_lossy());
    assert!(adapter.is_file(&fp).await.value());
    assert!(!adapter.is_directory(&fp).await.value());
}

// ---------------------------------------------------------------------------
// read_text / read_file
// ---------------------------------------------------------------------------

#[tokio::test]
async fn read_text_returns_content() {
    let adapter = OSFileSystemAdapter::new();
    let (dir, _) = setup_temp_dir("read_test");
    let file_path = dir.path().join("hello.txt");
    fs::write(&file_path, "Hello, World!").unwrap();
    let fp = make_fp(&file_path.to_string_lossy());
    let content = adapter.read_text(&fp).await.unwrap();
    assert_eq!(content.value(), "Hello, World!");
}

#[tokio::test]
async fn read_file_nonexistent_returns_error() {
    let adapter = OSFileSystemAdapter::new();
    let fp = make_fp("/nonexistent_read_file");
    let result = adapter.read_file(&fp).await;
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// write_text
// ---------------------------------------------------------------------------

#[tokio::test]
async fn write_text_creates_file() {
    let adapter = OSFileSystemAdapter::new();
    let (dir, _) = setup_temp_dir("write_test");
    let file_path = dir.path().join("output.txt");
    let fp = make_fp(&file_path.to_string_lossy());
    let result = adapter.write_text(&fp, &ContentString::new("test content"), None).await;
    assert!(result.is_ok());
    assert!(result.unwrap().value());
    assert!(file_path.exists());
    let read_back = fs::read_to_string(&file_path).unwrap();
    assert_eq!(read_back, "test content");
}

// ---------------------------------------------------------------------------
// get_line_count
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_line_count_returns_correct_count() {
    let adapter = OSFileSystemAdapter::new();
    let (dir, _) = setup_temp_dir("line_count_test");
    let file_path = dir.path().join("lines.txt");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();
    let fp = make_fp(&file_path.to_string_lossy());
    assert_eq!(adapter.get_line_count(&fp).await.value(), 3);
}

#[tokio::test]
async fn get_line_count_nonexistent_returns_zero() {
    let adapter = OSFileSystemAdapter::new();
    let fp = make_fp("/nonexistent_lines");
    assert_eq!(adapter.get_line_count(&fp).await.value(), 0);
}

// ---------------------------------------------------------------------------
// get_basename
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_basename_returns_filename() {
    let adapter = OSFileSystemAdapter::new();
    let fp = make_fp("/home/user/project/src/main.rs");
    assert_eq!(adapter.get_basename(&fp).await.value(), "main.rs");
}

// ---------------------------------------------------------------------------
// get_relative_path
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_relative_path_computes_correctly() {
    let adapter = OSFileSystemAdapter::new();
    let base = make_fp("/home/user/project");
    let full = make_fp("/home/user/project/src/main.rs");
    let rel = adapter.get_relative_path(&full, &base).await;
    assert_eq!(rel.value(), "src/main.rs");
}

#[tokio::test]
async fn get_relative_path_no_prefix_returns_original() {
    let adapter = OSFileSystemAdapter::new();
    let base = make_fp("/different/path");
    let full = make_fp("/home/user/project/src/main.rs");
    let rel = adapter.get_relative_path(&full, &base).await;
    assert_eq!(rel.value(), full.value());
}

// ---------------------------------------------------------------------------
// get_parent
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_parent_deep_path() {
    let adapter = OSFileSystemAdapter::new();
    let fp = make_fp("/home/user/project/src/main.rs");
    assert_eq!(adapter.get_parent(&fp).await.value(), "/home/user/project/src");
}

#[tokio::test]
async fn get_parent_root_returns_self() {
    let adapter = OSFileSystemAdapter::new();
    let fp = make_fp("/");
    assert_eq!(adapter.get_parent(&fp).await.value(), "/");
}

// ---------------------------------------------------------------------------
// path_join
// ---------------------------------------------------------------------------

#[tokio::test]
async fn path_join_combines_parts() {
    let adapter = OSFileSystemAdapter::new();
    let parts = vec![
        Identity::new("home"),
        Identity::new("user"),
        Identity::new("project"),
    ];
    let result = adapter.path_join(&parts).await;
    assert_eq!(result.value(), "home/user/project");
}

// ---------------------------------------------------------------------------
// walk
// ---------------------------------------------------------------------------

#[tokio::test]
async fn walk_finds_all_files() {
    let adapter = OSFileSystemAdapter::new();
    let (dir, fp) = setup_temp_dir("walk_test");
    fs::write(dir.path().join("a.rs"), "").unwrap();
    fs::write(dir.path().join("b.py"), "").unwrap();
    let _ = fs::create_dir(dir.path().join("sub"));
    fs::write(dir.path().join("sub/c.js"), "").unwrap();

    let files = adapter.walk(&fp, None).await;
    // Should find 3 files (a.rs, b.py, sub/c.js)
    assert_eq!(files.values.len(), 3);
}

#[tokio::test]
async fn walk_ignores_patterns() {
    let adapter = OSFileSystemAdapter::new();
    let (dir, fp) = setup_temp_dir("walk_ignore");
    fs::write(dir.path().join("keep.rs"), "").unwrap();
    fs::write(dir.path().join("ignore.py"), "").unwrap();
    fs::write(dir.path().join("target"), "").unwrap();

    let files = adapter.walk(
        &fp,
        Some(&PatternList::new(vec!["target".to_string(), "node_modules".to_string()])),
    ).await;
    // Should find 2 files (keep.rs, ignore.py) - target is ignored by name
    // Note: the walk ignores by file name alone, not extension
    assert_eq!(files.values.len(), 2);
}

// ---------------------------------------------------------------------------
// get_cwd
// ---------------------------------------------------------------------------

#[tokio::test]
async fn get_cwd_returns_non_empty() {
    let adapter = OSFileSystemAdapter::new();
    let cwd = adapter.get_cwd().await;
    assert!(!cwd.value().is_empty(), "cwd should not be empty");
}
