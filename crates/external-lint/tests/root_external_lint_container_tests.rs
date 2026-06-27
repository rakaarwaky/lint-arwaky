use external_lint_lint_arwaky::root_external_lint_container::ExternalLintContainer;
use shared::common::taxonomy_path_vo::FilePath;

#[tokio::test]
async fn container_can_be_constructed_with_default() {
    let container = ExternalLintContainer::new_default();
    let aggregate = container.aggregate();
    let names = aggregate.adapter_names();
    assert!(names.contains(&"ruff".to_string()));
    assert!(names.contains(&"bandit".to_string()));
    assert!(names.contains(&"mypy".to_string()));
    assert!(names.contains(&"eslint".to_string()));
    assert!(names.contains(&"prettier".to_string()));
    assert!(names.contains(&"tsc".to_string()));
    assert!(names.contains(&"clippy".to_string()));
    assert!(names.contains(&"rustfmt".to_string()));
    assert!(names.contains(&"cargo-audit".to_string()));
    assert_eq!(names.len(), 9);
}

#[tokio::test]
async fn container_aggregate_scan_returns_ok_for_empty_dir() {
    let container = ExternalLintContainer::new_default();
    let aggregate = container.aggregate();
    let dir = std::env::temp_dir().join(format!("extlint_empty_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let results = aggregate.scan_all(&path).await;
    assert!(results.is_empty());
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn container_aggregate_scan_nonexistent_path_does_not_panic() {
    let container = ExternalLintContainer::new_default();
    let aggregate = container.aggregate();
    let path = FilePath::new("/nonexistent/path/xyz_extlint".to_string()).unwrap_or_default();
    let results = aggregate.scan_all(&path).await;
    assert!(results.is_empty());
}
