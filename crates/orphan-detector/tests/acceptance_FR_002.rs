// PURPOSE: Acceptance test — FR-002 Entry Point Discovery.
// Requirement: Identify valid entry points using stem/suffix/prefix/extension matching without substring false-positives.

use orphan_detector_lint_arwaky::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::orphan_detector::{IOrphanGraphResolverProtocol, OrphanFileListVO};

#[test]
fn fr002_entry_point_discovery_exact_matching() {
    let resolver = OrphanGraphResolver::default();

    let files = vec![
        "crates/app/src/root_cli_main_entry.rs".to_string(),
        "crates/app/src/root_composition_container.rs".to_string(),
        "crates/app/src/main.rs".to_string(),
        "crates/app/src/germanic_utils.rs".to_string(), // Should NOT match "main"
        "crates/app/src/capabilities_user_service.rs".to_string(),
    ];

    let files_vo = OrphanFileListVO::new(files);
    let entry_points = resolver.identify_entry_points(&[files_vo], &[]);

    assert!(
        entry_points
            .values
            .contains(&"crates/app/src/root_cli_main_entry.rs".to_string()),
        "FR-002 FAIL: root_ file must be recognized as entry point"
    );
    assert!(
        entry_points
            .values
            .contains(&"crates/app/src/root_composition_container.rs".to_string()),
        "FR-002 FAIL: _container file must be recognized as entry point"
    );
    assert!(
        entry_points
            .values
            .contains(&"crates/app/src/main.rs".to_string()),
        "FR-002 FAIL: main.rs must be recognized as entry point"
    );
    assert!(
        !entry_points
            .values
            .contains(&"crates/app/src/germanic_utils.rs".to_string()),
        "FR-002 FAIL: germanic_utils.rs must NOT be falsely matched as main entry point"
    );
}
