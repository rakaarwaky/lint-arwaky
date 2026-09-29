// Smoke tests — quick boot + respond within 5s.
use naming_rules_lint_arwaky::capabilities_naming_checker::NamingChecker;
use naming_rules_lint_arwaky::root_naming_rules_container::NamingContainer;
use shared::common::PatternList;
use shared::common::SuffixPolicyVO;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::LintResultList;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::SUFFIX_POLICY_STRICT;
use shared::naming_rules::taxonomy_naming_request::NamingRequest;
use shared::naming_rules::taxonomy_naming_response::NamingResponse;
use shared::naming_rules::utility_naming_checker::{get_stem, get_suffix};
use std::collections::HashMap;
use std::sync::Arc;

fn make_layer_map() -> LayerMapVO {
    let mut def = LayerDefinition::default();
    def.naming.suffix_policy = SuffixPolicyVO::new(SUFFIX_POLICY_STRICT.to_string());
    def.naming.allowed_suffix =
        PatternList::new(vec!["checker".to_string(), "adapter".to_string()]);
    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("capabilities"), def);
    LayerMapVO::new(layers)
}

#[test]
fn container_creation_smoke() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let container = NamingContainer::new(config, layer_map);
    let orch = container.orchestrator();
    match orch.execute(NamingRequest::Name) {
        NamingResponse::Name { name } => assert_eq!(name, "naming-rules"),
        NamingResponse::Audit { .. } => panic!("expected a name response"),
    }
}

#[test]
fn orchestrator_basic_check_smoke() {
    let config = Arc::new(ArchitectureConfig::default());
    let layer_map = Arc::new(make_layer_map());
    let container = NamingContainer::new(config, layer_map);
    let _orch = container.orchestrator();

    let files = FilePathList::new(vec![
        FilePath::new("src/capabilities_user_checker.rs".to_string()).unwrap(),
    ]);
    let root = FilePath::new(".".to_string()).unwrap();
    let mut results = LintResultList::new(Vec::new());

    container.naming_convention().check_file_naming(
        &ArchitectureConfig::default(),
        &make_layer_map(),
        &files,
        &root,
        &mut results,
    );
    // Just verify it ran without panicking
    let _ = results;
}

#[test]
fn get_stem_smoke() {
    assert_eq!(
        get_stem("capabilities_user_checker.rs"),
        Some("capabilities_user_checker")
    );
    assert_eq!(get_stem("foo.spec.rs"), Some("foo.spec"));
    assert_eq!(get_stem("noext"), Some("noext"));
}

#[test]
fn get_suffix_smoke() {
    assert_eq!(get_suffix("capabilities_user_checker"), Some("checker"));
    assert_eq!(get_suffix("no_underscore"), Some("underscore"));
    assert_eq!(get_suffix("singleword"), None);
}

#[test]
fn naming_checker_construction_smoke() {
    let _ = NamingChecker::new();
}
