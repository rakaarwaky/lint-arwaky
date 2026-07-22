// PURPOSE: CommandCatalogVO — single source of truth for CLI command catalog
// Consolidates: taxonomy_catalog_constant, taxonomy_metadata_vo, taxonomy_command_catalog_vo
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::common::taxonomy_suggestion_vo::Suggestion;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ─── COMMAND_CATALOG constant (from taxonomy_catalog_constant) ───

pub static COMMAND_CATALOG: &[(&str, &str, &str)] = &[
    (
        "check",
        "Run full architecture compliance analysis",
        "lint-arwaky-cli check /path",
    ),
    (
        "scan",
        "Deep directory scan (alias for check)",
        "lint-arwaky-cli scan ./src/",
    ),
    ("fix", "Apply safe fixes", "lint-arwaky-cli fix file.py"),
    (
        "ci",
        "CI-optimized with exit codes",
        "lint-arwaky-cli ci /path --threshold 80",
    ),
    (
        "doctor",
        "System health diagnostics",
        "lint-arwaky-cli doctor",
    ),
    (
        "version",
        "Show version information",
        "lint-arwaky-cli version",
    ),
    (
        "init",
        "Initialize project configuration",
        "lint-arwaky-cli init",
    ),
    (
        "watch",
        "Watch for file changes and lint",
        "lint-arwaky-cli watch ./src/",
    ),
    (
        "mcp",
        "Start MCP server for IDE integration",
        "lint-arwaky-cli mcp",
    ),
    ("tui", "Launch terminal UI", "lint-arwaky-cli tui"),
];

// ─── CommandMetadataVO (from taxonomy_metadata_vo) ───

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommandMetadataVO {
    pub description: DescriptionVO,
    pub example: Suggestion,
}

impl CommandMetadataVO {
    pub fn new(description: DescriptionVO, example: Suggestion) -> Self {
        Self {
            description,
            example,
        }
    }
}

impl std::fmt::Display for CommandMetadataVO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.description, self.example)
    }
}

// ─── CommandCatalogVO (original) ───

pub struct CommandCatalogVO {}

impl CommandCatalogVO {
    /// Derive the full command catalog from COMMAND_CATALOG (single source of truth).
    pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
        let mut catalog = HashMap::new();
        for (name, description, example) in COMMAND_CATALOG {
            catalog.insert(
                ActionName::from(*name),
                CommandMetadataVO::new(DescriptionVO::new(*description), Suggestion::new(*example)),
            );
        }
        catalog
    }
}

pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
    CommandCatalogVO::command_catalog()
}
