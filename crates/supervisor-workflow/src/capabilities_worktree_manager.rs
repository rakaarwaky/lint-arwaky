// PURPOSE: Mock orca worktree manager — deterministic per-issue handles, no filesystem.
//
// `MockWorktreeManager` implements `IWorktreeManagerProtocol` by deriving the
// id/path/branch purely from the issue number, so every run is reproducible
// and the orchestrator's pipeline is testable without an Orca session.

use shared_supervisor::contract_supervisor_protocol::IWorktreeManagerProtocol;
use shared_supervisor::taxonomy_supervisor_vo::GitHubIssueVo;
use shared_supervisor::taxonomy_supervisor_vo::SupervisorError;
use shared_supervisor::taxonomy_supervisor_vo::WorktreeHandle;

// ─── Block 1: Struct Definition ───────────────────────────

/// Mock `IWorktreeManagerProtocol`: no subprocess, no filesystem — the
/// handle is derived deterministically from `issue.number`.
#[derive(Debug, Default, Clone, Copy)]
pub struct MockWorktreeManager;

// ─── Block 2: Protocol Trait Implementation ──────────────

impl IWorktreeManagerProtocol for MockWorktreeManager {
    fn create_worktree(&self, issue: &GitHubIssueVo) -> Result<WorktreeHandle, SupervisorError> {
        Ok(WorktreeHandle {
            id: format!("lint-arwaky::supervisor-issue-{}", issue.number),
            path: format!(
                "/home/raka/orca/workspaces/lint-arwaky/supervisor-issue-{}",
                issue.number
            ),
            branch_name: format!("rakaarwaky/supervisor-issue-{}", issue.number),
        })
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl MockWorktreeManager {
    /// Construct the stateless mock.
    pub fn new() -> Self {
        Self
    }
}
