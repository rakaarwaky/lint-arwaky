// PURPOSE: FRD Requirement — Debounced event aggregation.
// "Debounced event aggregation so rapid changes do not trigger multiple lint runs."

use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::taxonomy_watch_event_vo::{WatchEvent, WatchEventKind};

#[test]
fn frd_002_rapid_changes_deduplicated_to_single_event() {
    let analyzer = ChangeAnalyzer::new();

    // Simulate 10 rapid modifications to the same file (as debouncer would batch).
    let events: Vec<WatchEvent> = (0..10)
        .map(|_| WatchEvent::new("src/hot_file.rs".into(), WatchEventKind::Modified))
        .collect();

    let result = analyzer.analyze(events);

    // All 10 events collapse to 1.
    assert_eq!(
        result.len(),
        1,
        "Rapid changes to same file must deduplicate to 1"
    );
    assert_eq!(result[0].path, "src/hot_file.rs");
}

#[test]
fn frd_002_distinct_files_not_collapsed() {
    let analyzer = ChangeAnalyzer::new();

    let events = vec![
        WatchEvent::new("a.rs".into(), WatchEventKind::Modified),
        WatchEvent::new("b.rs".into(), WatchEventKind::Modified),
        WatchEvent::new("c.rs".into(), WatchEventKind::Modified),
    ];

    let result = analyzer.analyze(events);
    assert_eq!(result.len(), 3, "Distinct files must not be collapsed");
}

#[test]
fn frd_002_debounce_config_default_is_500ms() {
    let config =
        shared::file_watch::taxonomy_watch_config_vo::WatchConfig::from_path("/tmp".to_string());
    assert_eq!(config.debounce_ms, 500, "Default debounce must be 500ms");
}
