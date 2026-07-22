// PURPOSE: Smoke test — crate boots, container initializes, basic call succeeds in < 5s.
// Layer: Smoke
// Speed: < 5s

use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;

#[test]
fn orphan_detector_boots_and_responds() {
    let start = std::time::Instant::now();

    let container = OrphanContainer::new();
    let analyzer = container.analyzer();

    // Basic operation: identify entry points
    let entries = analyzer.identify_orphan_entry_points(&[
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
        "src/root_container.rs".to_string(),
    ]);
    assert!(!entries.is_empty());

    // Basic operation: check orphans on empty set
    let results = analyzer.check_orphans(&[], "/tmp/smoke");
    assert!(results.is_empty());

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test took {:?}, must be < 5s",
        elapsed
    );
}
