// Contract tests — verify all concrete types implement their declared contract traits.
use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::git_hooks::contract_diff_protocol::IDiffProtocol;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::contract_hook_protocol::IHookProtocol;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;
use shared::git_hooks::contract_orchestrator_aggregate::HookManagementOrchestratorAggregate;

#[test]
fn diff_checker_implements_diff_protocol() {
    fn assert_trait<T: IDiffProtocol>() {}
    assert_trait::<DiffChecker>();
}

#[test]
fn git_hook_adapter_implements_hook_manager_protocol() {
    fn assert_trait<T: IHookManagerProtocol>() {}
    assert_trait::<GitHookAdapter>();
}

#[test]
fn hook_manager_implements_hook_protocol() {
    fn assert_trait<T: IHookProtocol>() {}
    assert_trait::<HookManager>();
}

#[test]
fn orchestrator_implements_git_hooks_aggregate() {
    fn assert_trait<T: GitHooksAggregate>() {}
    assert_trait::<GitHooksOrchestrator>();
}

#[test]
fn orchestrator_implements_hook_management_aggregate() {
    fn assert_trait<T: HookManagementOrchestratorAggregate>() {}
    assert_trait::<GitHooksOrchestrator>();
}

#[test]
fn all_capabilities_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DiffChecker>();
    assert_send_sync::<GitHookAdapter>();
    assert_send_sync::<HookManager>();
    assert_send_sync::<GitHooksOrchestrator>();
}

#[test]
fn orchestrator_can_be_boxed_as_trait_object() {
    fn assert_object_safe<T: GitHooksAggregate>() {}
    assert_object_safe::<GitHooksOrchestrator>();
}

#[test]
fn hook_manager_can_be_arc_trait_object() {
    fn assert_object_safe<T: IHookProtocol>() {}
    assert_object_safe::<HookManager>();
}
