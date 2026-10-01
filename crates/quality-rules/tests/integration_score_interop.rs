// PURPOSE: Integration tests — shared VO interop that terminates in compute_score.
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_common::taxonomy_definition_vo::LayerMapVO;
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_result_vo::{LintResult, LintResultList};
use shared_common::taxonomy_lint_vo::{LocationList, ScopeRef};
use shared_common::taxonomy_message_vo::LintMessage;
use shared_common::taxonomy_path_vo::FilePath;
use shared_common::taxonomy_severity_vo::Severity;
use shared_config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared_quality_rules::utility_compliance_score::compute_score;

/// Verify VO interop: FilePath → LintResult → LintResultList → compute_score.
#[test]
fn vo_interop_filepath_to_lintresultlist_to_score() {
    let fp = FilePath::new("src/surface/scan.rs").unwrap();
    let scope = ScopeRef::new("scan_action");

    let result = LintResult {
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
    assert_eq!(
        list.iter().map(|r| r.code.code()).collect::<Vec<_>>(),
        vec!["AES401", "AES401"]
    );

    // Compute compliance score
    let score = compute_score(&list.values);
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
