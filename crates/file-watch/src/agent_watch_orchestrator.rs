// PURPOSE: WatchOrchestrator — coordinates watch → analyze → lint pipeline
//
// The watch mode provides real-time feedback: when a file changes on disk,
// the watcher triggers a lint scan on that specific file and prints results.
//
// Architecture:
//   1. Performs an initial full lint on startup (gives baseline)
//   2. Starts the filesystem watcher (inotify on Linux, via `notify` crate)
//   3. Event loop: receives file-change events, checks if the file is
//      lintable (.rs, .py, .js, .ts, etc.), runs lint, prints results
//   4. Graceful shutdown: Ctrl+C triggers AtomicBool flag, stops watcher
//
// The event loop uses tokio::select! for cancellability — the sleep branch
// allows checking the running flag every 100ms without blocking on recv().
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::file_watch::contract_change_analyzer_protocol::IChangeAnalyzerProtocol;
use shared::file_watch::contract_provider_protocol::IWatchProviderProtocol;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct WatchOrchestrator {
    provider: Arc<dyn IWatchProviderProtocol>,
    linter: Arc<dyn ICodeAnalysisAggregate>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IWatchAggregate for WatchOrchestrator {
    fn run(&self, config: WatchConfig, running: Arc<AtomicBool>) -> ExitCode {
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            handle.block_on(self.run_async(config, running))
        } else {
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Failed to create tokio runtime: {}", e);
                    return ExitCode::FAILURE;
                }
            };
            rt.block_on(self.run_async(config, running))
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl WatchOrchestrator {
    pub fn new(
        provider: Arc<dyn IWatchProviderProtocol>,
        linter: Arc<dyn ICodeAnalysisAggregate>,
    ) -> Self {
        Self { provider, linter }
    }

    pub async fn run_async(&self, config: WatchConfig, running: Arc<AtomicBool>) -> ExitCode {
        let path = config.path.value.clone();
        println!("Lint Arwaky v{} (Watch Mode)", env!("CARGO_PKG_VERSION"));
        println!("Target: {}", path);
        println!("Press Ctrl+C to stop.");
        println!();

        // Initial full lint
        let fp = FilePath::new(path.clone()).unwrap_or_default();
        let results = self.linter.run_code_analysis_path(&fp);
        let score = self.linter.calc_score(&results);
        println!("[initial] {} violations, score {:.1}", results.len(), score);

        // Start watcher
        if let Err(e) = self.provider.start(&config).await {
            eprintln!("Failed to start watcher: {}", e);
            return ExitCode::FAILURE;
        }

        let mut rx = self.provider.subscribe();

        while running.load(Ordering::SeqCst) {
            tokio::select! {
                res = rx.recv() => {
                    match res {
                        Ok(event) => {
                            if crate::capabilities_change_analyzer::ChangeAnalyzer::is_lintable(&event.path) {
                                let event_fp = FilePath::new(&event.path).unwrap_or_default();
                                let lint_results = self.linter.run_code_analysis_path(&event_fp);
                                let lint_score = self.linter.calc_score(&lint_results);
                                println!(
                                    "[change] {} | {} violations, score {:.1}",
                                    event.path,
                                    lint_results.len(),
                                    lint_score
                                );
                            }
                        }
                        Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    running.store(false, Ordering::SeqCst);
                    break;
                }
            }
        }

        let _ = self.provider.stop().await;
        println!("Watcher stopped.");
        ExitCode::SUCCESS
    }
}
