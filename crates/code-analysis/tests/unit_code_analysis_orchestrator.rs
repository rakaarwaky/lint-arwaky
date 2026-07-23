// PURPOSE: Unit tests for CodeAnalysisOrchestrator — agent layer orchestration,
// score calculation, critical check, report formatting.

use code_analysis_lint_arwaky::{has_critical, CodeAnalysisOrchestrator};
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;

fn orchestrator() -> CodeAnalysisOrchestrator {
    CodeAnalysisOrchestrator::new_with_defaults()
}

// ─── calc_score: Perfect score with no violations ────────────────────

#[test]
fn calc_score_perfect_with_no_violations() {
    let orch = orchestrator();
    let score = orch.calc_score(&[]);
    assert_eq!(score.value, 100.0);
}

// ─── calc_score: Deductions per severity ─────────────────────────────

#[test]
fn calc_score_deducts_per_severity() {
    let orch = orchestrator();
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES301", Severity::HIGH, "too large"),
        LintResult::new_arch("b.rs", 1, "AES304", Severity::CRITICAL, "bypass"),
    ];
    let score = orch.calc_score(&results);
    // HIGH = 3.0, CRITICAL = 5.0 → 100 - 8 = 92
    assert_eq!(score.value, 92.0);
}

#[test]
fn calc_score_clamped_at_zero() {
    let orch = orchestrator();
    let results: Vec<LintResult> = (0..30)
        .map(|i| {
            LintResult::new_arch(
                &format!("file_{}.rs", i),
                1,
                "AES304",
                Severity::CRITICAL,
                "bypass",
            )
        })
        .collect();
    let score = orch.calc_score(&results);
    assert_eq!(score.value, 0.0);
}

// ─── check_critical ──────────────────────────────────────────────────

#[test]
fn check_critical_true_when_critical_exists() {
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES301", Severity::HIGH, "msg"),
        LintResult::new_arch("b.rs", 1, "AES304", Severity::CRITICAL, "msg"),
    ];
    assert!(has_critical(&results));
}

#[test]
fn check_critical_false_when_no_critical() {
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES301", Severity::HIGH, "msg"),
        LintResult::new_arch("b.rs", 1, "AES302", Severity::MEDIUM, "msg"),
    ];
    assert!(!has_critical(&results));
}

#[test]
fn check_critical_false_for_empty() {
    assert!(!has_critical(&[]));
}

// ─── format_report ───────────────────────────────────────────────────

#[test]
fn format_report_contains_header() {
    let orch = orchestrator();
    let results = vec![LintResult::new_arch(
        "src/foo.rs",
        10,
        "AES304",
        Severity::CRITICAL,
        "unwrap detected",
    )];
    let list: Vec<LintResult> = results;
    let root = FilePath::new("/project".to_string()).unwrap();
    let report = orch.format_report(&list, &root);
    assert!(report.contains("AES Architecture Compliance Report"));
    assert!(report.contains("/project"));
    assert!(report.contains("Violations: 1"));
    assert!(report.contains("AES304"));
}

#[test]
fn format_report_empty_results() {
    let orch = orchestrator();
    let list: Vec<LintResult> = vec![];
    let root = FilePath::new("/project".to_string()).unwrap();
    let report = orch.format_report(&list, &root);
    assert!(report.contains("Violations: 0"));
}

// ─── active_rules ────────────────────────────────────────────────────

#[test]
fn active_rules_returns_configured_rules() {
    let orch = orchestrator();
    let rules = orch.active_rules();
    // Default config has no rules, so this should be empty.
    assert!(rules.is_empty());
}

// ─── run_code_analysis on non-existent path ──────────────────────────

#[test]
fn run_code_analysis_nonexistent_path_returns_empty() {
    let orch = orchestrator();
    let path = FilePath::new("/nonexistent/path/xyz".to_string()).unwrap();
    let results = orch.run_code_analysis_path(&path);
    assert!(results.is_empty());
}

// ─── lint_path public function ───────────────────────────────────────

#[test]
fn lint_path_nonexistent_returns_empty() {
    let results = code_analysis_lint_arwaky::lint_path("/nonexistent/path/xyz");
    assert!(results.is_empty());
}

// ─── Aggregate trait: run_code_analysis ──────────────────────────────

#[test]
fn aggregate_run_code_analysis_nonexistent() {
    let orch = orchestrator();
    let path = FilePath::new("/nonexistent/xyz".to_string()).unwrap();
    let result_list = orch.run_code_analysis(&path);
    assert!(result_list.is_empty());
}

// ─── Aggregate trait: run_code_analysis_dir ──────────────────────────

#[test]
fn aggregate_run_code_analysis_dir_nonexistent() {
    let orch = orchestrator();
    let path = FilePath::new("/nonexistent/dir".to_string()).unwrap();
    let result_list = orch.run_code_analysis_dir(&path);
    assert!(result_list.is_empty());
}