// PURPOSE: Supervisor value objects — GitHub issue, worktree, PR, CI status,
//          subagent brief, per-issue outcome, and the domain error type.
//
// Pure data, no I/O, no upward imports. Primitives are allowed in `_vo`
// files (AES401); this file introduces no new domain model beyond the
// shapes the supervisor feature needs.

use serde::{Deserialize, Serialize};

// ─── Supervisor constants ──────────────────────────────────────────────────

/// Maximum consecutive `check_ci_status` polls before giving up on an
/// issue. Policy constants live here so every layer reads one value.
pub const SUP_VISIBILITY_MAX_CI_RETRIES: usize = 3;

// ─── GitHub issue ─────────────────────────────────────────────────────────

/// A single open GitHub issue as surfaced by the discovery capability.
///
/// Field shapes mirror `github.com/rakaarwaky/lint-arwaky`'s open issue
/// list closely enough that a real reqwest-backed implementation can fill
/// these from the REST API payload without a mapping layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GitHubIssueVo {
    pub number: i64,
    pub title: String,
    pub body: String,
    /// "open" or "closed".
    pub state: String,
    pub comments_count: i64,
    pub reactions_total: i64,
    pub user_login: String,
    /// RFC 3339 timestamps, empty when unknown.
    pub created_at: String,
    pub updated_at: String,
    pub html_url: String,
    pub labels: Vec<String>,
}

impl GitHubIssueVo {
    /// Engagement signal the selection logic ranks on: issues with less
    /// total engagement are treated as "less busy" and picked first.
    pub fn busy_score(&self) -> i64 {
        self.comments_count + self.reactions_total
    }
}

// ─── CI status ─────────────────────────────────────────────────────────────

/// CI pipeline state observed for a pull request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CIStatus {
    /// No CI run has been observed yet (or the poll itself failed).
    #[default]
    Unknown,
    /// A CI run is in flight.
    Pending,
    /// CI is green.
    Passing,
    /// CI is red; the subagent is expected to re-run / fail-forward.
    Failing,
}

impl CIStatus {
    /// True only when CI is definitively green.
    pub fn is_passing(&self) -> bool {
        matches!(self, Self::Passing)
    }
}

// ─── Worktree handle ───────────────────────────────────────────────────────

/// A handle to one orca worktree opened for a single issue. Deterministic
/// `id`/`path`/`branch_name` make mock-based orchestration reproducible in
/// tests; a real implementation would fill these from the `orca worktree
/// create` JSON output instead.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WorktreeHandle {
    /// Stable identifier (e.g. `<repo>::<worktreePath>` in real orca usage).
    pub id: String,
    /// Absolute filesystem path of the worktree.
    pub path: String,
    /// Branch name the subagent works on.
    pub branch_name: String,
}

// ─── PR info ───────────────────────────────────────────────────────────────

/// Minimal PR metadata the PR monitor needs to poll CI and merge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PrInfo {
    pub number: i64,
    pub base: String,
    pub head: String,
    /// "open", "closed", or "merged".
    pub state: String,
    pub mergeable: bool,
}

// ─── Subagent brief ───────────────────────────────────────────────────────

/// The briefing a subagent receives: the issue it should work on, where to
/// find it, and which worktree/branch to check out.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SubagentBrief {
    pub issue_number: i64,
    pub title: String,
    pub description: String,
    pub worktree_path: String,
}

// ─── Per-issue outcome + cycle report ──────────────────────────────────────

/// The outcome of one issue's walk through the supervisor pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SupervisorIssueOutcome {
    pub issue_number: i64,
    /// True when this issue made it into the selected batch.
    pub selected: bool,
    /// Branch the worktree was created on (empty if creation failed).
    pub worktree_branch: String,
    /// The subagent brief text, ready to hand off as-is.
    pub brief_text: String,
    /// Final observed CI status after the retry loop settled.
    pub final_ci_status: CIStatus,
    /// True when the PR was actually merged this cycle.
    pub merged: bool,
    /// Worktree path the subagent brief points at (empty if creation failed).
    pub worktree_path: String,
}

/// End-to-end report for one full supervisor cycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SupervisorCycleReport {
    pub outcomes: Vec<SupervisorIssueOutcome>,
}

impl SupervisorCycleReport {
    /// True when every outcome in this cycle was merged.
    pub fn all_merged(&self) -> bool {
        self.outcomes.iter().all(|o| o.merged)
    }

    /// True when no outcomes were recorded.
    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }
}

// ─── Supervisor error ──────────────────────────────────────────────────────

/// Domain error type for supervisor workflow operations.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum SupervisorError {
    /// `discover_issues` could not reach the issue source.
    #[error("GitHub issue discovery failed: {0}")]
    Discovery(String),
    /// `create_worktree` failed for a specific issue.
    #[error("Worktree creation failed for issue {issue_number}: {message}")]
    Worktree { issue_number: i64, message: String },
    /// `check_ci_status` hit an unexpected transport/parse error.
    #[error("PR monitor failed: {0}")]
    PrMonitor(String),
    /// `merge_pr` was rejected (e.g. non-mergeable, closed PR).
    #[error("PR merge failed: {0}")]
    Merge(String),
}
