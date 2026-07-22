// PURPOSE: Smoke test — verify the import-rules crate boots and responds.
// Must complete in under 5 seconds.

use import_rules_lint_arwaky::root_import_rules_container::ImportContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;

#[tokio::test]
async fn import_rules_boots_and_runs_audit() {
    let start = std::time::Instant::now();

    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);
    let orch = container.orchestrator();

    // Audit the crate's own src directory
    let target = FilePath::new("src").unwrap_or_default();
    let result = orch.run_audit(&target).await;

    // Must not panic, must return Ok
    assert!(result.is_ok(), "Smoke: run_audit must not error");

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test must complete in under 5 seconds, took {:?}",
        elapsed
    );
}

#[tokio::test]
async fn container_wiring_does_not_panic() {
    let config = ArchitectureConfig::default();
    let container = ImportContainer::new_with_config(config);

    // All accessors must not panic
    let _ = container.mandatory();
    let _ = container.forbidden();
    let _ = container.unused();
    let _ = container.dummy();
    let _ = container.cycle();
    let _ = container.orchestrator();
    let _ = container.config();
}
