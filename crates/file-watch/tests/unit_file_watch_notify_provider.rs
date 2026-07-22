// PURPOSE: Unit tests for NotifyWatchProvider — start/stop lifecycle, subscribe, availability.
// Layer: Capabilities (target ≥ 70% coverage)

use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

// ─── Helpers ────────────────────────────────────────────────

fn temp_dir_config() -> WatchConfig {
    let dir = std::env::temp_dir().join(format!("fw_test_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    WatchConfig::from_path(dir.to_string_lossy().to_string())
}

fn nonexistent_path_config() -> WatchConfig {
    WatchConfig::from_path("/nonexistent/path/that/does/not/exist".to_string())
}

// ─── new / default ──────────────────────────────────────────

#[test]
fn new_returns_provider() {
    let provider = NotifyWatchProvider::new();
    let _ = &provider;
}

#[test]
fn default_returns_provider() {
    let provider = NotifyWatchProvider::default();
    let _ = &provider;
}

// ─── is_available ───────────────────────────────────────────

#[tokio::test]
async fn is_available_returns_boolean_vo() {
    let provider = NotifyWatchProvider::new();
    let available = provider.is_available().await;
    // The "watch" feature may or may not be enabled in test builds.
    // We just verify it returns a valid BooleanVO without panicking.
    let _ = available.value();
}

// ─── subscribe ──────────────────────────────────────────────

#[test]
fn subscribe_returns_receiver() {
    let provider = NotifyWatchProvider::new();
    let _rx = provider.subscribe();
    // Receiver is valid — no panic.
}

#[test]
fn subscribe_multiple_receivers() {
    let provider = NotifyWatchProvider::new();
    let _rx1 = provider.subscribe();
    let _rx2 = provider.subscribe();
    // Multiple subscribers allowed (broadcast channel).
}

// ─── start with invalid path ────────────────────────────────

#[tokio::test]
async fn start_nonexistent_path_returns_error() {
    let provider = NotifyWatchProvider::new();
    let config = nonexistent_path_config();
    let result = provider.start(&config).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn start_nonexistent_path_error_contains_message() {
    let provider = NotifyWatchProvider::new();
    let config = nonexistent_path_config();
    let err = provider.start(&config).await.unwrap_err();
    let msg = err.message.value().to_string();
    assert!(
        msg.contains("does not exist"),
        "Expected 'does not exist' in: {}",
        msg
    );
}

// ─── start + stop lifecycle ─────────────────────────────────

#[tokio::test]
async fn start_valid_path_succeeds() {
    let provider = NotifyWatchProvider::new();
    let config = temp_dir_config();
    let result = provider.start(&config).await;
    assert!(result.is_ok());
    let _ = provider.stop().await;
}

#[tokio::test]
async fn stop_without_start_succeeds() {
    let provider = NotifyWatchProvider::new();
    let result = provider.stop().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn start_then_stop_then_stop_again() {
    let provider = NotifyWatchProvider::new();
    let config = temp_dir_config();
    provider.start(&config).await.unwrap();
    provider.stop().await.unwrap();
    // Second stop should also succeed (idempotent).
    let result = provider.stop().await;
    assert!(result.is_ok());
}

// ─── WatchConfig defaults ───────────────────────────────────

#[test]
fn watch_config_from_path_defaults() {
    let config = WatchConfig::from_path("/tmp/test".to_string());
    assert!(config.recursive);
    assert_eq!(config.debounce_ms, 500);
    assert!(config.ignore_patterns.contains(&".git".to_string()));
    assert!(config.ignore_patterns.contains(&"node_modules".to_string()));
    assert!(config.ignore_patterns.contains(&"target".to_string()));
}

#[test]
fn watch_config_ignore_patterns_comprehensive() {
    let config = WatchConfig::from_path("/tmp".to_string());
    let expected = vec![
        ".git",
        "node_modules",
        "__pycache__",
        "target",
        ".venv",
        "dist",
        "build",
    ];
    for pattern in expected {
        assert!(
            config.ignore_patterns.contains(&pattern.to_string()),
            "Missing ignore pattern: {}",
            pattern
        );
    }
}
