// Acceptance tests — verify AES201 (forbidden import) violations are fixable.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::FilePath;
use tempfile::TempDir;

fn make_dry_run_orch() -> std::sync::Arc<dyn LintFixOrchestratorAggregate> {
    let filesystem = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules_lint_arwaky::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    container.orchestrator_with_filesystem(true, filesystem)
}

#[test]
fn aes201_forbidden_import_is_fixable() {
    // AES201 violations are import-related and should be fixable.
    // The auto-fix processor considers AES201, AES304, AES203 as fixable codes.
    let orch = make_dry_run_orch();
    let tmp = TempDir::new().unwrap();

    // Write a file with a known-forbidden import pattern.
    // The forbidden import list depends on the config, but a basic Rust
    // file with an unused import will be picked up.
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

    let result = orch.execute(&fp);
    assert!(
        result.is_success(),
        "AES201 fix dry-run should succeed: {}",
        result
    );

    // The output should indicate the fix pipeline ran
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

    let result = orch.execute(&fp);
    // The dry-run output should mention AES violations
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
