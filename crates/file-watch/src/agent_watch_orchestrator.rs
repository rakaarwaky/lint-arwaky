// PURPOSE: WatchOrchestrator — coordinates watch → analyze → lint pipeline
//
// The watch mode provides real-time feedback: when a file changes on disk,
// the watcher triggers a lint scan on that specific file and prints results.
//
// Architecture:
//   1. Performs an initial full lint on startup (gives baseline)
//   2. Starts the filesystem watcher (inotify on Linux, via `notify` crate)
//   3. Event loop: receives file-change events, batches + deduplicates via
//      IChangeAnalyzerProtocol, filters to lintable files, runs lint, prints results
//   4. Graceful shutdown: Ctrl+C triggers AtomicBool flag, stops watcher

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use tracing::{error, info, warn};

use shared::common::ExitCode;
use shared::common::taxonomy_path_vo::FilePath;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;
use shared::file_watch::taxonomy_watch_request::WatchRequest;
use shared::file_watch::taxonomy_watch_response::WatchResponse;
use shared::file_watch::{IChangeAnalyzerProtocol, IWatchProviderProtocol};
use shared::quality_rules::CodeAnalysisRequest;
use shared::quality_rules::ICodeAnalysisAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WatchOrchestrator {
    provider: Arc<dyn IWatchProviderProtocol>,
    analyzer: Arc<dyn IChangeAnalyzerProtocol>,
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
                lintable: self.analyzer.is_lintable(path.value()),
            },
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl WatchOrchestrator {
    pub fn new(
        provider: Arc<dyn IWatchProviderProtocol>,
        analyzer: Arc<dyn IChangeAnalyzerProtocol>,
        linter: Arc<dyn ICodeAnalysisAggregate>,
    ) -> Self {
        Self {
            provider,
            analyzer,
            linter,
        }
    }

    fn run_watch_loop(&self, config: &WatchConfig, running: Arc<AtomicBool>) -> ExitCode {
        info!(
            version = env!("CARGO_PKG_VERSION"),
            target = config.path.value(),
            "Watch mode started, press Ctrl+C to stop"
        );

        // Initial full lint
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
        if let Err(e) = rt.block_on(self.provider.start(config)) {
            error!(error = %e, "failed to start watcher");
            return ExitCode::RUNTIME_ERROR;
        }

        // Subscribe to file-change events
        let mut rx = self.provider.subscribe();

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
                    let deduped = self.analyzer.analyze(batch);
                    let lintable = self.analyzer.filter_lintable(deduped);

                    for event in lintable {
                        let _event_fp = match FilePath::new(&event.path) {
                            Ok(fp) => fp,
                            Err(_) => continue,
                        };
                        let lint_results = self
                            .linter
                            .execute(CodeAnalysisRequest::run_analysis(&[]))
                            .into_violations();
                        let lint_score = self
                            .linter
                            .execute(CodeAnalysisRequest::calc_score(&lint_results))
                            .into_score();
                        info!(
                            file = %event.path,
                            violations = lint_results.len(),
                            score = lint_score.value(),
                            "File change linted"
                        );
                    }
                }
                Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {}
                Err(tokio::sync::broadcast::error::TryRecvError::Lagged(_)) => continue,
                Err(tokio::sync::broadcast::error::TryRecvError::Closed) => break,
            }
            thread::sleep(Duration::from_millis(100));
        }

        // Stop watcher — log error on failure
        if let Err(e) = rt.block_on(self.provider.stop()) {
            warn!(error = %e, "failed to stop watcher cleanly");
        }
        info!("Watcher stopped");
        ExitCode::OK
    }
}
