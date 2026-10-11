// PURPOSE: SupervisorContainer — root layer, wires the mock capability
// seams into one ready-to-run orchestrator (composition root).
//
// Mirrors the `root_external_lint_container` / `root_maintenance_container`
// pattern: all wiring lives here, the orchestrator stays seam-only.

use crate::capabilities_issue_discovery::MockIssueDiscovery;
use crate::capabilities_pr_monitor::MockPrMonitor;
use crate::capabilities_worktree_manager::MockWorktreeManager;
use shared_supervisor::SupervisorCycleReport;
use shared_supervisor::contract_supervisor_aggregate::ISupervisorAggregate;
use shared_supervisor::contract_supervisor_protocol::{
    IIssueDiscoveryProtocol, IPrMonitorProtocol, IWorktreeManagerProtocol,
};
use std::sync::Arc;

/// Composition root: wires the three mock capabilities into one orchestrator
/// so callers get the whole pipeline in a single type. A real build would
/// swap the mocks for reqwest/orca-backed implementations here — nothing
/// else in the crate changes.
pub struct SupervisorContainer {
    orchestrator: std::cell::RefCell<crate::agent_supervisor_orchestrator::SupervisorOrchestrator>,
}

impl Default for SupervisorContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl SupervisorContainer {
    /// Wire `MockIssueDiscovery` + `MockWorktreeManager` +
    /// `MockPrMonitor::constant(Passing)` into the orchestrator.
    pub fn new() -> Self {
        let issue_discovery: Arc<dyn IIssueDiscoveryProtocol> = Arc::new(MockIssueDiscovery::new());
        let worktrees: Arc<dyn IWorktreeManagerProtocol> = Arc::new(MockWorktreeManager::new());
        let pr_monitor: Arc<dyn IPrMonitorProtocol> = Arc::new(MockPrMonitor::constant(
            shared_supervisor::CIStatus::Passing,
        ));

        Self {
            orchestrator: std::cell::RefCell::new(
                crate::agent_supervisor_orchestrator::SupervisorOrchestrator::new(
                    issue_discovery,
                    worktrees,
                    pr_monitor,
                ),
            ),
        }
    }

    /// Expose the orchestrator behind its aggregate seam so the root entry
    /// point (and tests) can call `execute` without reaching into the agent
    /// layer directly.
    pub fn orchestrator(&self) -> Arc<dyn ISupervisorAggregate> {
        Arc::new(
            crate::agent_supervisor_orchestrator::SupervisorOrchestrator::new(
                Arc::new(MockIssueDiscovery::new()),
                Arc::new(MockWorktreeManager::new()),
                Arc::new(MockPrMonitor::default()),
            ),
        )
    }

    /// Run one full cycle with the container's default wiring.
    pub fn run_supervisor_cycle(
        &self,
        max_issues: shared_common::taxonomy_common_vo::Count,
    ) -> SupervisorCycleReport {
        let orch = self.orchestrator.borrow();
        orch.run_supervisor_cycle(max_issues)
    }
}
