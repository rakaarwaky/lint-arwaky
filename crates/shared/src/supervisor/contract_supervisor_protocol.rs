// PURPOSE: Supervisor protocol contracts (AES102 `_protocol`)
//
// Three capability seams the agent orchestrator depends on; each is
// implemented by a mock capability in `crates/supervisor-workflow` and
// is designed to be swapped for a real GitHub/orca-backed implementation
// without touching the orchestrator.
//
// AES201: this file imports taxonomy only.
use shared_common::taxonomy_common_vo::Count;

use crate::taxonomy_supervisor_vo::{
    CIStatus, GitHubIssueVo, PrInfo, SupervisorError, WorktreeHandle,
};

/// FR-001: discover open GitHub issues (mocked; real impl polls the
/// GitHub REST API via reqwest).
pub trait IIssueDiscoveryProtocol: Send + Sync {
    /// Return up to `max` open issues. Ordering is not guaranteed —
    /// the agent layer is responsible for selection.
    fn discover_issues(&self, max: Count) -> Result<Vec<GitHubIssueVo>, SupervisorError>;
}

/// FR-002: open one orca worktree per selected issue (mocked; real impl
/// shells out to `orca worktree create`).
pub trait IWorktreeManagerProtocol: Send + Sync {
    /// Create (or reuse) a worktree for `issue` and return its handle.
    fn create_worktree(&self, issue: &GitHubIssueVo) -> Result<WorktreeHandle, SupervisorError>;
}

/// FR-003: poll PR CI until green, then merge (mocked; real impl uses
/// `gh pr view` / `gh pr merge` or the GitHub Checks API).
pub trait IPrMonitorProtocol: Send + Sync {
    /// One CI poll; the caller is expected to loop until `Passing`.
    fn check_ci_status(&self, pr: &PrInfo) -> Result<CIStatus, SupervisorError>;

    /// Merge the PR, assuming CI has already passed.
    fn merge_pr(&self, pr: &PrInfo) -> Result<(), SupervisorError>;
}
