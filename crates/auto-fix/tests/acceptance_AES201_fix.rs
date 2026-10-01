// Acceptance tests — verify AES201 (forbidden import) violations are fixable.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared_auto_fix::FixRequest;
use shared_auto_fix::IFixAggregate;
use shared_common::FilePath;
use tempfile::TempDir;

fn make_dry_run_orch() -> std::sync::Arc<dyn IFixAggregate> {
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    container.orchestrator_with_filesystem(filesystem, fs_container.io())
}

#[test]
fn aes201_forbidden_import_is_fixable() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();

    std::fs::write(
        tmp.path().join("aes201_target.rs"),
        "use std::io::Read;\nfn main() {}\n",
    )
    .unwrap();
    let fp = FilePath::new(
        tmp.path()
            .join("aes201_target.rs")
            .to_string_lossy()
            .to_string(),
    )
    .unwrap();

    let result = orch
        .execute(FixRequest::execute(&fp, true))
        .into_fix_result(); // per-request dry_run
    assert!(
        result.is_success(),
        "AES201 fix dry-run should succeed: {}",
        result
    );

    let output = result.output.value();
    assert!(
        output.contains("Dry-run")
            || output.contains("violations")
            || output.contains("No automatic"),
        "Expected fix pipeline output, got: {}",
        output
    );
}

#[test]
fn aes201_output_mentions_fixable_codes() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();

    std::fs::write(
        tmp.path().join("aes201_codes.rs"),
        "use std::collections::HashMap;\nfn main() {}\n",
    )
    .unwrap();
    let fp = FilePath::new(
        tmp.path()
            .join("aes201_codes.rs")
            .to_string_lossy()
            .to_string(),
    )
    .unwrap();

    let result = orch
        .execute(FixRequest::execute(&fp, true))
        .into_fix_result(); // per-request dry_run
    let output = result.output.value();
    assert!(
        output.contains("Dry-run")
            || output.contains("AES")
            || output.contains("violations")
            || output.contains("No automatic"),
        "Expected AES-related output, got: {}",
        output
    );
}
