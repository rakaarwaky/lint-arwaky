use std::collections::HashMap;
use super::*;

pub fn command_catalog() -> HashMap<String, CommandMetadataVO> {
    let mut catalog = HashMap::new();
    catalog.insert("check".into(), CommandMetadataVO::new(DescriptionVO::new("Run full architecture compliance analysis"), Suggestion::new("auto-lint check /path")));
    catalog.insert("scan".into(), CommandMetadataVO::new(DescriptionVO::new("Deep directory scan"), Suggestion::new("auto-lint scan ./src/")));
    catalog.insert("fix".into(), CommandMetadataVO::new(DescriptionVO::new("Apply safe fixes"), Suggestion::new("auto-lint fix file.py")));
    catalog.insert("report".into(), CommandMetadataVO::new(DescriptionVO::new("Generate quality reports"), Suggestion::new("auto-lint report ./src --format json")));
    catalog.insert("ci".into(), CommandMetadataVO::new(DescriptionVO::new("CI-optimized with exit codes"), Suggestion::new("auto-lint ci /path --exit-zero")));
    catalog.insert("batch".into(), CommandMetadataVO::new(DescriptionVO::new("Check multiple paths"), Suggestion::new("auto-lint batch path1.py path2.js")));
    catalog.insert("watch".into(), CommandMetadataVO::new(DescriptionVO::new("Watch files for changes"), Suggestion::new("auto-lint watch ./src/")));
    catalog.insert("security".into(), CommandMetadataVO::new(DescriptionVO::new("Bandit vulnerability scanning"), Suggestion::new("auto-lint security /path")));
    catalog.insert("complexity".into(), CommandMetadataVO::new(DescriptionVO::new("Cyclomatic complexity"), Suggestion::new("auto-lint complexity ./src/")));
    catalog.insert("duplicates".into(), CommandMetadataVO::new(DescriptionVO::new("Code duplication detection"), Suggestion::new("auto-lint duplicates /path")));
    catalog.insert("trends".into(), CommandMetadataVO::new(DescriptionVO::new("Quality trend over time"), Suggestion::new("auto-lint trends .")));
    catalog.insert("dependencies".into(), CommandMetadataVO::new(DescriptionVO::new("Dependency vulnerability scan"), Suggestion::new("auto-lint dependencies .")));
    catalog.insert("diff".into(), CommandMetadataVO::new(DescriptionVO::new("Compare two versions"), Suggestion::new("auto-lint diff v1.py v2.py")));
    catalog.insert("suggest".into(), CommandMetadataVO::new(DescriptionVO::new("AI-powered suggestions"), Suggestion::new("auto-lint suggest file.py")));
    catalog.insert("stats".into(), CommandMetadataVO::new(DescriptionVO::new("Statistics dashboard"), Suggestion::new("auto-lint stats ./src/")));
    catalog.insert("init".into(), CommandMetadataVO::new(DescriptionVO::new("Initialize config"), Suggestion::new("auto-lint init /path")));
    catalog.insert("config".into(), CommandMetadataVO::new(DescriptionVO::new("Edit configuration"), Suggestion::new("auto-lint config get thresholds")));
    catalog.insert("ignore".into(), CommandMetadataVO::new(DescriptionVO::new("Manage ignore rules"), Suggestion::new("auto-lint ignore add E501")));
    catalog.insert("export".into(), CommandMetadataVO::new(DescriptionVO::new("Export reports"), Suggestion::new("auto-lint export --format sarif")));
    catalog.insert("clean".into(), CommandMetadataVO::new(DescriptionVO::new("Cleanup cache"), Suggestion::new("auto-lint clean")));
    catalog.insert("update".into(), CommandMetadataVO::new(DescriptionVO::new("Update adapters"), Suggestion::new("auto-lint update")));
    catalog.insert("doctor".into(), CommandMetadataVO::new(DescriptionVO::new("Diagnose issues"), Suggestion::new("auto-lint doctor")));
    catalog.insert("adapters".into(), CommandMetadataVO::new(DescriptionVO::new("List enabled adapters"), Suggestion::new("auto-lint adapters")));
    catalog.insert("install-hook".into(), CommandMetadataVO::new(DescriptionVO::new("Install git pre-commit hook"), Suggestion::new("auto-lint install-hook")));
    catalog.insert("uninstall-hook".into(), CommandMetadataVO::new(DescriptionVO::new("Remove git pre-commit hook"), Suggestion::new("auto-lint uninstall-hook")));
    catalog.insert("cancel".into(), CommandMetadataVO::new(DescriptionVO::new("Cancel a running lint job"), Suggestion::new("auto-lint cancel <job_id>")));
    catalog.insert("plugins".into(), CommandMetadataVO::new(DescriptionVO::new("List discovered plugins"), Suggestion::new("auto-lint plugins")));
    catalog.insert("multi-project".into(), CommandMetadataVO::new(DescriptionVO::new("Run lint across multiple projects"), Suggestion::new("auto-lint multi-project proj1/ proj2/")));
    catalog.insert("version".into(), CommandMetadataVO::new(DescriptionVO::new("Show version"), Suggestion::new("auto-lint version")));
    catalog
}
