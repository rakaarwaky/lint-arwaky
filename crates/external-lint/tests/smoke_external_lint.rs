// PURPOSE: Smoke test — verify the external-lint subsystem boots and responds
// within 5 seconds. If this fails, nothing else matters.

use external_lint_lint_arwaky::ExternalLintContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;

#[tokio::test]
async fn external_lint_boots_and_returns_adapter_names() {
    let start = std::time::Instant::now();

    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();

    assert!(
        !names.is_empty(),
        "Container must register at least one adapter"
    );
    assert_eq!(
        names.len(),
        9,
        "Expected 9 adapters (3 Rust + 3 Python + 3 JS)"
    );

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test must complete in under 5 seconds, took {:?}",
        elapsed
    );
}

#[tokio::test]
async fn scan_all_on_empty_dir_completes_without_panic() {
    let dir = tempfile::tempdir().unwrap();
    let path = FilePath::new(dir.path().to_string_lossy().to_string()).unwrap();

    let container = ExternalLintContainer::new();
    let aggregate = container.aggregate();

    let start = std::time::Instant::now();
    let results = aggregate.scan_all(&path).await;
    let elapsed = start.elapsed();

    // Empty dir → no language detected → no adapters run → empty results
    assert!(results.is_empty());
    assert!(
        elapsed.as_secs() < 5,
        "Smoke scan must complete in under 5 seconds, took {:?}",
        elapsed
    );
}
