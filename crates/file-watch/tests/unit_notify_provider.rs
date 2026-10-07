// PURPOSE: Unit test — #901 regression: real debounced events must be forwarded,
// never discarded. Moved out of capabilities_notify_provider.rs (AES403 test
// placement: source files must not contain test modules).

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use file_watch_lint_arwaky::capabilities_notify_provider::NotifyWatchProvider;
use shared_file_watch::contract_watch_protocol::IWatchLifecycleProtocol;
use shared_file_watch::taxonomy_file_watch_vo::WatchConfig;

/// Regression for #901: the old code only forwarded events whose kind
/// matched `DebouncedEventKind::Any` via a comparison that could never
/// match, so every filesystem event was silently discarded. The fixed
/// callback must forward real debounced events regardless of kind.
///
/// This test drives the real debouncer on a temp directory: a file write
/// must produce a `WatchEvent` on the broadcast channel.
#[tokio::test]
async fn real_debounced_event_is_not_discarded() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path_buf: PathBuf = tmp.path().to_path_buf();

    // Pre-create a file so the watcher has something to observe.
    let target = path_buf.join("probe.txt");
    fs::write(&target, "initial").expect("write probe");

    let config = WatchConfig {
        path: shared_common::taxonomy_path_vo::FilePath::new(
            path_buf.to_string_lossy().into_owned(),
        )
        .expect("valid path"),
        recursive: false,
        debounce_ms: 50,
        ignore_patterns: Vec::new(),
    };

    let provider = NotifyWatchProvider::new();
    let mut rx = provider.subscribe();

    provider
        .start(&config)
        .await
        .expect("watcher starts on an existing path");

    // Trigger a real filesystem change; the debouncer will fire.
    fs::write(&target, "changed").expect("write probe again");

    // Give the debouncer a bounded window to deliver the event.
    let deadline = Duration::from_secs(3);
    let start_instant = std::time::Instant::now();
    let mut received = 0usize;
    while received < 1 && start_instant.elapsed() < deadline {
        match rx.recv().await {
            Ok(event) => {
                received += 1;
                assert!(
                    event.path.contains("probe.txt"),
                    "event must name the probed file, got: {}",
                    event.path
                );
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    assert_eq!(
        received, 1,
        "every real debounced event must be forwarded, none may be discarded"
    );
}
