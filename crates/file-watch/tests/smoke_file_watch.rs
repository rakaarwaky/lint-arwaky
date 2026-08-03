// Smoke tests — verify container creation and provider creation complete within 5s.
use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use std::sync::Arc;

#[test]
fn file_watch_container_creates() {
    let start = std::time::Instant::now();
    let _container = FileWatchContainer::new();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn notify_watch_provider_creates() {
    let start = std::time::Instant::now();
    let _provider = NotifyWatchProvider::new();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn notify_watch_provider_default_creates() {
    let start = std::time::Instant::now();
    let _provider = NotifyWatchProvider::default();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn change_analyzer_creates() {
    let start = std::time::Instant::now();
    let _analyzer = ChangeAnalyzer::new();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn container_provider_is_trait_object() {
    let start = std::time::Instant::now();
    let container = FileWatchContainer::new();
    let _: Arc<dyn IWatchProviderProtocol> = container.provider();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
