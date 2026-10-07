// Smoke test — verify the filesystem crate boots and core operations respond within 5s.
use filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer;
use shared_common::PatternList;
use shared_common::taxonomy_language_vo::Language;
use shared_filesystem::FilesystemRequest;
use shared_filesystem::taxonomy_filesystem_vo::FileEntry;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn filesystem_boots_and_container_creates() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let _ = orch.execute(FilesystemRequest::FileList).into_file_list();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn filesystem_io_operations_respond() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let io = container.io();
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("smoke.txt");
    io.write_string(&file, "smoke test").unwrap();
    let content = io.read_to_string(&file).unwrap();
    assert_eq!(content.value, "smoke test");
    io.remove_file(&file).unwrap();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn filesystem_parse_operations_respond() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let parser = container.parser();
    let mut files = vec![FileEntry {
        path: PathBuf::from("/smoke.rs"),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: 20,
        content: "fn main() {}".to_string(),
        parse_ok: false,
        parse_metadata: None,
    }];
    parser.parse_all(&mut files);
    assert!(files[0].parse_ok);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn filesystem_workspace_detection_responds() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let workspace = container.workspace();
    let lang = workspace.detect_language_from_path("src/main.rs");
    assert_eq!(
        lang,
        shared_common::taxonomy_language_vo::ConfigLanguage::Rust
    );
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn filesystem_tool_resolution_responds() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let tools = container.tool_resolution();
    let name = shared_common::taxonomy_adapter_name_vo::ToolName::new("sh");
    assert!(tools.is_binary_available(&name));
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn filesystem_scan_directory_responds() {
    let start = std::time::Instant::now();
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.rs"), "").unwrap();
    std::fs::write(tmp.path().join("b.py"), "").unwrap();
    let container = FilesystemContainer::new();
    let io = container.io();
    let files = io.scan_directory_with_ignored(tmp.path(), &PatternList::default());
    assert!(!files.is_empty());
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
