// PURPOSE: SupervisorRequest — request payload for the supervisor aggregate

use shared_common::taxonomy_common_vo::Count;

use crate::taxonomy_supervisor_vo::GitHubIssueVo;

/// The request the supervisor aggregate dispatches on.
pub enum SupervisorRequest {
    /// Run one full supervisor cycle: discover up to `max_issues`, open a
    /// worktree + PR per issue, poll CI until green (or give up), and merge
    /// the green ones.
    RunCycle { max_issues: Count },
    /// Run a cycle over a caller-supplied issue set (used by tests and by
    /// the real GitHub-backed discovery that hands the orchestrator a batch).
    RunCycleOnIssues { issues: Vec<GitHubIssueVo> },
}

impl SupervisorRequest {
    pub fn run_cycle(max_issues: Count) -> Self {
        Self::RunCycle { max_issues }
    }
    pub fn run_cycle_on_issues(issues: Vec<GitHubIssueVo>) -> Self {
        Self::RunCycleOnIssues { issues }
    }
}
