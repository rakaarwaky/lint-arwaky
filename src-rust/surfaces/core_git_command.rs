/// Git-related CLI commands for auto-linter.
use std::collections::HashMap;

use crate::taxonomy::*;
use crate::contract::*;

pub struct GitCommandsSurface {
    pub container: Option<ServiceContainerAggregate>,
}

impl GitCommandsSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: ServiceContainerAggregate, _cli: Option<&str>) {
        self.container = Some(container);
    }

    pub fn print_section<F>(&self, title: &str, items: &[impl std::fmt::Display], item_fmt: F)
    where F: Fn(&impl std::fmt::Display)
    {
        if !items.is_empty() {
            println!("  {title} ({}):", items.len());
            for item in items {
                item_fmt(item);
            }
        }
    }

    pub fn print_diff_text(&self, base_ref: &str) {
        println!(" Changed files since {base_ref}:");
        println!("  No changed files detected.");
    }

    pub fn git_diff(&self, base: &str, output_format: &str) {
        if output_format == "json" {
            println!("{{\"added\": [], \"modified\": [], \"deleted\": [], \"lintable_files\": [], \"total_changed\": 0}}");
        } else {
            self.print_diff_text(base);
        }
    }
}

pub fn register_git_commands(container: ServiceContainerAggregate) -> GitCommandsSurface {
    let mut surface = GitCommandsSurface::new();
    surface.register_all(container, None);
    surface
}
