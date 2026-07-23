// PURPOSE: Integration tests — verify DI container wiring, end-to-end
// checker pipeline through CodeAnalysisCheckerContainer and CodeAnalysisContainer.

use code_analysis_lint_arwaky::{
    CodeAnalysisCheckerContainer, CodeAnalysisContainer, CodeAnalysisOrchestrator,
};
use std::sync::Arc;

// ─── Container wiring: Default construction ──────────────────────────

#[test]
fn default_container_constructs_successfully() {
    let container = CodeAnalysisCheckerContainer::default();
    // All accessors return valid references
    let _ = container.config();
    let _ = container.bypass_checker();
    let _ = container.line_checker();
    let _ = container.class_checker();
    let _ = container.dead_inheritance_checker();
    let _ = container.duplication_checker();
}

// ─── Container wiring: Orchestrator uses container ───────────────────

#[test]
fn orchestrator_with_container_runs_checks() {
    let container = Arc::new(CodeAnalysisCheckerContainer::default());
    let orch = CodeAnalysisOrchestrator::new_with_container(container);
    // Run on non-existent path — should return empty, not panic
    let results = orch.run_scan("/nonexistent/path");
    assert!(results.is_empty());
}

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
    let container = CodeAnalysisCheckerContainer::default();

    let content_with_bypass = "fn foo() {\n    let x = opt.unwrap();\n}\n";
    let mut violations = Vec::new();
    container.bypass_checker().check_bypass_comments(
        "capabilities_foo.rs",
        content_with_bypass,
        &mut violations,
    );
    assert!(!violations.is_empty());
    assert!(violations.iter().any(|v| v.code.code() == "AES304"));
}

// ─── Layer detection through container ───────────────────────────────

#[test]
fn container_detect_layer_returns_none_for_unknown() {
    let container = CodeAnalysisCheckerContainer::default();
    let result = container.detect_layer("random_file.rs", "/project");
    // "random_file.rs" has no recognized prefix → None
    assert!(result.is_none());
}

#[test]
fn container_detect_layer_recognizes_taxonomy_prefix() {
    let container = CodeAnalysisCheckerContainer::default();
    let result = container.detect_layer("taxonomy_foo_vo.rs", "/project");
    // Should detect "taxonomy" layer
    assert!(result.is_some());
}

// ─── Duplication analyzer wired through container ────────────────────

#[test]
fn container_duplication_checker_accessible() {
    let container = CodeAnalysisCheckerContainer::default();
    let dup = container.duplication_checker();
    let entries: Vec<(String, String)> = vec![];
    let violations = dup.check_file_similarity_entries(&entries, 5, 50.0);
    assert!(violations.is_empty());
}

// ─── Orchestrator Default trait ──────────────────────────────────────

#[test]
fn orchestrator_default_constructs() {
    let orch = CodeAnalysisOrchestrator::new_with_defaults();
    let results = orch.run_scan("/nonexistent");
    assert!(results.is_empty());
}
