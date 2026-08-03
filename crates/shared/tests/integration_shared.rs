// PURPOSE: Integration tests — verify shared types compose correctly end-to-end.

use std::collections::HashMap;

use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_vo::{BooleanVO, ColumnNumber, Count, LineNumber, Score};
use shared::common::taxonomy_definition_vo::{LayerDefinition, LayerMapVO, NamingConfig};
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_layer_vo::LayerNameVO;
use shared::common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_lint_vo::{Location, LocationList, ScopeRef};
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use shared::common::utility_compliance_score::compute_score;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;

/// Build a full lint result chain: Config → LayerMap → LintResult.
#[test]
fn config_to_layermap_to_lintresult_chain() {
    // 1. Create architecture config with layers
    let mut layers = HashMap::new();
    layers.insert(
        LayerNameVO::new("taxonomy"),
        LayerDefinition::default(),
    );
    layers.insert(
        LayerNameVO::new("contract"),
        LayerDefinition::default(),
    );
    let config = ArchitectureConfig::new(
        BooleanVO::new(true),
        layers,
        Vec::new(),
        NamingConfig::new(Count::new(3)),
        FilePathList { values: vec![] },
        BooleanVO::new(false),
    );

    // 2. Extract layer names from config to build a LayerMapVO
    let layer_map = LayerMapVO::new(config.layers.clone());
    assert_eq!(layer_map.values.len(), 2);
    assert!(layer_map.values.contains_key(&LayerNameVO::new("taxonomy")));
    assert!(layer_map.values.contains_key(&LayerNameVO::new("contract")));

    // 3. Create a lint result referencing one of the configured layers
    let result = LintResult::new_arch(
        "src/taxonomy/my_vo.rs",
        10,
        "AES201",
        Severity::HIGH,
        "Surface layer imports directly from taxonomy layer",
    );
    assert_eq!(result.file.value(), "src/taxonomy/my_vo.rs");
    assert_eq!(result.code.code(), "AES201");
}

/// Verify VO interop: FilePath → LintResult → LintResultList → compute_score.
#[test]
fn vo_interop_filepath_to_lintresultlist_to_score() {
    let fp = FilePath::new("src/surface/scan.rs").unwrap();
    let scope = ScopeRef::new("scan_action");

    let mut result = LintResult {
        file: fp,
        line: LineNumber::new(42),
        column: ColumnNumber::new(5),
        code: ErrorCode::raw("AES401"),
        message: LintMessage::new("Surface must not import capabilities directly"),
        source: Some(AdapterName::raw("architecture")),
        severity: Severity::HIGH,
        enclosing_scope: Some(scope),
        related_locations: LocationList::new(),
    };

    // Build a list
    let mut list = LintResultList::new(Vec::new());
    list.push(result.clone());
    list.push(result.clone());

    assert_eq!(list.len(), 2);
    assert_eq!(list.iter().map(|r| r.code.code()).collect::<Vec<_>>(), vec!["AES401", "AES401"]);

    // Compute compliance score
    let score = compute_score(&list);
    // 2 × HIGH = 2 × 3 = 6 penalty → 94.0
    assert_eq!(score, 94.0);
}

/// Verify default config produces valid state for linting workflow.
#[test]
fn default_config_valid_for_linting_workflow() {
    let config = ArchitectureConfig::default();

    // Default config is enabled
    assert!(config.enabled.value);
    // No layers means no violations can come from layer checks
    let layer_map = LayerMapVO::new(config.layers.clone());
    assert!(layer_map.values.is_empty());

    // An empty results list should yield perfect score
    let empty_results: Vec<LintResult> = Vec::new();
    let score = compute_score(&empty_results);
    assert_eq!(score, 100.0);
}

/// Verify LocationList with related locations composes with LintResult.
#[test]
fn lintresult_with_related_locations() {
    let mut related = LocationList::new();
    related.push(Location {
        file: Some(FilePath::new("src/other.rs").unwrap()),
        line: Some(LineNumber::new(10)),
        column: Some(ColumnNumber::new(0)),
        description: DescriptionVO::new("imported here"),
    });

    let result = LintResult {
        file: FilePath::new("src/main.rs").unwrap(),
        line: LineNumber::new(5),
        column: ColumnNumber::new(10),
        code: ErrorCode::raw("AES301"),
        message: LintMessage::new("bypass detected"),
        source: None,
        severity: Severity::CRITICAL,
        enclosing_scope: None,
        related_locations: related,
    };

    assert_eq!(result.related_locations.len(), 1);
    let loc = &result.related_locations.values[0];
    assert_eq!(loc.file.as_ref().unwrap().value(), "src/other.rs");
    assert_eq!(loc.description.value, "imported here");
}

/// Verify LintResult identity is deterministic.
#[test]
fn lintresult_identity_deterministic() {
    let a = LintResult::new_arch("x.rs", 1, "AES101", Severity::Error, "msg");
    let b = LintResult::new_arch("x.rs", 1, "AES101", Severity::Error, "msg");
    assert_eq!(a.identity().value, b.identity().value);
}

/// Verify LintResult serializes/deserializes via serde.
#[test]
fn lintresult_serde_roundtrip() {
    let result = LintResult::new_arch("src/main.rs", 10, "AES201", Severity::HIGH, "test");
    let json = serde_json::to_string(&result).unwrap();
    let deserialized: LintResult = serde_json::from_str(&json).unwrap();
    assert_eq!(result.file, deserialized.file);
    assert_eq!(result.line, deserialized.line);
    assert_eq!(result.code, deserialized.code);
    assert_eq!(result.severity, deserialized.severity);
}

/// Verify ArchitectureConfig serde roundtrip.
#[test]
fn architecture_config_serde_roundtrip() {
    let config = ArchitectureConfig::default();
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: ArchitectureConfig = serde_json::from_str(&json).unwrap();
    assert_eq!(config.enabled, deserialized.enabled);
    assert_eq!(config.layers, deserialized.layers);
}

/// Verify Score interacts with Severity in a realistic workflow.
#[test]
fn score_severity_workflow() {
    let mut score = Score::new(100.0);
    // Simulate deducting for each violation severity
    score = score.deduct(&Severity::LOW);
    score = score.deduct(&Severity::MEDIUM);
    score = score.deduct(&Severity::HIGH);
    score = score.deduct(&Severity::CRITICAL);
    // 100 - 1 - 2 - 3 - 5 = 89.0
    assert_eq!(score.value, 89.0);
    assert!(score.is_passing(&Score::new(80.0)));
    assert!(!score.is_passing(&Score::new(90.0)));
}

/// Verify FilePath normalization feeds into LintResult correctly.
#[test]
fn filepath_normalization_in_lintresult() {
    // FilePath normalizes backslashes
    let fp = FilePath::new("src\\main.rs").unwrap();
    assert_eq!(fp.value(), "src/main.rs");

    let result = LintResult::new_arch(&fp.value, 1, "AES101", Severity::Error, "test");
    assert_eq!(result.file.value(), "src/main.rs");
}

/// Verify LintResultList push + iter + len work together.
#[test]
fn lintresultlist_mutation_and_iteration() {
    let mut list = LintResultList::new(Vec::new());
    assert!(list.is_empty());

    for i in 0..5 {
        list.push(LintResult::new_arch(
            &format!("file_{}.rs", i),
            (i + 1) as usize,
            "AES101",
            Severity::MEDIUM,
            &format!("violation {}", i),
        ));
    }

    assert_eq!(list.len(), 5);
    let codes: Vec<&str> = list.iter().map(|r| r.code.code()).collect();
    assert_eq!(codes, vec!["AES101", "AES101", "AES101", "AES101", "AES101"]);
}

/// Verify LayerDefinition default and custom fields.
#[test]
fn layer_definition_customization() {
    let mut ld = LayerDefinition::default();
    ld.allowed = shared::common::taxonomy_common_vo::PatternList::new(vec!["*_vo.rs"]);
    ld.forbidden = shared::common::taxonomy_common_vo::PatternList::new(vec!["*_test.rs"]);
    ld.word_count = Count::new(2);

    assert_eq!(ld.allowed.len(), 1);
    assert_eq!(ld.forbidden.len(), 1);
    assert_eq!(ld.word_count.value, 2);
}
