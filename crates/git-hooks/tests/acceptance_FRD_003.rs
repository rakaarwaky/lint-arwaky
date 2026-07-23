// PURPOSE: Acceptance test — FRD Requirement 3: Commit blocking.
// REQ: Commits that violate AES rules are successfully blocked.
// Maps to: FRD Success Indicator #3

use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;

fn create_temp_repo() -> (tempfile::TempDir, String) {
    let tmp_dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp_dir.path().join(".git")).unwrap();
    let path_str = tmp_dir.path().to_str().unwrap().to_string();
    (tmp_dir, path_str)
}

/// FRD-003: Hook script exits with code 1 when lint check fails
#[test]
fn frd_003_hook_exits_nonzero_on_lint_failure() {
    let (tmp_dir, path_str) = create_temp_repo();

    let adapter = GitHookAdapter::new(FilePath::new(path_str).unwrap_or_default());
    let exe = FilePath::new("lint-arwaky").unwrap_or_default();
    let _ = adapter.install_pre_commit(&exe);

    let hook_path = tmp_dir.path().join(".git").join("hooks").join("pre-commit");
    let content = std::fs::read_to_string(&hook_path).unwrap();

    // The hook must check exit code and block commit
    assert!(
        content.contains("if [ $? -ne 0 ]"),
        "Hook must check lint exit code"
    );
    assert!(
        content.contains("exit 1"),
        "Hook must exit 1 to block commit on failure"
    );
}

/// FRD-003: Hook script exits with code 0 when lint passes
#[test]
fn frd_003_hook_exits_zero_on_lint_success() {
    let (tmp_dir, path_str) = create_temp_repo();

    let adapter = GitHookAdapter::new(FilePath::new(path_str).unwrap_or_default());
    let exe = FilePath::new("lint-arwaky").unwrap_or_default();
    let _ = adapter.install_pre_commit(&exe);

    let hook_path = tmp_dir.path().join(".git").join("hooks").join("pre-commit");
    let content = std::fs::read_to_string(&hook_path).unwrap();

    assert!(
        content.contains("exit 0"),
        "Hook must exit 0 to allow commit on success"
    );
}

/// FRD-003: Hook invokes `lint-arwaky check .` to scan the project
#[test]
fn frd_003_hook_invokes_lint_check_command() {
    let (tmp_dir, path_str) = create_temp_repo();

    let adapter = GitHookAdapter::new(FilePath::new(path_str).unwrap_or_default());
    let exe = FilePath::new("lint-arwaky").unwrap_or_default();
    let _ = adapter.install_pre_commit(&exe);

    let hook_path = tmp_dir.path().join(".git").join("hooks").join("pre-commit");
    let content = std::fs::read_to_string(&hook_path).unwrap();

    assert!(
        content.contains("check ."),
        "Hook must run lint-arwaky check on the project"
    );
}

/// FRD-003: Hook displays failure message to user
#[test]
fn frd_003_hook_shows_failure_message() {
    let (tmp_dir, path_str) = create_temp_repo();

    let adapter = GitHookAdapter::new(FilePath::new(path_str).unwrap_or_default());
    let exe = FilePath::new("lint-arwaky").unwrap_or_default();
    let _ = adapter.install_pre_commit(&exe);

    let hook_path = tmp_dir.path().join(".git").join("hooks").join("pre-commit");
    let content = std::fs::read_to_string(&hook_path).unwrap();

    assert!(
        content.contains("Linting failed"),
        "Hook must show failure message"
    );
}
