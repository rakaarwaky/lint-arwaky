// Smoke tests — verify container creation and aggregate access complete within 5s.
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared_common::FilePath;
use shared_quality_rules::ICodeAnalysisAggregate;
use std::sync::Arc;

fn test_linter() -> Arc<dyn ICodeAnalysisAggregate> {
    quality_rules::root_quality_rules_container::CodeAnalysisContainer::new().code_analysis_linter()
}

#[test]
fn git_container_creates() {
    let start = std::time::Instant::now();
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new("/tmp".to_string()).unwrap();
    let _container = GitContainer::new(fp, filesystem, io, test_linter());
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
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new("/tmp".to_string()).unwrap();
    let container = GitContainer::new(fp, filesystem, io, test_linter());
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
    let fc = filesystem::root_filesystem_container::FilesystemContainer::new();
    let filesystem = fc.orchestrator();
    let io = fc.io();
    let fp = FilePath::new("/tmp".to_string()).unwrap();
    let container = GitContainer::new(fp, filesystem, io, test_linter());
    let _: Arc<dyn shared_git_hooks::contract_git_hooks_aggregate::IGitHooksAggregate> =
        container.aggregate();
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs() < 5,
        "Smoke test exceeded 5s: {:?}",
        elapsed
    );
}
