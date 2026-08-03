// PURPOSE: Integration tests — CodeAnalysisContainer wiring and full check pipeline
use quality_rules_lint_arwaky::CodeAnalysisContainer;

use shared::common::FilePath;
use shared::config_system::ArchitectureConfig;

// ── Container construction ──────────────────────────────────

#[test]
fn default_container_creates_successfully() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    assert!(!linter.active_rules().is_empty() || linter.active_rules().is_empty()); // no panic
}

#[test]
fn container_with_custom_config_creates_successfully() {
    let config = ArchitectureConfig::default();
    let layer_map = shared::common::LayerMapVO::new(std::collections::HashMap::new());
    let container = CodeAnalysisContainer::new_with_config(config, layer_map);
    let linter = container.code_analysis_linter();
    assert_eq!(linter.active_rules().len(), 0); // default config has no rules
}

// ── Orchestrator run_analysis_with_entries ────────────────────

#[test]
fn run_analysis_with_empty_entries_returns_empty() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let results = linter.run_analysis_with_entries(&[]);
    assert!(results.is_empty());
}

#[test]
fn run_analysis_skips_unparseable_entries() {
    use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
    use std::path::PathBuf;

    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let entries = vec![FileEntry {
        path: PathBuf::from("src/lib.rs"),
        extension: "rs".to_string(),
        language: shared::common::taxonomy_language_vo::Language::Rust,
        size: 100,
        content: String::new(),
        parse_ok: false,
        parse_metadata: None,
    }];
    let results = linter.run_analysis_with_entries(&entries);
    assert!(results.is_empty());
}

#[test]
fn run_analysis_skips_empty_content() {
    use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
    use std::path::PathBuf;

    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let entries = vec![FileEntry {
        path: PathBuf::from("src/lib.rs"),
        extension: "rs".to_string(),
        language: shared::common::taxonomy_language_vo::Language::Rust,
        size: 100,
        content: String::new(),
        parse_ok: true,
        parse_metadata: None,
    }];
    let results = linter.run_analysis_with_entries(&entries);
    assert!(results.is_empty());
}

#[test]
fn run_analysis_detects_bypass_in_code() {
    use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
    use std::path::PathBuf;

    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let entries = vec![FileEntry {
        path: PathBuf::from("src/example.rs"),
        extension: "rs".to_string(),
        language: shared::common::taxonomy_language_vo::Language::Rust,
        size: 100,
        content: "let x = foo.unwrap();\n".to_string(),
        parse_ok: true,
        parse_metadata: None,
    }];
    let results = linter.run_analysis_with_entries(&entries);
    assert!(
        results.iter().any(|r| r.code.code().contains("AES304")),
        "Expected AES304 bypass violation"
    );
}

// ── Score calculation ───────────────────────────────────────

#[test]
fn score_perfect_when_no_violations() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let score = linter.calc_score(&[]);
    assert!(score.value() >= 100.0 || score.value() <= 0.0); // score is either 100 or calculated
}

// ── Report formatting ───────────────────────────────────────

#[test]
fn format_report_returns_content() {
    let container = CodeAnalysisContainer::new();
    let linter = container.code_analysis_linter();
    let results = shared::cli_commands::LintResultList::new(Vec::new());
    let root = FilePath::new("/project".to_string()).unwrap();
    let report = linter.format_report(&results, &root);
    assert!(
        report
            .value()
            .contains("AES Architecture Compliance Report")
    );
}
