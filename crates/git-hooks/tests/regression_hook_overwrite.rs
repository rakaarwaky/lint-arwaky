// Regression tests — QA #640 (BE-cycle CRITICAL/WARNING defect guard).
//
// Defect: `lint-arwaky-cli install-hook` silently overwrote a pre-existing
// custom `.git/hooks/pre-commit` hook with no backup, destroying user content.
// Fixed behavior: the installer writes the user's hook to
// `pre-commit.lint-arwaky.bak` before overwriting. These tests lock that
// behavior in permanently (see TEST.md §2.0 Regression Test Convention).

use git_hooks_lint_arwaky::capabilities_hook_installer::HookInstaller;
use shared_common::FilePath;
use shared_git_hooks::IHookInstallProtocol;
use tempfile::TempDir;

fn make_installer(tmp: &TempDir) -> HookInstaller {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let io = fc.io();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    HookInstaller::new(fp, io)
}

fn setup_git_repo_with_hook(tmp: &TempDir, hook_content: &str) -> std::path::PathBuf {
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();
    let hook_path = hooks_dir.join("pre-commit");
    std::fs::write(&hook_path, hook_content).unwrap();
    hook_path
}

const CUSTOM_HOOK: &str = "#!/bin/bash\n# my team's custom hook\necho custom-check\nexit 0\n";

// QA #640: a pre-existing custom hook must be backed up, not silently
// destroyed, when install-hook runs.
#[test]
fn regression_640_existing_custom_hook_is_backed_up_on_install() {
    let tmp = TempDir::new().unwrap();
    let hook_path = setup_git_repo_with_hook(&tmp, CUSTOM_HOOK);

    let installer = make_installer(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let result = installer.install_pre_commit(&exec_path);
    assert!(result.is_ok(), "install should succeed: {:?}", result.err());
    assert!(result.unwrap().value, "install should report success");

    // The new managed hook replaced the file...
    let installed = std::fs::read_to_string(&hook_path).unwrap();
    assert!(
        installed.contains("Lint Arwaky Pre-Commit Hook"),
        "managed hook should be installed: {}",
        installed
    );
    assert!(
        !installed.contains("custom-check"),
        "custom content should no longer be the active hook"
    );

    // ...and the original custom hook survived in the backup file.
    let backup_path = tmp
        .path()
        .join(".git")
        .join("hooks")
        .join("pre-commit.lint-arwaky.bak");
    assert!(
        backup_path.exists(),
        "backup of the pre-existing custom hook must exist"
    );
    let backup = std::fs::read_to_string(&backup_path).unwrap();
    assert_eq!(
        backup, CUSTOM_HOOK,
        "backup must preserve the original hook byte-for-byte"
    );
}

// QA #640: re-running install-hook over an already-managed hook is idempotent
// — identical content must not produce a spurious backup.
#[test]
fn regression_640_reinstall_over_managed_hook_creates_no_backup() {
    let tmp = TempDir::new().unwrap();
    setup_git_repo_with_hook(&tmp, CUSTOM_HOOK);

    let installer = make_installer(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    assert!(installer.install_pre_commit(&exec_path).unwrap().value);
    assert!(installer.install_pre_commit(&exec_path).unwrap().value);

    let backup_path = tmp
        .path()
        .join(".git")
        .join("hooks")
        .join("pre-commit.lint-arwaky.bak");
    let backup = std::fs::read_to_string(&backup_path).unwrap();
    assert_eq!(
        backup, CUSTOM_HOOK,
        "backup must still hold the original custom hook, not the managed hook"
    );
}

// QA #640: an empty placeholder hook carries no user content worth keeping —
// overwriting it must succeed without creating a backup file.
#[test]
fn regression_640_empty_hook_is_overwritten_without_backup() {
    let tmp = TempDir::new().unwrap();
    setup_git_repo_with_hook(&tmp, "");

    let installer = make_installer(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    assert!(installer.install_pre_commit(&exec_path).unwrap().value);

    let hook_path = tmp.path().join(".git").join("hooks").join("pre-commit");
    let installed = std::fs::read_to_string(&hook_path).unwrap();
    assert!(installed.contains("Lint Arwaky Pre-Commit Hook"));
    assert!(
        !tmp.path()
            .join(".git")
            .join("hooks")
            .join("pre-commit.lint-arwaky.bak")
            .exists(),
        "empty hooks must not produce backup files"
    );
}

// QA #640: install-hook outside a git repository stays a no-op and must not
// touch hook paths at all.
#[test]
fn regression_640_non_git_repo_is_noop() {
    let tmp = TempDir::new().unwrap();
    let installer = make_installer(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let result = installer.install_pre_commit(&exec_path).unwrap();
    assert!(!result.value, "non-git repo must report no-op (false)");
    assert!(
        !tmp.path().join(".git").exists(),
        "no .git dir must be created"
    );
}
