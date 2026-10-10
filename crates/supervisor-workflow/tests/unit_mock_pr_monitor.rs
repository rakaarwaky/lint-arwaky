// PURPOSE: unit test — one public function: `MockPrMonitor::check_ci_status`
// cycling behaviour and the merge gate.

use shared_supervisor::IPrMonitorProtocol;
use shared_supervisor::{CIStatus, PrInfo};
use supervisor_workflow_lint_arwaky::MockPrMonitor;

fn open_pr() -> PrInfo {
    PrInfo {
        number: 42,
        base: "main".into(),
        head: "rakaarwaky/supervisor-issue-42".into(),
        state: "open".into(),
        mergeable: true,
    }
}

#[test]
fn unit_pr_monitor_cycles_a_seed_sequence() {
    let m = MockPrMonitor::with_statuses(vec![
        CIStatus::Failing,
        CIStatus::Failing,
        CIStatus::Passing,
    ]);
    let pr = open_pr();
    assert_eq!(m.check_ci_status(&pr).ok(), Some(CIStatus::Failing));
    assert_eq!(m.check_ci_status(&pr).ok(), Some(CIStatus::Failing));
    assert_eq!(m.check_ci_status(&pr).ok(), Some(CIStatus::Passing));
    assert_eq!(
        m.check_ci_status(&pr).ok(),
        Some(CIStatus::Failing),
        "sequence wraps back to the head"
    );
}

#[test]
fn unit_pr_monitor_constant_always_same_status() {
    let m = MockPrMonitor::constant(CIStatus::Passing);
    let pr = open_pr();
    for _ in 0..5 {
        assert_eq!(m.check_ci_status(&pr).ok(), Some(CIStatus::Passing));
    }
}

#[test]
fn unit_pr_monitor_empty_seed_reports_unknown() {
    let m = MockPrMonitor::with_statuses(Vec::new());
    assert_eq!(m.check_ci_status(&open_pr()).ok(), Some(CIStatus::Unknown));
}

#[test]
fn unit_merge_gates_on_pr_state() {
    let m = MockPrMonitor::default();
    assert!(m.merge_pr(&open_pr()).is_ok());
    let closed = PrInfo {
        state: "closed".into(),
        ..open_pr()
    };
    assert!(m.merge_pr(&closed).is_err(), "closed PR must not merge");
}
