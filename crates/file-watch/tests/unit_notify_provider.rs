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

/// The `Events` arm must honour `ignore_patterns`: a path matching an ignore
/// pattern is suppressed while a non-matching path in the same watch is still
/// forwarded. This covers the skip branch inside `DebouncedEventBatch::Events`
/// that the single-event regression test never exercises.
#[tokio::test]
async fn ignored_pattern_is_not_forwarded_but_other_paths_are() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let root = tmp.path().to_path_buf();
    let kept = root.join("keep.txt");
    let ignored = root.join("skip.log");
    fs::write(&kept, "a").expect("write kept");
    fs::write(&ignored, "a").expect("write ignored");

    let config = WatchConfig {
        path: shared_common::taxonomy_path_vo::FilePath::new(root.to_string_lossy().into_owned())
            .expect("valid path"),
        recursive: false,
        debounce_ms: 50,
        ignore_patterns: vec!["skip.log".to_string()],
    };

    let provider = NotifyWatchProvider::new();
    let mut rx = provider.subscribe();
    provider
        .start(&config)
        .await
        .expect("watcher starts on an existing path");

    fs::write(&ignored, "b").expect("write ignored again");
    fs::write(&kept, "b").expect("write kept again");

    let deadline = Duration::from_secs(3);
    let start = std::time::Instant::now();
    let mut saw_kept = false;
    let mut saw_ignored = false;
    while start.elapsed() < deadline && !saw_kept {
        // Bound the recv so a quiet channel (the ignored event is correctly
        // suppressed, so no further events arrive) cannot hang the test.
        match tokio::time::timeout(Duration::from_millis(500), rx.recv()).await {
            Ok(Ok(event)) => {
                saw_kept |= event.path.contains("keep.txt");
                saw_ignored |= event.path.contains("skip.log");
            }
            Ok(Err(tokio::sync::broadcast::error::RecvError::Lagged(_))) => continue,
            Ok(Err(tokio::sync::broadcast::error::RecvError::Closed)) => break,
            Err(_elapsed) => break,
        }
    }
    assert!(saw_kept, "non-ignored path must be forwarded");
    assert!(!saw_ignored, "ignored path must not be forwarded");
}
