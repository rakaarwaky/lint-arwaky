// PURPOSE: Unit tests for GitHooksOrchestrator — Agent layer orchestration.
// Covers: delegation to protocols, aggregate access, hook management.
// Layer: Agent (GitHooksOrchestrator)
// Speed: ms

use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::common::taxonomy_path_vo::FilePath;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;
use shared::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;
use std::sync::Arc;

fn sut() -> GitHooksOrchestrator {
    let diff: Arc<dyn IDiffProtocol> = Arc::new(DiffChecker::new());
    let hook_adapter: Arc<dyn IHookManagerProtocol> = Arc::new(GitHookAdapter::new(
        FilePath::new("/tmp/nonexistent").unwrap_or_default(),
    ));
    let hook_protocol: Arc<dyn IHookProtocol> =
        Arc::new(HookManager::new(Arc::clone(&hook_adapter)));
    GitHooksOrchestrator::new(diff, hook_protocol, hook_adapter)
}

// ─── Construction ─────────────────────────────────────────

#[test]
fn new_creates_orchestrator() {
    let _orch = sut();
}

// ─── GitHooksAggregate: diff_protocol access ──────────────

#[test]
fn diff_protocol_returns_reference() {
    let orch = sut();
    let _diff = orch.diff_protocol();
    // Should not panic; returns a valid reference
}

// ─── GitHooksAggregate: hook_protocol access ──────────────

#[test]
fn hook_protocol_returns_reference() {
    let orch = sut();
    let _hook = orch.hook_protocol();
}

// ─── GitHooksAggregate: run_git_hooks_check ───────────────

#[tokio::test]
async fn run_git_hooks_check_delegates_to_diff_protocol() {
    let orch = sut();
    let path = FilePath::new(".").unwrap_or_default();
    let results = orch.run_git_hooks_check(&path).await;
    // Current DiffChecker returns empty list
    assert!(results.is_empty());
}

// ─── GitHooksAggregate: install_hook ──────────────────────

#[tokio::test]
async fn install_hook_delegates_to_hook_protocol() {
    let orch = sut();
    let exe = FilePath::new("/usr/bin/lint-arwaky").unwrap_or_default();
    let result = orch.install_hook(&exe).await;
    assert!(result.is_ok());
    // Non-repo → false
    assert!(!result.unwrap().value());
}

// ─── GitHooksAggregate: uninstall_hook ────────────────────

#[tokio::test]
async fn uninstall_hook_delegates_to_hook_protocol() {
    let orch = sut();
    let result = orch.uninstall_hook().await;
    assert!(result.is_ok());
    assert!(!result.unwrap().value());
}

// ─── HookManagementOrchestratorAggregate ──────────────────

#[test]
fn get_hook_manager_returns_reference() {
    let orch = sut();
    let _manager = orch.get_hook_manager();
}

#[test]
fn get_hook_manager_identity_returns_expected() {
    let orch = sut();
    let identity = orch.get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");
}
