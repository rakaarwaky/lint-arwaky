// PURPOSE: E2E test — full pre-commit hook lifecycle.
// Simulates: create repo → install hook → verify hook content → trigger check → uninstall.
// Layer: Full request lifecycle (no internal mocks)
// Speed: s

use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;
use std::sync::Arc;

fn create_temp_git_repo() -> (std::path::PathBuf, String) {
    let tmp_dir = std::env::temp_dir().join(format!("git_hooks_e2e_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir).unwrap();
    std::fs::create_dir_all(tmp_dir.join(".git")).unwrap();
    let path_str = tmp_dir.to_str().unwrap().to_string();
    (tmp_dir, path_str)
}

#[tokio::test]
async fn full_pre_commit_hook_lifecycle() {
    let (tmp_dir, path_str) = create_temp_git_repo();

    // Step 1: Wire container with real adapter pointed at temp repo
    let hook_adapter: Arc<dyn IHookManagerProtocol> = Arc::new(
        git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter::new(
            FilePath::new(path_str.clone()).unwrap_or_default(),
        ),
    );
    let container = GitContainer::new(hook_adapter);
    let aggregate = container.aggregate();

    // Step 2: Install pre-commit hook
    let exe_path = FilePath::new("/usr/local/bin/lint-arwaky").unwrap_or_default();
    let install_result = aggregate.install_hook(&exe_path).await;
    assert!(install_result.is_ok());
    assert!(install_result.unwrap().value());

    // Step 3: Verify hook file exists and has correct content
    let hook_path = tmp_dir.join(".git").join("hooks").join("pre-commit");
    assert!(hook_path.exists(), "pre-commit hook file must exist");

    let content = std::fs::read_to_string(&hook_path).unwrap();
    assert!(
        content.contains("#!/bin/bash"),
        "Hook must have bash shebang"
    );
    assert!(
        content.contains("lint-arwaky check ."),
        "Hook must invoke lint-arwaky check"
    );
    assert!(
        content.contains("exit 1"),
        "Hook must exit 1 on lint failure"
    );

    // Step 4: Run diff check (simulates what the hook would trigger)
    let check_path = FilePath::new(path_str.clone()).unwrap_or_default();
    let lint_results = aggregate.run_git_hooks_check(&check_path).await;
    // In a fresh empty repo, no violations expected
    assert!(lint_results.is_empty());

    // Step 5: Uninstall hook
    let uninstall_result = aggregate.uninstall_hook().await;
    assert!(uninstall_result.is_ok());
    assert!(uninstall_result.unwrap().value());
    assert!(
        !hook_path.exists(),
        "Hook file must be removed after uninstall"
    );

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp_dir);
}

#[tokio::test]
async fn pre_commit_hook_blocks_on_violation_simulation() {
    // This test verifies the hook script structure would block a commit
    // by checking the exit-code logic in the generated script.
    let (tmp_dir, path_str) = create_temp_git_repo();

    let hook_adapter: Arc<dyn IHookManagerProtocol> = Arc::new(
        git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter::new(
            FilePath::new(path_str).unwrap_or_default(),
        ),
    );
    let container = GitContainer::new(hook_adapter);
    let aggregate = container.aggregate();

    let exe_path = FilePath::new("lint-arwaky").unwrap_or_default();
    let _ = aggregate.install_hook(&exe_path).await;

    let hook_path = tmp_dir.join(".git").join("hooks").join("pre-commit");
    let content = std::fs::read_to_string(&hook_path).unwrap();

    // Verify blocking logic: if lint fails ($? -ne 0), exit 1
    assert!(content.contains("if [ $? -ne 0 ]"));
    assert!(content.contains("exit 1"));
    // Verify success path: exit 0
    assert!(content.contains("exit 0"));

    let _ = std::fs::remove_dir_all(&tmp_dir);
}
