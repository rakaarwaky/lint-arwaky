use super::*;
use std::collections::HashMap;

pub fn command_catalog() -> HashMap<String, CommandMetadataVO> {
    let mut catalog = HashMap::new();
    catalog.insert(
        "check".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Run full architecture compliance analysis"),
            Suggestion::new("lint-arwaky check /path"),
        ),
    );
    catalog.insert(
        "scan".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Deep directory scan"),
            Suggestion::new("lint-arwaky scan ./src/"),
        ),
    );
    catalog.insert(
        "fix".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Apply safe fixes"),
            Suggestion::new("lint-arwaky fix file.py"),
        ),
    );
    catalog.insert(
        "report".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Generate quality reports"),
            Suggestion::new("lint-arwaky report ./src --format json"),
        ),
    );
    catalog.insert(
        "ci".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("CI-optimized with exit codes"),
            Suggestion::new("lint-arwaky ci /path --exit-zero"),
        ),
    );
    catalog.insert(
        "batch".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Check multiple paths"),
            Suggestion::new("lint-arwaky batch path1.py path2.js"),
        ),
    );
    catalog.insert(
        "watch".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Watch files for changes"),
            Suggestion::new("lint-arwaky watch ./src/"),
        ),
    );
    catalog.insert(
        "security".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Bandit vulnerability scanning"),
            Suggestion::new("lint-arwaky security /path"),
        ),
    );
    catalog.insert(
        "complexity".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Cyclomatic complexity"),
            Suggestion::new("lint-arwaky complexity ./src/"),
        ),
    );
    catalog.insert(
        "duplicates".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Code duplication detection"),
            Suggestion::new("lint-arwaky duplicates /path"),
        ),
    );
    catalog.insert(
        "trends".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Quality trend over time"),
            Suggestion::new("lint-arwaky trends ."),
        ),
    );
    catalog.insert(
        "dependencies".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Dependency vulnerability scan"),
            Suggestion::new("lint-arwaky dependencies ."),
        ),
    );
    catalog.insert(
        "diff".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Compare two versions"),
            Suggestion::new("lint-arwaky diff v1.py v2.py"),
        ),
    );
    catalog.insert(
        "suggest".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("AI-powered suggestions"),
            Suggestion::new("lint-arwaky suggest file.py"),
        ),
    );
    catalog.insert(
        "stats".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Statistics dashboard"),
            Suggestion::new("lint-arwaky stats ./src/"),
        ),
    );
    catalog.insert(
        "init".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Initialize config"),
            Suggestion::new("lint-arwaky init /path"),
        ),
    );
    catalog.insert(
        "config".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Edit configuration"),
            Suggestion::new("lint-arwaky config get thresholds"),
        ),
    );
    catalog.insert(
        "ignore".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Manage ignore rules"),
            Suggestion::new("lint-arwaky ignore add E501"),
        ),
    );
    catalog.insert(
        "export".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Export reports"),
            Suggestion::new("lint-arwaky export --format sarif"),
        ),
    );
    catalog.insert(
        "clean".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Cleanup cache"),
            Suggestion::new("lint-arwaky clean"),
        ),
    );
    catalog.insert(
        "update".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Update adapters"),
            Suggestion::new("lint-arwaky update"),
        ),
    );
    catalog.insert(
        "doctor".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Diagnose issues"),
            Suggestion::new("lint-arwaky doctor"),
        ),
    );
    catalog.insert(
        "adapters".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("List enabled adapters"),
            Suggestion::new("lint-arwaky adapters"),
        ),
    );
    catalog.insert(
        "install-hook".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Install git pre-commit hook"),
            Suggestion::new("lint-arwaky install-hook"),
        ),
    );
    catalog.insert(
        "uninstall-hook".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Remove git pre-commit hook"),
            Suggestion::new("lint-arwaky uninstall-hook"),
        ),
    );
    catalog.insert(
        "cancel".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Cancel a running lint job"),
            Suggestion::new("lint-arwaky cancel <job_id>"),
        ),
    );
    catalog.insert(
        "plugins".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("List discovered plugins"),
            Suggestion::new("lint-arwaky plugins"),
        ),
    );
    catalog.insert(
        "multi-project".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Run lint across multiple projects"),
            Suggestion::new("lint-arwaky multi-project proj1/ proj2/"),
        ),
    );
    catalog.insert(
        "version".into(),
        CommandMetadataVO::new(
            DescriptionVO::new("Show version"),
            Suggestion::new("lint-arwaky version"),
        ),
    );
    catalog
}
