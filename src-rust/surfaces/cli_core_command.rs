/// Core CLI commands: cli, check, scan, fix, report, version, adapters, security.
use std::sync::Mutex;

use crate::contract::*;

use super::cli_check_command::register_check_commands;
use super::cli_fix_command::register_fix_commands;
use super::cli_dev_command::register_dev_commands;
use super::cli_setup_command::register_setup_commands;
use super::core_git_command::register_git_commands;
use super::core_multi_command::register_multi_commands;
use super::core_plugin_command::register_plugin_commands;
use super::core_report_command::register_report_commands;

pub struct CoreCommandsSurface {
    pub container: Option<ServiceContainerAggregate>,
}

impl CoreCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: ServiceContainerAggregate) {
        self.container = Some(container.clone());

        // Register all subcommands (in Rust, handled via clap in main.rs)
        // The Python version registers commands; here we expose the factory calls.
        let _ = register_check_commands(container.clone());
        let _ = register_report_commands(container.clone());
        let _ = register_fix_commands(container.clone());
        let _ = register_git_commands(container.clone());
        let _ = register_plugin_commands(container.clone());
        let _ = register_multi_commands(container.clone());
        let _ = register_setup_commands(container.clone());
        let _ = register_dev_commands(container.clone());
    }

    pub fn version() {
        let ver = env!("CARGO_PKG_VERSION");
        println!("Auto-Linter v{ver} (AES Semantic Builder)");
    }
}

// Lazy singleton
static INSTANCE: Mutex<Option<CoreCommandsSurface>> = Mutex::new(None);

fn get_instance() -> std::sync::MutexGuard<'static, Option<CoreCommandsSurface>> {
    let mut guard = INSTANCE.lock().unwrap();
    if guard.is_none() {
        *guard = Some(CoreCommandsSurface::new());
    }
    guard
}

pub fn get_cli() -> bool {
    // In Rust, cli is built via clap. Return true to indicate cli is available.
    true
}

pub fn get_surface() -> CoreCommandsSurface {
    get_instance().clone().unwrap()
}
