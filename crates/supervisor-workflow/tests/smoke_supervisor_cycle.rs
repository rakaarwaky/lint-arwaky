// PURPOSE: smoke test — the root container constructs and runs a full cycle
// within 5 s, proving the app boots and responds with no live dependency.

use shared_common::taxonomy_common_vo::Count;
use shared_supervisor::CIStatus;
use supervisor_workflow_lint_arwaky::root_supervisor_container::SupervisorContainer;

#[test]
fn smoke_container_boots_and_runs_one_cycle() {
    let start = std::time::Instant::now();
    let container = SupervisorContainer::new();
    let report = container.run_supervisor_cycle(Count::new(5));
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "smoke test exceeded 5 s: {elapsed:?}"
    );
    assert_eq!(
        report.outcomes.len(),
        5,
        "default container ships exactly 5 issues"
    );
    assert!(
        report
            .outcomes
            .iter()
            .all(|o| o.final_ci_status == CIStatus::Passing),
        "default monitor is a constant-Passing seed"
    );
}

#[test]
fn smoke_zero_cycle_is_instant() {
    let start = std::time::Instant::now();
    let container = SupervisorContainer::new();
    let report = container.run_supervisor_cycle(Count::new(0));
    let elapsed = start.elapsed();
    assert!(report.is_empty());
    assert!(
        elapsed.as_secs() < 5,
        "zero-issue cycle must be instant, got {elapsed:?}"
    );
}
