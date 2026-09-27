// PURPOSE: Smoke tests — quick boot + respond within 5s
use quality_rules_lint_arwaky::CodeAnalysisContainer;
use quality_rules_lint_arwaky::contains_critical_severity;

use shared::cli_commands::LintResult;
use shared::common::Severity;
use shared::quality_rules::CodeAnalysisRequest;

#[test]
fn container_creation_smoke() {
    let _container = CodeAnalysisContainer::new();
}

#[test]
fn orchestrator_creation_smoke() {
    let container = CodeAnalysisContainer::new();
    let _linter = container.code_analysis_linter();
}

#[test]
fn basic_check_on_simple_file() {
    use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
    use std::path::PathBuf;

    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let entries = vec![FileEntry {
        path: PathBuf::from("src/simple.rs"),
        extension: "rs".to_string(),
        language: shared::common::taxonomy_language_vo::Language::Rust,
        size: 50,
        content: "fn main() {}\n".to_string(),
        parse_ok: true,
        parse_metadata: None,
    }];
    let results = linter
        .execute(CodeAnalysisRequest::run_analysis(&entries))
        .into_violations();
    // Just verify it doesn't panic and returns a Vec
    let _count = results.len();
}

#[test]
fn score_calculation_smoke() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let results = vec![LintResult::new_arch(
        "src/lib.rs",
        1,
        "AES304",
        Severity::CRITICAL,
        "test violation",
    )];
    let score = linter
        .execute(CodeAnalysisRequest::calc_score(&results))
        .into_score();
    assert!(score.value() >= 0.0 && score.value() <= 100.0);
}

#[test]
fn critical_severity_detection_smoke() {
    let results_with_critical = vec![LintResult::new_arch(
        "src/lib.rs",
        1,
        "AES304",
        Severity::CRITICAL,
        "critical issue",
    )];
    assert!(contains_critical_severity(&results_with_critical));

    let results_without_critical = vec![LintResult::new_arch(
        "src/lib.rs",
        1,
        "AES302",
        Severity::LOW,
        "minor issue",
    )];
    assert!(!contains_critical_severity(&results_without_critical));
}
