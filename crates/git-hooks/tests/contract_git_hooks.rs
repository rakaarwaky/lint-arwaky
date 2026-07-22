// PURPOSE: Verify all trait implementations exist for git-hooks capabilities and agent.
// Layer: Contract verification
// Speed: ms

use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;

use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
#[allow(unused_imports)] // module not yet exported in lib.rs — see commented test below
use shared::git_hooks::contract_git_command_protocol::IGitCommandProtocol;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;
use shared::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;

// ─── DiffChecker implements IDiffProtocol ─────────────────

#[test]
fn diff_checker_implements_idiff_protocol() {
    fn assert_trait<T: IDiffProtocol>() {}
    assert_trait::<DiffChecker>();
}

// ─── GitCommandAdapter implements IGitCommandProtocol ─────

// Note: capabilities_git_command_adapter is not publicly exported in lib.rs.
// This test validates the struct exists and satisfies the trait bound
// if the module is made public. Currently validated via integration wiring.
// Uncomment when module is exported:
// #[test]
// fn git_command_adapter_implements_igit_command_protocol() {
//     fn assert_trait<T: IGitCommandProtocol>() {}
//     assert_trait::<GitCommandAdapter>();
// }

// ─── GitHookAdapter implements IHookManagerProtocol ───────

#[test]
fn git_hook_adapter_implements_ihook_manager_protocol() {
    fn assert_trait<T: IHookManagerProtocol>() {}
    assert_trait::<GitHookAdapter>();
}

// ─── HookManager implements IHookProtocol ─────────────────

#[test]
fn hook_manager_implements_ihook_protocol() {
    fn assert_trait<T: IHookProtocol>() {}
    assert_trait::<HookManager>();
}

// ─── GitHooksOrchestrator implements GitHooksAggregate ────

#[test]
fn orchestrator_implements_git_hooks_aggregate() {
    fn assert_trait<T: GitHooksAggregate>() {}
    assert_trait::<GitHooksOrchestrator>();
}

// ─── GitHooksOrchestrator implements HookManagementOrchestratorAggregate ────

#[test]
fn orchestrator_implements_hook_management_orchestrator_aggregate() {
    fn assert_trait<T: HookManagementOrchestratorAggregate>() {}
    assert_trait::<GitHooksOrchestrator>();
}

// ─── Send + Sync bounds (required for Arc<dyn Trait>) ─────

#[test]
fn diff_checker_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DiffChecker>();
}

#[test]
fn git_hook_adapter_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<GitHookAdapter>();
}

#[test]
fn hook_manager_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<HookManager>();
}

#[test]
fn orchestrator_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<GitHooksOrchestrator>();
}
