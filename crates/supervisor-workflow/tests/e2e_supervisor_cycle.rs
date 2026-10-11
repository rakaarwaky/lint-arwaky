// PURPOSE: e2e test — full discover→select→worktree→brief+CI-poll→merge
// lifecycle through the root container, with a seeded flaky-CI monitor.

use shared_common::taxonomy_common_vo::Count;
use supervisor_workflow_lint_arwaky::root_supervisor_container::SupervisorContainer;

#[test]
fn e2e_default_cycle_merges_all_five_issues() {
    let container = SupervisorContainer::new();
    let report = container.run_supervisor_cycle(Count::new(5));

    assert_eq!(report.outcomes.len(), 5);
    assert!(
        report.all_merged(),
        "constant-Passing seed merges every issue"
    );
    for o in &report.outcomes {
        assert!(o.selected);
        assert!(!o.worktree_branch.is_empty());
        assert!(!o.worktree_path.is_empty());
        assert!(o.merged);
    }
}

#[test]
fn e2e_cycle_shorter_than_issue_count_merges_only_requested() {
    let container = SupervisorContainer::new();
    let report = container.run_supervisor_cycle(Count::new(2));
    assert_eq!(report.outcomes.len(), 2, "honour the max selection bound");
    assert!(report.all_merged());
}
