// Integration tests — full DI wiring via AutoFixContainer with real quality-rules.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::IFileAdapterProtocol;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::ContentString;
use shared::common::FilePath;
use std::sync::Arc;
use tempfile::TempDir;

#[test]
fn container_creates_with_quality_rules() {
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let _ = container;
}

#[test]
fn container_orchestrator_with_filesystem() {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem);
    let _: Arc<dyn LintFixOrchestratorAggregate> = orch;
}

#[test]
fn container_orchestrator_with_custom_file_adapter() {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());

    let file_adapter: Arc<dyn IFileAdapterProtocol> =
        Arc::new(auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter::new(filesystem));
    let orch = container.orchestrator(file_adapter);
    let _: Arc<dyn LintFixOrchestratorAggregate> = orch;
}

#[test]
fn orchestrator_file_adapter_returns_adapter() {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem);
    let _adapter = orch.file_adapter();
}

#[test]
fn file_adapter_read_write_path_exists() {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let adapter = auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter::new(filesystem);
    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("test.txt");
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    assert!(!adapter.path_exists(&fp));
    assert!(adapter.write_file(&fp, &ContentString::new("hello".to_string())));
    assert!(adapter.path_exists(&fp));

    let content = adapter.read_file(&fp);
    assert!(content.is_some());
    assert_eq!(content.unwrap().value(), "hello");
}

#[test]
fn orchestrator_execute_on_empty_project_dry_run() {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem);

    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.rs"), "fn main() {}\n").unwrap();
    let fp = FilePath::new(tmp.path().join("main.rs").to_string_lossy().to_string()).unwrap();

    let result = orch.execute(&fp, true); // per-request dry_run=true
    assert!(result.is_success(), "Dry-run should succeed: {}", result);
}

#[test]
fn orchestrator_execute_per_request_dry_run_false() {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem);

    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.rs"), "fn main() {}\n").unwrap();
    let fp = FilePath::new(tmp.path().join("main.rs").to_string_lossy().to_string()).unwrap();

    let result = orch.execute(&fp, false); // per-request dry_run=false
    assert!(
        result.is_success(),
        "Non-dry-run should succeed: {}",
        result
    );
}

#[test]
fn orchestrator_manual_report_empty() {
    let filesystem = filesystem::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem);

    let report = orch.manual_report(&[]);
    assert!(report.is_empty(), "Empty violations → empty report");
}
