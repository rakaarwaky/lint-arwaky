// E2E tests — full pipeline: create container → dry-run fix → verify result.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::{ContentString, FilePath};
use std::sync::Arc;
use tempfile::TempDir;

fn make_dry_run_orch() -> Arc<dyn LintFixOrchestratorAggregate> {
    let filesystem = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules_lint_arwaky::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    container.orchestrator_with_filesystem(filesystem)
}

#[test]
fn e2e_dry_run_clean_file() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("clean.rs"), "fn main() {}\n").unwrap();
    let fp = FilePath::new(tmp.path().join("clean.rs").to_string_lossy().to_string()).unwrap();

    let result = orch.execute(&fp, true); // per-request dry_run
    assert!(
        result.is_success(),
        "Should succeed on clean file: {}",
        result
    );
}

#[test]
fn e2e_dry_run_file_with_unused_import() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("unused.rs"),
        "use std::collections::HashMap;\nfn main() {}\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().join("unused.rs").to_string_lossy().to_string()).unwrap();

    let result = orch.execute(&fp, true); // per-request dry_run
    assert!(result.is_success(), "Dry-run should succeed: {}", result);
    // Verify file not modified in dry-run
    let content = std::fs::read_to_string(tmp.path().join("unused.rs")).unwrap();
    assert!(
        content.contains("use std::collections::HashMap"),
        "Dry-run should not modify file"
    );
}

#[test]
fn e2e_dry_run_file_with_bypass_comment() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("bypass.rs"),
        "#[allow(dead_code)]\nfn unused() {}\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().join("bypass.rs").to_string_lossy().to_string()).unwrap();

    let result = orch.execute(&fp, true); // per-request dry_run
    assert!(result.is_success(), "Dry-run should succeed: {}", result);
}

#[test]
fn e2e_file_adapter_round_trip() {
    let filesystem = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let adapter = auto_fix_lint_arwaky::capabilities_file_adapter::FileAdapter::new(filesystem);

    let tmp = TempDir::new().unwrap();
    let file = tmp.path().join("round_trip.txt");
    let fp = FilePath::new(file.to_string_lossy().to_string()).unwrap();

    // Write, read, verify
    let content = ContentString::new("round trip content".to_string());
    assert!(adapter.write_file(&fp, &content));
    let read = adapter.read_file(&fp).unwrap();
    assert_eq!(read.value(), "round trip content");

    // Overwrite
    let new_content = ContentString::new("overwritten".to_string());
    assert!(adapter.write_file(&fp, &new_content));
    assert_eq!(adapter.read_file(&fp).unwrap().value(), "overwritten");
}

#[test]
fn e2e_per_request_dry_run_toggle() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();
    std::fs::write(
        tmp.path().join("toggle.rs"),
        "use std::collections::HashMap;\nfn main() {}\n",
    )
    .unwrap();
    let fp = FilePath::new(tmp.path().join("toggle.rs").to_string_lossy().to_string()).unwrap();

    // First call: dry_run=true (should not modify)
    let result1 = orch.execute(&fp, true);
    assert!(result1.is_success());
    let content_after_dry = std::fs::read_to_string(tmp.path().join("toggle.rs")).unwrap();
    assert!(
        content_after_dry.contains("use std::collections::HashMap"),
        "Dry-run must not modify file"
    );

    // Second call: dry_run=false (may apply fixes)
    let result2 = orch.execute(&fp, false);
    assert!(result2.is_success());
}
