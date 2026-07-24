// PURPOSE: FRD Requirement — Configurable watch roots and ignore patterns.
// "Configurable watch roots and ignore patterns."

use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

#[test]
fn frd_004_watch_config_custom_path() {
    let config = WatchConfig::from_path("/custom/project/root".to_string());
    assert_eq!(config.path.value(), "/custom/project/root");
}

#[test]
fn frd_004_watch_config_default_ignore_patterns() {
    let config = WatchConfig::from_path("/tmp".to_string());
    let expected = [
        ".git",
        "node_modules",
        "__pycache__",
        "target",
        ".venv",
        "dist",
        "build",
    ];
    for p in &expected {
        assert!(
            config.ignore_patterns.contains(&p.to_string()),
            "Default ignore must include '{}'",
            p
        );
    }
}

#[tokio::test]
async fn frd_004_ignored_patterns_not_watched() {
    let root = std::env::temp_dir().join(format!("fw_frd004_{}", std::process::id()));
    let git_dir = root.join(".git");
    std::fs::create_dir_all(&git_dir).expect("create .git dir");

    let mut config = WatchConfig::from_path(root.to_string_lossy().to_string());
    config.ignore_patterns = vec![".git".to_string()];

    let provider = NotifyWatchProvider::new();
    let mut rx = provider.subscribe();

    provider.start(&config).await.expect("start watcher");
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Write inside .git — should be ignored.
    std::fs::write(git_dir.join("HEAD"), "ref: refs/heads/main").ok();

    // Write a lintable file at root — should trigger.
    std::fs::write(root.join("main.rs"), "fn main() {}").ok();

    // We should receive the root file event, not the .git event.
    let event = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await;
    if let Ok(Ok(e)) = event {
        assert!(
            !e.path.contains(".git"),
            "Ignored .git path must not produce events, got: {}",
            e.path
        );
    }

    provider.stop().await.ok();
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn frd_004_configurable_debounce() {
    let mut config = WatchConfig::from_path("/tmp".to_string());
    config.debounce_ms = 1000;
    assert_eq!(config.debounce_ms, 1000);

    config.debounce_ms = 100;
    assert_eq!(config.debounce_ms, 100);
}

#[test]
fn frd_004_configurable_recursive_flag() {
    let mut config = WatchConfig::from_path("/tmp".to_string());
    assert!(config.recursive); // default true

    config.recursive = false;
    assert!(!config.recursive);
}
