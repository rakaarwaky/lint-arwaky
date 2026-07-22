// PURPOSE: Unit tests for FixOrchestrator — agent layer delegation.
// Covers: execute, run_fix, manual_report.

use auto_fix_lint_arwaky::agent_fix_orchestrator::FixOrchestrator;
use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::auto_fix::taxonomy_fix_applied_event::FixApplied;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{Count, LineNumber, Score};
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use std::sync::Arc;

// ─── Mock IFixProtocol ────────────────────────────────────

struct MockFixProtocol;

impl IFixProtocol for MockFixProtocol {
    fn execute(&self, _path: &FilePath) -> FixResult {
        FixResult::new(
            shared::common::taxonomy_suggestion_vo::DescriptionVO::new(
                "mock fix result".to_string(),
            ),
            None,
        )
    }

    fn fix_bypass_comments(&self, _file_path: &str, _line: LineNumber) -> bool {
        true
    }

    fn fix_unused_import(&self, _file_path: &str, _line: LineNumber) -> bool {
        true
    }

    fn emit_fix_event(&self, path: &FilePath, error_code: ErrorCode, changes: Count) -> FixApplied {
        FixApplied::new(
            path.clone(),
            shared::common::taxonomy_adapter_name_vo::AdapterName::raw("mock"),
            error_code,
            changes,
        )
    }

    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage> {
        violations
            .iter()
            .filter(|v| v.code.code() == "AES305")
            .map(|v| LintMessage::new(format!("{}: {}", v.code, v.message)))
            .collect()
    }

    fn is_fixable(&self, violation: &LintResult) -> bool {
        violation.code.code() != "AES305"
    }

    fn fixable_codes(&self) -> &[ErrorCode] {
        &[ErrorCode::raw("AES203"), ErrorCode::raw("AES304")]
    }
}

fn sut() -> FixOrchestrator {
    FixOrchestrator::new(Arc::new(MockFixProtocol))
}

// ─── execute (aggregate trait) ────────────────────────────

#[test]
fn execute_delegates_to_fix_protocol() {
    let path = FilePath::new("src/main.rs".to_string()).unwrap();
    let result = sut().execute(&path);
    assert_eq!(result.output.value(), "mock fix result");
    assert!(result.is_success());
}

// ─── run_fix ──────────────────────────────────────────────

#[test]
fn run_fix_delegates_to_fix_protocol() {
    let path = FilePath::new("src/lib.rs".to_string()).unwrap();
    let result = sut().run_fix(&path);
    assert_eq!(result.output.value(), "mock fix result");
}

// ─── manual_report ────────────────────────────────────────

#[test]
fn manual_report_returns_non_fixable_violations() {
    let violations = vec![
        LintResult::new_arch("a.rs", 1, "AES203", Severity::LOW, "unused"),
        LintResult::new_arch("b.rs", 5, "AES305", Severity::LOW, "dead inheritance"),
    ];
    let report = sut().manual_report(&violations);
    assert_eq!(report.len(), 1);
    assert!(report[0].contains("AES305"));
}

#[test]
fn manual_report_empty_when_all_fixable() {
    let violations = vec![
        LintResult::new_arch("a.rs", 1, "AES203", Severity::LOW, "unused"),
        LintResult::new_arch("b.rs", 2, "AES304", Severity::LOW, "bypass"),
    ];
    let report = sut().manual_report(&violations);
    assert!(report.is_empty());
}

#[test]
fn manual_report_empty_for_empty_input() {
    let report = sut().manual_report(&[]);
    assert!(report.is_empty());
}

// ─── Constructor ──────────────────────────────────────────

#[test]
fn new_accepts_arc_dyn_ifixprotocol() {
    let protocol: Arc<dyn IFixProtocol> = Arc::new(MockFixProtocol);
    let orchestrator = FixOrchestrator::new(protocol);
    let path = FilePath::new("test.rs".to_string()).unwrap();
    let result = orchestrator.execute(&path);
    assert!(result.is_success());
}
