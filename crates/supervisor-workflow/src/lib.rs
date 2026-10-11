// PURPOSE: Supervisor workflow feature crate — capabilities + agent + root wiring.
//
// All external I/O (GitHub API, orca subprocess, gh CLI) is mocked so the
// pipeline is deterministic and fully unit-testable without a network or a
// live Orca session (plan risk #1). Real implementations are a follow-up
// behind feature flags; see FRD.md.

pub mod agent_supervisor_orchestrator;
pub mod capabilities_issue_discovery;
pub mod capabilities_pr_monitor;
pub mod capabilities_worktree_manager;
pub mod root_supervisor_container;
pub mod root_supervisor_entry;

// ─── Re-exports ────────────────────────────────────────────

pub use agent_supervisor_orchestrator::SupervisorOrchestrator;
pub use capabilities_issue_discovery::{IssueSelectionLogic, MockIssueDiscovery};
pub use capabilities_pr_monitor::MockPrMonitor;
pub use capabilities_worktree_manager::MockWorktreeManager;
pub use root_supervisor_container::SupervisorContainer;
pub use root_supervisor_entry::{default_supervisor, run_default_cycle};
