use maintenance_lint_arwaky::agent_maintenance_orchestrator::MaintenanceCommandsOrchestrator;
use shared::common::taxonomy_path_vo::FilePath;

#[tokio::test]
async fn test_stats_returns_non_negative() {
    let orch = MaintenanceCommandsOrchestrator::new();
    let path = FilePath::new(std::env::current_dir().unwrap().to_string_lossy().to_string()).unwrap_or_default();
    let stats = orch.stats(&path).await;
    assert!(stats.total_files.value >= 0);
    assert!(stats.test_files.value >= 0);
    assert!(stats.test_ratio.value >= 0.0);
    assert!(stats.test_ratio.value <= 1.0);
}

#[tokio::test]
async fn test_stats_counts_python_files() {
    let dir = std::env::temp_dir().join("lint_arwaky_test_stats");
    let _ = std::fs::create_dir_all(&dir);
    std::fs::write(dir.join("a.py"), "x = 1").unwrap();
    std::fs::write(dir.join("test_b.py"), "def test_x(): pass").unwrap();
    std::fs::write(dir.join("readme.txt"), "hello").unwrap();

    let orch = MaintenanceCommandsOrchestrator::new();
    let path = FilePath::new(dir.to_string_lossy().to_string()).unwrap_or_default();
    let stats = orch.stats(&path).await;
    assert_eq!(stats.python_files.value, 2);
    assert_eq!(stats.test_files.value, 1);

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn test_doctor_returns_result() {
    let orch = MaintenanceCommandsOrchestrator::new();
    let result = orch.doctor().await;
    assert_eq!(result.python_version.value, "3.12");
}

#[tokio::test]
async fn test_container_creates() {
    let container = maintenance_lint_arwaky::root_maintenance_container::MaintenanceContainer::new();
    let orch = container.orchestrator();
    let path = FilePath::new(std::env::current_dir().unwrap().to_string_lossy().to_string()).unwrap_or_default();
    let stats = orch.stats(&path).await;
    assert!(stats.total_files.value >= 0);
}
