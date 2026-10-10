// PURPOSE: contract test — every capability seam in the supervisor crate
// implements its declared protocol trait from `shared-supervisor`.

use shared_supervisor::ISupervisorAggregate;
use shared_supervisor::contract_supervisor_protocol::{
    IIssueDiscoveryProtocol, IPrMonitorProtocol, IWorktreeManagerProtocol,
};
use std::sync::Arc;
use supervisor_workflow_lint_arwaky::{
    MockIssueDiscovery, MockPrMonitor, MockWorktreeManager, SupervisorOrchestrator,
};

#[test]
fn mock_issue_discovery_implements_protocol() {
    fn assert_trait<T: IIssueDiscoveryProtocol>() {}
    assert_trait::<MockIssueDiscovery>();
}

#[test]
fn mock_worktree_manager_implements_protocol() {
    fn assert_trait<T: IWorktreeManagerProtocol>() {}
    assert_trait::<MockWorktreeManager>();
}

#[test]
fn mock_pr_monitor_implements_protocol() {
    fn assert_trait<T: IPrMonitorProtocol>() {}
    assert_trait::<MockPrMonitor>();
}

#[test]
fn all_seams_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<MockIssueDiscovery>();
    assert_send_sync::<MockWorktreeManager>();
    assert_send_sync::<MockPrMonitor>();
}

#[test]
fn all_seams_are_arc_trait_objects() {
    fn assert_arc<T: ?Sized>() {}
    assert_arc::<dyn IIssueDiscoveryProtocol>();
    assert_arc::<dyn IWorktreeManagerProtocol>();
    assert_arc::<dyn IPrMonitorProtocol>();
    let _ = Arc::new(MockIssueDiscovery::new());
}

#[test]
fn supervisor_orchestrator_implements_aggregate() {
    fn assert_trait<T: ISupervisorAggregate>() {}
    assert_trait::<SupervisorOrchestrator>();
}
