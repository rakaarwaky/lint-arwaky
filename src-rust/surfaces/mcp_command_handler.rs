/// MCP Tools: list_commands and read_skill_context.
use std::collections::HashMap;
use std::path::Path;

use crate::taxonomy::*;
use crate::contract::*;

/// COMMAND_CATALOG — mirrors the Python dict exactly.
pub struct CommandEntry {
    pub description: &'static str,
    pub example: &'static str,
}

pub static COMMAND_CATALOG: &[(&str, &str, &str)] = &[
    ("check", "Run full architecture compliance analysis", "auto-lint check /path"),
    ("scan", "Deep directory scan (alias for check)", "auto-lint scan ./src/"),
    ("fix", "Apply safe fixes", "auto-lint fix file.py"),
    ("report", "Generate quality reports", "auto-lint report ./src --format json"),
    ("ci", "CI-optimized with exit codes", "auto-lint ci /path --exit-zero"),
    ("batch", "Check multiple paths", "auto-lint batch path1.py path2.js"),
    ("watch", "Watch files for changes", "auto-lint watch ./src/"),
    ("security", "Bandit vulnerability scanning", "auto-lint security /path"),
    ("complexity", "Cyclomatic complexity", "auto-lint complexity ./src/"),
    ("duplicates", "Code duplication detection", "auto-lint duplicates /path"),
    ("trends", "Quality trend over time", "auto-lint trends ."),
    ("dependencies", "Dependency vulnerability scan", "auto-lint dependencies ."),
    ("diff", "Compare two versions", "auto-lint diff v1.py v2.py"),
    ("suggest", "AI-powered suggestions", "auto-lint suggest file.py"),
    ("stats", "Statistics dashboard", "auto-lint stats ./src/"),
    ("init", "Initialize config", "auto-lint init /path"),
    ("config", "Edit configuration", "auto-lint config get thresholds"),
    ("ignore", "Manage ignore rules", "auto-lint ignore add E501"),
    ("import", "Import configurations", "auto-lint import config.json"),
    ("export", "Export reports", "auto-lint export --format sarif"),
    ("clean", "Cleanup cache", "auto-lint clean"),
    ("update", "Update adapters", "auto-lint update"),
    ("doctor", "Diagnose issues", "auto-lint doctor"),
    ("adapters", "List enabled adapters", "auto-lint adapters"),
    ("install-hook", "Install git pre-commit hook", "auto-lint install-hook"),
    ("uninstall-hook", "Remove git pre-commit hook", "auto-lint uninstall-hook"),
    ("cancel", "Cancel a running lint job", "auto-lint cancel <job_id>"),
    ("plugins", "List discovered and registered plugins", "auto-lint plugins"),
    ("multi-project", "Run lint across multiple projects", "auto-lint multi-project proj1/ proj2/"),
    ("version", "Show version", "auto-lint version"),
];

pub fn list_commands_func(domain: Option<&str>) -> HashMap<String, HashMap<String, String>> {
    let mut result = HashMap::new();

    for &(name, desc, example) in COMMAND_CATALOG {
        if let Some(d) = domain {
            if !d.is_empty() && !name.contains(d) {
                continue;
            }
        }
        let mut info = HashMap::new();
        info.insert("description".to_string(), desc.to_string());
        info.insert("example_usage".to_string(), example.to_string());
        result.insert(name.to_string(), info);
    }

    result
}

pub struct McpCommandCatalogSurface {
    pub container: Option<ServiceContainerAggregate>,
}

impl McpCommandCatalogSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: ServiceContainerAggregate) {
        self.container = Some(container);
    }

    pub fn list_commands(&self, domain: Option<&str>) -> HashMap<String, HashMap<String, String>> {
        list_commands_func(domain)
    }

    pub fn read_skill_context(&self, section: Option<&str>) -> String {
        // In real impl: read SKILL.md from project root
        let skill_path = Path::new("SKILL.md");
        if !skill_path.exists() {
            return format!("{{\"error\": \"SKILL.md not found\", \"path\": \"{}\"}}", skill_path.display());
        }

        match std::fs::read_to_string(skill_path) {
            Ok(content) => {
                match section {
                    None | Some("") | Some("all") | Some("full") | Some("entire") => content,
                    Some(_sec) => {
                        // Find section
                        "Section not found".to_string()
                    }
                }
            }
            Err(e) => format!("{{\"error\": \"Failed to read documentation: {e}\"}}"),
        }
    }
}

pub fn register_catalog_commands(container: ServiceContainerAggregate) -> McpCommandCatalogSurface {
    let mut surface = McpCommandCatalogSurface::new();
    surface.register_all(container);
    surface
}

pub fn register_list_commands(container: ServiceContainerAggregate) -> McpCommandCatalogSurface {
    register_catalog_commands(container)
}

pub fn register_read_skill_context(container: ServiceContainerAggregate) -> McpCommandCatalogSurface {
    register_catalog_commands(container)
}
