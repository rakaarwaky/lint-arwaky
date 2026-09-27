// Integration tests — full DI wiring via FilesystemContainer.
use filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer;
use shared::common::PatternList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::FilesystemRequest;
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
fn io_capability_reports_existing_path() {
    let container = FilesystemContainer::new();
    let io = container.io();
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.txt");
    std::fs::write(&file, "hello").unwrap();
    assert!(io.path_exists(&file));
}

#[test]
fn workspace_capability_detects_language() {
    let container = FilesystemContainer::new();
    let workspace = container.workspace();
    let lang = workspace.detect_language_from_path("src/main.rs");
    assert_eq!(
        lang,
        shared::common::taxonomy_config_language_vo::ConfigLanguage::Rust
    );
}

#[test]
fn tool_resolution_capability_checks_path() {
    let container = FilesystemContainer::new();
    let tools = container.tool_resolution();
    let name = shared::common::taxonomy_tool_name_vo::ToolName::new("sh");
    assert!(tools.is_binary_available(&name));
}

#[test]
fn orchestrator_file_list_initially_empty() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    assert!(
        orch.execute(FilesystemRequest::FileList)
            .into_file_list()
            .is_empty()
    );
}

#[test]
fn parser_capability_import_list_initially_empty() {
    let container = FilesystemContainer::new();
    let parser = container.parser();
    assert!(parser.import_list().is_empty());
}

#[test]
fn parser_capability_warnings_initially_empty() {
    let container = FilesystemContainer::new();
    let parser = container.parser();
    assert!(parser.parse_warnings().is_empty());
}

#[test]
fn orchestrator_read_cached_returns_empty_for_missing() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let fp = FilePath::new("/nonexistent".to_string()).unwrap();
    let content = orch
        .execute(FilesystemRequest::read_cached(&fp))
        .into_content();
    assert!(content.value.is_empty());
}

#[test]
fn orchestrator_has_file_false_before_scan() {
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    assert!(
        !orch
            .execute(FilesystemRequest::has_file(Path::new("/any/path.rs")))
            .into_exists()
    );
}

#[test]
fn graph_capability_definitions_initially_empty() {
    let container = FilesystemContainer::new();
    let graph = container.graph();
    assert!(graph.symbol_definitions().is_empty());
}

#[test]
fn graph_capability_implementations_initially_empty() {
    let container = FilesystemContainer::new();
    let graph = container.graph();
    assert!(graph.implementations().is_empty());
}

#[test]
fn graph_capability_reverse_links_initially_empty() {
    let container = FilesystemContainer::new();
    let graph = container.graph();
    assert!(graph.reverse_links().is_empty());
}

#[test]
fn orchestrator_collect_file_entries_reads_from_disk() {
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.rs");
    std::fs::write(&file, "fn main() {}").unwrap();
    let container = FilesystemContainer::new();
    let orch = container.orchestrator();
    let entries = orch
        .execute(FilesystemRequest::collect_file_entries(&PatternList::new(
            vec![file.to_string_lossy().to_string()],
        )))
        .into_pairs();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].content, "fn main() {}");
}

#[test]
fn io_capability_timing_returns_default() {
    let container = FilesystemContainer::new();
    let io = container.io();
    let timing = io.timing();
    assert_eq!(timing.total_ms, 0);
}

#[test]
fn parser_capability_extracts_imports_from_snippet() {
    let container = FilesystemContainer::new();
    let parser = container.parser();
    let imports = parser.extract(
        &PathBuf::from("/test.rs"),
        "use std::collections::HashMap;\n",
        shared::common::taxonomy_language_vo::Language::Rust,
    );
    assert!(!imports.is_empty());
}

#[test]
fn io_capability_runs_git_version() {
    let container = FilesystemContainer::new();
    let io = container.io();
    let result = io.run_git_command(&["version"], ".");
    assert!(result.success);
    assert!(result.stdout.contains("git version"));
}

#[test]
fn io_capability_parse_output_lines_filters_empty() {
    let container = FilesystemContainer::new();
    let io = container.io();
    let result = io.parse_output_lines("a\n\nb\n  \nc\n");
    assert_eq!(result.lines, vec!["a", "b", "c"]);
}
