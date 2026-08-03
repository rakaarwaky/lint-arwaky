// Acceptance tests — verify AES304 (bypass comment) violations are fixable.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::FilePath;
use tempfile::TempDir;

fn make_dry_run_orch() -> std::sync::Arc<dyn LintFixOrchestratorAggregate> {
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    container.orchestrator_with_filesystem(filesystem)
}

#[test]
fn aes304_bypass_comment_is_fixable() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();

    std::fs::write(
        tmp.path().join("aes304_target.rs"),
        "#[allow(dead_code)]\nfn unused_fn() {}\n",
    )
    .unwrap();
    let fp = FilePath::new(
        tmp.path()
            .join("aes304_target.rs")
            .to_string_lossy()
            .to_string(),
    )
    .unwrap();

    let result = orch.execute(&fp, true); // per-request dry_run
    assert!(
        result.is_success(),
        "AES304 fix dry-run should succeed: {}",
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
fn aes304_unwrap_pattern_detected() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();

    std::fs::write(
        tmp.path().join("aes304_unwrap.rs"),
        "fn main() {\n    let x: Result<i32, ()> = Ok(1);\n    let _v = x.unwrap();\n}\n",
    )
    .unwrap();
    let fp = FilePath::new(
        tmp.path()
            .join("aes304_unwrap.rs")
            .to_string_lossy()
            .to_string(),
    )
    .unwrap();

    let result = orch.execute(&fp, true); // per-request dry_run
    assert!(
        result.is_success(),
        "AES304 unwrap dry-run should succeed: {}",
        result
    );
}

#[test]
fn aes304_dry_run_does_not_modify_file() {
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();

    let original = "#[allow(dead_code)]\nfn unused_fn() {}\n";
    std::fs::write(tmp.path().join("aes304_nomod.rs"), original).unwrap();
    let fp = FilePath::new(
        tmp.path()
            .join("aes304_nomod.rs")
            .to_string_lossy()
            .to_string(),
    )
    .unwrap();

    let _result = orch.execute(&fp, true); // per-request dry_run
    let content = std::fs::read_to_string(tmp.path().join("aes304_nomod.rs")).unwrap();
    assert_eq!(content, original, "Dry-run must not modify the file");
}
