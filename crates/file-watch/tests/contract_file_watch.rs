// PURPOSE: Verify that all concrete types implement their declared contract traits.
// Layer: Contract verification — runs in ms, every PR.

use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use file_watch_lint_arwaky::agent_watch_orchestrator::WatchOrchestrator;
use file_watch_lint_arwaky::root_file_watch_container::FileWatchContainer;

use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;

// ─── ChangeAnalyzer implements IChangeAnalyzerProtocol ─────

#[test]
fn change_analyzer_implements_change_analyzer_protocol() {
    fn assert_trait<T: IChangeAnalyzerProtocol>() {}
    assert_trait::<ChangeAnalyzer>();
}

#[test]
fn change_analyzer_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ChangeAnalyzer>();
}

// ─── NotifyWatchProvider implements IWatchProviderProtocol ──

#[test]
fn notify_watch_provider_implements_provider_protocol() {
    fn assert_trait<T: IWatchProviderProtocol>() {}
    assert_trait::<NotifyWatchProvider>();
}

#[test]
fn notify_watch_provider_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NotifyWatchProvider>();
}

// ─── WatchOrchestrator implements IWatchAggregate ───────────

#[test]
fn watch_orchestrator_implements_watch_aggregate() {
    fn assert_trait<T: IWatchAggregate>() {}
    assert_trait::<WatchOrchestrator>();
}

#[test]
fn watch_orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<WatchOrchestrator>();
}

// ─── FileWatchContainer wiring surface ──────────────────────

#[test]
fn file_watch_container_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<FileWatchContainer>();
}

// ─── Trait object safety (dyn compatibility) ────────────────

#[test]
fn change_analyzer_protocol_is_object_safe() {
    let _boxed: Box<dyn IChangeAnalyzerProtocol> = Box::new(ChangeAnalyzer::new());
}

#[test]
fn watch_provider_protocol_is_object_safe() {
    let _boxed: Box<dyn IWatchProviderProtocol> = Box::new(NotifyWatchProvider::new());
}

#[test]
fn watch_aggregate_is_object_safe() {
    // WatchOrchestrator requires Arc<dyn IWatchProviderProtocol> + Arc<dyn ICodeAnalysisAggregate>
    // so we verify the trait itself is dyn-compatible via a compile-time check.
    fn assert_object_safe<T: IWatchAggregate + ?Sized>() {}
    assert_object_safe::<dyn IWatchAggregate>();
}
