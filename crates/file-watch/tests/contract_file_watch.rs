// Contract tests — verify all concrete types implement their declared contract traits.
use file_watch_lint_arwaky::agent_watch_orchestrator::WatchOrchestrator;
use file_watch_lint_arwaky::capabilities_change_filter::ChangeFilter;
use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared_file_watch::contract_watch_aggregate::IWatchAggregate;
use shared_file_watch::contract_watch_protocol::{
    IChangeFilterProtocol, IChangeLintProtocol, IWatchLifecycleProtocol,
};

#[test]
fn change_filter_implements_change_filter_protocol() {
    fn assert_trait<T: IChangeFilterProtocol>() {}
    assert_trait::<ChangeFilter>();
}

#[test]
fn notify_watch_provider_implements_watch_lifecycle_protocol() {
    fn assert_trait<T: IWatchLifecycleProtocol>() {}
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
    assert_send_sync::<ChangeFilter>();
    assert_send_sync::<NotifyWatchProvider>();
    assert_send_sync::<WatchOrchestrator>();
}

#[test]
fn watch_aggregate_can_be_boxed_as_trait_object() {
    fn assert_object_safe<T: IWatchAggregate>() {}
    assert_object_safe::<WatchOrchestrator>();
}

#[test]
fn lifecycle_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IWatchLifecycleProtocol>() {}
    assert_object_safe::<NotifyWatchProvider>();
}

#[test]
fn filter_protocol_can_be_arc_trait_object() {
    fn assert_object_safe<T: IChangeFilterProtocol>() {}
    assert_object_safe::<ChangeFilter>();
}

#[test]
fn change_lint_protocol_is_object_safe() {
    fn assert_object_safe<T: IChangeLintProtocol + ?Sized>() {}
    assert_object_safe::<dyn IChangeLintProtocol>();
}
