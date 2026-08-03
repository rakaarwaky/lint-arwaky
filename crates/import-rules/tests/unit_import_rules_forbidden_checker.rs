// Unit tests for ArchImportForbiddenChecker — AES201 forbidden import rules.
use import_rules_lint_arwaky::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use shared::common::{
    BooleanVO, Count, FilePathList, LayerDefinition, LayerMapVO, LayerNameVO, NamingConfig,
    PatternList,
};
use shared::config_system::ArchitectureConfig;
use std::collections::HashMap;

/// Build a minimal ArchitectureConfig with a capabilities layer that forbids
/// importing from the "agent" and "surfaces" layers.
fn test_config() -> (ArchitectureConfig, LayerMapVO) {
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent", "surfaces"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility"]),
            ..Default::default()
        },
    );
    let layer_map = LayerMapVO::new(layers.clone());
    let config = ArchitectureConfig::new(
        BooleanVO::new(true),
        layers,
        vec![],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    );
    (config, layer_map)
}

#[test]
fn detects_forbidden_agent_import() {
    let checker = ArchImportForbiddenChecker::new();
    let (config, layer_map) = test_config();
    let content = "use agent::import_orchestrator;\n";
    let result = checker
        .check_single_file(
            "/tmp/project/capabilities_checker.rs",
            content,
            "/tmp/project",
            &config,
            &layer_map,
        )
        .unwrap();
    assert!(
        !result.values.is_empty(),
        "Should detect forbidden 'agent' import from capabilities layer"
    );
    assert_eq!(result.values[0].code.code(), "AES201");
}

#[test]
fn detects_forbidden_surfaces_import() {
    let checker = ArchImportForbiddenChecker::new();
    let (config, layer_map) = test_config();
    let content = "use surfaces::ui_component;\n";
    let result = checker
        .check_single_file(
            "/tmp/project/capabilities_service.rs",
            content,
            "/tmp/project",
            &config,
            &layer_map,
        )
        .unwrap();
    assert!(
        !result.values.is_empty(),
        "Should detect forbidden 'surfaces' import from capabilities layer"
    );
}

#[test]
fn no_violation_for_allowed_imports() {
    let checker = ArchImportForbiddenChecker::new();
    let (config, layer_map) = test_config();
    // taxonomy and contract are allowed for capabilities
    let content = "use taxonomy::definition_vo;\nuse contract::import_protocol;\n";
    let result = checker
        .check_single_file(
            "/tmp/project/capabilities_handler.rs",
            content,
            "/tmp/project",
            &config,
            &layer_map,
        )
        .unwrap();
    assert!(
        result.values.is_empty(),
        "Allowed imports should produce no violations, got {}",
        result.values.len()
    );
}

#[test]
fn no_violation_for_empty_content() {
    let checker = ArchImportForbiddenChecker::new();
    let (config, layer_map) = test_config();
    let result = checker
        .check_single_file(
            "/tmp/project/capabilities_empty.rs",
            "",
            "/tmp/project",
            &config,
            &layer_map,
        )
        .unwrap();
    assert!(
        result.values.is_empty(),
        "Empty content should produce no violations"
    );
}

#[test]
fn no_violation_for_non_prefixed_file() {
    let checker = ArchImportForbiddenChecker::new();
    let (config, layer_map) = test_config();
    // File without a recognized layer prefix — layer detection fails, no check
    let content = "use agent::something;\n";
    let result = checker
        .check_single_file(
            "/tmp/project/helper.rs",
            content,
            "/tmp/project",
            &config,
            &layer_map,
        )
        .unwrap();
    assert!(
        result.values.is_empty(),
        "File without layer prefix should produce no violations"
    );
}
