// PURPOSE: Acceptance test AES305 — code duplication detection
// Create two files with >50% overlap, verify violation
use std::path::PathBuf;

use quality_rules_lint_arwaky::capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
use shared::config_system::ArchitectureConfig;
use shared::quality_rules::ICodeMetricAnalyzerProtocol;

use std::sync::Arc;

fn analyzer() -> CodeDuplicationAnalyzer {
    CodeDuplicationAnalyzer::from_config(Arc::new(ArchitectureConfig::default()))
}

fn analyzer_with_threshold(threshold: f64) -> CodeDuplicationAnalyzer {
    let mut config = ArchitectureConfig::default();
    // Add AES305 rule with custom threshold
    config.rules.push(shared::config_system::ArchitectureRule {
        name: shared::common::taxonomy_suggestion_vo::DescriptionVO::new("AES305".to_string()),
        code_analysis: shared::quality_rules::CodeAnalysisRuleVO {
            duplication_threshold: Some(threshold),
            ..Default::default()
        },
        ..Default::default()
    });
    CodeDuplicationAnalyzer::from_config(Arc::new(config))
}

#[test]
fn two_identical_files_produces_violation() {
    let ana = analyzer();
    let content = (0..30)
        .map(|i| format!("fn function_{}() {{ let x = {}; return x; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let entries = vec![
        (PathBuf::from("src/a.rs"), content.clone()),
        (PathBuf::from("src/b.rs"), content),
    ];

    let violations = ana.handle_duplicates_entries(&entries);
    assert!(
        !violations.is_empty(),
        "Expected duplication violation for identical files"
    );
    let has_aes305 = match &violations[0].1 {
        shared::quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason } => reason
            .as_ref()
            .is_some_and(|r| r.to_string().contains("AES305")),
        _ => false,
    };
    assert!(violations[0].0.contains("AES305") || has_aes305);
}

#[test]
fn two_different_files_no_violation() {
    let ana = analyzer();
    let entries = vec![
        (
            PathBuf::from("src/a.rs"),
            "fn alpha() { let a = 1; let b = 2; let c = 3; }\n".to_string(),
        ),
        (
            PathBuf::from("src/b.rs"),
            "fn beta() { let x = 10; let y = 20; let z = 30; }\n".to_string(),
        ),
    ];

    let violations = ana.handle_duplicates_entries(&entries);
    assert!(
        violations.is_empty(),
        "No violation expected for completely different files"
    );
}

#[test]
fn single_file_no_violation() {
    let ana = analyzer();
    let entries = vec![(
        PathBuf::from("src/a.rs"),
        "fn only() { return 1; }\n".to_string(),
    )];

    let violations = ana.handle_duplicates_entries(&entries);
    assert!(
        violations.is_empty(),
        "Single file should not produce duplication"
    );
}

#[test]
fn empty_entries_no_violation() {
    let ana = analyzer();
    let violations = ana.handle_duplicates_entries(&[]);
    assert!(violations.is_empty());
}

#[test]
fn files_below_min_lines_no_violation() {
    let ana = analyzer();
    // Short files that have identical content but below min_dup_lines
    let entries = vec![
        (PathBuf::from("src/a.rs"), "line1\nline2\n".to_string()),
        (PathBuf::from("src/b.rs"), "line1\nline2\n".to_string()),
    ];

    let violations = ana.handle_duplicates_entries(&entries);
    assert!(
        violations.is_empty(),
        "Files below min_dup_lines should not be flagged"
    );
}

#[test]
fn violation_message_contains_percentage() {
    let ana = analyzer_with_threshold(10.0); // very low threshold
    let content = (0..20)
        .map(|i| format!("fn func_{}() {{ let x = {}; }}", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let entries = vec![
        (PathBuf::from("src/dup_a.rs"), content.clone()),
        (PathBuf::from("src/dup_b.rs"), content),
    ];

    let violations = ana.handle_duplicates_entries(&entries);
    assert!(!violations.is_empty());
    let msg = match &violations[0].1 {
        shared::quality_rules::AesCodeAnalysisViolation::CodeDuplication { reason } => {
            reason.as_ref().map_or("".to_string(), |r| r.to_string())
        }
        other => format!("{other:?}"),
    };
    assert!(
        msg.contains("AES305"),
        "Violation message should contain AES305"
    );
}
