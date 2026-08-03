// Contract tests — verify all concrete types implement their declared contract traits.
use file_watch_lint_arwaky::agent_watch_orchestrator::WatchOrchestrator;
use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;

#[test]
fn change_analyzer_implements_change_analyzer_protocol() {
    fn assert_trait<T: IChangeAnalyzerProtocol>() {}
    assert_trait::<ChangeAnalyzer>();
}

#[test]
fn notify_watch_provider_implements_watch_provider_protocol() {
    fn assert_trait<T: IWatchProviderProtocol>() {}
    assert_trait::<NotifyWatchProvider>();
}

#[test]
fn watch_orchestrator_implements_watch_aggregate() {
    fn assert_trait<T: IWatchAggregate>() {}
    assert_trait::<WatchOrchestrator>();
}

#[test]
fn all_capabilities_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ChangeAnalyzer>();
    assert_send_sync::<NotifyWatchProvider>();
    assert_send_sync::<WatchOrchestrator>();
}

#[test]
fn watch_aggregate_can_be_boxed_as_trait_object() {
    fn assert_object_safe<T: IWatchAggregate>() {}
    assert_object_safe::<WatchOrchestrator>();
}

#[test]
fn provider_can_be_arc_trait_object() {
    fn assert_object_safe<T: IWatchProviderProtocol>() {}
    assert_object_safe::<NotifyWatchProvider>();
}

#[test]
fn change_analyzer_can_be_arc_trait_object() {
    fn assert_object_safe<T: IChangeAnalyzerProtocol>() {}
    assert_object_safe::<ChangeAnalyzer>();
}
