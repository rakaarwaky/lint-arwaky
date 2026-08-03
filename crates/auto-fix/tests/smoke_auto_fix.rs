// Smoke tests — verify container creation and orchestrator creation complete within 5s.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::LintFixOrchestratorAggregate;

#[test]
fn auto_fix_container_creates() {
    let start = std::time::Instant::now();
    let qa = quality_rules_lint_arwaky::CodeAnalysisContainer::new();
    let _container = AutoFixContainer::new(qa.code_analysis_linter());
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn auto_fix_orchestrator_creates() {
    let start = std::time::Instant::now();
    let filesystem = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules_lint_arwaky::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let _orch = container.orchestrator_with_filesystem(filesystem);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn auto_fix_orchestrator_is_trait_object() {
    let start = std::time::Instant::now();
    let filesystem = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let qa = quality_rules_lint_arwaky::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem);
    let _: std::sync::Arc<dyn LintFixOrchestratorAggregate> = orch;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
