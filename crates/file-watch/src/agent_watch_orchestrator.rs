// PURPOSE: WatchOrchestrator — coordinates watch → filter → lint pipeline

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use tracing::{error, info, warn};

use shared_common::ExitCode;
use shared_file_watch::contract_watch_aggregate::IWatchAggregate;
use shared_file_watch::contract_watch_protocol::{
    IChangeFilterProtocol, IChangeLintProtocol, IWatchLifecycleProtocol,
};
use shared_file_watch::taxonomy_file_watch_request::WatchRequest;
use shared_file_watch::taxonomy_file_watch_response::WatchResponse;
use shared_file_watch::taxonomy_file_watch_vo::WatchConfig;
use shared_quality_rules::CodeAnalysisRequest;
use shared_quality_rules::ICodeAnalysisAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WatchOrchestrator {
    lifecycle: Arc<dyn IWatchLifecycleProtocol>,
    filter: Arc<dyn IChangeFilterProtocol>,
    lint: Arc<dyn IChangeLintProtocol>,
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl WatchOrchestrator {
    pub fn new(
        lifecycle: Arc<dyn IWatchLifecycleProtocol>,
        filter: Arc<dyn IChangeFilterProtocol>,
        lint: Arc<dyn IChangeLintProtocol>,
        linter: Arc<dyn ICodeAnalysisAggregate>,
    ) -> Self {
        Self {
            lifecycle,
            filter,
            lint,
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
        if let Err(e) = rt.block_on(self.lifecycle.start(config)) {
            error!(error = %e, "failed to start watcher");
            return ExitCode::RUNTIME_ERROR;
        }

        // Subscribe to file-change events
        let mut rx = self.lifecycle.subscribe();

        // Sync event loop — poll every 100ms, check running flag each iteration
        while running.load(Ordering::SeqCst) {
            match rx.try_recv() {
                Ok(event) => {
                    // Batch: collect all pending events before processing
                    let mut batch = vec![event];
                    while let Ok(ev) = rx.try_recv() {
                        batch.push(ev);
                    }

                    // FR-002: deduplicate and filter to lintable files
                    let filtered = self.filter.filter_events(batch);

                    for event in filtered {
                        // FR-003: lint the changed file; failures are non-fatal
                        let _ = self.lint.lint_changed(&event);
                    }
                }
                Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {}
                Err(tokio::sync::broadcast::error::TryRecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::TryRecvError::Closed) => break,
            }
            thread::sleep(Duration::from_millis(100));
        }

        // Stop watcher — log error on failure
        if let Err(e) = rt.block_on(self.lifecycle.stop()) {
            warn!(error = %e, "failed to stop watcher cleanly");
        }
        info!("Watcher stopped");
        ExitCode::OK
    }
}
