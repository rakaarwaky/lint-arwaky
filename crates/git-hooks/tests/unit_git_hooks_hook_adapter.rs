// PURPOSE: Unit tests for GitHookAdapter — IHookManagerProtocol implementation.
// Covers: install/uninstall pre-commit hook, git repo detection.
// Layer: Capabilities (GitHookAdapter)
// Speed: ms

use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;

fn sut_in_non_repo() -> GitHookAdapter {
    GitHookAdapter::new(FilePath::new("/tmp/nonexistent_repo_test_xyz").unwrap_or_default())
}

// ─── Construction ─────────────────────────────────────────

#[test]
fn new_creates_instance_with_root_dir() {
    let path = FilePath::new("/tmp/test_repo").unwrap_or_default();
    let _adapter = GitHookAdapter::new(path);
}

// ─── install_pre_commit (non-repo) ────────────────────────

#[test]
fn install_pre_commit_non_repo_returns_false() {
    let adapter = sut_in_non_repo();
    let exe_path = FilePath::new("/usr/bin/lint-arwaky").unwrap_or_default();
    let result = adapter.install_pre_commit(&exe_path);
    assert!(result.is_ok());
    assert!(!result.unwrap().value());
}

// ─── uninstall_pre_commit (non-repo) ──────────────────────

#[test]
fn uninstall_pre_commit_non_repo_returns_false() {
    let adapter = sut_in_non_repo();
    let result = adapter.uninstall_pre_commit();
    assert!(result.is_ok());
    assert!(!result.unwrap().value());
}

// ─── install_pre_commit (real temp git repo) ──────────────

#[test]
fn install_pre_commit_in_git_repo_creates_hook() {
    let tmp_dir = std::env::temp_dir().join(format!("git_hooks_test_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);
    let _ = std::fs::create_dir_all(tmp_dir.join(".git"));

    let root = FilePath::new(tmp_dir.to_str().unwrap().to_string()).unwrap_or_default();
    let adapter = GitHookAdapter::new(root);
    let exe_path = FilePath::new("/usr/local/bin/lint-arwaky").unwrap_or_default();

    let result = adapter.install_pre_commit(&exe_path);
    assert!(result.is_ok());
    assert!(result.unwrap().value());

    // Verify hook file exists
    let hook_path = tmp_dir.join(".git").join("hooks").join("pre-commit");
    assert!(hook_path.exists());

    // Verify hook content contains the executable path
    let content = std::fs::read_to_string(&hook_path).unwrap_or_default();
    assert!(content.contains("lint-arwaky"));
    assert!(content.contains("#!/bin/bash"));

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp_dir);
}

#[test]
fn install_pre_commit_with_empty_executable_uses_default_name() {
    let tmp_dir = std::env::temp_dir().join(format!("git_hooks_test_empty_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);
    let _ = std::fs::create_dir_all(tmp_dir.join(".git"));

    let root = FilePath::new(tmp_dir.to_str().unwrap().to_string()).unwrap_or_default();
    let adapter = GitHookAdapter::new(root);
    let exe_path = FilePath::new("").unwrap_or_default();

    let result = adapter.install_pre_commit(&exe_path);
    assert!(result.is_ok());

    let hook_path = tmp_dir.join(".git").join("hooks").join("pre-commit");
    let content = std::fs::read_to_string(&hook_path).unwrap_or_default();
    assert!(content.contains("lint-arwaky check ."));

    let _ = std::fs::remove_dir_all(&tmp_dir);
}

// ─── uninstall_pre_commit (real temp git repo) ────────────

#[test]
fn uninstall_pre_commit_removes_hook_file() {
    let tmp_dir = std::env::temp_dir().join(format!("git_hooks_uninstall_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);
    let hooks_dir = tmp_dir.join(".git").join("hooks");
    let _ = std::fs::create_dir_all(&hooks_dir);

    // Create a pre-commit hook first
    let hook_path = hooks_dir.join("pre-commit");
    std::fs::write(&hook_path, "#!/bin/bash\necho test").unwrap();

    let root = FilePath::new(tmp_dir.to_str().unwrap().to_string()).unwrap_or_default();
    let adapter = GitHookAdapter::new(root);

    let result = adapter.uninstall_pre_commit();
    assert!(result.is_ok());
    assert!(result.unwrap().value());
    assert!(!hook_path.exists());

    let _ = std::fs::remove_dir_all(&tmp_dir);
}

#[test]
fn uninstall_pre_commit_no_hook_file_still_succeeds() {
    let tmp_dir =
        std::env::temp_dir().join(format!("git_hooks_uninstall_noop_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);
    let _ = std::fs::create_dir_all(tmp_dir.join(".git").join("hooks"));

    let root = FilePath::new(tmp_dir.to_str().unwrap().to_string()).unwrap_or_default();
    let adapter = GitHookAdapter::new(root);

    let result = adapter.uninstall_pre_commit();
    assert!(result.is_ok());
    assert!(result.unwrap().value());

    let _ = std::fs::remove_dir_all(&tmp_dir);
}
