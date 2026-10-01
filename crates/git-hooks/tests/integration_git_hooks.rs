// Integration tests — full DI wiring via GitContainer.
use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_config_init::ConfigInit;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_installer::HookInstaller;
use git_hooks_lint_arwaky::capabilities_hook_uninstaller::HookUninstaller;
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared_common::FilePath;
use shared_git_hooks::GitHooksRequest;
use shared_git_hooks::IHookInstallProtocol;
use shared_git_hooks::IHookUninstallProtocol;
use shared_git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use std::sync::Arc;
use tempfile::TempDir;

fn make_orchestrator() -> Arc<GitHooksOrchestrator> {
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let _filesystem = fc.orchestrator();
    let io = fc.io();
    let tmp_path = std::env::temp_dir();
    let fp = FilePath::new(tmp_path.to_string_lossy().to_string()).unwrap();
    let hook_installer: Arc<dyn IHookInstallProtocol> =
        Arc::new(HookInstaller::new(fp.clone(), io.clone()));
    let hook_uninstaller: Arc<dyn IHookUninstallProtocol> = Arc::new(HookUninstaller::new(
        tmp_path.to_string_lossy().to_string(),
        io.clone(),
    ));
    let diff_checker = Arc::new(DiffChecker::new(io.clone()));
    let config_init = Arc::new(ConfigInit::new(io));
    Arc::new(GitHooksOrchestrator::new(
        diff_checker,
        hook_installer,
        hook_uninstaller,
        config_init,
    ))
}

fn make_container() -> (TempDir, Arc<dyn IGitHooksAggregate>) {
    let tmp = TempDir::new().unwrap();
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let container = GitContainer::new(fp, filesystem, io);
    (tmp, container.aggregate())
}

#[test]
fn container_creates_with_filesystem() {
    let tmp = TempDir::new().unwrap();
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let _container = GitContainer::new(fp, filesystem, io);
}

#[test]
fn container_aggregate_is_trait_object() {
    let (_, aggregate) = make_container();
    let _: Arc<dyn IGitHooksAggregate> = aggregate;
}

#[test]
fn orchestrator_diff_protocol_accessible() {
    let orch = make_orchestrator();
    let _diff = orch.diff_protocol();
}

#[test]
fn orchestrator_hook_protocol_accessible() {
    let orch = make_orchestrator();
    let _hook = orch.hook_protocol();
}

#[test]
fn run_git_hooks_check_on_temp_dir() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    // Should not panic even on a non-git directory
    let _results = aggregate
        .execute(GitHooksRequest::run_check(&fp))
        .into_results();
}

#[test]
fn install_hook_on_non_git_dir_returns_ok() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    // Non-git directory: should return SuccessStatus with false
    let result = aggregate
        .execute(GitHooksRequest::install(&fp))
        .into_status();
    assert!(
        result.is_ok(),
        "install_hook should not error: {:?}",
        result.err()
    );
}

#[test]
fn uninstall_hook_on_non_git_dir_returns_ok() {
    let (_, aggregate) = make_container();
    let result = aggregate
        .execute(GitHooksRequest::uninstall())
        .into_status();
    assert!(
        result.is_ok(),
        "uninstall_hook should not error: {:?}",
        result.err()
    );
}
