// PURPOSE: Acceptance tests — AES201 forbidden import detection.
// Verifies: taxonomy→capabilities, surface→agent, capabilities→agent, etc.
use import_rules_lint_arwaky::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use shared::common::NamingConfig;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::IImportForbiddenProtocol;
use std::collections::HashMap;

/// Full AES config with 3 layers and their forbidden/allowed rules.
fn aes_config() -> (ArchitectureConfig, LayerMapVO) {
    let mut layers = HashMap::new();
    // taxonomy cannot import capabilities, agent, or surfaces
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["capabilities", "agent", "surfaces"]),
            allowed: PatternList::new(vec!["utility"]),
            ..Default::default()
        },
    );
    // capabilities cannot import agent or surfaces
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent", "surfaces"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility"]),
            ..Default::default()
        },
    );
    // surfaces cannot import agent
    layers.insert(
        LayerNameVO::new("surfaces"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility", "capabilities"]),
            ..Default::default()
        },
    );
    // agent cannot import surfaces
    layers.insert(
        LayerNameVO::new("agent"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["surfaces"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility", "capabilities"]),
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

fn detect(content: &str, file: &str) -> usize {
    let checker = ArchImportForbiddenChecker::new();
    let (config, layer_map) = aes_config();
    let result = checker
        .check_single_file(file, content, "/tmp/project", &config, &layer_map)
        .unwrap();
    result.values.len()
}

// ─── AES201: taxonomy layer violations ────────────────────

#[test]
fn aes201_taxonomy_importing_capabilities_is_forbidden() {
    let count = detect(
        "use capabilities::orchestrator;\n",
        "/tmp/project/taxonomy_vo.rs",
    );
    assert!(
        count > 0,
        "taxonomy importing capabilities must be forbidden"
    );
}

#[test]
fn aes201_taxonomy_importing_agent_is_forbidden() {
    let count = detect(
        "use agent::runner;\n",
        "/tmp/project/taxonomy_definition.rs",
    );
    assert!(count > 0, "taxonomy importing agent must be forbidden");
}

#[test]
fn aes201_taxonomy_importing_surfaces_is_forbidden() {
    let count = detect(
        "use surfaces::ui_handler;\n",
        "/tmp/project/taxonomy_value.rs",
    );
    assert!(count > 0, "taxonomy importing surfaces must be forbidden");
}

// ─── AES201: capabilities layer violations ────────────────

#[test]
fn aes201_capabilities_importing_agent_is_forbidden() {
    let count = detect(
        "use agent::import_orchestrator;\n",
        "/tmp/project/capabilities_checker.rs",
    );
    assert!(count > 0, "capabilities importing agent must be forbidden");
}

#[test]
fn aes201_capabilities_importing_surfaces_is_forbidden() {
    let count = detect(
        "use surfaces::ui_component;\n",
        "/tmp/project/capabilities_handler.rs",
    );
    assert!(
        count > 0,
        "capabilities importing surfaces must be forbidden"
    );
}

// ─── AES201: surfaces layer violations ────────────────────

#[test]
fn aes201_surfaces_importing_agent_is_forbidden() {
    let count = detect("use agent::runner;\n", "/tmp/project/surface_handler.rs");
    assert!(count > 0, "surfaces importing agent must be forbidden");
}

// ─── AES201: agent layer violations ───────────────────────

#[test]
fn aes201_agent_importing_surfaces_is_forbidden() {
    let count = detect("use surfaces::ui;\n", "/tmp/project/agent_orchestrator.rs");
    assert!(count > 0, "agent importing surfaces must be forbidden");
}

// ─── AES201: allowed imports (no violations) ──────────────

#[test]
fn aes201_taxonomy_importing_utility_is_allowed() {
    let count = detect("use utility::helper;\n", "/tmp/project/taxonomy_vo.rs");
    assert_eq!(count, 0, "taxonomy importing utility should be allowed");
}

#[test]
fn aes201_capabilities_importing_taxonomy_is_allowed() {
    let count = detect(
        "use taxonomy::definition_vo;\n",
        "/tmp/project/capabilities_checker.rs",
    );
    assert_eq!(
        count, 0,
        "capabilities importing taxonomy should be allowed"
    );
}

#[test]
fn aes201_capabilities_importing_utility_is_allowed() {
    let count = detect(
        "use utility::path_utils;\n",
        "/tmp/project/capabilities_handler.rs",
    );
    assert_eq!(count, 0, "capabilities importing utility should be allowed");
}

// ─── AES201: protocol trait compliance ────────────────────

#[test]
fn aes201_protocol_rule_name_returns_aes201() {
    let checker = ArchImportForbiddenChecker::new();
    assert_eq!(checker.rule_name().value(), "AES201");
}

#[test]
fn aes201_empty_import_map_produces_no_violations() {
    let checker = ArchImportForbiddenChecker::new();
    let (config, layer_map) = aes_config();
    use shared::common::FilePathList;
    use std::collections::HashMap;
    let files = FilePathList::new(vec![
        FilePath::new("/tmp/project/capabilities_handler.rs".to_string()).unwrap(),
    ]);
    let root = FilePath::new("/tmp/project".to_string()).unwrap();
    let result = checker
        .check_forbidden_imports(
            &config,
            &layer_map,
            &files,
            &root,
            &HashMap::new(),
            &HashMap::new(),
        )
        .unwrap();
    assert!(
        result.values.is_empty(),
        "Empty imports_map should produce no violations"
    );
}
