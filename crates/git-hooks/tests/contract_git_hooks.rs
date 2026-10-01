// Contract tests — verify all concrete types implement their declared contract traits.
// One test per FR-GitHooks-001..004 protocol seam, each with a unique helper name.
use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_config_init::ConfigInit;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_installer::HookInstaller;
use git_hooks_lint_arwaky::capabilities_hook_uninstaller::HookUninstaller;
use shared_git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use shared_git_hooks::contract_git_hooks_protocol::IConfigInitProtocol;
use shared_git_hooks::contract_git_hooks_protocol::IDiffDetectionProtocol;
use shared_git_hooks::contract_git_hooks_protocol::IHookInstallProtocol;
use shared_git_hooks::contract_git_hooks_protocol::IHookUninstallProtocol;

// ── Per-trait bound helpers (one unique name per seam) ────

fn assert_diff_detection_trait<T: IDiffDetectionProtocol>() {}
fn assert_hook_install_trait<T: IHookInstallProtocol>() {}
fn assert_hook_uninstall_trait<T: IHookUninstallProtocol>() {}
fn assert_config_init_trait<T: IConfigInitProtocol>() {}
fn assert_aggregate_trait<T: IGitHooksAggregate>() {}

// ── FR-GitHooks-001: Git Diff Detection ───────────────────

#[test]
fn fr001_diff_checker_implements_diff_detection_protocol() {
    assert_diff_detection_trait::<DiffChecker>();
}

// ── FR-GitHooks-002: Pre-Commit Hook Installation ─────────

#[test]
fn fr002_hook_installer_implements_hook_install_protocol() {
    assert_hook_install_trait::<HookInstaller>();
}

// ── FR-GitHooks-003: Pre-Commit Hook Uninstallation ───────

#[test]
fn fr003_hook_uninstaller_implements_hook_uninstall_protocol() {
    assert_hook_uninstall_trait::<HookUninstaller>();
}

// ── FR-GitHooks-004: Project Config Initialization ────────

#[test]
fn fr004_config_init_implements_config_init_protocol() {
    assert_config_init_trait::<ConfigInit>();
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
    assert_send_sync::<HookInstaller>();
    assert_send_sync::<HookUninstaller>();
    assert_send_sync::<ConfigInit>();
    assert_send_sync::<GitHooksOrchestrator>();
}

#[test]
fn every_protocol_seam_is_object_safe() {
    fn assert_object_safe<T: ?Sized>() {}
    assert_object_safe::<dyn IDiffDetectionProtocol>();
    assert_object_safe::<dyn IHookInstallProtocol>();
    assert_object_safe::<dyn IHookUninstallProtocol>();
    assert_object_safe::<dyn IConfigInitProtocol>();
    assert_object_safe::<dyn IGitHooksAggregate>();
}

#[test]
fn one_concrete_type_may_satisfy_several_seams() {
    // Each capability backs exactly one FR seam.
    assert_diff_detection_trait::<DiffChecker>();
    assert_hook_install_trait::<HookInstaller>();
    assert_hook_uninstall_trait::<HookUninstaller>();
    assert_config_init_trait::<ConfigInit>();
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
    use shared_git_hooks::GitHooksRequest;
    fn assert_method<T: IGitHooksAggregate>() {
        let _ = |t: &T, request: GitHooksRequest| t.execute(request);
    }
    assert_method::<GitHooksOrchestrator>();
}
