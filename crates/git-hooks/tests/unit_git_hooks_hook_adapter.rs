// Unit tests — HookInstaller (FR-002) and HookUninstaller (FR-003)
// each capability implements exactly one protocol.

use git_hooks_lint_arwaky::capabilities_hook_installer::HookInstaller;
use git_hooks_lint_arwaky::capabilities_hook_uninstaller::HookUninstaller;
use shared_common::FilePath;
use shared_git_hooks::IHookInstallProtocol;
use shared_git_hooks::IHookUninstallProtocol;
use tempfile::TempDir;

fn make_installer(tmp: &TempDir) -> HookInstaller {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let _filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    HookInstaller::new(fp, io)
}

fn make_uninstaller(tmp: &TempDir) -> HookUninstaller {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let _filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = tmp.path().to_string_lossy().to_string();
    HookUninstaller::new(fp, io)
}

fn make_git_repo(tmp: &TempDir) {
    std::fs::create_dir_all(tmp.path().join(".git/hooks")).unwrap();
}

// ─── FR-002: Pre-Commit Hook Installation ─────────────────

#[test]
fn fr002_1_normal_install_creates_hook_script() {
    let tmp = TempDir::new().unwrap();
    make_git_repo(&tmp);
    let installer = make_installer(&tmp);
    let exe = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let result = installer.install_pre_commit(&exe);
    assert!(result.is_ok(), "install should succeed: {:?}", result.err());
    let hook_path = tmp.path().join(".git/hooks/pre-commit");
    assert!(hook_path.exists(), "hook script should exist");
    let content = std::fs::read_to_string(&hook_path).unwrap();
    assert!(
        content.contains("lint-arwaky-cli check ."),
        "hook should contain executable"
    );
    assert!(
        content.starts_with("#!/bin/bash"),
        "hook should start with bash shebang"
    );
}

#[test]
fn fr002_2_creates_hooks_dir_if_missing() {
    let tmp = TempDir::new().unwrap();
    // Only create .git, not .git/hooks
    std::fs::create_dir_all(tmp.path().join(".git")).unwrap();
    let installer = make_installer(&tmp);
    let exe = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let result = installer.install_pre_commit(&exe);
    assert!(result.is_ok(), "install should succeed: {:?}", result.err());
    assert!(tmp.path().join(".git/hooks/pre-commit").exists());
}

#[test]
fn fr002_3_hook_file_already_exists_overwritten() {
    let tmp = TempDir::new().unwrap();
    make_git_repo(&tmp);
    let hook_path = tmp.path().join(".git/hooks/pre-commit");
    std::fs::write(&hook_path, "old content").unwrap();
    let installer = make_installer(&tmp);
    let exe = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let result = installer.install_pre_commit(&exe);
    assert!(result.is_ok());
    let content = std::fs::read_to_string(&hook_path).unwrap();
    assert_ne!(content, "old content", "hook should be overwritten");
    assert!(content.contains("lint-arwaky-cli check ."));
}

#[test]
fn fr002_4_not_git_repo_returns_success_false() {
    let tmp = TempDir::new().unwrap();
    // No .git directory
    let installer = make_installer(&tmp);
    let exe = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let result = installer.install_pre_commit(&exe);
    assert!(result.is_ok());
    assert!(
        !result.unwrap().value,
        "should return false for non-git repo"
    );
}

#[test]
fn fr002_5_unix_permissions_set() {
    #[cfg(unix)]
    {
        let tmp = TempDir::new().unwrap();
        make_git_repo(&tmp);
        let installer = make_installer(&tmp);
        let exe = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
        installer.install_pre_commit(&exe).unwrap();
        let hook_path = tmp.path().join(".git/hooks/pre-commit");
        let perms = std::fs::metadata(&hook_path).unwrap().permissions();
        let mode = std::os::unix::fs::PermissionsExt::mode(&perms);
        assert_eq!(mode & 0o777, 0o755, "permissions should be 0o755");
    }
}

// Note: FR-002 Scenario 7 (empty executable path defaults to lint-arwaky-cli)
// cannot be tested because FilePath rejects empty strings.
// The empty-check in install_pre_commit is dead code — unreachable via the public API.

// ─── FR-003: Pre-Commit Hook Uninstallation ───────────────

#[test]
fn fr003_1_hook_exists_removed() {
    let tmp = TempDir::new().unwrap();
    make_git_repo(&tmp);
    let installer = make_installer(&tmp);
    let uninstaller = make_uninstaller(&tmp);
    let exe = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    installer.install_pre_commit(&exe).unwrap();
    assert!(tmp.path().join(".git/hooks/pre-commit").exists());
    let result = IHookUninstallProtocol::uninstall_pre_commit(&uninstaller);
    assert!(result.is_ok());
    assert!(!tmp.path().join(".git/hooks/pre-commit").exists());
}

#[test]
fn fr003_2_hook_does_not_exist_returns_success() {
    let tmp = TempDir::new().unwrap();
    make_git_repo(&tmp);
    let uninstaller = make_uninstaller(&tmp);
    let result = IHookUninstallProtocol::uninstall_pre_commit(&uninstaller);
    assert!(result.is_ok());
    assert!(result.unwrap().value, "should return true (idempotent)");
}

#[test]
fn fr003_3_not_git_repo_returns_success_false() {
    let tmp = TempDir::new().unwrap();
    let uninstaller = make_uninstaller(&tmp);
    let result = IHookUninstallProtocol::uninstall_pre_commit(&uninstaller);
    assert!(result.is_ok());
    assert!(
        !result.unwrap().value,
        "should return false for non-git repo"
    );
}
