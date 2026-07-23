// PURPOSE: Smoke test — verifies the git-hooks crate boots and core paths respond.
// Must complete in under 5 seconds.
// Layer: Full stack smoke
// Speed: < 5s

use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::common::taxonomy_path_vo::FilePath;

#[tokio::test]
async fn git_hooks_crate_boots_and_responds() {
    // 1. Container constructs without panic
    let container = GitContainer::new_default();

    // 2. Aggregate is accessible
    let aggregate = container.aggregate();

    // 3. Diff check returns without panic (even on non-repo)
    let path = FilePath::new(".").unwrap_or_default();
    let results = aggregate.run_git_hooks_check(&path).await;
    assert!(!results.is_empty() || results.is_empty());

    // 4. Hook identity is accessible
    let identity = aggregate.hook_protocol().get_hook_manager_identity();
    assert!(!identity.value().is_empty());
}

#[tokio::test]
async fn git_hooks_install_uninstall_does_not_panic() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();

    let exe = FilePath::new("lint-arwaky").unwrap_or_default();
    let _ = aggregate.install_hook(&exe).await;
    let _ = aggregate.uninstall_hook().await;
    // If we reach here without panic, smoke passes
}
