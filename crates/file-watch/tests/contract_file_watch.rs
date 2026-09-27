// Contract tests — verify all concrete types implement their declared contract traits.
use file_watch_lint_arwaky::agent_watch_orchestrator::WatchOrchestrator;
use file_watch_lint_arwaky::capabilities_change_analyzer::ChangeAnalyzer;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::contract_watch_protocol::{
    IChangeLintProtocol, IEventDedupProtocol, ILintableFilterProtocol, IWatchBroadcastProtocol,
    IWatchShutdownProtocol, IWatchStartProtocol,
};

#[test]
fn change_analyzer_implements_lintable_filter_protocol() {
    fn assert_trait<T: ILintableFilterProtocol>() {}
    assert_trait::<ChangeAnalyzer>();
}

#[test]
fn change_analyzer_implements_event_dedup_protocol() {
    fn assert_trait<T: IEventDedupProtocol>() {}
    assert_trait::<ChangeAnalyzer>();
}

#[test]
fn notify_watch_provider_implements_watch_start_protocol() {
    fn assert_trait<T: IWatchStartProtocol>() {}
    assert_trait::<NotifyWatchProvider>();
}

#[test]
fn notify_watch_provider_implements_watch_broadcast_protocol() {
    fn assert_trait<T: IWatchBroadcastProtocol>() {}
    assert_trait::<NotifyWatchProvider>();
}

#[test]
fn notify_watch_provider_implements_watch_shutdown_protocol() {
    fn assert_trait<T: IWatchShutdownProtocol>() {}
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
fn start_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IWatchStartProtocol>() {}
    assert_object_safe::<NotifyWatchProvider>();
}

#[test]
fn broadcast_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IWatchBroadcastProtocol>() {}
    assert_object_safe::<NotifyWatchProvider>();
}

#[test]
fn shutdown_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IWatchShutdownProtocol>() {}
    assert_object_safe::<NotifyWatchProvider>();
}

#[test]
fn lintable_filter_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: ILintableFilterProtocol>() {}
    assert_object_safe::<ChangeAnalyzer>();
}

#[test]
fn event_dedup_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IEventDedupProtocol>() {}
    assert_object_safe::<ChangeAnalyzer>();
}

#[test]
fn change_lint_protocol_is_object_safe() {
    fn assert_object_safe<T: IChangeLintProtocol + ?Sized>() {}
    assert_object_safe::<dyn IChangeLintProtocol>();
}
