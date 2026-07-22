// PURPOSE: Acceptance test — FRD Requirement 1: Hook installation.
// REQ: Hooks correctly installed on all supported system types (Linux, macOS, Windows).
// Maps to: FRD Success Indicator #1

use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;

fn create_temp_repo() -> (tempfile::TempDir, String) {
    let tmp_dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(tmp_dir.path().join(".git")).unwrap();
    let path_str = tmp_dir.path().to_str().unwrap().to_string();
    (tmp_dir, path_str)
}

/// FRD-001: Pre-commit hook is correctly installed in .git/hooks/pre-commit
#[test]
fn frd_001_hook_installed_in_correct_location() {
    let (tmp_dir, path_str) = create_temp_repo();

    let adapter = GitHookAdapter::new(FilePath::new(path_str).unwrap_or_default());
    let exe = FilePath::new("/usr/bin/lint-arwaky").unwrap_or_default();
    let result = adapter.install_pre_commit(&exe);

    assert!(result.is_ok());
    assert!(result.unwrap().value());

    let hook_path = tmp_dir.path().join(".git").join("hooks").join("pre-commit");
    assert!(
        hook_path.exists(),
        "Hook must exist at .git/hooks/pre-commit"
    );
}

/// FRD-001: Hook script is executable (Unix permissions set)
#[test]
#[cfg(unix)]
fn frd_001_hook_has_executable_permission() {
    use std::os::unix::fs::PermissionsExt;

    let (tmp_dir, path_str) = create_temp_repo();

    let adapter = GitHookAdapter::new(FilePath::new(path_str).unwrap_or_default());
    let exe = FilePath::new("/usr/bin/lint-arwaky").unwrap_or_default();
    let _ = adapter.install_pre_commit(&exe);

    let hook_path = tmp_dir.path().join(".git").join("hooks").join("pre-commit");
    let metadata = std::fs::metadata(&hook_path).unwrap();
    let mode = metadata.permissions().mode();
    // Check executable bit (owner)
    assert!(mode & 0o100 != 0, "Hook must be executable");
}

/// FRD-001: Hook script contains valid bash shebang
#[test]
fn frd_001_hook_has_valid_shebang() {
    let (tmp_dir, path_str) = create_temp_repo();

    let adapter = GitHookAdapter::new(FilePath::new(path_str).unwrap_or_default());
    let exe = FilePath::new("/usr/bin/lint-arwaky").unwrap_or_default();
    let _ = adapter.install_pre_commit(&exe);

    let hook_path = tmp_dir.path().join(".git").join("hooks").join("pre-commit");
    let content = std::fs::read_to_string(&hook_path).unwrap();
    assert!(
        content.starts_with("#!/bin/bash"),
        "Hook must start with bash shebang"
    );
}

/// FRD-001: Installation in non-git directory returns false (no-op)
#[test]
fn frd_001_non_git_dir_returns_false() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let adapter = GitHookAdapter::new(FilePath::new(tmp_dir.path().to_str().unwrap().to_string()).unwrap_or_default());
    let exe = FilePath::new("/usr/bin/lint-arwaky").unwrap_or_default();
    let result = adapter.install_pre_commit(&exe);
    assert!(result.is_ok());
    assert!(!result.unwrap().value());
}
