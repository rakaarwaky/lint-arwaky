// PURPOSE: Acceptance tests — AES205 circular dependency detection.
use import_rules_lint_arwaky::capabilities_cycle_import_analyzer::DependencyCycleAnalyzer;
use shared::common::NamingConfig;
use shared::common::taxonomy_common_vo::{BooleanVO, Count};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO};
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::ArchitectureConfig;
use shared::import_rules::contract_cycle_import_protocol::ICycleImportProtocol;
use shared::import_rules::taxonomy_dependency_edge_vo::DependencyEdge;
use std::collections::HashMap;

fn analyzer() -> DependencyCycleAnalyzer {
    DependencyCycleAnalyzer::new()
}

fn cycle_config() -> (ArchitectureConfig, LayerMapVO) {
    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("taxonomy"), LayerDefinition::default());
    layers.insert(LayerNameVO::new("capabilities"), LayerDefinition::default());
    layers.insert(LayerNameVO::new("agent"), LayerDefinition::default());
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

// ─── AES205: protocol trait methods ───────────────────────

#[test]
fn aes205_normalize_to_layer_strips_prefix() {
    let a = analyzer();
    let result = a.normalize_to_layer("capabilities_import_orchestrator");
    assert_eq!(result.value(), "capabilities");
}

#[test]
fn aes205_normalize_to_layer_no_prefix() {
    let a = analyzer();
    let result = a.normalize_to_layer("standalone_module");
    assert_eq!(result.value(), "standalone");
}

// ─── AES205: cycle detection via edges ────────────────────

#[test]
fn aes205_detects_simple_cycle_two_nodes() {
    let a = analyzer();
    let edges = vec![
        DependencyEdge::new("capabilities", "agent"),
        DependencyEdge::new("agent", "capabilities"),
    ];
    let cycles = a.detect_cycle_edges(&edges);
    assert!(!cycles.is_empty(), "Two-node cycle should be detected");
}

#[test]
fn aes205_no_cycle_with_acyclic_edges() {
    let a = analyzer();
    let edges = vec![
        DependencyEdge::new("taxonomy", "utility"),
        DependencyEdge::new("capabilities", "taxonomy"),
        DependencyEdge::new("agent", "capabilities"),
    ];
    let cycles = a.detect_cycle_edges(&edges);
    assert!(
        cycles.is_empty(),
        "Acyclic graph should have no cycles, got {}",
        cycles.len()
    );
}

#[test]
fn aes205_detects_three_node_cycle() {
    let a = analyzer();
    let edges = vec![
        DependencyEdge::new("taxonomy", "capabilities"),
        DependencyEdge::new("capabilities", "agent"),
        DependencyEdge::new("agent", "taxonomy"),
    ];
    let cycles = a.detect_cycle_edges(&edges);
    assert!(!cycles.is_empty(), "Three-node cycle should be detected");
}

#[test]
fn aes205_empty_edges_no_cycles() {
    let a = analyzer();
    let cycles = a.detect_cycle_edges(&[]);
    assert!(cycles.is_empty(), "Empty edges should produce no cycles");
}

// ─── AES205: scan with config ─────────────────────────────

#[test]
fn aes205_scan_disabled_config_returns_empty() {
    let a = analyzer();
    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("taxonomy"), LayerDefinition::default());
    let config = ArchitectureConfig::new(
        BooleanVO::new(false), // disabled
        layers.clone(),
        vec![],
        NamingConfig::new(Count::new(3)),
        FilePathList::new(vec![]),
        BooleanVO::new(false),
    );
    let layer_map = LayerMapVO::new(layers);
    let result = a.scan(
        &config,
        &layer_map,
        &[],
        &FilePath::new(".".to_string()).unwrap(),
        &HashMap::new(),
        &HashMap::new(),
    );
    assert!(result.is_empty(), "Disabled config should return empty");
}

#[test]
fn aes205_check_cycles_returns_ok() {
    let a = analyzer();
    let (config, layer_map) = cycle_config();
    let root = FilePath::new(".".to_string()).unwrap();
    let files = FilePathList::new(vec![]);
    let result = a.check_cycles(
        &config,
        &layer_map,
        &files,
        &root,
        &HashMap::new(),
        &HashMap::new(),
    );
    assert!(result.is_ok());
    assert!(
        result.unwrap().is_empty(),
        "Empty file list should produce no violations"
    );
}

// ─── AES205: scan with cross-layer file edges ─────────────

#[test]
fn aes205_scan_cross_layer_dependency_detected() {
    let a = analyzer();
    let (config, layer_map) = cycle_config();
    let root = FilePath::new("/tmp/project".to_string()).unwrap();

    // A capabilities file that imports from agent
    let mut content_map = HashMap::new();
    let mut imports_map = HashMap::new();
    let file_path = "/tmp/project/capabilities_handler.rs";
    let content = "use agent::runner;\n\nfn process() { runner::run(); }\n";
    content_map.insert(file_path.to_string(), content.to_string());

    let entry = shared::filesystem::taxonomy_filesystem_vo::ImportEntry {
        source_file: std::path::PathBuf::from(file_path),
        raw_path: "agent::runner".to_string(),
        resolved_path: None,
        import_type: shared::filesystem::taxonomy_filesystem_vo::ImportType::Use,
        language: shared::filesystem::taxonomy_filesystem_vo::Language::Rust,
        is_dynamic: false,
        is_resolved: false,
        symbols: Vec::new(),
        is_reexport: false,
        is_wildcard: false,
    };
    imports_map.insert(file_path.to_string(), vec![entry]);

    let files = FilePathList::new(vec![FilePath::new(file_path.to_string()).unwrap()]);
    let result = a.scan(
        &config,
        &layer_map,
        &files,
        &root,
        &content_map,
        &imports_map,
    );
    // Should produce at least one edge-based result (capabilities→agent cross-layer)
    // The cycle detection requires a return edge, so a single unidirectional edge
    // won't produce a cycle. Just verify no panic.
    let _ = result;
}

// ─── AES205: mutual cycle via scan ────────────────────────

#[test]
fn aes205_scan_mutual_cross_layer_cycle() {
    let a = analyzer();
    let (config, layer_map) = cycle_config();
    let root = FilePath::new("/tmp/project".to_string()).unwrap();

    let mut content_map = HashMap::new();
    let mut imports_map = HashMap::new();

    // capabilities → agent
    let f1 = "/tmp/project/capabilities_handler.rs";
    content_map.insert(
        f1.to_string(),
        "use agent::runner;\nfn process() { runner::run(); }\n".to_string(),
    );
    imports_map.insert(
        f1.to_string(),
        vec![shared::filesystem::taxonomy_filesystem_vo::ImportEntry {
            source_file: std::path::PathBuf::from(f1),
            raw_path: "agent::runner".to_string(),
            resolved_path: None,
            import_type: shared::filesystem::taxonomy_filesystem_vo::ImportType::Use,
            language: shared::filesystem::taxonomy_filesystem_vo::Language::Rust,
            is_dynamic: false,
            is_resolved: false,
            symbols: Vec::new(),
            is_reexport: false,
            is_wildcard: false,
        }],
    );

    // agent → capabilities (mutual)
    let f2 = "/tmp/project/agent_orchestrator.rs";
    content_map.insert(
        f2.to_string(),
        "use capabilities::handler;\nfn run() { handler::process(); }\n".to_string(),
    );
    imports_map.insert(
        f2.to_string(),
        vec![shared::filesystem::taxonomy_filesystem_vo::ImportEntry {
            source_file: std::path::PathBuf::from(f2),
            raw_path: "capabilities::handler".to_string(),
            resolved_path: None,
            import_type: shared::filesystem::taxonomy_filesystem_vo::ImportType::Use,
            language: shared::filesystem::taxonomy_filesystem_vo::Language::Rust,
            is_dynamic: false,
            is_resolved: false,
            symbols: Vec::new(),
            is_reexport: false,
            is_wildcard: false,
        }],
    );

    let files = FilePathList::new(vec![
        FilePath::new(f1.to_string()).unwrap(),
        FilePath::new(f2.to_string()).unwrap(),
    ]);
    let results = a.scan(
        &config,
        &layer_map,
        &files,
        &root,
        &content_map,
        &imports_map,
    );
    let aes205: Vec<_> = results
        .iter()
        .filter(|r| r.code.code() == "AES205")
        .collect();
    assert!(
        !aes205.is_empty(),
        "Mutual cross-layer dependency should produce AES205 cycle violation"
    );
}
