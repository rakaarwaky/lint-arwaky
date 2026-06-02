/// Main entry surface for the Auto-Linter CLI.
use std::sync::Mutex;

use crate::taxonomy::*;
use crate::contract::*;
use crate::surfaces::*;

pub struct MainHandlerSurface {
    pub container: Option<ServiceContainerAggregate>,
}

impl MainHandlerSurface {
    pub fn new(container: Option<ServiceContainerAggregate>) -> Self {
        let mut surface = Self { container };
        if surface.container.is_some() {
            surface.register_extensions();
        }
        surface
    }

    pub fn register_extensions(&mut self) {
        if let Some(ref container) = self.container {
            set_container(container.clone());

            let mut core_surface = get_surface();
            core_surface.register_all(container.clone());

            register_analysis_commands(container.clone());
            register_maintenance_commands(container.clone());
            register_watch_command(container.clone());
        }
    }

    pub fn execute(&self) {
        // In Python: sets up logging, calls cli()
        // In Rust: CLI entry is handled by clap in main.rs
        // This method is the structural equivalent
        println!("Auto-Linter CLI initialized");
    }
}
