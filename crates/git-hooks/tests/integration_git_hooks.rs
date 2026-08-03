// Integration tests — full DI wiring via GitContainer.
use git_hooks_lint_arwaky::root_git_hooks_container::GitContainer;
use shared::common::FilePath;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::git_hooks::{GitDiffStatus, HookIgnoreUpdateVO};
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

// ─── New aggregate delegation methods ─────────────────────

#[test]
fn aggregate_initialize_config_on_temp_dir() {
    let (tmp, aggregate) = make_container();
    let result = aggregate.initialize_config(tmp.path().to_str().unwrap());
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
    let result = aggregate.update_ignore_rule(request);
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
    let result = aggregate.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::BothMissing);
}

#[test]
fn aggregate_get_diff_data_identical_files() {
    let (tmp, aggregate) = make_container();
    let p1 = tmp.path().join("a.txt");
    let p2 = tmp.path().join("b.txt");
    std::fs::write(&p1, "same content").unwrap();
    std::fs::write(&p2, "same content").unwrap();
    let result = aggregate.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
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
    let result = aggregate.get_diff_data(p1.to_str().unwrap(), p2.to_str().unwrap());
    assert_eq!(result.status, GitDiffStatus::Modified);
    assert!(result.difference > 0.0);
}
