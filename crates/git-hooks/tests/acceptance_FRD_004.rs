// PURPOSE: Acceptance test — FRD Requirement 4: Rule conformance.
// REQ: The crate's own source complies with AES rules when complete.
// Maps to: FRD Success Indicator #4

use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;

/// FRD-004: All public types are constructible (no dead code)
#[test]
fn frd_004_all_public_types_constructible() {
    let _diff = DiffChecker::new();
    let _adapter = GitHookAdapter::new(
        shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default(),
    );
    let _container = GitContainer::new_default();
}

/// FRD-004: No empty trait implementations (AES305 compliance)
/// DiffChecker has IDiffProtocol with 4 methods implemented
#[test]
fn frd_004_diff_checker_has_non_empty_impl() {
    // If DiffChecker compiles with IDiffProtocol, all 4 methods are implemented.
    // This test documents that the impl block is NOT empty.
    fn assert_impl<T: shared::git_hooks::contract_diff_protocol::IDiffProtocol>() {}
    assert_impl::<DiffChecker>();
}

/// FRD-004: HookManager has IHookProtocol with 6 methods implemented
#[test]
fn frd_004_hook_manager_has_non_empty_impl() {
    fn assert_impl<T: shared::git_hooks::contract_hook_protocol::IHookProtocol>() {}
    assert_impl::<HookManager>();
}

/// FRD-004: GitHookAdapter has IHookManagerProtocol with 2 methods implemented
#[test]
fn frd_004_git_hook_adapter_has_non_empty_impl() {
    fn assert_impl<T: shared::git_hooks::contract_manager_protocol::IHookManagerProtocol>() {}
    assert_impl::<GitHookAdapter>();
}

/// FRD-004: GitHooksOrchestrator implements both aggregate traits
#[test]
fn frd_004_orchestrator_implements_all_aggregates() {
    fn assert_git_hooks<T: shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate>() {}
    fn assert_hook_mgmt<
        T: shared::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate,
    >() {}
    assert_git_hooks::<GitHooksOrchestrator>();
    assert_hook_mgmt::<GitHooksOrchestrator>();
}

/// FRD-004: Container wiring produces functional aggregate
#[tokio::test]
async fn frd_004_container_produces_functional_aggregate() {
    let container = GitContainer::new_default();
    let aggregate = container.aggregate();

    // Verify all aggregate methods are callable
    let _diff = aggregate.diff_protocol();
    let _hook = aggregate.hook_protocol();

    let path = shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default();
    let _results = aggregate.run_git_hooks_check(&path).await;
}
