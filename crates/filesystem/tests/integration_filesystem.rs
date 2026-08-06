// Integration tests — full DI wiring via FilesystemContainer.
use filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::PatternList;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tempfile::TempDir;

#[test]
fn container_creates_default() {
    let _ = FilesystemContainer::new();
}

#[test]
fn container_default_equivalent_to_new() {
    let _ = FilesystemContainer::new().orchestrator();
    let _ = FilesystemContainer::default().orchestrator();
}

#[test]
fn container_orchestrator_is_arc_trait_object() {
    let orch = FilesystemContainer::new().orchestrator();
    let _: Arc<dyn IFilesystemAggregate> = orch;
}

#[test]
fn orchestrator_io_delegates_to_capability() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.txt");
    std::fs::write(&file, "hello").unwrap();
    assert!(orch.path_exists(&file));
}

#[test]
fn orchestrator_workspace_detects_language() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let lang = orch.detect_language_from_path("src/main.rs");
    assert_eq!(
        lang,
        shared::common::taxonomy_config_language_vo::ConfigLanguage::Rust
    );
}

#[test]
fn orchestrator_tool_resolution_checks_path() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let name = shared::filesystem::taxonomy_filesystem_vo::ToolName::new("sh").unwrap();
    assert!(orch.is_binary_available(&name));
}

#[test]
fn orchestrator_file_list_initially_empty() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    assert!(orch.file_list().is_empty());
}

#[test]
fn orchestrator_import_list_initially_empty() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    assert!(orch.import_list().is_empty());
}

#[test]
fn orchestrator_parse_warnings_initially_empty() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    assert!(orch.parse_warnings().is_empty());
}

#[test]
fn orchestrator_read_cached_returns_empty_for_missing() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new("/nonexistent".to_string()).unwrap();
    let content = orch.read_cached(&fp);
    assert!(content.value.is_empty());
}

#[test]
fn orchestrator_has_file_false_before_scan() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    assert!(!orch.has_file(Path::new("/any/path.rs")));
}

#[test]
fn orchestrator_symbol_definitions_initially_empty() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    assert!(orch.symbol_definitions().is_empty());
}

#[test]
fn orchestrator_implementations_initially_empty() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    assert!(orch.implementations().is_empty());
}

#[test]
fn orchestrator_reverse_links_initially_empty() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    assert!(orch.reverse_links().is_empty());
}

#[test]
fn orchestrator_collect_file_entries_reads_from_disk() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.rs");
    std::fs::write(&file, "fn main() {}").unwrap();
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let entries = orch.collect_file_entries(&PatternList::new(vec![file.to_string_lossy().to_string()]));
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].content, "fn main() {}");
}

#[test]
fn orchestrator_timing_returns_default() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let timing = orch.timing();
    assert_eq!(timing.total_ms, 0);
}

#[test]
fn orchestrator_extract_imports_from_snippet() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let imports = orch.extract(
        &PathBuf::from("/test.rs"),
        "use std::collections::HashMap;\n",
        shared::common::taxonomy_language_vo::Language::Rust,
    );
    assert!(!imports.is_empty());
}

#[test]
fn orchestrator_run_git_version() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let result = orch.run_git_command(&["version"], ".");
    assert!(result.success);
    assert!(result.stdout.contains("git version"));
}

#[test]
fn orchestrator_parse_output_lines_filters_empty() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let result = orch.parse_output_lines("a\n\nb\n  \nc\n");
    assert_eq!(result.lines, vec!["a", "b", "c"]);
}
