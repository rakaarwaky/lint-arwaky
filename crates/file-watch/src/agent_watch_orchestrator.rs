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

use tracing::{error, warn};

use shared::common::{ExitCode, FilePath};
use shared::file_watch::IChangeAnalyzerProtocol;
use shared::file_watch::{IWatchAggregate, IWatchProviderProtocol, WatchConfig};
use shared::quality_rules::ICodeAnalysisAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct WatchOrchestrator {
    provider: Arc<dyn IWatchProviderProtocol>,
    analyzer: Arc<dyn IChangeAnalyzerProtocol>,
    linter: Arc<dyn ICodeAnalysisAggregate>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl IWatchAggregate for WatchOrchestrator {
    fn run(&self, config: WatchConfig, running: Arc<AtomicBool>) -> ExitCode {
        println!("Lint Arwaky v{} (Watch Mode)", env!("CARGO_PKG_VERSION"));
        println!("Target: {}", config.path.value());
        println!("Press Ctrl+C to stop.");
        println!();

        // Initial full lint
        let results = self.linter.run_code_analysis_path(&config.path);
        let score = self.linter.calc_score(&results);
        println!(
            "[initial] {} violations, score {:.1}",
            results.len(),
            score.value()
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
        if let Err(e) = rt.block_on(self.provider.start(&config)) {
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
                        let event_fp = match FilePath::new(&event.path) {
                            Ok(fp) => fp,
                            Err(_) => continue,
                        };
                        let lint_results = self.linter.run_code_analysis_path(&event_fp);
                        let lint_score = self.linter.calc_score(&lint_results);
                        println!(
                            "[change] {} | {} violations, score {:.1}",
                            event.path,
                            lint_results.len(),
                            lint_score.value()
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
        println!("Watcher stopped.");
        ExitCode::OK
    }

    fn provider(&self) -> Arc<dyn IWatchProviderProtocol> {
        self.provider.clone()
    }

    fn is_lintable(&self, path: &str) -> bool {
        self.analyzer.is_lintable(path)
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
}
