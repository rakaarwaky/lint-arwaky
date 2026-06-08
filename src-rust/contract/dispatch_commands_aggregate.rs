use crate::taxonomy::{ActionName, CommandMetadataVO, DescriptionVO, Suggestion};
use std::collections::HashMap;

pub struct DispatchCommandsAggregate;

impl DispatchCommandsAggregate {
    pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
        let mut catalog = HashMap::new();
        catalog.insert(
            ActionName::from("check"),
            CommandMetadataVO::new(
                DescriptionVO::new("Run full architecture compliance analysis"),
                Suggestion::new("lint-arwaky check /path"),
            ),
        );
        catalog.insert(
            ActionName::from("scan"),
            CommandMetadataVO::new(
                DescriptionVO::new("Deep directory scan"),
                Suggestion::new("lint-arwaky scan ./src/"),
            ),
        );
        catalog.insert(
            ActionName::from("fix"),
            CommandMetadataVO::new(
                DescriptionVO::new("Apply safe fixes"),
                Suggestion::new("lint-arwaky fix file.py"),
            ),
        );
        catalog.insert(
            ActionName::from("report"),
            CommandMetadataVO::new(
                DescriptionVO::new("Generate quality reports"),
                Suggestion::new("lint-arwaky report ./src --format json"),
            ),
        );
        catalog.insert(
            ActionName::from("ci"),
            CommandMetadataVO::new(
                DescriptionVO::new("CI-optimized with exit codes"),
                Suggestion::new("lint-arwaky ci /path --exit-zero"),
            ),
        );
        catalog.insert(
            ActionName::from("batch"),
            CommandMetadataVO::new(
                DescriptionVO::new("Check multiple paths"),
                Suggestion::new("lint-arwaky batch path1.py path2.js"),
            ),
        );
        catalog.insert(
            ActionName::from("watch"),
            CommandMetadataVO::new(
                DescriptionVO::new("Watch files for changes"),
                Suggestion::new("lint-arwaky watch ./src/"),
            ),
        );
        catalog.insert(
            ActionName::from("security"),
            CommandMetadataVO::new(
                DescriptionVO::new("Bandit vulnerability scanning"),
                Suggestion::new("lint-arwaky security /path"),
            ),
        );
        catalog.insert(
            ActionName::from("complexity"),
            CommandMetadataVO::new(
                DescriptionVO::new("Cyclomatic complexity"),
                Suggestion::new("lint-arwaky complexity ./src/"),
            ),
        );
        catalog.insert(
            ActionName::from("duplicates"),
            CommandMetadataVO::new(
                DescriptionVO::new("Code duplication detection"),
                Suggestion::new("lint-arwaky duplicates /path"),
            ),
        );
        catalog.insert(
            ActionName::from("trends"),
            CommandMetadataVO::new(
                DescriptionVO::new("Quality trend over time"),
                Suggestion::new("lint-arwaky trends ."),
            ),
        );
        catalog.insert(
            ActionName::from("dependencies"),
            CommandMetadataVO::new(
                DescriptionVO::new("Dependency vulnerability scan"),
                Suggestion::new("lint-arwaky dependencies ."),
            ),
        );
        catalog.insert(
            ActionName::from("diff"),
            CommandMetadataVO::new(
                DescriptionVO::new("Compare two versions"),
                Suggestion::new("lint-arwaky diff v1.py v2.py"),
            ),
        );
        catalog.insert(
            ActionName::from("suggest"),
            CommandMetadataVO::new(
                DescriptionVO::new("AI-powered suggestions"),
                Suggestion::new("lint-arwaky suggest file.py"),
            ),
        );
        catalog.insert(
            ActionName::from("stats"),
            CommandMetadataVO::new(
                DescriptionVO::new("Statistics dashboard"),
                Suggestion::new("lint-arwaky stats ./src/"),
            ),
        );
        catalog.insert(
            ActionName::from("init"),
            CommandMetadataVO::new(
                DescriptionVO::new("Initialize config"),
                Suggestion::new("lint-arwaky init /path"),
            ),
        );
        catalog.insert(
            ActionName::from("config"),
            CommandMetadataVO::new(
                DescriptionVO::new("Edit configuration"),
                Suggestion::new("lint-arwaky config get thresholds"),
            ),
        );
        catalog.insert(
            ActionName::from("ignore"),
            CommandMetadataVO::new(
                DescriptionVO::new("Manage ignore rules"),
                Suggestion::new("lint-arwaky ignore add E501"),
            ),
        );
        catalog.insert(
            ActionName::from("export"),
            CommandMetadataVO::new(
                DescriptionVO::new("Export reports"),
                Suggestion::new("lint-arwaky export --format sarif"),
            ),
        );
        catalog.insert(
            ActionName::from("clean"),
            CommandMetadataVO::new(
                DescriptionVO::new("Cleanup cache"),
                Suggestion::new("lint-arwaky clean"),
            ),
        );
        catalog.insert(
            ActionName::from("update"),
            CommandMetadataVO::new(
                DescriptionVO::new("Update adapters"),
                Suggestion::new("lint-arwaky update"),
            ),
        );
        catalog.insert(
            ActionName::from("doctor"),
            CommandMetadataVO::new(
                DescriptionVO::new("Diagnose issues"),
                Suggestion::new("lint-arwaky doctor"),
            ),
        );
        catalog.insert(
            ActionName::from("adapters"),
            CommandMetadataVO::new(
                DescriptionVO::new("List enabled adapters"),
                Suggestion::new("lint-arwaky adapters"),
            ),
        );
        catalog.insert(
            ActionName::from("install-hook"),
            CommandMetadataVO::new(
                DescriptionVO::new("Install git pre-commit hook"),
                Suggestion::new("lint-arwaky install-hook"),
            ),
        );
        catalog.insert(
            ActionName::from("uninstall-hook"),
            CommandMetadataVO::new(
                DescriptionVO::new("Remove git pre-commit hook"),
                Suggestion::new("lint-arwaky uninstall-hook"),
            ),
        );
        catalog.insert(
            ActionName::from("cancel"),
            CommandMetadataVO::new(
                DescriptionVO::new("Cancel a running lint job"),
                Suggestion::new("lint-arwaky cancel <job_id>"),
            ),
        );
        catalog.insert(
            ActionName::from("plugins"),
            CommandMetadataVO::new(
                DescriptionVO::new("List discovered plugins"),
                Suggestion::new("lint-arwaky plugins"),
            ),
        );
        catalog.insert(
            ActionName::from("multi-project"),
            CommandMetadataVO::new(
                DescriptionVO::new("Run lint across multiple projects"),
                Suggestion::new("lint-arwaky multi-project proj1/ proj2/"),
            ),
        );
        catalog.insert(
            ActionName::from("version"),
            CommandMetadataVO::new(
                DescriptionVO::new("Show version"),
                Suggestion::new("lint-arwaky version"),
            ),
        );
        catalog
    } // end command_catalog
}

// Forward-compat free function wrapper for backward compatibility
pub fn command_catalog() -> HashMap<ActionName, CommandMetadataVO> {
    DispatchCommandsAggregate::command_catalog()
}
