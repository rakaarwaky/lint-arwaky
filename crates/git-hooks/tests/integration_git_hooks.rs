// Integration tests — full DI wiring via GitContainer.
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::common::FilePath;
use shared::git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate;
use shared::git_hooks::GitHooksRequest;
use shared::git_hooks::{GitDiffStatus, HookIgnoreUpdateVO};
use git_hooks_lint_arwaky::agent_git_hooks_orchestrator::GitHooksOrchestrator;
use git_hooks_lint_arwaky::capabilities_diff_checker::DiffChecker;
use git_hooks_lint_arwaky::capabilities_hook_adapter::GitHookAdapter;
use git_hooks_lint_arwaky::capabilities_hook_manager::HookManager;
use shared::git_hooks::{IDiffProtocol, IHookManagerProtocol, IHookProtocol};
use std::sync::Arc;
use tempfile::TempDir;

fn make_orchestrator() -> Arc<GitHooksOrchestrator> {
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let tmp_path = std::env::temp_dir();
    let fp = FilePath::new(tmp_path.to_string_lossy().to_string()).unwrap();
    let hook_adapter: Arc<dyn IHookManagerProtocol> =
        Arc::new(GitHookAdapter::new(fp, filesystem.clone()));
    let diff_protocol: Arc<dyn IDiffProtocol> = Arc::new(DiffChecker::new(filesystem.clone()));
    let hook_protocol: Arc<dyn IHookProtocol> =
        Arc::new(HookManager::new(hook_adapter.clone(), filesystem.clone()));
    Arc::new(GitHooksOrchestrator::new(
        diff_protocol,
        hook_protocol,
        hook_adapter,
    ))
}

fn make_container() -> (TempDir, Arc<dyn IGitHooksAggregate>) {
    let tmp = TempDir::new().unwrap();
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let container = GitContainer::new(fp, filesystem);
    (tmp, container.aggregate())
}

#[test]
fn container_creates_with_filesystem() {
    let tmp = TempDir::new().unwrap();
    let filesystem =
        filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    let _container = GitContainer::new(fp, filesystem);
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
    let _results = aggregate.execute(GitHooksRequest::run_check(&fp)).into_results();
}

#[test]
fn install_hook_on_non_git_dir_returns_ok() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    // Non-git directory: should return SuccessStatus with false
    let result = aggregate.execute(GitHooksRequest::install(&fp)).into_status();
    assert!(
        result.is_ok(),
        "install_hook should not error: {:?}",
        result.err()
    );
}

#[test]
fn uninstall_hook_on_non_git_dir_returns_ok() {
    let (_, aggregate) = make_container();
    let result = aggregate.execute(GitHooksRequest::uninstall()).into_status();
    assert!(
        result.is_ok(),
        "uninstall_hook should not error: {:?}",
        result.err()
    );
}

// ─── New aggregate delegation methods ─────────────────────

#[test]
fn aggregate_initialize_config_on_temp_dir() {
    let (tmp, aggregate) = make_container();
    let result = aggregate
        .execute(GitHooksRequest::initialize_config(tmp.path().to_str().unwrap()))
        .into_description();
    assert!(
        result.value.contains("Initialized"),
        "should initialize config: {}",
        result.value
    );
}

#[test]
fn aggregate_update_ignore_rule_config_not_found() {
    let (tmp, aggregate) = make_container();
    let config_path = tmp.path().join("nonexistent.yaml");
    let request = HookIgnoreUpdateVO::new(
        "test_rule",
        false,
        config_path.to_str().unwrap().to_string(),
    );
    let result = aggregate.execute(GitHooksRequest::update_ignore_rule(request)).into_description();
    assert!(
        result.value.contains("not found") || result.value.contains("Run lint-arwaky-cli"),
        "should report not found: {}",
        result.value
    );
}

#[test]
fn aggregate_get_diff_data_both_missing() {
    let (tmp, aggregate) = make_container();
    let p1 = tmp.path().join("missing1.txt");
    let p2 = tmp.path().join("missing2.txt");
    let result = aggregate.execute(GitHooksRequest::diff_data(p1.to_str().unwrap(), p2.to_str().unwrap())).into_diff_data();
    assert_eq!(result.status, GitDiffStatus::BothMissing);
}

#[test]
fn aggregate_get_diff_data_identical_files() {
    let (tmp, aggregate) = make_container();
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    std::fs::write(&p1, "same content").unwrap();
    std::fs::write(&p2, "same content").unwrap();
    let result = aggregate.execute(GitHooksRequest::diff_data(p1.to_str().unwrap(), p2.to_str().unwrap())).into_diff_data();
    assert_eq!(result.status, GitDiffStatus::Unchanged);
    assert!((result.difference - 0.0).abs() < f64::EPSILON);
}

#[test]
fn aggregate_get_diff_data_different_files() {
    let (tmp, aggregate) = make_container();
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    std::fs::write(&p1, "hello").unwrap();
    std::fs::write(&p2, "world").unwrap();
    let result = aggregate.execute(GitHooksRequest::diff_data(p1.to_str().unwrap(), p2.to_str().unwrap())).into_diff_data();
    assert_eq!(result.status, GitDiffStatus::Modified);
    assert!(result.difference > 0.0);
}
