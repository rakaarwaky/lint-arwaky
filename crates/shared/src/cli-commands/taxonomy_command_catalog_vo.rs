// PURPOSE: CommandCatalogVO — maps ActionName to CommandMetadataVO for all CLI commands
// Derives from COMMAND_CATALOG (single source of truth in taxonomy_catalog_constant)
use crate::cli_commands::taxonomy_catalog_constant::COMMAND_CATALOG;
use crate::cli_commands::taxonomy_metadata_vo::CommandMetadataVO;
use crate::common::taxonomy_suggestion_vo::DescriptionVO;
use crate::common::taxonomy_suggestion_vo::Suggestion;
use crate::mcp_server::taxonomy_action_vo::ActionName;
use std::collections::HashMap;

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
