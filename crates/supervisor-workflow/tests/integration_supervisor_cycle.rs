// PURPOSE: Integration test — drives the full root container → run_supervisor_cycle
// flow end-to-end and asserts the final report matches the mock seed.

use shared_common::taxonomy_common_vo::Count;
use shared_supervisor::CIStatus;
use supervisor_workflow_lint_arwaky::root_supervisor_container::SupervisorContainer;

#[test]
fn full_cycle_via_root_container_reports_five_unique_branches_all_merged() {
    let container = SupervisorContainer::new();
    let report = container.run_supervisor_cycle(Count::new(5));

    assert_eq!(
        report.outcomes.len(),
        5,
        "mock discovery ships exactly 5 issues"
    );

    let branches: Vec<&String> = report.outcomes.iter().map(|o| &o.worktree_branch).collect();
    let unique: std::collections::HashSet<&String> = branches.iter().copied().collect();
    assert_eq!(
        branches.len(),
        unique.len(),
        "every issue gets its own distinct worktree branch"
    );

    assert!(report.outcomes.iter().all(|o| o.selected));
    assert!(
        report
            .outcomes
            .iter()
            .all(|o| o.final_ci_status == CIStatus::Passing),
        "default container wires a constant-Passing monitor, so every issue settles green"
    );
    assert!(report.outcomes.iter().all(|o| o.merged));
    assert!(report.all_merged());

    for o in &report.outcomes {
        assert!(
            !o.brief_text.is_empty(),
            "every outcome carries a subagent brief"
        );
        assert!(o.brief_text.contains(&o.worktree_path));
    }
}

#[test]
fn empty_cycle_when_max_zero() {
    let container = SupervisorContainer::new();
    let report = container.run_supervisor_cycle(Count::new(0));
    assert!(report.is_empty());
}
