use code_analysis_lint_arwaky::agent_code_analysis_orchestrator::{
    detect_source_dir, has_critical, resolve_target, CodeAnalysisOrchestrator,
};
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;


// ─── detect_source_dir ──────────────────────────────────────────────────────

#[test]
fn detects_crates_dir() {
    // Navigate from crate dir up to workspace root
    let crate_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_dir.parent().and_then(|p| p.parent()).unwrap_or(crate_dir);
    let result = detect_source_dir(workspace_root);
    let name = result.file_name().and_then(|n| n.to_str()).unwrap_or("");
    assert_eq!(name, "crates", "expected 'crates' dir from workspace root, got: {name}");
}

#[test]
fn falls_back_to_project_root_when_no_source_dir() {
    let tmp = std::env::temp_dir();
    let result = detect_source_dir(&tmp);
    assert_eq!(result, tmp);
}

// ─── resolve_target ─────────────────────────────────────────────────────────

#[test]
fn resolve_target_some_path() {
    assert_eq!(resolve_target(Some("src".to_string())), "src");
}

#[test]
fn resolve_target_none_returns_dot() {
    assert_eq!(resolve_target(None), ".");
}

// ─── has_critical ───────────────────────────────────────────────────────────

#[test]
fn has_critical_true_when_critical_exists() {
    let results = vec![
        LintResult::new_arch("f.rs", 0, "AES001", Severity::CRITICAL, "critical!"),
    ];
    assert!(has_critical(&results));
}

#[test]
fn has_critical_false_when_only_high() {
    let results = vec![
        LintResult::new_arch("f.rs", 0, "AES001", Severity::HIGH, "high"),
    ];
    assert!(!has_critical(&results));
}

#[test]
fn has_critical_false_for_empty() {
    assert!(!has_critical(&[]));
}

#[test]
fn has_critical_detects_critical_among_others() {
    let results = vec![
        LintResult::new_arch("f.rs", 0, "AES001", Severity::LOW, "low"),
        LintResult::new_arch("f.rs", 1, "AES002", Severity::CRITICAL, "critical!"),
        LintResult::new_arch("f.rs", 2, "AES003", Severity::MEDIUM, "medium"),
    ];
    assert!(has_critical(&results));
}

// ─── format_report ──────────────────────────────────────────────────────────

#[test]
fn format_report_empty_no_panic() {
    let orch = CodeAnalysisOrchestrator::new();
    let report = orch.format_report(&[], "test_project");
    assert!(report.contains("Compliance Report"));
    assert!(report.contains("0"));
}

#[test]
fn format_report_with_violations() {
    let orch = CodeAnalysisOrchestrator::new();
    let results = vec![
        LintResult::new_arch("f.rs", 5, "AES301", Severity::HIGH, "file too large"),
    ];
    let report = orch.format_report(&results, "test_project");
    assert!(report.contains("AES301"));
    assert!(report.contains("f.rs"));
    assert!(report.contains("1"));
}

// ─── Orchestrator construction ──────────────────────────────────────────────

#[test]
fn orchestrator_can_be_constructed() {
    let orch = CodeAnalysisOrchestrator::new();
    let _ = orch;
}

#[test]
fn orchestrator_scan_nonexistent_dir_returns_empty() {
    let orch = CodeAnalysisOrchestrator::new();
    let results = orch.run_scan("/nonexistent/path/12345");
    assert!(results.is_empty());
}
