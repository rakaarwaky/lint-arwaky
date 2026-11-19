// Unit tests for TaxonomyOrphanAnalyzer — orphan detection for taxonomy-layer files.
use orphan_rules_lint_arwaky::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer;
use shared_orphan_rules::ITaxonomyOrphanProtocol;

use shared_common::taxonomy_path_vo::FilePath;
use shared_quality_rules::taxonomy_quality_rules_vo::{InboundLinkMap, ReachabilityResult};
use std::collections::HashSet;

fn empty_reachability() -> ReachabilityResult {
    ReachabilityResult::new(HashSet::new())
}

fn reachable_for(fp: &FilePath) -> ReachabilityResult {
    ReachabilityResult::new(HashSet::from([fp.clone()]))
}

use std::collections::HashMap;

#[test]
fn test_constructor() {
    let analyzer = TaxonomyOrphanAnalyzer::new();
    // Verify construction succeeds
    let _analyzer2 = TaxonomyOrphanAnalyzer;
    // Just ensure the analyzer can be created without panicking
    let fp = FilePath::new(".".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let empty_map = InboundLinkMap::new(HashMap::new());
    let _result = analyzer.is_taxonomy_orphan(
        &fp,
        &root,
        None,
        &empty_map,
        &[],
        &HashMap::new(),
        &empty_reachability(),
    );
}

#[test]
fn test_file_with_no_importers_is_orphan() {
    let analyzer = TaxonomyOrphanAnalyzer::new();
    let fp = FilePath::new("crates/shared/src/taxonomy_color.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let empty_map = InboundLinkMap::new(HashMap::new());

    let result = analyzer.is_taxonomy_orphan(
        &fp,
        &root,
        None,
        &empty_map,
        &[],
        &HashMap::new(),
        &empty_reachability(),
    );
    assert!(result.is_orphan);
    assert!(!result.message.is_empty());
}

#[test]
fn test_file_imported_by_other_layer_is_not_orphan() {
    let analyzer = TaxonomyOrphanAnalyzer::new();
    let target = "crates/shared/src/taxonomy_color.rs";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let mut mapping = HashMap::new();
    // An agent-layer file imports the taxonomy file
    mapping.insert(
        target.to_string(),
        vec!["crates/orphan-rules/src/agent_orphan_orchestrator.rs".to_string()],
    );
    let inbound = InboundLinkMap::new(mapping);

    let result = analyzer.is_taxonomy_orphan(
        &fp,
        &root,
        None,
        &inbound,
        &[],
        &HashMap::new(),
        &reachable_for(&fp),
    );
    assert!(!result.is_orphan);
}

#[test]
fn test_file_only_imported_by_taxonomy_layer_is_orphan() {
    let analyzer = TaxonomyOrphanAnalyzer::new();
    let target = "crates/shared/src/taxonomy_color.rs";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let mut mapping = HashMap::new();
    // Only taxonomy-layer files import this taxonomy file
    mapping.insert(
        target.to_string(),
        vec!["crates/shared/src/taxonomy_size.rs".to_string()],
    );
    let inbound = InboundLinkMap::new(mapping);

    let result = analyzer.is_taxonomy_orphan(
        &fp,
        &root,
        None,
        &inbound,
        &[],
        &HashMap::new(),
        &empty_reachability(),
    );
    assert!(result.is_orphan);
    assert!(result.message.contains("taxonomy_color"));
}

#[test]
fn test_utility_suffix_categorization() {
    let analyzer = TaxonomyOrphanAnalyzer::new();
    let fp = FilePath::new("crates/shared/src/taxonomy_helper.rs".to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();
    let empty_map = InboundLinkMap::new(HashMap::new());

    let result = analyzer.is_taxonomy_orphan(
        &fp,
        &root,
        None,
        &empty_map,
        &[],
        &HashMap::new(),
        &empty_reachability(),
    );
    // "taxonomy_helper" has suffix "helper" → category is "utility"
    assert!(result.is_orphan);
    assert!(!result.message.is_empty());
}

#[test]
fn test_barrel_fallback_across_default_rule_exceptions() {
    let analyzer = TaxonomyOrphanAnalyzer::new();
    let target = "packages/shared/src/models/taxonomy_user_vo.ts";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    let barrels = ["index.ts", "index.tsx", "index.js", "mod.rs", "__init__.py"];
    for barrel_name in barrels {
        let barrel_path = format!("packages/shared/src/models/{barrel_name}");
        let mut mapping = HashMap::new();
        mapping.insert(
            barrel_path,
            vec!["packages/shared/src/client/contract_client_protocol.ts".to_string()],
        );
        let inbound = InboundLinkMap::new(mapping);

        let result = analyzer.is_taxonomy_orphan(
            &fp,
            &root,
            None,
            &inbound,
            &[],
            &HashMap::new(),
            &reachable_for(&fp),
        );
        assert!(
            !result.is_orphan,
            "barrel '{barrel_name}' with higher-layer importer should rescue taxonomy file"
        );
    }
}

#[test]
fn test_barrel_fallback_self_import_filtered() {
    let analyzer = TaxonomyOrphanAnalyzer::new();
    let target = "packages/shared/src/models/taxonomy_user_vo.ts";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    // The barrel only lists the file itself as an importer (self-import)
    let barrel_path = "packages/shared/src/models/index.ts".to_string();
    let mut mapping = HashMap::new();
    mapping.insert(barrel_path, vec![target.to_string()]);
    let inbound = InboundLinkMap::new(mapping);

    let result = analyzer.is_taxonomy_orphan(
        &fp,
        &root,
        None,
        &inbound,
        &[],
        &HashMap::new(),
        &reachable_for(&fp),
    );
    assert!(
        result.is_orphan,
        "self-import in barrel must be filtered out and file declared orphan"
    );
}

#[test]
fn test_barrel_fallback_only_lower_layer_importer_is_orphan() {
    let analyzer = TaxonomyOrphanAnalyzer::new();
    let target = "packages/shared/src/models/taxonomy_user_vo.ts";
    let fp = FilePath::new(target.to_string()).unwrap();
    let root = FilePath::new(".".to_string()).unwrap();

    // Barrel is imported by another taxonomy file (not a higher-layer rescuer)
    let barrel_path = "packages/shared/src/models/index.ts".to_string();
    let mut mapping = HashMap::new();
    mapping.insert(
        barrel_path,
        vec!["packages/shared/src/models/taxonomy_other_vo.ts".to_string()],
    );
    let inbound = InboundLinkMap::new(mapping);

    let result = analyzer.is_taxonomy_orphan(
        &fp,
        &root,
        None,
        &inbound,
        &[],
        &HashMap::new(),
        &reachable_for(&fp),
    );
    assert!(
        result.is_orphan,
        "barrel imported only by taxonomy-layer files should not rescue target"
    );
}
