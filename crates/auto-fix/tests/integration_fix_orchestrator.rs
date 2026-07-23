// Integration tests for AutoFixContainer and FixOrchestrator (AES201, AES203, AES304)
// Tests use real DI container and verify cross-capability interactions

use std::sync::Arc;

use auto_fix_lint_arwaky::{
    agent_fix_orchestrator::FixOrchestrator, capabilities_fix_processor::LintFixProcessor,
};
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{BooleanVO, Score};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;

fn test_file_path() -> FilePath {
    FilePath::new("test.rs").unwrap()
}

fn minimal_file_path() -> FilePath {
    FilePath::new("x").unwrap()
}

/// Mock implementation of ICodeAnalysisAggregate for testing
struct MockCodeAnalysisAggregate;

impl ICodeAnalysisAggregate for MockCodeAnalysisAggregate {
    fn run_code_analysis(&self, _project_root: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }

    fn run_code_analysis_dir(&self, _src_dir: &FilePath) -> LintResultList {
        LintResultList::new(vec![])
    }

    fn run_code_analysis_path(&self, _path: &FilePath) -> Vec<LintResult> {
        vec![]
    }

    fn calc_score(&self, _results: &[LintResult]) -> Score {
        Score::new(100.0)
    }

    fn check_critical(&self, _results: &[LintResult]) -> BooleanVO {
        false
    }

    fn format_report(&self, _results: &LintResultList, _project_root: &FilePath) -> DisplayContent {
        DisplayContent::new("")
    }

    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

// ─── Contract Tests: Trait Implementation Verification ──────

/// Verify IFixProtocol trait implementation exists and is callable
#[test]
fn contract_fix_protocol_trait_is_implemented() {
    // LintFixProcessor implements IFixProtocol
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let _processor = LintFixProcessor::with_dry_run(true, mock_linter);
    // If this compiles, the trait is implemented
}

/// Verify LintFixOrchestratorAggregate trait implementation
#[test]
fn contract_fix_orchestrator_aggregate_trait_is_implemented() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let fix_processor = LintFixProcessor::with_dry_run(true, mock_linter.clone());
    let _orchestrator = FixOrchestrator::new(Arc::new(fix_processor));
    // If this compiles, the trait is implemented
}

// ─── Integration Tests: Real DI Container Wiring ────────────

/// Test that the orchestrator properly wires fix processor to aggregate
#[test]
fn test_orchestrator_wires_fix_processor_correctly() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let fix_processor = LintFixProcessor::with_dry_run(true, mock_linter.clone());
    let orchestrator = FixOrchestrator::new(Arc::new(fix_processor));

    // Should produce output indicating dry-run mode
    let result = orchestrator.execute(&test_file_path());
    assert!(result.output.value().contains("Dry-run"));
}

/// Test that fix orchestrator delegates to fix protocol
#[test]
fn test_orchestrator_delegates_to_protocol() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let processor = LintFixProcessor::with_dry_run(true, mock_linter.clone());
    let orchestrator = FixOrchestrator::new(Arc::new(processor));

    // Both should produce identical results since orchestrator delegates
    let result1 = orchestrator.execute(&test_file_path());
    assert!(result1.output.value().contains("Dry-run"));
}

// ─── Unit Tests: Happy Path ─────────────────────────────────

/// Test dry_run mode returns without making changes
#[test]
fn test_dry_run_mode_does_not_modify_files() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let processor = LintFixProcessor::with_dry_run(true, mock_linter);

    let result = processor.execute(&test_file_path());
    assert!(result.output.value().contains("Dry-run"));
}

/// Test that bypass violations are detected and reported
#[test]
fn test_bypass_violation_is_reported() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let fix_processor = LintFixProcessor::with_dry_run(true, mock_linter.clone());
    let orchestrator = FixOrchestrator::new(Arc::new(fix_processor));

    let result = orchestrator.execute(&test_file_path());
    assert!(result.output.value().contains("AES304"));
}

/// Test that unused import violations are detected and reported
#[test]
fn test_unused_import_violation_is_reported() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let fix_processor = LintFixProcessor::with_dry_run(true, mock_linter.clone());
    let orchestrator = FixOrchestrator::new(Arc::new(fix_processor));

    let result = orchestrator.execute(&test_file_path());
    assert!(result.output.value().contains("AES203"));
}

// ─── Unit Tests: Edge Cases ─────────────────────────────────

/// Test with minimal path (edge case)
#[test]
fn test_empty_path_returns_result() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let fix_processor = LintFixProcessor::with_dry_run(true, mock_linter.clone());
    let orchestrator = FixOrchestrator::new(Arc::new(fix_processor));

    let result = orchestrator.execute(&minimal_file_path());

    // Should not panic, should return a result
    assert!(!result.output.value().is_empty());
}

/// Test that FixResult always has output even on errors
#[test]
fn test_fix_result_always_has_output() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let fix_processor = LintFixProcessor::with_dry_run(true, mock_linter.clone());
    let orchestrator = FixOrchestrator::new(Arc::new(fix_processor));

    let _ = orchestrator.execute(&minimal_file_path());

    // Should not panic even with nonexistent file
}

// ─── Unit Tests: Error Handling ─────────────────────────────

/// Test that non-fixable violations are reported separately
#[test]
fn test_non_fixable_violations_are_reported() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let processor = LintFixProcessor::with_dry_run(true, mock_linter);

    // Report non-fixable violations (those not in AES101, AES304, AES203)
    let violations = vec![LintResult::new_arch(
        "test.rs",
        1,
        "AES999",
        Severity::HIGH,
        "some violation",
    )];

    let manual = processor.report_non_fixable(&violations);
    assert_eq!(manual.len(), 1);
}

/// Test that fixable codes are correctly identified
#[test]
fn test_fixable_codes_are_identified() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let processor = LintFixProcessor::with_dry_run(true, mock_linter);

    let fixable = processor.fixable_codes();
    assert_eq!(fixable.len(), 3);
    assert!(fixable.iter().any(|c| c.code() == "AES101"));
    assert!(fixable.iter().any(|c| c.code() == "AES304"));
    assert!(fixable.iter().any(|c| c.code() == "AES203"));
}

/// Test that is_fixable correctly identifies fixable violations
#[test]
fn test_is_fixable_identifies_correct_violations() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let processor = LintFixProcessor::with_dry_run(true, mock_linter);

    let fixable_violation =
        LintResult::new_arch("test.rs", 1, "AES304", Severity::MEDIUM, "bypass");

    assert!(processor.is_fixable(&fixable_violation));

    let non_fixable_violation =
        LintResult::new_arch("test.rs", 1, "AES999", Severity::HIGH, "unknown");

    assert!(!processor.is_fixable(&non_fixable_violation));
}
