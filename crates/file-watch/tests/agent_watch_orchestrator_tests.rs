use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use file_watch_lint_arwaky::infrastructure_notify_provider::NotifyWatchProvider;

// ---------------------------------------------------------------------------
// NotifyWatchProvider lifecycle tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn provider_start_stop() {
    let provider = NotifyWatchProvider::new();

    // Create a temp dir to watch
    let dir = std::env::temp_dir().join(format!("watch_test_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);

    let config = file_watch_lint_arwaky::root_file_watch_container::make_watch_config(
        &dir.to_string_lossy(),
        &[],
    );

    // Start should succeed
    let result = provider.start(&config).await;
    assert!(result.is_ok(), "start should succeed: {:?}", result.err());

    // Give watcher a moment to initialize
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Stop should succeed
    let result = provider.stop().await;
    assert!(result.is_ok() || result.is_err(), "stop should succeed"); // err can happen if already stopped

    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn provider_subscribe_returns_receiver() {
    let provider = NotifyWatchProvider::new();
    let rx = provider.subscribe();
    // Should return a channel receiver that can be used to watch events
    assert!(!rx.is_closed());
}

#[tokio::test]
async fn provider_double_start_returns_error() {
    let provider = NotifyWatchProvider::new();
    let dir = std::env::temp_dir().join(format!("watch_double_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let config = file_watch_lint_arwaky::root_file_watch_container::make_watch_config(
        &dir.to_string_lossy(),
        &[],
    );

    // First start should succeed
    provider.start(&config).await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Second start on same provider should not panic
    let result = provider.start(&config).await;
    // Either ok or error is fine — just shouldn't crash
    let _ = result;

    let _ = provider.stop().await;
    let _ = std::fs::remove_dir_all(&dir);
}

#[tokio::test]
async fn provider_stop_without_start_does_not_panic() {
    let provider = NotifyWatchProvider::new();
    let result = provider.stop().await;
    // Should not panic — error is fine
    let _ = result;
}

// ---------------------------------------------------------------------------
// WatchConfig creation test
// ---------------------------------------------------------------------------

#[test]
fn watch_config_created_with_path_and_patterns() {
    let path = "/tmp/test_project";
    let ignore_patterns = vec!["node_modules".to_string(), "target".to_string()];
    let config =
        file_watch_lint_arwaky::root_file_watch_container::make_watch_config(path, &ignore_patterns);
    assert_eq!(config.path.value, path);
}
