// PURPOSE: acceptance test — the supervisor workflow meets its FRD acceptance
// criteria: deterministic per-issue outcomes, flaky-CI recovery, and the
// documented give-up behaviour.

use shared_common::taxonomy_common_vo::Count;
use shared_supervisor::contract_supervisor_protocol::{
    IIssueDiscoveryProtocol, IPrMonitorProtocol,
};
use shared_supervisor::{CIStatus, GitHubIssueVo, PrInfo, SupervisorCycleReport};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use supervisor_workflow_lint_arwaky::MockWorktreeManager;

struct FixedDiscovery(Vec<GitHubIssueVo>);
impl IIssueDiscoveryProtocol for FixedDiscovery {
    fn discover_issues(
        &self,
        max: Count,
    ) -> Result<Vec<GitHubIssueVo>, shared_supervisor::SupervisorError> {
        let limit = usize::try_from(max.value()).unwrap_or(usize::MAX);
        Ok(self.0.iter().take(limit).cloned().collect())
    }
}

struct QueueMonitor {
    queue: Mutex<VecDeque<CIStatus>>,
}
impl IPrMonitorProtocol for QueueMonitor {
    fn check_ci_status(
        &self,
        _pr: &PrInfo,
    ) -> Result<CIStatus, shared_supervisor::SupervisorError> {
        Ok(self.queue.lock().map_or(CIStatus::Unknown, |mut q| {
            q.pop_front().unwrap_or(CIStatus::Unknown)
        }))
    }
    fn merge_pr(&self, _pr: &PrInfo) -> Result<(), shared_supervisor::SupervisorError> {
        Ok(())
    }
}

fn one_issue(n: i64) -> GitHubIssueVo {
    GitHubIssueVo {
        number: n,
        title: format!("issue {n}"),
        body: "body".into(),
        ..Default::default()
    }
}

fn make_orch(issues: Vec<GitHubIssueVo>, ci_sequence: Vec<CIStatus>) -> SupervisorCycleReport {
    let orchestrator = supervisor_workflow_lint_arwaky::SupervisorOrchestrator::new(
        Arc::new(FixedDiscovery(issues)),
        Arc::new(MockWorktreeManager::new()),
        Arc::new(QueueMonitor {
            queue: Mutex::new(VecDeque::from(ci_sequence)),
        }),
    );
    orchestrator.run_supervisor_cycle(Count::new(5))
}

#[test]
fn acceptance_flaky_ci_recovers_on_third_poll_and_merges() {
    let report = make_orch(
        vec![one_issue(9001)],
        vec![CIStatus::Failing, CIStatus::Failing, CIStatus::Passing],
    );
    let o = &report.outcomes[0];
    assert_eq!(
        o.final_ci_status,
        CIStatus::Passing,
        "CI went green on the 3rd poll"
    );
    assert!(o.merged, "green CI leads to a merge");
}

#[test]
fn acceptance_give_up_after_three_consecutive_failures() {
    let report = make_orch(
        vec![one_issue(9002)],
        vec![CIStatus::Failing, CIStatus::Failing, CIStatus::Failing],
    );
    let o = &report.outcomes[0];
    assert_eq!(
        o.final_ci_status,
        CIStatus::Failing,
        "3 retries exhausted, CI still red"
    );
    assert!(!o.merged, "failing CI must not merge");
}

#[test]
fn acceptance_unique_worktree_per_issue_number() {
    let issues: Vec<GitHubIssueVo> = (100..105).map(one_issue).collect();
    let report = make_orch(issues, vec![CIStatus::Passing; 5]);
    let branches: std::collections::HashSet<_> = report
        .outcomes
        .iter()
        .map(|o| o.worktree_branch.clone())
        .collect();
    assert_eq!(
        branches.len(),
        5,
        "one distinct branch per distinct issue number"
    );
}
