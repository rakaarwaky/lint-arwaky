// Contract tests — verify all concrete types implement their declared contract traits.
// One test per FR-GitHooks-001..007 protocol seam, each with a unique helper name.
use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use shared::git_hooks::contract_git_hooks_protocol::IConfigInitProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IDiffDataProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IDiffDetectionProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IHookCheckProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IHookInstallProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IHookUninstallProtocol;
use shared::git_hooks::contract_git_hooks_protocol::IIgnoreRuleProtocol;

// ── Per-trait bound helpers (one unique name per seam) ────

fn assert_diff_detection_trait<T: IDiffDetectionProtocol>() {}
fn assert_hook_install_trait<T: IHookInstallProtocol>() {}
fn assert_hook_uninstall_trait<T: IHookUninstallProtocol>() {}
fn assert_hook_check_trait<T: IHookCheckProtocol>() {}
fn assert_diff_data_trait<T: IDiffDataProtocol>() {}
fn assert_ignore_rule_trait<T: IIgnoreRuleProtocol>() {}
fn assert_config_init_trait<T: IConfigInitProtocol>() {}
fn assert_aggregate_trait<T: IGitHooksAggregate>() {}

// ── FR-GitHooks-001: Git Diff Detection ───────────────────

#[test]
fn fr001_diff_checker_implements_diff_detection_protocol() {
    assert_diff_detection_trait::<DiffChecker>();
}

// ── FR-GitHooks-002: Pre-Commit Hook Installation ─────────

#[test]
fn fr002_hook_adapter_implements_hook_install_protocol() {
    assert_hook_install_trait::<GitHookAdapter>();
}

// ── FR-GitHooks-003: Pre-Commit Hook Uninstallation ───────

#[test]
fn fr003_hook_adapter_implements_hook_uninstall_protocol() {
    assert_hook_uninstall_trait::<GitHookAdapter>();
}

// ── FR-GitHooks-004: Git Hooks Check Execution ────────────

#[test]
fn fr004_diff_checker_implements_hook_check_protocol() {
    assert_hook_check_trait::<DiffChecker>();
}

// ── FR-GitHooks-005: Diff Data Comparison ─────────────────

#[test]
fn fr005_hook_manager_implements_diff_data_protocol() {
    assert_diff_data_trait::<HookManager>();
}

// ── FR-GitHooks-006: Ignore Rule Management ───────────────

#[test]
fn fr006_hook_manager_implements_ignore_rule_protocol() {
    assert_ignore_rule_trait::<HookManager>();
}

// ── FR-GitHooks-007: Config Initialization ────────────────

#[test]
fn fr007_hook_manager_implements_config_init_protocol() {
    assert_config_init_trait::<HookManager>();
}

// ── Cross-cutting trait obligations ───────────────────────

#[test]
fn orchestrator_implements_git_hooks_aggregate() {
    assert_aggregate_trait::<GitHooksOrchestrator>();
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
fn every_protocol_seam_is_object_safe() {
    fn assert_object_safe<T: ?Sized>() {}
    assert_object_safe::<dyn IDiffDetectionProtocol>();
    assert_object_safe::<dyn IHookInstallProtocol>();
    assert_object_safe::<dyn IHookUninstallProtocol>();
    assert_object_safe::<dyn IHookCheckProtocol>();
    assert_object_safe::<dyn IDiffDataProtocol>();
    assert_object_safe::<dyn IIgnoreRuleProtocol>();
    assert_object_safe::<dyn IConfigInitProtocol>();
    assert_object_safe::<dyn IGitHooksAggregate>();
}

#[test]
fn one_concrete_type_may_satisfy_several_seams() {
    // DiffChecker backs both FR-001 and FR-004; HookManager backs FR-005..007.
    assert_diff_detection_trait::<DiffChecker>();
    assert_hook_check_trait::<DiffChecker>();
    assert_hook_install_trait::<HookManager>();
    assert_hook_uninstall_trait::<HookManager>();
    assert_diff_data_trait::<HookManager>();
    assert_ignore_rule_trait::<HookManager>();
    assert_config_init_trait::<HookManager>();
}

// ─── Aggregate contract tests ──────────────────────────────
// AES101 `_aggregate`: exactly one method, the request/response entry point.

#[test]
fn aggregate_is_object_safe() {
    fn assert_object_safe<T: IGitHooksAggregate + Send + Sync>() {}
    assert_object_safe::<GitHooksOrchestrator>();
}

#[test]
fn aggregate_exposes_a_single_execute_entry_point() {
    use shared::git_hooks::GitHooksRequest;
    fn assert_method<T: IGitHooksAggregate>() {
        let _ = |t: &T, request: GitHooksRequest| t.execute(request);
    }
    assert_method::<GitHooksOrchestrator>();
}
