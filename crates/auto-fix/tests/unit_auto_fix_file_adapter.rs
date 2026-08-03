// Unit tests for FileAdapter — path_exists operation via filesystem aggregate.
use auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter;
use shared::auto_fix::IFileAdapterProtocol;
use shared::common::FilePath;
use tempfile::TempDir;

fn make_adapter() -> (FileAdapter, tempfile::TempDir) {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let tmp = TempDir::new().unwrap();
    let adapter = FileAdapter::new(filesystem);
    (adapter, tmp)
}

#[test]
fn path_exists_true_for_existing_file() {
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

#[test]
fn path_exists_true_for_directory() {
    let (adapter, tmp) = make_adapter();
    let dir = tmp.path().join("subdir");
    std::fs::create_dir(&dir).unwrap();
    let fp = FilePath::new(dir.to_string_lossy().to_string()).unwrap();
    assert!(adapter.path_exists(&fp));
}

#[test]
fn path_exists_false_for_empty_path() {
    let (adapter, _tmp) = make_adapter();
    let fp = FilePath::new("/nonexistent/path/that/does/not/exist.rs".to_string()).unwrap();
    assert!(!adapter.path_exists(&fp));
}
