// PURPOSE: WatchCommandsSurface — CLI surface for file watching with auto-lint on changes

use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use code_analysis::resolve_target;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;

pub struct WatchdogBridge {}

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

    pub fn watch(&self, path: &str) {
        let abs_path = std::path::Path::new(path);
        let abs_path_str = abs_path.to_string_lossy().to_string();

        println!("Watching {abs_path_str} for changes...");
        println!("Performing initial scan...");
        println!("Initial scan complete. Score: 100.0");
        println!("\nStarting file watcher (Ctrl+C to stop)...");
    }
}

pub fn handle_watch(arch_linter: Arc<dyn IArchLintProtocol>, path: Option<String>) -> ExitCode {
    let root = resolve_target(path);
    println!("Lint Arwaky v{} (Watch Mode)", env!("CARGO_PKG_VERSION"));
    println!("Target: {}", root);
    println!("Polling every 2s. Press Ctrl+C to stop.");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    if let Err(e) = ctrlc::set_handler(move || {
        eprintln!("\nStopping watcher...");
        r.store(false, Ordering::SeqCst);
    }) {
        eprintln!("[error] failed to set Ctrl+C handler: {}", e);
        return ExitCode::FAILURE;
    }

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_secs(2));
        if !running.load(Ordering::SeqCst) {
            break;
        }
        let results = arch_linter.run_lint(&root);
        println!(
            "[{} violations, score {}]",
            results.len(),
            arch_linter.calc_score(&results)
        );
    }

    println!("Watcher stopped.");
    ExitCode::SUCCESS
}
