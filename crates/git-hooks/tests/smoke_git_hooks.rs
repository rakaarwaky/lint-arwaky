// Smoke tests — verify container creation and aggregate access complete within 5s.
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::common::FilePath;
use std::sync::Arc;

#[test]
fn git_container_creates() {
    let start = std::time::Instant::now();
    let filesystem = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let fp = FilePath::new("/tmp".to_string()).unwrap();
    let _container = GitContainer::new(fp, filesystem);
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn git_container_aggregate_accessible() {
    let start = std::time::Instant::now();
    let filesystem = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let fp = FilePath::new("/tmp".to_string()).unwrap();
    let container = GitContainer::new(fp, filesystem);
    let _agg = container.aggregate();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}

#[test]
fn git_container_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<GitContainer>();
}

#[test]
fn git_container_aggregate_trait_object() {
    let start = std::time::Instant::now();
    let filesystem = filesystem_lint_arwaky::root_filesystem_container::FilesystemContainer::new()
        .orchestrator();
    let fp = FilePath::new("/tmp".to_string()).unwrap();
    let container = GitContainer::new(fp, filesystem);
    let _: Arc<dyn shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate> =
        container.aggregate();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
