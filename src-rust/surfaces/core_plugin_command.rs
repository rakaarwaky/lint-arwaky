/// Plugin and adapter-related CLI commands for auto-linter.
use crate::taxonomy::*;
use crate::contract::*;

pub struct PluginCommandsSurface {
    pub container: Option<ServiceContainerAggregate>,
}

impl PluginCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: ServiceContainerAggregate) {
        self.container = Some(container);
    }

    pub fn adapters(&self) {
        println!("Enabled Adapters:");
        // In real impl: iterate self.container.adapters
        println!(" - ruff");
        println!(" - mypy");
        println!(" - bandit");
        println!(" - radon");
        println!(" - architecture");
    }

    pub fn plugins(&self) {
        println!("Discovered Plugins:");
        println!("No plugins or custom adapters found.");
        println!("\nTo register a plugin, add entry point in Cargo.toml:");
        println!("  [lib.adapter]");
        println!("  my_adapter = my_package::MyAdapterClass");
    }
}

pub fn register_plugin_commands(container: ServiceContainerAggregate) -> PluginCommandsSurface {
    let mut surface = PluginCommandsSurface::new();
    surface.register_all(container);
    surface
}
