use auto_fix_lint_arwaky::agent_fix_orchestrator::FixOrchestrator;
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::Count;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::LintMessage;
use std::sync::Arc;

struct MockFixProtocol;

fn make_result(code: &str) -> LintResult {
    LintResult {
        file: FilePath::new("test.rs".to_string()).unwrap_or_default(),
        line: LineNumber::new(1),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new("test"),
        source: None,
        severity: Severity::HIGH,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}

impl IFixProtocol for MockFixProtocol {
    fn execute(&self, _path: &FilePath) -> FixResult {
        FixResult {
            output: shared::taxonomy_suggestion_vo::DescriptionVO::new("mock fix executed"),
            error: None,
        }
    }

    fn fix_bypass_comments(&self, _file_path: &str, _line: LineNumber) -> bool {
        true
    }

    fn fix_unused_import(&self, _file_path: &str, _line: LineNumber) -> bool {
        true
    }

    fn emit_fix_event(
        &self,
        _path: &FilePath,
        _error_code: ErrorCode,
        _changes: Count,
    ) -> shared::auto_fix::taxonomy_fix_applied_event::FixApplied {
        shared::auto_fix::taxonomy_fix_applied_event::FixApplied::new(
            _path.clone(),
            shared::taxonomy_adapter_name_vo::AdapterName::raw("mock"),
            _error_code,
            _changes,
        )
    }

    fn report_non_fixable(&self, violations: &[LintResult]) -> Vec<LintMessage> {
        violations
            .iter()
            .map(|v| LintMessage::new(format!("MANUAL: {}", v.code.to_string())))
            .collect()
    }

    fn is_fixable(&self, _violation: &LintResult) -> bool {
        false
    }

    fn fixable_codes(&self) -> &[ErrorCode] {
        &[]
    }
}

#[test]
fn orchestrator_can_be_constructed() {
    let orch = FixOrchestrator::new(Arc::new(MockFixProtocol));
    let _ = orch;
}

#[test]
fn run_fix_delegates_to_protocol() {
    let orch = FixOrchestrator::new(Arc::new(MockFixProtocol));
    let path = FilePath::new("test.rs".to_string()).unwrap_or_default();
    let result = orch.run_fix(&path);
    assert!(result.output.to_string().contains("mock fix executed"));
    assert!(result.error.is_none());
}

#[test]
fn execute_trait_delegates_to_protocol() {
    let orch = FixOrchestrator::new(Arc::new(MockFixProtocol));
    let path = FilePath::new("test.rs".to_string()).unwrap_or_default();
    let result = LintFixOrchestratorAggregate::execute(&orch, &path);
    assert!(result.output.to_string().contains("mock fix executed"));
}

#[test]
fn manual_report_filters_violations() {
    let orch = FixOrchestrator::new(Arc::new(MockFixProtocol));
    let violations = vec![make_result("AES101"), make_result("AES500")];
    let report = orch.manual_report(&violations);
    assert_eq!(report.len(), 2);
    assert!(report[0].contains("AES101"));
    assert!(report[1].contains("AES500"));
}

#[test]
fn manual_report_empty_for_no_violations() {
    let orch = FixOrchestrator::new(Arc::new(MockFixProtocol));
    let report = orch.manual_report(&[]);
    assert!(report.is_empty());
}
