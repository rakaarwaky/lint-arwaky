// Smoke tests — verify container creation and orchestrator creation complete within 5s.
use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared_auto_fix::IFixAggregate;

#[test]
fn auto_fix_container_creates() {
    let start = std::time::Instant::now();
    let qa = quality_rules::CodeAnalysisContainer::new();
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
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let _orch = container.orchestrator_with_filesystem(filesystem, fs_container.io());
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
    let fs_container = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fs_container.orchestrator();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let container = AutoFixContainer::new(qa.code_analysis_linter());
    let orch = container.orchestrator_with_filesystem(filesystem, fs_container.io());
    let _: std::sync::Arc<dyn IFixAggregate> = orch;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
