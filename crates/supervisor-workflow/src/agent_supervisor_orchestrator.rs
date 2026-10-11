// PURPOSE: Supervisor orchestrator — agent layer, coordinates the full pipeline.
//
// discover → select → worktree → brief + CI-poll → merge.
//
// Depends on contracts only (AES201 rule 8): `IIssueDiscoveryProtocol`,
// `IWorktreeManagerProtocol`, `IPrMonitorProtocol`. No I/O of its own —
// every external effect is delegated to a capability.
//
// Design notes:
// - PR opening is a mock "record only" step (see FRD.md): the head branch
//   becomes the worktree branch and CI polling starts immediately.
// - CI retries: up to `SUP_VISIBILITY_MAX_CI_RETRIES` consecutive polls;
//   `Pending`/`Failing` count as a retry, `Passing` settles, `Unknown` is
//   recorded and the issue is treated as not-merged.

use shared_common::taxonomy_common_vo::Count;
use shared_supervisor::contract_supervisor_aggregate::ISupervisorAggregate;
use shared_supervisor::contract_supervisor_protocol::{
    IIssueDiscoveryProtocol, IPrMonitorProtocol, IWorktreeManagerProtocol,
};
use shared_supervisor::taxonomy_supervisor_request::SupervisorRequest;
use shared_supervisor::taxonomy_supervisor_response::SupervisorResponse;
use shared_supervisor::{
    CIStatus, GitHubIssueVo, PrInfo, SUP_VISIBILITY_MAX_CI_RETRIES, SubagentBrief,
    SupervisorCycleReport, SupervisorIssueOutcome,
};
use std::sync::Arc;

/// Agent-layer orchestrator for one supervisor cycle. Holds its three
/// capability seams as trait objects so tests can swap in mocks freely.
// ─── Block 1: Struct Definition ───────────────────────────

pub struct SupervisorOrchestrator {
    issue_discovery: Arc<dyn IIssueDiscoveryProtocol>,
    worktrees: Arc<dyn IWorktreeManagerProtocol>,
    pr_monitor: Arc<dyn IPrMonitorProtocol>,
}

/// `SupervisorOrchestrator` is the composition root for the supervisor
/// feature: it implements the feature aggregate the root container exposes.
// ─── Block 2: Aggregate Trait Implementation ──────────────

impl ISupervisorAggregate for SupervisorOrchestrator {
    fn execute(&self, request: SupervisorRequest) -> SupervisorResponse {
        match request {
            SupervisorRequest::RunCycle { max_issues } => SupervisorResponse::Cycle {
                report: self.run_supervisor_cycle(max_issues),
            },
            SupervisorRequest::RunCycleOnIssues { issues } => SupervisorResponse::Cycle {
                report: self.run_supervisor_cycle_on_issues(&issues),
            },
        }
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ───────────

impl SupervisorOrchestrator {
    /// Wire the three seams (DI; the root container does this in production,
    /// tests do it directly).
    pub fn new(
        issue_discovery: Arc<dyn IIssueDiscoveryProtocol>,
        worktrees: Arc<dyn IWorktreeManagerProtocol>,
        pr_monitor: Arc<dyn IPrMonitorProtocol>,
    ) -> Self {
        Self {
            issue_discovery,
            worktrees,
            pr_monitor,
        }
    }

    /// Run one full supervisor cycle: discover up to `max_issues`, open a
    /// worktree + PR per issue, poll CI until green (or give up), and merge
    /// the green ones. Returns the per-issue report.
    ///
    /// Errors from discovery abort the whole cycle with an empty report;
    /// errors from individual worktree/CI/merge steps are recorded on that
    /// issue's outcome and the cycle continues with the next issue.
    pub fn run_supervisor_cycle(&self, max_issues: Count) -> SupervisorCycleReport {
        let discovered = match self.issue_discovery.discover_issues(max_issues) {
            Ok(issues) => issues,
            Err(e) => {
                tracing::warn!("supervisor cycle aborted: {e}");
                return SupervisorCycleReport::default();
            }
        };

        let mut report = SupervisorCycleReport::default();
        for issue in discovered {
            let outcome = self.process_one_issue(&issue);
            report.outcomes.push(outcome);
        }
        report
    }

    /// One issue's walk: worktree → brief → (mock) PR open → CI retry loop →
    /// merge. Never panics; every failure path lands in the outcome.
    fn process_one_issue(&self, issue: &GitHubIssueVo) -> SupervisorIssueOutcome {
        let mut outcome = SupervisorIssueOutcome {
            issue_number: issue.number,
            selected: true,
            ..Default::default()
        };

        let worktree = match self.worktrees.create_worktree(issue) {
            Ok(w) => w,
            Err(e) => {
                tracing::warn!("issue #{}: worktree failed: {e}", issue.number);
                outcome.final_ci_status = CIStatus::Unknown;
                return outcome;
            }
        };
        outcome.worktree_branch = worktree.branch_name.clone();
        outcome.worktree_path = worktree.path.clone();

        let brief = SubagentBrief {
            issue_number: issue.number,
            title: issue.title.clone(),
            description: issue.body.clone(),
            worktree_path: worktree.path.clone(),
        };
        outcome.brief_text = Self::render_brief(&brief);

        // Mock PR open: head = worktree branch, base = main.
        let pr = PrInfo {
            number: issue.number,
            base: "main".to_string(),
            head: worktree.branch_name.clone(),
            state: "open".to_string(),
            mergeable: true,
        };

        let final_status = self.poll_ci_until_settled(&pr);
        outcome.final_ci_status = final_status;
        outcome.merged = final_status.is_passing();

        if outcome.merged {
            if let Err(e) = self.pr_monitor.merge_pr(&pr) {
                tracing::warn!("issue #{}: merge rejected: {e}", issue.number);
                outcome.merged = false;
            }
        }

        outcome
    }

    /// Poll `check_ci_status` up to `SUP_VISIBILITY_MAX_CI_RETRIES` times;
    /// stop early on `Passing`. `Pending` and `Failing` both consume a
    /// retry (Failing models "CI is red, re-run it"); `Unknown` settles
    /// immediately with that value.
    fn poll_ci_until_settled(&self, pr: &PrInfo) -> CIStatus {
        for _ in 0..SUP_VISIBILITY_MAX_CI_RETRIES {
            let status = match self.pr_monitor.check_ci_status(pr) {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!("CI poll transport error: {e}");
                    return CIStatus::Unknown;
                }
            };
            if status.is_passing() {
                return CIStatus::Passing;
            }
            if status == CIStatus::Unknown {
                return CIStatus::Unknown;
            }
            // Pending / Failing: consume a retry and poll again.
        }
        CIStatus::Failing
    }

    /// Run one full supervisor cycle over a caller-supplied issue set (used
    /// by tests and by a real GitHub-backed discovery that hands the
    /// orchestrator a batch).
    pub fn run_supervisor_cycle_on_issues(
        &self,
        issues: &[GitHubIssueVo],
    ) -> SupervisorCycleReport {
        let mut report = SupervisorCycleReport::default();
        for issue in issues {
            let outcome = self.process_one_issue(issue);
            report.outcomes.push(outcome);
        }
        report
    }

    /// Render the subagent brief as plain text — this is exactly what would
    /// be handed off to a subagent prompt.
    fn render_brief(brief: &SubagentBrief) -> String {
        format!(
            "Issue #{} — {}\nWorktree: {}\n\n{}",
            brief.issue_number, brief.title, brief.worktree_path, brief.description
        )
    }
}
