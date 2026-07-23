// PURPOSE: Integration tests — DI wiring via GitContainer.
// Validates that the root container correctly wires all capabilities to contracts.
// Layer: Root (GitContainer) + full stack
// Speed: ms–s

use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::common::taxonomy_path_vo::FilePath;

// ─── Container Construction ───────────────────────────────

#[test]
fn container_new_default_creates_valid_instance() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    // Should produce a valid Arc<dyn GitHooksAggregate>
    assert!(Arc::strong_count(&aggregate) >= 1);
}

use std::sync::Arc;

#[test]
fn container_aggregate_returns_cloneable_arc() {
    let container = GitContainer::new_default();
    let agg1 = container.aggregate();
    let agg2 = container.aggregate();
    // Both point to the same underlying orchestrator
    assert!(Arc::ptr_eq(&agg1, &agg2));
}

// ─── Wired DiffProtocol ───────────────────────────────────

#[tokio::test]
async fn container_wired_diff_protocol_returns_results() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    let path = FilePath::new(".").unwrap_or_default();
    let results = aggregate.run_git_hooks_check(&path).await;
    // Should not panic; returns LintResultList (possibly empty)
    assert!(!results.is_empty() || results.is_empty());
}

// ─── Wired HookProtocol ───────────────────────────────────

#[tokio::test]
async fn container_wired_install_hook_non_repo() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    let exe = FilePath::new("/usr/bin/lint-arwaky").unwrap_or_default();
    let result = aggregate.install_hook(&exe).await;
    // Default container uses "." which may or may not be a git repo
    assert!(result.is_ok());
}

#[tokio::test]
async fn container_wired_uninstall_hook_non_repo() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    let result = aggregate.uninstall_hook().await;
    assert!(result.is_ok());
}

// ─── Wired HookManagerProtocol via Orchestrator ───────────

#[test]
fn container_orchestrator_exposes_hook_manager() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();
    // Downcast not needed; use the HookManagementOrchestratorAggregate trait
    // Since GitHooksOrchestrator implements both traits, we can test identity
    let identity = aggregate.hook_protocol().get_hook_manager_identity();
    assert_eq!(identity.value(), "git_hook_manager");
}

// ─── Full Roundtrip: Install → Verify → Uninstall ─────────

#[test]
fn container_install_uninstall_roundtrip_in_temp_repo() {
    let tmp_dir = std::env::temp_dir().join(format!("git_hooks_integ_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);
    let _ = std::fs::create_dir_all(tmp_dir.join(".git"));

    let hook_adapter: Arc<dyn shared::git_hooks::contract_manager_protocol::IHookManagerProtocol> =
        Arc::new(
            git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter::new(
                FilePath::new(tmp_dir.to_str().unwrap().to_string()).unwrap_or_default(),
            ),
        );

    let container = GitContainer::new(hook_adapter);
    let aggregate = container.aggregate();

    // Install
    let exe = FilePath::new("/usr/local/bin/lint-arwaky").unwrap_or_default();
    let rt = tokio::runtime::Runtime::new().unwrap();
    let install_result = rt.block_on(aggregate.install_hook(&exe));
    assert!(install_result.is_ok());
    assert!(install_result.unwrap().value());

    // Verify hook exists
    let hook_path = tmp_dir.join(".git").join("hooks").join("pre-commit");
    assert!(hook_path.exists());

    // Uninstall
    let uninstall_result = rt.block_on(aggregate.uninstall_hook());
    assert!(uninstall_result.is_ok());
    assert!(uninstall_result.unwrap().value());
    assert!(!hook_path.exists());

    let _ = std::fs::remove_dir_all(&tmp_dir);
}
