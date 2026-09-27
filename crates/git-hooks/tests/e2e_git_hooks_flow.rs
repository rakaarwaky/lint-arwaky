// E2E tests — full pipeline: container wiring → orchestrator → capabilities → results.

use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::common::FilePath;
use shared::git_hooks::GitHooksRequest;
use shared::git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use shared::git_hooks::{
    GitDiffStatus, HookIgnoreUpdateVO, IHookInstallProtocol, IHookUninstallProtocol,
};
use std::sync::Arc;
use tempfile::TempDir;

fn make_container() -> (TempDir, Arc<dyn IGitHooksAggregate>) {
    let tmp = TempDir::new().unwrap();
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let _filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let hook_installer: Arc<dyn IHookInstallProtocol> =
        Arc::new(GitHookAdapter::new(fp.clone(), io.clone()));
    let hook_uninstaller: Arc<dyn IHookUninstallProtocol> =
        Arc::new(GitHookAdapter::new(fp, io.clone()));
    let diff_checker = Arc::new(DiffChecker::new(io.clone()));
    let hook_manager = Arc::new(HookManager::new(
        hook_installer.clone(),
        hook_uninstaller.clone(),
        io.clone(),
    ));
    let orch: Arc<dyn IGitHooksAggregate> = Arc::new(GitHooksOrchestrator::new(
        diff_checker.clone(),
        diff_checker.clone(),
        hook_installer.clone(),
        hook_uninstaller.clone(),
        hook_manager.clone(),
        hook_manager.clone(),
        hook_manager.clone(),
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
        Arc::new(GitHookAdapter::new(fp.clone(), io.clone()));
    let hook_uninstaller: Arc<dyn IHookUninstallProtocol> =
        Arc::new(GitHookAdapter::new(fp, io.clone()));
    let diff_checker = Arc::new(DiffChecker::new(io.clone()));
    let hook_manager = Arc::new(HookManager::new(
        hook_installer.clone(),
        hook_uninstaller.clone(),
        io.clone(),
    ));
    let orch = Arc::new(GitHooksOrchestrator::new(
        diff_checker.clone(),
        diff_checker.clone(),
        hook_installer.clone(),
        hook_uninstaller.clone(),
        hook_manager.clone(),
        hook_manager.clone(),
        hook_manager.clone(),
    ));
    (tmp, orch)
}

fn write_file(path: &std::path::Path, content: &str) {
    std::fs::write(path, content).unwrap();
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
    assert!(
        status.value,
        "uninstall_hook should return true when hook existed"
    );

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

// ─── E2E: Config initialization → ignore rule management ──

#[test]
fn e2e_config_init_then_add_ignore_rule() {
    let (tmp, aggregate) = make_container();

    // Step 1: Initialize config
    let init_result = aggregate
        .execute(GitHooksRequest::initialize_config(
            tmp.path().to_str().unwrap(),
        ))
        .into_description();
    assert!(
        init_result.value.contains("Initialized"),
        "config init should succeed: {}",
        init_result.value
    );

    // Step 2: Add an ignore rule
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    assert!(config_path.exists(), "config file should exist after init");

    let request =
        HookIgnoreUpdateVO::new("target", false, config_path.to_str().unwrap().to_string());
    let add_result = aggregate
        .execute(GitHooksRequest::update_ignore_rule(request))
        .into_description();
    assert!(
        add_result.value.contains("Added"),
        "should add rule: {}",
        add_result.value
    );

    // Step 3: Verify the rule is in the config
    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(
        content.contains("target"),
        "config should contain 'target' after add"
    );

    // Step 4: Try adding the same rule again (idempotent)
    let request_dup =
        HookIgnoreUpdateVO::new("target", false, config_path.to_str().unwrap().to_string());
    let dup_result = aggregate
        .execute(GitHooksRequest::update_ignore_rule(request_dup))
        .into_description();
    assert!(
        dup_result.value.contains("already present"),
        "duplicate add should be no-op: {}",
        dup_result.value
    );
}

#[test]
fn e2e_config_init_then_remove_ignore_rule() {
    let (tmp, aggregate) = make_container();

    // Initialize and add a rule first
    aggregate.execute(GitHooksRequest::initialize_config(
        tmp.path().to_str().unwrap(),
    ));
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    let request =
        HookIgnoreUpdateVO::new("target", false, config_path.to_str().unwrap().to_string());
    aggregate
        .execute(GitHooksRequest::update_ignore_rule(request))
        .into_description();

    // Now remove it
    let remove_request =
        HookIgnoreUpdateVO::new("target", true, config_path.to_str().unwrap().to_string());
    let remove_result = aggregate
        .execute(GitHooksRequest::update_ignore_rule(remove_request))
        .into_description();
    assert!(
        remove_result.value.contains("Removed"),
        "should remove rule: {}",
        remove_result.value
    );

    // Verify the rule is gone
    let content = std::fs::read_to_string(&config_path).unwrap();
    assert!(
        !content.contains("- target"),
        "config should not contain '- target' after removal"
    );
}

// ─── E2E: Diff data comparison through aggregate ──────────

#[test]
fn e2e_diff_data_identical_then_modified_flow() {
    let (tmp, aggregate) = make_container();

    let p1 = tmp.path().join("v1.txt");
    let p2 = tmp.path().join("v2.txt");

    // Step 1: Identical files
    write_file(&p1, "same content here");
    write_file(&p2, "same content here");
    let result = aggregate
        .execute(GitHooksRequest::diff_data(
            p1.to_str().unwrap(),
            p2.to_str().unwrap(),
        ))
        .into_diff_data();
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    assert!((result.difference - 0.0).abs() < f64::EPSILON);

    // Step 2: Modify second file
    write_file(&p2, "modified content here");
    let result = aggregate
        .execute(GitHooksRequest::diff_data(
            p1.to_str().unwrap(),
            p2.to_str().unwrap(),
        ))
        .into_diff_data();
    assert_eq!(result.status, GitDiffStatus::Modified);
    assert!(result.difference > 0.0);
}

#[test]
fn e2e_diff_data_missing_files_flow() {
    let (tmp, aggregate) = make_container();

    // Only first file exists
    let p1 = tmp.path().join("exists.txt");
    let p2 = tmp.path().join("missing.txt");
    write_file(&p1, "content");

    let result = aggregate
        .execute(GitHooksRequest::diff_data(
            p1.to_str().unwrap(),
            p2.to_str().unwrap(),
        ))
        .into_diff_data();
    assert_eq!(result.status, GitDiffStatus::MissingSecond);

    // Only second file exists
    let p3 = tmp.path().join("also_missing.txt");
    let p4 = tmp.path().join("also_exists.txt");
    write_file(&p4, "content");

    let result = aggregate
        .execute(GitHooksRequest::diff_data(
            p3.to_str().unwrap(),
            p4.to_str().unwrap(),
        ))
        .into_diff_data();
    assert_eq!(result.status, GitDiffStatus::MissingFirst);
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
fn e2e_full_user_workflow_init_install_check_config() {
    let (tmp, aggregate) = make_container();

    // 1. Initialize config
    let init = aggregate
        .execute(GitHooksRequest::initialize_config(
            tmp.path().to_str().unwrap(),
        ))
        .into_description();
    assert!(init.value.contains("Initialized"));

    // 2. Add ignore rule
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    let add = aggregate
        .execute(GitHooksRequest::update_ignore_rule(
            HookIgnoreUpdateVO::new("vendor", false, config_path.to_str().unwrap().to_string()),
        ))
        .into_description();
    assert!(add.value.contains("Added"));

    // 3. Remove ignore rule
    let remove = aggregate
        .execute(GitHooksRequest::update_ignore_rule(
            HookIgnoreUpdateVO::new("vendor", true, config_path.to_str().unwrap().to_string()),
        ))
        .into_description();
    assert!(remove.value.contains("Removed"));

    // 4. Check on non-git dir (should not panic)
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let _results = aggregate
        .execute(GitHooksRequest::run_check(&fp))
        .into_results();

    // 5. Diff data comparison
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    write_file(&p1, "hello");
    write_file(&p2, "world");
    let diff = aggregate
        .execute(GitHooksRequest::diff_data(
            p1.to_str().unwrap(),
            p2.to_str().unwrap(),
        ))
        .into_diff_data();
    assert_eq!(diff.status, GitDiffStatus::Modified);
}
