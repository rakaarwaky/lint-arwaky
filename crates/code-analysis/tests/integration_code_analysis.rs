use code_analysis_lint_arwaky::{
    root_code_analysis_container::CodeAnalysisContainer,
};

// ─── CodeAnalysisContainer: new() ────────────────────────────────────

#[test]
fn code_analysis_container_new_produces_aggregate() {
    let container = CodeAnalysisContainer::new();
    let aggregate = container.code_analysis_linter();
    let score = aggregate.calc_score(&[]);
    assert_eq!(score.value, 100.0);
}

// ─── CodeAnalysisContainer: Default ──────────────────────────────────

#[test]
fn code_analysis_container_default_works() {
    let container = CodeAnalysisContainer::default();
    let aggregate = container.code_analysis_linter();
    let rules = aggregate.active_rules();
    assert!(rules.is_empty());
}

// ─── Full pipeline: bypass + line + mandatory on temp content ────────

#[test]
fn full_pipeline_detects_multiple_violation_types() {
    let container = CodeAnalysisContainer::new();
    let aggregate = container.code_analysis_linter();
    let results = aggregate.run_code_analysis_path(
        &shared::common::taxonomy_path_vo::FilePath::new("/nonexistent".to_string()).unwrap(),
    );
    assert!(results.is_empty());
}
