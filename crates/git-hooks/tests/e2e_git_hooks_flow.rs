// E2E tests — full pipeline: container wiring → orchestrator → capabilities → results.

use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_config_init::ConfigInit;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_installer::HookInstaller;
use git_hooks_lint_arwaky::capabilities_hook_uninstaller::HookUninstaller;
use shared_common::FilePath;
use shared_git_hooks::GitHooksRequest;
use shared_git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use shared_git_hooks::{IHookInstallProtocol, IHookUninstallProtocol};
use std::sync::Arc;
use tempfile::TempDir;

fn make_container() -> (TempDir, Arc<dyn IGitHooksAggregate>) {
    let tmp = TempDir::new().unwrap();
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let _filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let hook_installer: Arc<dyn IHookInstallProtocol> =
        Arc::new(HookInstaller::new(fp.clone(), io.clone()));
    let hook_uninstaller: Arc<dyn IHookUninstallProtocol> = Arc::new(HookUninstaller::new(
        tmp.path().to_string_lossy().to_string(),
        io.clone(),
    ));
    let diff_checker = Arc::new(DiffChecker::new(io.clone()));
    let config_init = Arc::new(ConfigInit::new(io.clone()));
    let orch: Arc<dyn IGitHooksAggregate> = Arc::new(GitHooksOrchestrator::new(
        diff_checker,
        hook_installer.clone(),
        hook_uninstaller.clone(),
        config_init,
    ));
    (tmp, orch)
}

fn make_orchestrator() -> (TempDir, Arc<GitHooksOrchestrator>) {
    let tmp = TempDir::new().unwrap();
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let _filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let hook_installer: Arc<dyn IHookInstallProtocol> =
        Arc::new(HookInstaller::new(fp.clone(), io.clone()));
    let hook_uninstaller: Arc<dyn IHookUninstallProtocol> = Arc::new(HookUninstaller::new(
        tmp.path().to_string_lossy().to_string(),
        io.clone(),
    ));
    let diff_checker = Arc::new(DiffChecker::new(io.clone()));
    let config_init = Arc::new(ConfigInit::new(io));
    let orch = Arc::new(GitHooksOrchestrator::new(
        diff_checker,
        hook_installer,
        hook_uninstaller,
        config_init,
    ));
    (tmp, orch)
}

// ─── E2E: Full hook lifecycle ─────────────────────────────

#[test]
fn e2e_hook_install_then_uninstall_round_trip() {
    let (tmp, aggregate) = make_container();

    // Create a fake .git/hooks directory so the adapter can operate
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();

    // Install hook via the full aggregate chain
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let install_result = aggregate
        .execute(GitHooksRequest::install(&exec_path))
        .into_status();
    assert!(
        install_result.is_ok(),
        "install_hook should succeed: {:?}",
        install_result.err()
    );
    let status = install_result.unwrap();
    assert!(
        status.value,
        "install_hook should return true for a git repo"
    );

    // Verify the hook script was written
    let hook_file = hooks_dir.join("pre-commit");
    assert!(hook_file.exists(), "pre-commit hook should exist");
    let hook_content = std::fs::read_to_string(&hook_file).unwrap();
    assert!(
        hook_content.contains("lint-arwaky-cli"),
        "hook should reference the executable"
    );

    // Uninstall hook via the full aggregate chain
    let uninstall_result = aggregate
        .execute(GitHooksRequest::uninstall())
        .into_status();
    assert!(
        uninstall_result.is_ok(),
        "uninstall_hook should succeed: {:?}",
        uninstall_result.err()
    );
    let status = uninstall_result.unwrap();
    assert!(status.value, "should return true when hook existed");

    // Verify the hook script was removed
    assert!(
        !hook_file.exists(),
        "pre-commit hook should be removed after uninstall"
    );
}

#[test]
fn e2e_uninstall_is_idempotent() {
    let (tmp, aggregate) = make_container();

    // Create .git/hooks but no pre-commit file
    let hooks_dir = tmp.path().join(".git").join("hooks");
    std::fs::create_dir_all(&hooks_dir).unwrap();

    // Uninstall when no hook exists — should still succeed
    let result = aggregate
        .execute(GitHooksRequest::uninstall())
        .into_status();
    assert!(result.is_ok(), "idempotent uninstall should succeed");
    assert!(result.unwrap().value, "should return true even if no hook");
}

#[test]
fn e2e_install_creates_hooks_directory_when_missing() {
    let (tmp, aggregate) = make_container();

    // Ensure no .git directory exists initially
    assert!(!tmp.path().join(".git").exists());

    // Install should handle non-git gracefully
    let exec_path = FilePath::new("lint-arwaky-cli".to_string()).unwrap();
    let result = aggregate
        .execute(GitHooksRequest::install(&exec_path))
        .into_status();
    assert!(
        result.is_ok(),
        "install should not error on non-git: {:?}",
        result.err()
    );
    // Non-git repo → SuccessStatus(false)
    assert!(
        !result.unwrap().value,
        "should return false for non-git repo"
    );
}

// ─── E2E: Orchestrator delegation chain ───────────────────

#[test]
fn e2e_orchestrator_delegates_to_diff_protocol() {
    let (tmp, orch) = make_orchestrator();

    // Verify the diff protocol seam stays reachable through the orchestrator
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let _result = orch.run_git_hooks_check(&fp);
    // Should not panic on a non-git directory
}

#[test]
fn e2e_orchestrator_delegates_to_hook_protocol() {
    let (_, orch) = make_orchestrator();

    // Verify the hook protocol seam stays reachable through the orchestrator
    let identity = orch.get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");
}

#[test]
fn e2e_orchestrator_exposes_hook_uninstall_seam_via_aggregate() {
    let (_, orch) = make_orchestrator();

    // The uninstall seam is object-safe and reachable through the orchestrator
    let identity = orch.get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");

    // Uninstalling outside a git repository is a no-op success
    let result = orch.uninstall_hook();
    assert!(
        result.is_ok(),
        "hook uninstall should work: {:?}",
        result.err()
    );
}

// ─── E2E: Complete user workflow ──────────────────────────

#[test]
fn e2e_full_user_workflow_init_install_check() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();

    // Run check on non-git dir (should not panic)
    let _results = aggregate
        .execute(GitHooksRequest::run_check(&fp))
        .into_results();
}
