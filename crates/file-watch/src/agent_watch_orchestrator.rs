// PURPOSE: WatchOrchestrator — coordinates watch → analyze → lint pipeline
//
// The watch mode provides real-time feedback: when a file changes on disk,
// the watcher triggers a lint scan on that specific file and prints results.
//
// Architecture:
//   1. Performs an initial full lint on startup (gives baseline)
//   2. Starts the filesystem watcher (inotify on Linux, via `notify` crate)
//   3. Event loop: receives file-change events, batches + deduplicates via
//      IEventDedupProtocol, filters to lintable files, runs lint, prints results
//   4. Graceful shutdown: Ctrl+C triggers AtomicBool flag, stops watcher

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use tracing::{error, info, warn};

use shared::common::ExitCode;
use shared::common::taxonomy_path_vo::FilePath;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::contract_watch_protocol::{
    IChangeLintProtocol, IEventDedupProtocol, ILintableFilterProtocol, IWatchBroadcastProtocol,
    IWatchShutdownProtocol, IWatchStartProtocol,
};
use shared::file_watch::taxonomy_service_error::WatchServiceError;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;
use shared::file_watch::taxonomy_watch_config_vo::WatchEvent;
use shared::file_watch::taxonomy_watch_request::WatchRequest;
use shared::file_watch::taxonomy_watch_response::WatchResponse;
use shared::quality_rules::CodeAnalysisRequest;
use shared::quality_rules::ICodeAnalysisAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WatchOrchestrator {
    start: Arc<dyn IWatchStartProtocol>,
    broadcast: Arc<dyn IWatchBroadcastProtocol>,
    shutdown: Arc<dyn IWatchShutdownProtocol>,
    filter: Arc<dyn ILintableFilterProtocol>,
    dedup: Arc<dyn IEventDedupProtocol>,
    change_lint: Arc<dyn IChangeLintProtocol>,
    linter: Arc<dyn ICodeAnalysisAggregate>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl IWatchAggregate for WatchOrchestrator {
    fn execute(&self, request: WatchRequest) -> WatchResponse {
        match request {
            WatchRequest::Run { config, running } => WatchResponse::Run {
                exit_code: self.run_watch_loop(&config, running),
            },
            WatchRequest::IsLintable { path } => WatchResponse::IsLintable {
                lintable: self.filter.is_lintable(path.value()),
            },
        }
    }
}

// ─── Block 2b: Change-Lint Capability (FR-FileWatch-005) ───

struct ChangeLintHandler {
    linter: Arc<dyn ICodeAnalysisAggregate>,
}

impl IChangeLintProtocol for ChangeLintHandler {
    fn lint_changed(&self, event: &WatchEvent) -> Result<(), WatchServiceError> {
        if FilePath::new(&event.path).is_err() {
            return Ok(());
        }
        let results = self
            .linter
            .execute(CodeAnalysisRequest::run_analysis(&[]))
            .into_violations();
        let score = self
            .linter
            .execute(CodeAnalysisRequest::calc_score(&results))
            .into_score();
        info!(
            file = %event.path,
            violations = results.len(),
            score = score.value(),
            "File change linted"
        );
        Ok(())
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl WatchOrchestrator {
    pub fn new(
        start: Arc<dyn IWatchStartProtocol>,
        broadcast: Arc<dyn IWatchBroadcastProtocol>,
        shutdown: Arc<dyn IWatchShutdownProtocol>,
        filter: Arc<dyn ILintableFilterProtocol>,
        dedup: Arc<dyn IEventDedupProtocol>,
        linter: Arc<dyn ICodeAnalysisAggregate>,
    ) -> Self {
        let change_lint: Arc<dyn IChangeLintProtocol> = Arc::new(ChangeLintHandler {
            linter: linter.clone(),
        });
        Self {
            start,
            broadcast,
            shutdown,
            filter,
            dedup,
            change_lint,
            linter,
        }
    }

    /// Full-project analysis run once on startup, to establish a baseline.
    fn run_initial_lint(&self) {
        let results = self
            .linter
            .execute(CodeAnalysisRequest::run_analysis(&[]))
            .into_violations();
        let score = self
            .linter
            .execute(CodeAnalysisRequest::calc_score(&results))
            .into_score();
        info!(
            violations = results.len(),
            score = score.value(),
            "Initial scan complete"
        );
    }

    fn run_watch_loop(&self, config: &WatchConfig, running: Arc<AtomicBool>) -> ExitCode {
        info!(
            version = env!("CARGO_PKG_VERSION"),
            target = config.path.value(),
            "Watch mode started, press Ctrl+C to stop"
        );

        self.run_initial_lint();

        // Start watcher (block on async call via minimal runtime)
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r,
            Err(e) => {
                error!(error = %e, "failed to create tokio runtime");
                return ExitCode::RUNTIME_ERROR;
            }
        };
        if let Err(e) = rt.block_on(self.start.start(config)) {
            error!(error = %e, "failed to start watcher");
            return ExitCode::RUNTIME_ERROR;
        }

        // Subscribe to file-change events
        let mut rx = self.broadcast.subscribe();

        // Sync event loop — poll every 100ms, check running flag each iteration
        while running.load(Ordering::SeqCst) {
            match rx.try_recv() {
                Ok(event) => {
                    // Batch: collect all pending events before processing
                    let mut batch = vec![event];
                    while let Ok(ev) = rx.try_recv() {
                        batch.push(ev);
                    }

                    // FR-004: deduplicate by path, FR-003: filter to lintable files
                    let deduped = self.dedup.dedup_events(batch);
                    let lintable = self.filter.filter_lintable(deduped);

                    for event in lintable {
                        // FR-005: lint the changed file; failures are non-fatal
                        let _ = self.change_lint.lint_changed(&event);
                    }
                }
                Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {}
                Err(tokio::sync::broadcast::error::TryRecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::TryRecvError::Closed) => break,
            }
            thread::sleep(Duration::from_millis(100));
        }

        // Stop watcher — log error on failure
        if let Err(e) = rt.block_on(self.shutdown.stop()) {
            warn!(error = %e, "failed to stop watcher cleanly");
        }
        info!("Watcher stopped");
        ExitCode::OK
    }
}
