use std::sync::Arc;
use std::time::Duration;

use file_watch_lint_arwaky::infrastructure_notify_provider::NotifyWatchProvider;
use shared::file_watch::contract_provider_port::IWatchProviderPort;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

fn make_config(path: &str) -> WatchConfig {
    WatchConfig {
        path: shared::common::taxonomy_path_vo::FilePath::new(path.to_string()).unwrap_or_default(),
        ignore_patterns: vec![],
        recursive: true,
        debounce_ms: 100,
    }
}

#[tokio::test]
async fn provider_subscribe_returns_receiver() {
    let provider = NotifyWatchProvider::new();
    let rx = provider.subscribe();
    assert!(!rx.is_closed());
}

#[tokio::test]
async fn provider_stop_without_start_does_not_panic() {
    let provider = NotifyWatchProvider::new();
    let result = provider.stop().await;
    let _ = result;
}

#[tokio::test]
async fn provider_start_stop() {
    let provider = NotifyWatchProvider::new();
    let dir = std::env::temp_dir().join(format!("watch_test_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);

    let config = make_config(&dir.to_string_lossy());
    let result = provider.start(&config).await;
    assert!(result.is_ok(), "start should succeed: {:?}", result.err());

    tokio::time::sleep(Duration::from_millis(200)).await;

    let result = provider.stop().await;
    assert!(result.is_ok(), "stop should succeed: {:?}", result.err());
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn provider_start_nonexistent_path_fails() {
    let provider = NotifyWatchProvider::new();
    let config = make_config("/nonexistent_path_xyz_12345");
    let result = provider.start(&config).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn provider_double_start_does_not_panic() {
    let provider = NotifyWatchProvider::new();
    let dir = std::env::temp_dir().join(format!("watch_double_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let config = make_config(&dir.to_string_lossy());

    let _ = provider.start(&config).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    let result = provider.start(&config).await;
    let _ = result;
    let _ = provider.stop().await;
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn watch_config_created_with_path() {
    let config = make_config("/tmp/test_project");
    assert_eq!(config.path.value, "/tmp/test_project");
    assert!(config.recursive);
}
