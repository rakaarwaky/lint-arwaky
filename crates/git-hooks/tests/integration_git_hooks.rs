// Integration tests — full DI wiring via GitContainer.
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::common::FilePath;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use std::sync::Arc;
use tempfile::TempDir;

fn make_container() -> (TempDir, Arc<dyn GitHooksAggregate>) {
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
    let _: Arc<dyn GitHooksAggregate> = aggregate;
}

#[test]
fn orchestrator_diff_protocol_accessible() {
    let (_, aggregate) = make_container();
    let _diff = aggregate.diff_protocol();
}

#[test]
fn orchestrator_hook_protocol_accessible() {
    let (_, aggregate) = make_container();
    let _hook = aggregate.hook_protocol();
}

#[test]
fn run_git_hooks_check_on_temp_dir() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    // Should not panic even on a non-git directory
    let _results = aggregate.run_git_hooks_check(&fp);
}

#[test]
fn install_hook_on_non_git_dir_returns_ok() {
    let (tmp, aggregate) = make_container();
    let fp = FilePath::new(tmp.path().to_string_lossy().to_string()).unwrap();
    // Non-git directory: should return SuccessStatus with false
    let result = aggregate.install_hook(&fp);
    assert!(
        result.is_ok(),
        "install_hook should not error: {:?}",
        result.err()
    );
}

#[test]
fn uninstall_hook_on_non_git_dir_returns_ok() {
    let (_, aggregate) = make_container();
    let result = aggregate.uninstall_hook();
    assert!(
        result.is_ok(),
        "uninstall_hook should not error: {:?}",
        result.err()
    );
}
