// PURPOSE: FRD Requirement — Recursive filesystem watching of project paths.
// "Recursive filesystem watching of project paths for create/modify/delete events."

use std::time::Duration;

use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared::file_watch::{IWatchProviderProtocol, WatchConfig};

#[tokio::test]
async fn frd_001_recursive_watch_detects_nested_file_change() {
    // Arrange: create nested directory structure.
    let root = std::env::temp_dir().join(format!("fw_frd001_{}", std::process::id()));
    let nested = root.join("src").join("deep");
    std::fs::create_dir_all(&nested).expect("create nested dirs");

    let mut config = WatchConfig::from_path(root.to_string_lossy().to_string());
    config.recursive = true;

    let provider = NotifyWatchProvider::new();
    let mut rx = provider.subscribe();

    // Act: start recursive watch.
    provider
        .start(&config)
        .await
        .expect("start recursive watcher");
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Create a file in the nested directory.
    let nested_file = nested.join("module.rs");
    std::fs::write(&nested_file, "pub fn hello() {}").expect("write nested file");

    // Assert: event received for nested file within the window.
    // Drain events for the full timeout — inotify may deliver directory-create
    // events ahead of the nested file event, so check across all received events.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(3);
    let mut found = false;
    while tokio::time::Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_millis(200), rx.recv()).await {
            // Event received: check if it's the nested file we created.
            Ok(Ok(e)) => {
                if e.path.contains("module.rs") {
                    found = true;
                    break;
                }
            }
            // Channel closed (or lagged) — no more events will arrive.
            Ok(Err(_)) => break,
            // Timeout: keep polling until the deadline (inotify may deliver
            // the nested file event after the directory-create event).
            Err(_) => continue,
        }
    }
    assert!(found, "Expected nested file event for module.rs");

    // Cleanup.
    provider.stop().await.ok();
    let _ = std::fs::remove_dir_all(&root);
}

#[tokio::test]
async fn frd_001_non_recursive_watch_ignores_nested() {
    let root = std::env::temp_dir().join(format!("fw_frd001b_{}", std::process::id()));
    let nested = root.join("sub");
    std::fs::create_dir_all(&nested).expect("create dirs");

    let mut config = WatchConfig::from_path(root.to_string_lossy().to_string());
    config.recursive = false;

    let provider = NotifyWatchProvider::new();
    let mut rx = provider.subscribe();

    provider
        .start(&config)
        .await
        .expect("start non-recursive watcher");
    tokio::time::sleep(Duration::from_millis(200)).await;

    // Write to nested — should NOT trigger event in non-recursive mode.
    std::fs::write(nested.join("hidden.rs"), "fn hidden() {}").ok();

    // Write to root — SHOULD trigger event.
    std::fs::write(root.join("visible.rs"), "fn visible() {}").ok();

    // Drain all events within a window, checking across all received events.
    // inotify may report the parent directory instead of the file, and events
    // may arrive out of order, so check all events across the full timeout.
    let deadline = tokio::time::Instant::now() + Duration::from_secs(3);
    let mut found_root = false;
    let mut found_nested = false;
    while tokio::time::Instant::now() < deadline {
        match tokio::time::timeout(Duration::from_millis(200), rx.recv()).await {
            Ok(Ok(e)) => {
                if e.path.contains("visible.rs") || e.path == root.to_string_lossy().as_ref() {
                    found_root = true;
                }
                if e.path.contains("hidden.rs")
                    || e.path.contains("/sub/")
                    || e.path.contains("\\sub\\")
                {
                    found_nested = true;
                }
            }
            Ok(Err(_)) => break,
            Err(_) => continue,
        }
    }
    assert!(
        found_root,
        "Expected root-level event for visible.rs or root directory, got none"
    );
    assert!(
        !found_nested,
        "Got unexpected nested event for sub/hidden.rs; nested should not fire in non-recursive mode"
    );

    provider.stop().await.ok();
    let _ = std::fs::remove_dir_all(&root);
}
