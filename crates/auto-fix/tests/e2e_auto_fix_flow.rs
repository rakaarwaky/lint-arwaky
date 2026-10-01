// E2E tests — full pipeline: create container → dry-run fix → verify result.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared_auto_fix::FixRequest;
use shared_auto_fix::IFixAggregate;
use shared_common::FilePath;
use std::sync::Arc;
use tempfile::TempDir;

fn make_dry_run_orch() -> Arc<dyn IFixAggregate> {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    container.orchestrator_with_filesystem(filesystem, fs_container.io())
}

#[test]
fn e2e_dry_run_clean_file() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();
    std::fs::write(tmp.path().join("clean.rs"), "fn main() {}\n").unwrap();
    let fp = FilePath::new(tmp.path().join("clean.rs").to_string_lossy().to_string()).unwrap();

    let result = orch
        .execute(FixRequest::execute(&fp, true))
        .into_fix_result(); // per-request dry_run
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

    let result = orch
        .execute(FixRequest::execute(&fp, true))
        .into_fix_result(); // per-request dry_run
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

    let result = orch
        .execute(FixRequest::execute(&fp, true))
        .into_fix_result(); // per-request dry_run
    assert!(result.is_success(), "Dry-run should succeed: {}", result);
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
    let result1 = orch
        .execute(FixRequest::execute(&fp, true))
        .into_fix_result();
    assert!(result1.is_success());
    let content_after_dry = std::fs::read_to_string(tmp.path().join("toggle.rs")).unwrap();
    assert!(
        content_after_dry.contains("use std::collections::HashMap"),
        "Dry-run must not modify file"
    );

    // Second call: dry_run=false (may apply fixes)
    let result2 = orch
        .execute(FixRequest::execute(&fp, false))
        .into_fix_result();
    assert!(result2.is_success());
}
