// PURPOSE: WatchOrchestrator — coordinates watch → analyze → lint pipeline
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use shared::code_analysis::contract_lint_aggregate::IArchLintAggregate;
use shared::file_watch::contract_provider_port::IWatchProviderPort;
use shared::file_watch::contract_watch_aggregate::IWatchAggregate;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

pub struct WatchOrchestrator {
    provider: Arc<dyn IWatchProviderPort>,
    linter: Arc<dyn IArchLintAggregate>,
}

impl WatchOrchestrator {
    pub fn new(provider: Arc<dyn IWatchProviderPort>, linter: Arc<dyn IArchLintAggregate>) -> Self {
        Self { provider, linter }
    }

    pub async fn run_async(&self, config: WatchConfig, running: Arc<AtomicBool>) -> ExitCode {
        let path = config.path.value.clone();
        println!("Lint Arwaky v{} (Watch Mode)", env!("CARGO_PKG_VERSION"));
        println!("Target: {}", path);
        println!("Press Ctrl+C to stop.");
        println!();

        // Initial full lint
        let results = self.linter.run_lint(&path);
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
                Ok(event) = rx.recv() => {
                    if crate::capabilities_change_analyzer::ChangeAnalyzer::is_lintable(&event.path) {
                        let lint_results = self.linter.run_lint(&event.path);
                        let lint_score = self.linter.calc_score(&lint_results);
                        println!(
                            "[change] {} | {} violations, score {:.1}",
                            event.path,
                            lint_results.len(),
                            lint_score
                        );
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {}
            }
        }

        let _ = self.provider.stop().await;
        println!("Watcher stopped.");
        ExitCode::SUCCESS
    }
}

impl IWatchAggregate for WatchOrchestrator {
    fn run(&self, config: WatchConfig, running: Arc<AtomicBool>) -> ExitCode {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to create tokio runtime: {}", e);
                std::process::exit(1);
            }
        };
        rt.block_on(self.run_async(config, running))
    }
}
