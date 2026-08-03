// Unit tests for FileAdapter — read, write, and path_exists operations.
use auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter;
use shared::auto_fix::IFileAdapterProtocol;
use shared::common::{ContentString, FilePath};
use std::sync::Arc;
use tempfile::TempDir;

fn make_adapter() -> (FileAdapter, tempfile::TempDir) {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let tmp = TempDir::new().unwrap();
    let adapter = FileAdapter::new(filesystem);
    (adapter, tmp)
}

#[test]
fn read_file_returns_content() {
    let (adapter, tmp) = make_adapter();
    let file = tmp.path().join("test_read.txt");
    std::fs::write(&file, "hello world").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let result = adapter.read_file(&fp);
    assert!(result.is_some());
    assert_eq!(result.unwrap().value(), "hello world");
}

#[test]
fn read_file_returns_none_for_missing() {
    let (adapter, tmp) = make_adapter();
    let fp = FilePath::new(tmp.path().join("nonexistent.txt").to_string_lossy().to_string()).unwrap();

    assert!(adapter.read_file(&fp).is_none());
}

#[test]
fn write_file_creates_file() {
    let (adapter, tmp) = make_adapter();
    let file = tmp.path().join("test_write.txt");
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let content = ContentString::new("written content".to_string());
    assert!(adapter.write_file(&fp, &content));

    let read = adapter.read_file(&fp).unwrap();
    assert_eq!(read.value(), "written content");
}

#[test]
fn write_file_overwrites_existing() {
    let (adapter, tmp) = make_adapter();
    let file = tmp.path().join("test_overwrite.txt");
    std::fs::write(&file, "original").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    let content = ContentString::new("overwritten".to_string());
    assert!(adapter.write_file(&fp, &content));

    let read = adapter.read_file(&fp).unwrap();
    assert_eq!(read.value(), "overwritten");
}

#[test]
fn path_exists_true_for_existing() {
    let (adapter, tmp) = make_adapter();
    let file = tmp.path().join("exists.txt");
    std::fs::write(&file, "data").unwrap();
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    assert!(adapter.path_exists(&fp));
}

#[test]
fn path_exists_false_for_missing() {
    let (adapter, tmp) = make_adapter();
    let fp = FilePath::new(tmp.path().join("nope.txt").to_string_lossy().to_string()).unwrap();

    assert!(!adapter.path_exists(&fp));
}
