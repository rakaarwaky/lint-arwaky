// Integration tests — full DI wiring via AutoFixContainer with real quality-rules.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::FixRequest;
use shared::auto_fix::IFixAggregate;
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
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem, fs_container.io());
    let _: Arc<dyn IFixAggregate> = orch;
}

#[test]
fn container_orchestrator_with_custom_file_adapter() {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let _filesystem = fs_container.orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());

    let _io = fs_container.io();
    let orch = container.orchestrator(_io);
    let _: Arc<dyn IFixAggregate> = orch;
}

#[test]
fn orchestrator_execute_on_empty_project_dry_run() {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem, fs_container.io());

    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.rs"), "fn main() {}\n").unwrap();
    let fp = FilePath::new(tmp.path().join("main.rs").to_string_lossy().to_string()).unwrap();

    let result = orch
        .execute(FixRequest::execute(&fp, true))
        .into_fix_result(); // per-request dry_run=true
    assert!(result.is_success(), "Dry-run should succeed: {}", result);
}

#[test]
fn orchestrator_execute_per_request_dry_run_false() {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem, fs_container.io());

    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.rs"), "fn main() {}\n").unwrap();
    let fp = FilePath::new(tmp.path().join("main.rs").to_string_lossy().to_string()).unwrap();

    let result = orch
        .execute(FixRequest::execute(&fp, false))
        .into_fix_result(); // per-request dry_run=false
    assert!(
        result.is_success(),
        "Non-dry-run should succeed: {}",
        result
    );
}

#[test]
fn orchestrator_report_no_manual_violations_on_empty() {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem, fs_container.io());

    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("main.rs"), "fn main() {}\n").unwrap();
    let fp = FilePath::new(tmp.path().join("main.rs").to_string_lossy().to_string()).unwrap();

    let result = orch
        .execute(FixRequest::execute(&fp, true))
        .into_fix_result();
    assert!(result.is_success());
    let output = result.output.value();
    assert!(
        output.contains("No automatic") || output.contains("0 violations"),
        "Expected no fixes or zero count, got: {}",
        output
    );
}
