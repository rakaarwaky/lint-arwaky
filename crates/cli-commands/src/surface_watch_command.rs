// PURPOSE: WatchCommandsSurface — CLI surface for file watching with auto-lint on changes
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use code_analysis::resolve_target;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use shared::file_watch::taxonomy_watch_config_vo::WatchConfig;

pub struct WatchCommandsSurface {}

impl Default for WatchCommandsSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl WatchCommandsSurface {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn handle_watch(arch_linter: Arc<dyn IArchLintProtocol>, path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    let config = WatchConfig::from_path(root);

    let container = file_watch::FileWatchContainer::new();
    let orchestrator = container.orchestrator(arch_linter);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if let Err(e) = ctrlc::set_handler(move || {
        eprintln!("\nStopping watcher...");
        r.store(false, Ordering::SeqCst);
    }) {
        eprintln!("[error] failed to set Ctrl+C handler: {}", e);
        return ExitCode::FAILURE;
    }

    let rt = tokio::runtime::Runtime::new().unwrap_or_else(|e| {
        eprintln!("Failed to create tokio runtime: {}", e);
        std::process::exit(1);
    });

    rt.block_on(orchestrator.run(config, running))
}
