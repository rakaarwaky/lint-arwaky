// Smoke test — verify the filesystem crate boots and core operations respond within 5s.
use filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer;
use shared::common::taxonomy_language_vo::Language;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn filesystem_boots_and_container_creates() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let _ = orch.file_list();
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5, "Smoke test exceeded 5s: {:?}", elapsed);
}

#[test]
fn filesystem_io_operations_respond() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("smoke.txt");
    orch.write_string(&file, "smoke test").unwrap();
    let content = orch.read_to_string(&file).unwrap();
    assert_eq!(content, "smoke test");
    orch.remove_file(&file).unwrap();
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5, "Smoke test exceeded 5s: {:?}", elapsed);
}

#[test]
fn filesystem_parse_operations_respond() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let mut files = vec![FileEntry {
        path: PathBuf::from("/smoke.rs"),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: 20,
        content: "fn main() {}".to_string(),
        parse_ok: false,
        parse_metadata: None,
    }];
    orch.parse_all(&mut files);
    assert!(files[0].parse_ok);
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5, "Smoke test exceeded 5s: {:?}", elapsed);
}

#[test]
fn filesystem_workspace_detection_responds() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let lang = orch.detect_language_from_path("src/main.rs");
    assert_eq!(lang, shared::common::taxonomy_config_language_vo::ConfigLanguage::Rust);
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5, "Smoke test exceeded 5s: {:?}", elapsed);
}

#[test]
fn filesystem_tool_resolution_responds() {
    let start = std::time::Instant::now();
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let name = shared::filesystem::taxonomy_filesystem_vo::ToolName::new("sh").unwrap();
    assert!(orch.is_binary_available(&name));
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5, "Smoke test exceeded 5s: {:?}", elapsed);
}

#[test]
fn filesystem_scan_directory_responds() {
    let start = std::time::Instant::now();
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("a.rs"), "").unwrap();
    std::fs::write(tmp.path().join("b.py"), "").unwrap();
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let files = orch.scan_directory_with_ignored(tmp.path(), &[]);
    assert!(!files.is_empty());
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs() < 5, "Smoke test exceeded 5s: {:?}", elapsed);
}
