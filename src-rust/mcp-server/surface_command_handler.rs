/// MCP Tools: list_commands and read_skill_context.
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;

/// COMMAND_CATALOG — mirrors the Python dict exactly.
pub struct CommandEntry {
    pub description: &'static str,
    pub example: &'static str,
}

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
    pub container: Option<Arc<dyn ServiceContainerAggregate>>,
}

impl McpCommandCatalogSurface {
    pub fn new() -> Self {
        Self { container: None }
    }

    pub fn register_all(&mut self, container: Arc<dyn ServiceContainerAggregate>) {
        self.container = Some(container);
    }

    pub fn list_commands(&self, domain: Option<&str>) -> HashMap<String, HashMap<String, String>> {
        list_commands_func(domain)
    }

    pub fn read_skill_context(&self, section: Option<&str>) -> String {
        // In real impl: read SKILL.md from project root
        let skill_path = Path::new("SKILL.md");
        if !skill_path.exists() {
            return format!(
                "{{\"error\": \"SKILL.md not found\", \"path\": \"{}\"}}",
                skill_path.display()
            );
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

pub fn register_catalog_commands(
    container: Arc<dyn ServiceContainerAggregate>,
) -> McpCommandCatalogSurface {
    let mut surface = McpCommandCatalogSurface::new();
    surface.register_all(container);
    surface
}

pub fn register_list_commands(
    container: Arc<dyn ServiceContainerAggregate>,
) -> McpCommandCatalogSurface {
    register_catalog_commands(container)
}

pub fn register_read_skill_context(
    container: Arc<dyn ServiceContainerAggregate>,
) -> McpCommandCatalogSurface {
    register_catalog_commands(container)
}
