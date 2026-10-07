// PURPOSE: Integration tests — verify shared types compose correctly end-to-end.

use std::collections::HashMap;

use shared_common::taxonomy_common_vo::{BooleanVO, ColumnNumber, Count, LineNumber, Score};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_layer_vo::LayerNameVO;
use shared_common::taxonomy_layer_vo::{LayerDefinition, LayerMapVO, NamingConfig};
use shared_common::taxonomy_lint_vo::{LintResult, LintResultList};
use shared_common::taxonomy_lint_vo::{Location, LocationList};
use shared_common::taxonomy_message_vo::DescriptionVO;
use shared_common::taxonomy_message_vo::LintMessage;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_path_vo::FilePathList;
use shared_common::taxonomy_severity_vo::Severity;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;

/// Build a full lint result chain: Config → LayerMap → LintResult.
#[test]
fn config_to_layermap_to_lintresult_chain() {
    // 1. Create architecture config with layers
    let mut layers = HashMap::new();
    layers.insert(LayerNameVO::new("taxonomy"), LayerDefinition::default());
    layers.insert(LayerNameVO::new("contract"), LayerDefinition::default());
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
        violation_name: String::new(),
        why: String::new(),
        fix: String::new(),
    };

    assert_eq!(result.related_locations.len(), 1);
    let loc = &result.related_locations.values[0];
    assert_eq!(loc.file.as_ref().unwrap().value(), "src/other.rs");
    assert_eq!(loc.description.value, "imported here");
}

/// Verify LintResult identity is deterministic.
#[test]
fn lintresult_identity_deterministic() {
    let a = LintResult::new_arch("x.rs", 1, "AES101", Severity::CRITICAL, "msg");
    let b = LintResult::new_arch("x.rs", 1, "AES101", Severity::CRITICAL, "msg");
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

    let result = LintResult::new_arch(&fp.value, 1, "AES101", Severity::CRITICAL, "test");
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
            format!("violation {}", i),
        ));
    }

    assert_eq!(list.len(), 5);
    let codes: Vec<&str> = list.iter().map(|r| r.code.code()).collect();
    assert_eq!(
        codes,
        vec!["AES101", "AES101", "AES101", "AES101", "AES101"]
    );
}

/// Verify LayerDefinition default and custom fields.
#[test]
fn layer_definition_customization() {
    let ld = LayerDefinition {
        allowed: shared_common::taxonomy_common_vo::PatternList::new(vec!["*_vo.rs"]),
        forbidden: shared_common::taxonomy_common_vo::PatternList::new(vec!["*_test.rs"]),
        word_count: Count::new(2),
        ..Default::default()
    };

    assert_eq!(ld.allowed.len(), 1);
    assert_eq!(ld.forbidden.len(), 1);
    assert_eq!(ld.word_count.value, 2);
}
