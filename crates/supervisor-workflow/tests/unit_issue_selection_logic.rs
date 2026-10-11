// PURPOSE: unit test — one public function: `IssueSelectionLogic::select_least_busy`.

use shared_supervisor::GitHubIssueVo;
use supervisor_workflow_lint_arwaky::IssueSelectionLogic;

fn make_issue(number: i64, comments: i64, reactions: i64) -> GitHubIssueVo {
    GitHubIssueVo {
        number,
        title: format!("issue {number}"),
        comments_count: comments,
        reactions_total: reactions,
        ..Default::default()
    }
}

#[test]
fn unit_select_least_busy_orders_by_total_engagement_ascending() {
    let issues = vec![
        make_issue(10, 10, 5), // busy = 15
        make_issue(20, 1, 1),  // busy = 2
        make_issue(30, 0, 0),  // busy = 0
        make_issue(40, 4, 2),  // busy = 6
    ];
    let logic = IssueSelectionLogic { max_issues: 10 };
    let picked = logic.select_least_busy(&issues);
    assert_eq!(
        picked.iter().map(|i| i.number).collect::<Vec<_>>(),
        vec![30, 20, 40, 10],
        "least-busy (lowest engagement) first"
    );
}

#[test]
fn unit_select_least_busy_caps_at_max_issues() {
    let issues: Vec<GitHubIssueVo> = (0..20).map(|n| make_issue(n, n, 0)).collect();
    let logic = IssueSelectionLogic { max_issues: 3 };
    let picked = logic.select_least_busy(&issues);
    assert_eq!(picked.len(), 3, "result capped at max_issues");
    assert_eq!(
        picked.iter().map(|i| i.number).collect::<Vec<_>>(),
        vec![0, 1, 2],
        "the three least-busy"
    );
}

#[test]
fn unit_select_least_busy_stable_on_ties() {
    let issues = vec![
        make_issue(1, 3, 0),
        make_issue(2, 3, 0),
        make_issue(3, 3, 0),
    ];
    let logic = IssueSelectionLogic { max_issues: 3 };
    let picked = logic.select_least_busy(&issues);
    assert_eq!(
        picked.iter().map(|i| i.number).collect::<Vec<_>>(),
        vec![1, 2, 3],
        "equal scores keep input order"
    );
}
