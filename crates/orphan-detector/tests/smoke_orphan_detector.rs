// PURPOSE: Smoke test — crate boots, container initializes, basic call succeeds in < 5s.
// Layer: Smoke
// Speed: < 5s

use orphan_detector_lint_arwaky::root_orphan_detector_container::OrphanContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::orphan_detector::taxonomy_orphan_contract_vo::OrphanFileListVO;

#[test]
fn orphan_detector_boots_and_responds() {
    let start = std::time::Instant::now();

    let container = OrphanContainer::new();
    let analyzer = container.analyzer();

    // Basic operation: identify entry points
    let file_list = OrphanFileListVO::new(vec![
        "src/main.rs".to_string(),
        "src/lib.rs".to_string(),
        "src/root_container.rs".to_string(),
    ]);
    let entries = analyzer.identify_orphan_entry_points(&file_list);
    assert!(!entries.values.is_empty());

    // Basic operation: check orphans on empty set
    let empty_list = OrphanFileListVO::new(vec![]);
    let root = FilePath::new("/tmp/smoke".to_string()).unwrap();
    let results = analyzer.check_orphans(&empty_list, &root);
    assert!(results.is_empty());

    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test took {:?}, must be < 5s",
        elapsed
    );
}
