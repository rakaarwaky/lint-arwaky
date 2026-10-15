// PURPOSE: Acceptance tests — AES201 forbidden import detection.
// Verifies: taxonomy→capabilities, surface→agent, capabilities→agent, etc.
use import_rules_lint_arwaky::capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
use shared_common::NamingConfig;
use shared_common::taxonomy_common_vo::{BooleanVO, Count, PatternList};
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_layer_vo::{LayerDefinition, LayerMapVO};
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_config_system::ArchitectureConfig;
use shared_import_rules::IImportForbiddenProtocol;
use std::collections::HashMap;

/// Full AES config with 3 layers and their forbidden/allowed rules.
fn aes_config() -> (ArchitectureConfig, LayerMapVO) {
    let mut layers = HashMap::new();
    // taxonomy cannot import capabilities, agent, or surface
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["capabilities", "agent", "surface"]),
            allowed: PatternList::new(vec!["utility"]),
            ..Default::default()
        },
    );
    // capabilities cannot import agent or surface
    layers.insert(
        LayerNameVO::new("capabilities"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent", "surface"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility"]),
            ..Default::default()
        },
    );
    // surface cannot import agent
    layers.insert(
        LayerNameVO::new("surface"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["agent"]),
            allowed: PatternList::new(vec!["taxonomy", "contract", "utility", "capabilities"]),
            ..Default::default()
        },
    );
    // agent cannot import surface
    layers.insert(
        LayerNameVO::new("agent"),
        LayerDefinition {
            forbidden: PatternList::new(vec!["surface"]),
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
fn aes201_taxonomy_importing_surface_is_forbidden() {
    let count = detect(
        "use surface::ui_handler;\n",
        "/tmp/project/taxonomy_value.rs",
    );
    assert!(count > 0, "taxonomy importing surface must be forbidden");
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
fn aes201_capabilities_importing_surface_is_forbidden() {
    let count = detect(
        "use surface::ui_component;\n",
        "/tmp/project/capabilities_handler.rs",
    );
    assert!(
        count > 0,
        "capabilities importing surface must be forbidden"
    );
}

// ─── AES201: surface layer violations ─────────────────────

#[test]
fn aes201_surface_importing_agent_is_forbidden() {
    let count = detect("use agent::runner;\n", "/tmp/project/surface_handler.rs");
    assert!(count > 0, "surface importing agent must be forbidden");
}

// ─── AES201: agent layer violations ───────────────────────

#[test]
fn aes201_agent_importing_surface_is_forbidden() {
    let count = detect("use surface::ui;\n", "/tmp/project/agent_orchestrator.rs");
    assert!(count > 0, "agent importing surface must be forbidden");
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
        "use utility::helper;\n",
        "/tmp/project/capabilities_handler.rs",
    );
    assert_eq!(count, 0, "capabilities importing utility should be allowed");
}
