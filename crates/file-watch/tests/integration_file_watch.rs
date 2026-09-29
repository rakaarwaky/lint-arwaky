// Integration tests — full DI wiring via FileWatchContainer.
use file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use std::sync::Arc;

#[test]
fn container_creates_with_default() {
    let _container = FileWatchContainer::new();
}

#[test]
fn container_default_creates() {
    let _container = FileWatchContainer::default();
}

#[test]
fn container_lifecycle_accessible() {
    let container = FileWatchContainer::new();
    let _lifecycle = container.lifecycle();
}

#[test]
fn container_filter_accessible() {
    let container = FileWatchContainer::new();
    let _filter = container.filter();
}

#[test]
fn container_aggregate_needs_linter() {
    let container = FileWatchContainer::new();
    // Need a linter to create the aggregate
    let qa = quality_rules::CodeAnalysisContainer::new();
    let _agg = container.aggregate(qa.code_analysis_linter());
}

#[test]
fn container_aggregate_is_trait_object() {
    let container = FileWatchContainer::new();
    let qa = quality_rules::CodeAnalysisContainer::new();
    let agg = container.aggregate(qa.code_analysis_linter());
    let _: Arc<dyn IWatchAggregate> = agg;
}

#[test]
fn filter_via_protocol() {
    use shared::file_watch::contract_watch_protocol::IChangeFilterProtocol;
    use shared::file_watch::{WatchEvent, WatchEventKind};
    let filter = file_watch_lint_arwaky::capabilities_change_filter::ChangeFilter::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("image.png".to_string(), WatchEventKind::Modified),
    ];
    let result = filter.filter_events(events);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "main.rs");
}

#[test]
fn filter_via_container() {
    use shared::file_watch::{WatchEvent, WatchEventKind};
    let container = FileWatchContainer::new();
    let events = vec![
        WatchEvent::new("main.rs".to_string(), WatchEventKind::Modified),
        WatchEvent::new("image.png".to_string(), WatchEventKind::Modified),
    ];
    let result = container.filter().filter_events(events);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].path, "main.rs");
}
