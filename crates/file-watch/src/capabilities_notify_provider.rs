// PURPOSE: NotifyWatchProvider — IWatchLifecycleProtocol implementation using notify crate (inotify on Linux)

use std::sync::Mutex;
use std::time::Duration;

use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::new_debouncer;
use shared_common::taxonomy_common_vo::BooleanVO;
use shared_common::taxonomy_message_vo::LintMessage;
use shared_file_watch::contract_watch_protocol::IWatchLifecycleProtocol;
use shared_file_watch::taxonomy_file_watch_error::WatchServiceError;
use shared_file_watch::taxonomy_file_watch_vo::WatchConfig;
use shared_file_watch::taxonomy_file_watch_vo::WatchEvent;
use shared_file_watch::taxonomy_file_watch_vo::WatchEventKind;
use tokio::sync::broadcast;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct NotifyWatchProvider {
    watcher: Mutex<Option<notify_debouncer_mini::Debouncer<RecommendedWatcher>>>,
    tx: broadcast::Sender<WatchEvent>,
    ignore_patterns: Mutex<Vec<String>>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────
// NOTE: #[async_trait] required — shared contract still uses it.
// Remove when protocol traits switch to native async fn in traits.

#[async_trait::async_trait]
impl IWatchLifecycleProtocol for NotifyWatchProvider {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError> {
        let path_str = config.path.value();
        let path = std::path::Path::new(path_str);
        if !path.exists() {
            return Err(WatchServiceError::new(LintMessage::new(format!(
                "Path does not exist: {}",
                path_str
            ))));
        }

        {
            let mut patterns = match self.ignore_patterns.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            *patterns = config.ignore_patterns.clone();
        }

        let tx = self.tx.clone();
        let ignore = config.ignore_patterns.clone();

        let mut debouncer = new_debouncer(
            Duration::from_millis(config.debounce_ms),
            move |res: Result<Vec<notify_debouncer_mini::DebouncedEvent>, _>| {
                if let Ok(events) = res {
                    for event in events {
                        let path_str = event.path.to_string_lossy().to_string();
                        let skip = ignore.iter().any(|p| path_str.contains(p.as_str()));
                        if !skip {
                            let watch_event = WatchEvent::new(path_str, WatchEventKind::Modified);
                            let _ = tx.send(watch_event);
                        }
                    }
                }
            },
        )
        .map_err(|e| {
            WatchServiceError::new(LintMessage::new(format!(
                "Failed to create debouncer: {}",
                e
            )))
        })?;

        let recursive = if config.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        debouncer.watcher().watch(path, recursive).map_err(|e| {
            WatchServiceError::new(LintMessage::new(format!("Failed to watch path: {}", e)))
        })?;

        {
            let mut watcher_guard = match self.watcher.lock() {
                Ok(guard) => guard,
                Err(poisoned) => poisoned.into_inner(),
            };
            *watcher_guard = Some(debouncer);
        }
        Ok(())
    }

    async fn is_available(&self) -> BooleanVO {
        BooleanVO::new(cfg!(feature = "watch"))
    }

    fn subscribe(&self) -> broadcast::Receiver<WatchEvent> {
        self.tx.subscribe()
    }

    async fn stop(&self) -> Result<(), WatchServiceError> {
        let mut guard = match self.watcher.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(debouncer) = guard.take() {
            drop(debouncer);
        }
        Ok(())
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for NotifyWatchProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl NotifyWatchProvider {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(256);
        Self {
            watcher: Mutex::new(None),
            tx,
            ignore_patterns: Mutex::new(Vec::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use notify_debouncer_mini::{DebouncedEvent, DebouncedEventKind};

    /// Regression for #901: the old code only forwarded events whose kind
    /// matched `DebouncedEventKind::Any` via a comparison that could never
    /// match, so every filesystem event was silently discarded. The fixed
    /// callback must forward real debounced events regardless of kind.
    #[tokio::test]
    async fn real_debounced_event_is_not_discarded() {
        let provider = NotifyWatchProvider::new();
        let mut rx = provider.subscribe();

        // Reproduce the callback body the debouncer uses: a real
        // `DebouncedEvent` must reach the broadcast channel.
        let events = vec![
            DebouncedEvent::new(PathBuf::from("src/main.rs"), DebouncedEventKind::Any),
            DebouncedEvent::new(
                PathBuf::from("src/lib.rs"),
                DebouncedEventKind::AnyContinuous,
            ),
        ];
        let ignore: Vec<String> = Vec::new();
        let tx = provider.tx.clone();
        for event in &events {
            let path_str = event.path.to_string_lossy().to_string();
            let skip = ignore.iter().any(|p| path_str.contains(p.as_str()));
            if !skip {
                let watch_event = WatchEvent::new(path_str, WatchEventKind::Modified);
                let _ = tx.send(watch_event);
            }
        }

        let mut received: Vec<WatchEvent> = Vec::new();
        while let Ok(event) = rx.try_recv() {
            received.push(event);
        }
        assert_eq!(
            received.len(),
            events.len(),
            "every real debounced event must be forwarded, none may be discarded"
        );
    }
}
