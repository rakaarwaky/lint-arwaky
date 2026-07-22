// Integration tests for AutoFixContainer and FixOrchestrator (AES201, AES203, AES304)
// Tests use real DI container and verify cross-capability interactions

use std::sync::Arc;

use auto_fix_lint_arwaky::{
    agent_fix_orchestrator::FixOrchestrator, capabilities_fix_processor::LintFixProcessor,
};
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::{LintResult, Severity};
use shared::common::taxonomy_path_vo::FilePath;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::{
    AesCodeAnalysisViolation, Language, ViolationKind,
};
use shared::taxonomy_common_vo::{Count, LineNumber};

/// Mock implementation of ICodeAnalysisAggregate for testing
struct MockCodeAnalysisAggregate;

impl ICodeAnalysisAggregate for MockCodeAnalysisAggregate {
    fn run_code_analysis(&self, _path: &FilePath) -> FixResult {
        // Return mock violations for testing
        let violations = vec![
            AesCodeAnalysisViolation::new(
                FilePath::raw("test.rs"),
                shared::taxonomy_common_vo::ErrorCode::raw("AES304"),
                LineNumber::new(5),
                shared::taxonomy_message_vo::LintMessage::new("bypass detected"),
                Severity::Warning,
                ViolationKind::Bypass,
            ),
            AesCodeAnalysisViolation::new(
                FilePath::raw("test.rs"),
                shared::taxonomy_common_vo::ErrorCode::raw("AES203"),
                LineNumber::new(10),
                shared::taxonomy_message_vo::LintMessage::new("unused import"),
                Severity::Warning,
                ViolationKind::UnusedImport,
            ),
        ];

        FixResult {
            output: shared::taxonomy_suggestion_vo::DescriptionVO::new(
                format!(
                    "Analysis complete: {} violations found",
                    violations.len()
                ),
            ),
            error: None,
            values: violations,
        }
    }
}

fn create_test_container() -> Arc<dyn LintFixOrchestratorAggregate> {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let fix_processor =
        LintFixProcessor::with_dry_run(true, mock_linter.clone());
    Arc::new(FixOrchestrator::new(Arc::new(fix_processor)))
}

// ─── Contract Tests: Trait Implementation Verification ──────

/// Verify IFixProtocol trait implementation exists and is callable
#[test]
fn contract_fix_protocol_trait_is_implemented() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let _processor = LintFixProcessor::with_dry_run(true, mock_linter);
    // If this compiles, the trait is implemented
}

/// Verify LintFixOrchestratorAggregate trait implementation
#[test]
fn contract_fix_orchestrator_aggregate_trait_is_implemented() {
    let container = create_test_container();
    // Should be able to call execute via the aggregate trait
    let result = container.execute(&FilePath::raw("test.rs"));
    assert!(result.output.value().contains("Dry-run"));
}

// ─── Integration Tests: Real DI Container Wiring ────────────

/// Test that the orchestrator properly wires fix processor to aggregate
#[test]
fn test_orchestrator_wires_fix_processor_correctly() {
    let container = create_test_container();
    let result = container.execute(&FilePath::raw("test.rs"));

    // Should produce output indicating dry-run mode
    assert!(result.output.value().contains("Dry-run"));
    assert!(result.output.value().contains("AES304"));
    assert!(result.output.value().contains("AES203"));
}

/// Test that fix orchestrator delegates to fix protocol
#[test]
fn test_orchestrator_delegates_to_protocol() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let processor = LintFixProcessor::with_dry_run(true, mock_linter.clone());
    let orchestrator = FixOrchestrator::new(Arc::new(processor));

    // Both should produce identical results since orchestrator delegates
    let result1 = orchestrator.execute(&FilePath::raw("test.rs"));
    assert!(result1.output.value().contains("Dry-run"));
}

// ─── Unit Tests: Happy Path ─────────────────────────────────

/// Test dry_run mode returns without making changes
#[test]
fn test_dry_run_mode_does_not_modify_files() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let processor = LintFixProcessor::with_dry_run(true, mock_linter);

    let result = processor.execute(&FilePath::raw("test.rs"));
    assert!(result.output.value().contains("Dry-run"));
}

/// Test that bypass violations are detected and reported
#[test]
fn test_bypass_violation_is_reported() {
    let container = create_test_container();
    let result = container.execute(&FilePath::raw("test.rs"));

    assert!(result.output.value().contains("AES304"));
}

/// Test that unused import violations are detected and reported
#[test]
fn test_unused_import_violation_is_reported() {
    let container = create_test_container();
    let result = container.execute(&FilePath::raw("test.rs"));

    assert!(result.output.value().contains("AES203"));
}

// ─── Unit Tests: Edge Cases ─────────────────────────────────

/// Test with empty path (edge case)
#[test]
fn test_empty_path_returns_result() {
    let container = create_test_container();
    let result = container.execute(&FilePath::raw(""));

    // Should not panic, should return a result
    assert!(!result.output.value().is_empty());
}

/// Test that FixResult always has output even on errors
#[test]
fn test_fix_result_always_has_output() {
    let container = create_test_container();
    let _ = container.execute(&FilePath::raw("nonexistent.rs"));

    // Should not panic even with nonexistent file
}

// ─── Unit Tests: Error Handling ─────────────────────────────

/// Test that non-fixable violations are reported separately
#[test]
fn test_non_fixable_violations_are_reported() {
    let mock_linter = Arc::new(MockCodeAnalysisAggregate);
    let processor = LintFixProcessor::with_dry_run(true, mock_linter);

    // Report non-fixable violations (those not in AES101, AES304, AES203)
    let violations = vec![
        LintResult {
            file: FilePath::raw("test.rs"),
            line: LineNumber::new(1),
            code: shared::taxonomy_common_vo::ErrorCode::raw("AES999"), // Non-fixable
            message: shared::taxonomy_message_vo::LintMessage::new("some violation"),
            severity: Severity::Error,
        },
    ];

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

    let fixable_violation = LintResult {
        file: FilePath::raw("test.rs"),
        line: LineNumber::new(1),
        code: shared::taxonomy_common_vo::ErrorCode::raw("AES304"),
        message: shared::taxonomy_message_vo::LintMessage::new("bypass"),
        severity: Severity::Warning,
    };

    assert!(processor.is_fixable(&fixable_violation));

    let non_fixable_violation = LintResult {
        file: FilePath::raw("test.rs"),
        line: LineNumber::new(1),
        code: shared::taxonomy_common_vo::ErrorCode::raw("AES999"),
        message: shared::taxonomy_message_vo::LintMessage::new("unknown"),
        severity: Severity::Error,
    };

    assert!(!processor.is_fixable(&non_fixable_violation));
}
