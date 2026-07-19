// PURPOSE: NotifyWatchProvider — IWatchProviderPort implementation using notify crate (inotify on Linux)
use std::sync::Mutex;
use std::time::Duration;

use notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use shared::common::taxonomy_common_vo::BooleanVO;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::file_watch::contract_provider_port::IWatchProviderPort;
use shared::file_watch::taxonomy_service_error::WatchServiceError;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;
use shared::file_watch::taxonomy_watch_event_vo::{WatchEvent, WatchEventKind};
use tokio::sync::broadcast;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct NotifyWatchProvider {
    watcher: Mutex<Option<notify_debouncer_mini::Debouncer<RecommendedWatcher>>>,
    tx: broadcast::Sender<WatchEvent>,
    ignore_patterns: Mutex<Vec<String>>,
}

#[async_trait::async_trait]
// ─── Block 2: Public Contract ─────────────────────────────
impl IWatchProviderPort for NotifyWatchProvider {
    async fn start(&self, config: &WatchConfig) -> Result<(), WatchServiceError> {
        let path_str = config.path.value.clone();
        let path = std::path::Path::new(&path_str);
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
                        if event.kind == DebouncedEventKind::Any {
                            let path_str = event.path.to_string_lossy().to_string();
                            let mut skip = false;
                            for pattern in &ignore {
                                if path_str.contains(pattern.as_str()) {
                                    skip = true;
                                    break;
                                }
                            }
                            if !skip {
                                let watch_event =
                                    WatchEvent::new(path_str, WatchEventKind::Modified);
                                let _ = tx.send(watch_event);
                            }
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

    async fn is_available(&self) -> BooleanVO {
        BooleanVO::new(cfg!(feature = "watch"))
    }

    fn subscribe(&self) -> broadcast::Receiver<WatchEvent> {
        self.tx.subscribe()
    }
}

// ─── Block 3: Constructors & Helpers ──────────────────────
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

impl Default for NotifyWatchProvider {
    fn default() -> Self {
        Self::new()
    }
}
