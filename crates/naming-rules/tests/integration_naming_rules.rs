// Integration tests — NamingContainer DI wiring + orchestrator round-trip.
use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::PatternList;
use shared::common::SuffixPolicyVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::INamingRunnerAggregate;
use shared::naming_rules::SUFFIX_POLICY_STRICT;
use std::collections::HashMap;
use std::sync::Arc;

fn make_layer_map() -> LayerMapVO {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
    def.naming.forbidden_suffix = PatternList::new(vec!["vo".to_string()]);

    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(layers)
}

#[test]
fn container_creates_with_default_config() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let container = NamingContainer::new(config, layer_map);
    let orch = container.orchestrator();
    assert_eq!(orch.name(), "naming-rules");
}

#[test]
fn container_provides_both_checkers() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let container = NamingContainer::new(config, layer_map);
    // Both checkers should be accessible
    let _nc = container.naming_convention_checker();
    let _sp = container.suffix_prefix_checker();
}

#[test]
fn orchestrator_produces_results_for_invalid_names() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let container = NamingContainer::new(config, layer_map);
    let orch = container.orchestrator();

    let files = FilePathList::new(vec![
        FilePath::new("src/capabilities_Bad_Caps.rs".to_string()).unwrap(),
    ]);
    let root = FilePath::new(".".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    // Run convention checker
    container
        .naming_convention_checker()
        .check_file_naming(
            &ArchitectureConfig::default(),
            &make_layer_map(),
            &files,
            &root,
            &mut results,
        );

    assert!(
        !results.is_empty(),
        "uppercase filename should produce violations"
    );
}

#[test]
fn orchestrator_clean_file_no_violations() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let container = NamingContainer::new(config, layer_map);

    let files = FilePathList::new(vec![
        FilePath::new("src/capabilities_user_checker.rs".to_string()).unwrap(),
    ]);
    let root = FilePath::new(".".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    container
        .naming_convention_checker()
        .check_file_naming(
            &ArchitectureConfig::default(),
            &make_layer_map(),
            &files,
            &root,
            &mut results,
        );

    assert!(
        results.is_empty(),
        "clean underscore file should pass naming convention"
    );
}
