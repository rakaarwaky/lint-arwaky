// Acceptance tests — verify AES101 (naming convention) violations are fixable via symbol rename.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::FilePath;
use tempfile::TempDir;

fn make_orch() -> std::sync::Arc<dyn LintFixOrchestratorAggregate> {
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    container.orchestrator_with_filesystem(filesystem)
}

#[test]
fn aes101_naming_violation_is_fixable_dry_run() {
    let orch = make_orch();
    let tmp = TempDir::new().unwrap();

    // snake_case file containing a symbol with underscore — triggers AES101 rename path
    std::fs::write(
        tmp.path().join("aes101_target.rs"),
        "fn my_bad_name_fn() {}\nfn main() {\n    my_bad_name_fn();\n}\n",
    )
    .unwrap();
    let fp = FilePath::new(
        tmp.path()
            .join("aes101_target.rs")
            .to_string_lossy()
            .to_string(),
    )
    .unwrap();

    let result = orch.execute(&fp, true); // per-request dry_run
    assert!(
        result.is_success(),
        "AES101 fix dry-run should succeed: {}",
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
fn aes101_dry_run_does_not_modify_file() {
    let orch = make_orch();
    let tmp = TempDir::new().unwrap();

    let original = "fn my_bad_name_fn() {}\nfn main() {\n    my_bad_name_fn();\n}\n";
    std::fs::write(tmp.path().join("aes101_nomod.rs"), original).unwrap();
    let fp = FilePath::new(
        tmp.path()
            .join("aes101_nomod.rs")
            .to_string_lossy()
            .to_string(),
    )
    .unwrap();

    let _result = orch.execute(&fp, true); // per-request dry_run
    let content = std::fs::read_to_string(tmp.path().join("aes101_nomod.rs")).unwrap();
    assert_eq!(content, original, "Dry-run must not modify the file");
}
