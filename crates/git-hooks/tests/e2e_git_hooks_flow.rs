// E2E tests — full pipeline: container wiring → orchestrator → capabilities → results.

use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::common::FilePath;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;
use shared::git_hooks::{GitDiffStatus, HookIgnoreUpdateVO, IDiffProtocol, IHookManagerProtocol, IHookProtocol};
use std::sync::Arc;
use tempfile::TempDir;

fn make_container() -> (TempDir, Arc<dyn GitHooksAggregate>) {
    let tmp = TempDir::new().unwrap();
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let hook_adapter: Arc<dyn IHookManagerProtocol> =
        Arc::new(GitHookAdapter::new(fp, filesystem.clone()));
    let diff_protocol: Arc<dyn IDiffProtocol> =
        Arc::new(DiffChecker::new(filesystem.clone()));
    let hook_protocol: Arc<dyn IHookProtocol> =
        Arc::new(HookManager::new(hook_adapter.clone(), filesystem.clone()));
    let orch: Arc<dyn GitHooksAggregate> = Arc::new(GitHooksOrchestrator::new(
        diff_protocol, hook_protocol, hook_adapter,
    ));
    (tmp, orch)
}

fn make_orchestrator() -> (TempDir, Arc<GitHooksOrchestrator>) {
    let tmp = TempDir::new().unwrap();
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let hook_adapter: Arc<dyn IHookManagerProtocol> =
        Arc::new(GitHookAdapter::new(fp, filesystem.clone()));
    let diff_protocol: Arc<dyn IDiffProtocol> =
        Arc::new(DiffChecker::new(filesystem.clone()));
    let hook_protocol: Arc<dyn IHookProtocol> =
        Arc::new(HookManager::new(hook_adapter.clone(), filesystem.clone()));
    let orch = Arc::new(GitHooksOrchestrator::new(
        diff_protocol, hook_protocol, hook_adapter,
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
    let install_result = aggregate.install_hook(&exec_path);
    assert!(
        install_result.is_ok(),
        "install_hook should succeed: {:?}",
        install_result.err()
    );
    let status = install_result.unwrap();
    assert!(status.value, "install_hook should return true for a git repo");

    // Verify the hook script was written
    let hook_file = hooks_dir.join("pre-commit");
    assert!(hook_file.exists(), "pre-commit hook should exist");
    let hook_content = std::fs::read_to_string(&hook_file).unwrap();
    assert!(
        hook_content.contains("lint-arwaky-cli"),
        "hook should reference the executable"
    );

    // Uninstall hook via the full aggregate chain
    let uninstall_result = aggregate.uninstall_hook();
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
    let result = aggregate.uninstall_hook();
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
    let result = aggregate.install_hook(&exec_path);
    assert!(
        result.is_ok(),
        "install should not error on non-git: {:?}",
        result.err()
    );
    // Non-git repo → SuccessStatus(false)
    assert!(!result.unwrap().value, "should return false for non-git repo");
}

// ─── E2E: Config initialization → ignore rule management ──

#[test]
fn e2e_config_init_then_add_ignore_rule() {
    let (tmp, aggregate) = make_container();

    // Step 1: Initialize config
    let init_result = aggregate.initialize_config(tmp.path().to_str().unwrap());
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
    let add_result = aggregate.update_ignore_rule(request);
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
    let dup_result = aggregate.update_ignore_rule(request_dup);
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
    aggregate.initialize_config(tmp.path().to_str().unwrap());
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    let request =
        HookIgnoreUpdateVO::new("target", false, config_path.to_str().unwrap().to_string());
    aggregate.update_ignore_rule(request);

    // Now remove it
    let remove_request =
        HookIgnoreUpdateVO::new("target", true, config_path.to_str().unwrap().to_string());
    let remove_result = aggregate.update_ignore_rule(remove_request);
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
    let result = aggregate.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    assert!((result.difference - 0.0).abs() < f64::EPSILON);

    // Step 2: Modify second file
    write_file(&p2, "modified content here");
    let result = aggregate.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
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

    let result = aggregate.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::MissingSecond);

    // Only second file exists
    let p3 = tmp.path().join("also_missing.txt");
    let p4 = tmp.path().join("also_exists.txt");
    write_file(&p4, "content");

    let result = aggregate.get_diff_data(p3.to_str().unwrap(), p4.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::MissingFirst);
}

// ─── E2E: Orchestrator delegation chain ───────────────────

#[test]
fn e2e_orchestrator_delegates_to_diff_protocol() {
    let (tmp, aggregate) = make_container();

    // Verify the diff_protocol accessor returns a usable reference
    let diff = aggregate.diff_protocol();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let _result = diff.run_git_diff_check(&fp);
    // Should not panic on a non-git directory
}

#[test]
fn e2e_orchestrator_delegates_to_hook_protocol() {
    let (_, aggregate) = make_container();

    // Verify the hook_protocol accessor returns a usable reference
    let hook = aggregate.hook_protocol();
    let identity = hook.get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");
}

#[test]
fn e2e_orchestrator_exposes_hook_manager_via_aggregate() {
    let (_, orch) = make_orchestrator();

    // HookManagementOrchestratorAggregate is object-safe and accessible
    let manager: &dyn IHookManagerProtocol = orch.get_hook_manager();
    let identity = orch.get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");

    // Manager should be usable
    let result = manager.uninstall_pre_commit();
    assert!(
        result.is_ok(),
        "hook manager uninstall should work: {:?}",
        result.err()
    );
}

// ─── E2E: Complete user workflow ──────────────────────────

#[test]
fn e2e_full_user_workflow_init_install_check_config() {
    let (tmp, aggregate) = make_container();

    // 1. Initialize config
    let init = aggregate.initialize_config(tmp.path().to_str().unwrap());
    assert!(init.value.contains("Initialized"));

    // 2. Add ignore rule
    let config_path = tmp.path().join("lint_arwaky.config.yaml");
    let add = aggregate.update_ignore_rule(HookIgnoreUpdateVO::new(
        "vendor",
        false,
        config_path.to_str().unwrap().to_string(),
    ));
    assert!(add.value.contains("Added"));

    // 3. Remove ignore rule
    let remove = aggregate.update_ignore_rule(HookIgnoreUpdateVO::new(
        "vendor",
        true,
        config_path.to_str().unwrap().to_string(),
    ));
    assert!(remove.value.contains("Removed"));

    // 4. Check on non-git dir (should not panic)
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let _results = aggregate.run_git_hooks_check(&fp);

    // 5. Diff data comparison
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    write_file(&p1, "hello");
    write_file(&p2, "world");
    let diff = aggregate.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(diff.status, GitDiffStatus::Modified);
}
