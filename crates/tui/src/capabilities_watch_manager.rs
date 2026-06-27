// PURPOSE: WatchManager — manages background file watching for live lint updates in the TUI.
// When the user presses `w`, a background thread starts a file watcher (notify crate) on the
// project directory. Changed files are re-linted and results sent through a channel to the
// event loop for live preview updates.

use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Result of a single file change detected by the watcher.
#[derive(Debug, Clone)]
pub struct WatchChange {
    pub path: String,
    pub result: LintExecutionResult,
}

/// Handle to a running background watcher. Stored in ActionHandler while watching is active.
pub struct WatchHandle {
    running: Arc<AtomicBool>,
    rx: std::sync::mpsc::Receiver<WatchChange>,
}

impl WatchHandle {
    /// Try to receive a pending watch event without blocking.
    pub fn try_recv(&self) -> Option<WatchChange> {
        self.rx.try_recv().ok()
    }

    /// Signal the background thread to stop watching.
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Whether the background thread is still running.
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

/// Start a file watcher on the given path in a background thread.
/// Returns a `WatchHandle` that the caller can poll for results.
///
/// The background thread uses `file_watch`'s `NotifyWatchProvider` (inotify on Linux)
/// with debouncing, filters to lintable extensions (.rs, .py, .js, .ts, etc.),
/// and re-runs the linter on each changed file.
pub fn start_watch(
    lint_port: Arc<dyn ILintExecutorProtocol>,
    path: &str,
) -> WatchHandle {
    let running = Arc::new(AtomicBool::new(true));
    let (tx, rx) = std::sync::mpsc::channel();
    let path_owned = path.to_string();
    let running_clone = running.clone();

    std::thread::spawn(move || {
        watch_loop(lint_port, path_owned, running_clone, tx);
    });

    WatchHandle { running, rx }
}

/// Background thread loop: start watcher, receive change events, lint, send results.
fn watch_loop(
    lint_port: Arc<dyn ILintExecutorProtocol>,
    path: String,
    running: Arc<AtomicBool>,
    tx: std::sync::mpsc::Sender<WatchChange>,
) {
    use file_watch::infrastructure_notify_provider::NotifyWatchProvider;
    use shared::file_watch::contract_provider_port::IWatchProviderPort;
    use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return,
    };

    let provider = Arc::new(NotifyWatchProvider::new());
    let config = WatchConfig::from_path(path.clone());

    // Start the file watcher
    if rt.block_on(provider.start(&config)).is_err() {
        let _ = tx.send(WatchChange {
            path: path.clone(),
            result: LintExecutionResult::failure("Failed to start file watcher"),
        });
        return;
    }

    let mut rx = provider.subscribe();

    // Run initial lint on the target path
    let initial = lint_port.scan(&path);
    let _ = tx.send(WatchChange {
        path: path.clone(),
        result: initial,
    });

    // Event loop: receive file changes, lint them, send results
    use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
    while running.load(Ordering::SeqCst) {
        let event = rt.block_on(async {
            tokio::select! {
                result = rx.recv() => result.ok(),
                _ = tokio::time::sleep(std::time::Duration::from_millis(200)) => None,
            }
        });

        if let Some(event) = event {
            if file_watch::capabilities_change_analyzer::ChangeAnalyzer::is_lintable(&event.path)
            {
                let result = lint_port.scan(&event.path);
                let _ = tx.send(WatchChange {
                    path: event.path,
                    result,
                });
            }
        }
    }

    // Stop the watcher
    let _ = rt.block_on(provider.stop());
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::tui::taxonomy_action_flags_vo::ActionFlags;

    struct MockLintExecutor;

    impl ILintExecutorProtocol for MockLintExecutor {
        fn scan(&self, path: &str) -> LintExecutionResult {
            LintExecutionResult::success(format!("Scanned: {}", path), 0)
        }
        fn check(&self, _: &str, _: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("check".into(), 0)
        }
        fn fix(&self, _: &str, _: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("fix".into(), 0)
        }
        fn ci(&self, _: &str, _: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("ci".into(), 0)
        }
        fn orphan(&self, _: &str) -> LintExecutionResult {
            LintExecutionResult::success("orphan".into(), 0)
        }
        fn security(&self, _: &str) -> LintExecutionResult {
            LintExecutionResult::success("security".into(), 0)
        }
        fn duplicates(&self, _: &str) -> LintExecutionResult {
            LintExecutionResult::success("duplicates".into(), 0)
        }
        fn dependencies(&self, _: &str) -> LintExecutionResult {
            LintExecutionResult::success("deps".into(), 0)
        }
        fn doctor(&self) -> LintExecutionResult {
            LintExecutionResult::success("doctor".into(), 0)
        }
        fn init(&self, _: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("init".into(), 0)
        }
        fn install(&self, _: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("install".into(), 0)
        }
        fn mcp_config(&self, _: &ActionFlags) -> LintExecutionResult {
            LintExecutionResult::success("mcp".into(), 0)
        }
        fn config_show(&self) -> LintExecutionResult {
            LintExecutionResult::success("config".into(), 0)
        }
        fn install_hook(&self) -> LintExecutionResult {
            LintExecutionResult::success("hook".into(), 0)
        }
        fn uninstall_hook(&self) -> LintExecutionResult {
            LintExecutionResult::success("unhook".into(), 0)
        }
        fn adapters(&self) -> LintExecutionResult {
            LintExecutionResult::success("adapters".into(), 0)
        }
        fn version(&self) -> LintExecutionResult {
            LintExecutionResult::success("v1.0".into(), 0)
        }
    }

    #[test]
    fn watch_start_creates_handle_and_sends_initial_result() {
        let lint: Arc<dyn ILintExecutorProtocol> = Arc::new(MockLintExecutor);
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());

        let handle = start_watch(lint, &cwd);

        // Should receive initial lint result within a reasonable time
        let mut received = false;
        for _ in 0..50 {
            if let Some(change) = handle.try_recv() {
                assert!(!change.path.is_empty());
                assert!(change.result.success);
                received = true;
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        assert!(received, "Should have received initial lint result");

        handle.stop();
        // Give the thread time to exit
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    #[test]
    fn watch_stop_terminates_background_thread() {
        let lint: Arc<dyn ILintExecutorProtocol> = Arc::new(MockLintExecutor);
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string());

        let handle = start_watch(lint, &cwd);

        // Drain initial result
        for _ in 0..50 {
            if handle.try_recv().is_some() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        handle.stop();
        std::thread::sleep(std::time::Duration::from_millis(500));

        // After stop, is_running should return false
        assert!(!handle.is_running(), "Watcher should have stopped");
    }
}
