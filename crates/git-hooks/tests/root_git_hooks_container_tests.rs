use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;

#[test]
fn container_default_constructs() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    let hp = aggregate.hook_protocol();
    let _dp = aggregate.diff_protocol();
    let _id = hp.get_hook_manager_identity();
}

#[ignore]
#[tokio::test]
async fn container_aggregate_install_hook_ok() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    let path = shared::common::taxonomy_path_vo::FilePath::new("lint-arwaky".to_string())
        .unwrap_or_default();
    let result = aggregate.install_hook(&path).await;
    let _ = result;
}

#[ignore]
#[tokio::test]
async fn container_aggregate_uninstall_hook_ok() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    let result = aggregate.uninstall_hook().await;
    let _ = result;
}

#[tokio::test]
async fn container_aggregate_run_hooks_check_does_not_panic() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    let path = shared::common::taxonomy_path_vo::FilePath::new(".".to_string()).unwrap_or_default();
    let results = aggregate.run_git_hooks_check(&path).await;
    assert!(results.values.is_empty());
}

#[test]
fn container_hook_protocol_identity() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    // get_hook_manager_identity is on IHookProtocol, accessible via hook_protocol()
    let id = aggregate.hook_protocol().get_hook_manager_identity();
    assert_eq!(id.value(), "git_hook_manager");
}
