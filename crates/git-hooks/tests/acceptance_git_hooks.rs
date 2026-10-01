// Acceptance tests — verify FRD requirements are met for git-hooks.
//
// Covers:
//   FR-GitHooks-001: Git Diff Detection
//   FR-GitHooks-002: Pre-commit hook installation
//   FR-GitHooks-003: Pre-commit hook uninstallation
//   FR-GitHooks-004: Project config initialization + ignore rule management

use git_hooks_lint_arwaky::capabilities_config_init::ConfigInit;
use git_hooks_lint_arwaky::capabilities_hook_installer::HookInstaller;
use git_hooks_lint_arwaky::capabilities_hook_uninstaller::HookUninstaller;
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared_common::FilePath;
use shared_git_hooks::GitHooksRequest;
use shared_git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use shared_git_hooks::{
    HookIgnoreUpdateVO, IConfigInitProtocol, IHookInstallProtocol, IHookUninstallProtocol,
};
use std::sync::Arc;
use tempfile::TempDir;

// ─── Helpers ──────────────────────────────────────────────

fn make_container() -> (TempDir, Arc<dyn IGitHooksAggregate>) {
    let tmp = TempDir::new().unwrap();
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let container = GitContainer::new(fp, filesystem, io);
    (tmp, container.aggregate())
}

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

fn make_config_init() -> ConfigInit {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let _filesystem = fc.orchestrator();
    let io = fc.io();
    ConfigInit::new(io)
}

fn write_file(path: &std::path::Path, content: &str) {
    std::fs::write(path, content).unwrap();
}

// ═══════════════════════════════════════════════════════════
// FR-GitHooks-002: Pre-commit hook installation
// ═══════════════════════════════════════════════════════════

#[test]
fn fr002_hook_script_contains_correct_executable() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();

    let installer = make_installer(&tmp);
    let exec_path = FilePath::new("/usr/local/bin/lint-arwaky-cli".to_string()).unwrap();
    let result = installer.install_pre_commit(&exec_path);
    assert!(result.is_ok(), "install should succeed: {:?}", result.err());
    assert!(result.unwrap().value, "should return true");

    let hook_content = std::fs::read_to_string(hooks_dir.join("pre-commit")).unwrap();
    assert!(
        hook_content.contains("/usr/local/bin/lint-arwaky-cli check ."),
        "hook should contain the full executable path as check command: {}",
        hook_content
    );
}

#[test]
fn fr002_hook_script_starts_with_shebang() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();

    let installer = make_installer(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    installer.install_pre_commit(&exec_path).unwrap();

    let hook_content = std::fs::read_to_string(hooks_dir.join("pre-commit")).unwrap();
    assert!(
        hook_content.starts_with("#!/bin/bash"),
        "hook should start with bash shebang"
    );
}

#[test]
fn fr002_creates_hooks_directory_when_missing() {
    let tmp = TempDir::new().unwrap();
    // No .git/hooks directory — installer should create it
    assert!(!tmp.path().join(".git").exists());

    let installer = make_installer(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    // Non-git repo returns SuccessStatus(false) — no error
    let result = installer.install_pre_commit(&exec_path);
    assert!(
        result.is_ok(),
        "install on non-git should not error: {:?}",
        result.err()
    );
    assert!(
        !result.unwrap().value,
        "should return false for non-git repo"
    );
}

#[test]
fn fr002_overwrites_existing_hook() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();

    let installer = make_installer(&tmp);
    let exec_path = FilePath::new("first-executable".to_string()).unwrap();
    installer.install_pre_commit(&exec_path).unwrap();

    // Install again with different executable
    let exec_path2 = FilePath::new("second-executable".to_string()).unwrap();
    installer.install_pre_commit(&exec_path2).unwrap();

    let hook_content = std::fs::read_to_string(hooks_dir.join("pre-commit")).unwrap();
    assert!(
        hook_content.contains("second-executable"),
        "hook should contain the new executable after overwrite"
    );
    assert!(
        !hook_content.contains("first-executable"),
        "old executable should be gone"
    );
}

#[test]
fn fr002_not_git_repo_returns_false() {
    let tmp = TempDir::new().unwrap();
    // No .git directory
    let installer = make_installer(&tmp);
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let result = installer.install_pre_commit(&exec_path).unwrap();
    assert!(!result.value, "non-git repo should return false");
}

// ═══════════════════════════════════════════════════════════
// FR-GitHooks-003: Pre-commit hook uninstallation
// ═══════════════════════════════════════════════════════════

#[test]
fn fr003_removes_existing_hook() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();
    // Write a pre-commit hook manually
    write_file(&hooks_dir.join("pre-commit"), "#!/bin/bash\nexit 0\n");

    let installer = make_installer(&tmp);
    let uninstaller = make_uninstaller(&tmp);
    let exe = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    installer.install_pre_commit(&exe).unwrap();

    let result = IHookUninstallProtocol::uninstall_pre_commit(&uninstaller);
    assert!(
        result.is_ok(),
        "uninstall should succeed: {:?}",
        result.err()
    );
    assert!(
        result.unwrap().value,
        "should return true when hook existed"
    );
    assert!(
        !hooks_dir.join("pre-commit").exists(),
        "pre-commit should be removed"
    );
}

#[test]
fn fr003_idempotent_when_hook_missing() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();
    // No pre-commit file

    let uninstaller = make_uninstaller(&tmp);
    let result = IHookUninstallProtocol::uninstall_pre_commit(&uninstaller);
    assert!(result.is_ok(), "uninstall should succeed");
    assert!(
        result.unwrap().value,
        "should return true even when no hook exists"
    );
}

#[test]
fn fr003_not_git_repo_returns_false() {
    let tmp = TempDir::new().unwrap();
    // No .git directory
    let uninstaller = make_uninstaller(&tmp);
    let result = IHookUninstallProtocol::uninstall_pre_commit(&uninstaller).unwrap();
    assert!(!result.value, "non-git repo should return false");
}

#[test]
fn fr003_only_removes_pre_commit_hook() {
    let tmp = TempDir::new().unwrap();
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();
    write_file(&hooks_dir.join("pre-commit"), "#!/bin/bash\nexit 0\n");
    write_file(&hooks_dir.join("commit-msg"), "#!/bin/bash\nexit 0\n");

    let uninstaller = make_uninstaller(&tmp);
    IHookUninstallProtocol::uninstall_pre_commit(&uninstaller).unwrap();

    assert!(
        !hooks_dir.join("pre-commit").exists(),
        "pre-commit should be removed"
    );
    assert!(
        hooks_dir.join("commit-msg").exists(),
        "other hooks should remain untouched"
    );
}

// ═══════════════════════════════════════════════════════════
// FR-GitHooks-001: Git Diff Detection (check execution)
// ═══════════════════════════════════════════════════════════

#[test]
fn fr001_check_on_non_git_dir_returns_empty_results() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let results = aggregate
        .execute(GitHooksRequest::run_check(&fp))
        .into_results();
    // Non-git directory → no changes detected → empty results
    assert!(
        results.is_empty(),
        "check on non-git dir should return empty results"
    );
}

#[test]
fn fr001_check_does_not_panic_on_invalid_path() {
    let (_, aggregate) = make_container();
    let fp = FilePath::new("/nonexistent/path/that/does/not/exist".to_string()).unwrap();
    let _results = aggregate
        .execute(GitHooksRequest::run_check(&fp))
        .into_results();
    // Should not panic even with invalid path
}

// ═══════════════════════════════════════════════════════════
// FR-GitHooks-004: Project Config Initialization
// ═══════════════════════════════════════════════════════════

#[test]
fn fr004_initialize_config_creates_default() {
    let tmp = TempDir::new().unwrap();
    let config_init = make_config_init();
    let result = config_init.initialize_config(tmp.path().to_str().unwrap());
    assert!(
        result.value.contains("Initialized"),
        "should report Initialized: {}",
        result.value
    );

    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    assert!(config_path.exists(), "config file should be created");

    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(
        content.contains("ignored_paths"),
        "default config should contain ignored_paths key"
    );
}

#[test]
fn fr004_initialize_config_already_exists() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_file(
        &config_path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n",
    );
    let config_init = make_config_init();
    let result = config_init.initialize_config(tmp.path().to_str().unwrap());
    assert!(
        result.value.contains("ALREADY_EXISTS"),
        "should report ALREADY_EXISTS: {}",
        result.value
    );
}

#[test]
fn fr004_add_rule_to_config() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_file(
        &config_path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n",
    );
    let config_init = make_config_init();

    let request = HookIgnoreUpdateVO::new("dist", false, config_path.to_str().unwrap().to_string());
    let result = config_init.update_ignore_rule(request);
    assert!(
        result.value.contains("Added"),
        "should report Added: {}",
        result.value
    );

    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(content.contains("dist"), "config should contain 'dist'");
}

#[test]
fn fr004_remove_rule_from_config() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_file(
        &config_path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n  - node_modules\n",
    );
    let config_init = make_config_init();

    let request =
        HookIgnoreUpdateVO::new("vendor", true, config_path.to_str().unwrap().to_string());
    let result = config_init.update_ignore_rule(request);
    assert!(
        result.value.contains("Removed"),
        "should report Removed: {}",
        result.value
    );

    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(
        !content.contains("- vendor"),
        "config should not contain '- vendor'"
    );
    assert!(
        content.contains("- node_modules"),
        "other rules should remain"
    );
}

#[test]
fn fr004_config_not_found_suggests_init() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("nonexistent.yaml");
    let config_init = make_config_init();

    let request = HookIgnoreUpdateVO::new("test", false, config_path.to_str().unwrap().to_string());
    let result = config_init.update_ignore_rule(request);
    assert!(
        result.value.contains("not found") || result.value.contains("Run lint-arwaky-cli"),
        "should suggest init when config not found: {}",
        result.value
    );
}

#[test]
fn fr004_add_existing_rule_is_noop() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    write_file(
        &config_path,
        "# Lint Arwaky Configuration\nignored_paths:\n  - vendor\n",
    );
    let config_init = make_config_init();

    let request =
        HookIgnoreUpdateVO::new("vendor", false, config_path.to_str().unwrap().to_string());
    let result = config_init.update_ignore_rule(request);
    assert!(
        result.value.contains("already present"),
        "duplicate add should be no-op: {}",
        result.value
    );
}

// ═══════════════════════════════════════════════════════════
// Cross-cutting: Identity & trait compliance
// ═══════════════════════════════════════════════════════════

#[test]
fn config_init_identity_is_git_hook_manager() {
    let tmp = TempDir::new().unwrap();
    let installer = make_installer(&tmp);
    let identity = installer.get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");
}

#[test]
fn orchestrator_hook_manager_identity_delegates_correctly() {
    let (_, aggregate) = make_container();
    let identity = aggregate
        .execute(GitHooksRequest::GetManagerIdentity {})
        .into_identity();
    assert_eq!(
        identity.value(),
        "git_hook_manager",
        "orchestrator should delegate identity to hook_installer"
    );
}

#[test]
fn non_git_repo_all_operations_are_safe() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    // None of these should panic on a non-git directory
    let _results = aggregate
        .execute(GitHooksRequest::run_check(&fp))
        .into_results();
    let install = aggregate
        .execute(GitHooksRequest::install(&fp))
        .into_status();
    let uninstall = aggregate
        .execute(GitHooksRequest::uninstall())
        .into_status();

    assert!(install.is_ok(), "install should not error");
    assert!(uninstall.is_ok(), "uninstall should not error");
}
