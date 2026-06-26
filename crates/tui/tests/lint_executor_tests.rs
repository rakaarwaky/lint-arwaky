use tui_lint_arwaky::capabilities_lint_executor::LintExecutor;
use tui_lint_arwaky::contract_lint_executor_protocol::ILintExecutorProtocol;
use tui_lint_arwaky::taxonomy_action_flags_vo::ActionFlags;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::LineNumber;
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_lint_vo::ScopeRef;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_suggestion_vo::DescriptionVO;
use std::sync::Arc;

struct MockCodeAnalysis { results: LintResultList, score: f64, critical: bool }
impl MockCodeAnalysis {
    fn empty() -> Self { Self { results: LintResultList::new(vec![]), score: 95.0, critical: false } }
    fn with_violations(count: usize, critical: bool) -> Self {
        let results: Vec<LintResult> = (0..count).map(|i| LintResult {
            file: FilePath::new(format!("file_{}.rs", i)).unwrap_or_default(),
            line: LineNumber::new(1), column: Default::default(),
            code: ErrorCode::raw("TEST001"), message: LintMessage::new(format!("violation {}", i)),
            source: None,
            severity: if critical && i == 0 { Severity::CRITICAL } else { Severity::LOW },
            enclosing_scope: Some(ScopeRef { name: DescriptionVO::new(String::new()), kind: DescriptionVO::new(String::new()), file: None, start_line: None, end_line: None }),
            related_locations: Default::default(),
        }).collect();
        Self { results: LintResultList::new(results), score: if critical { 50.0 } else { 85.0 }, critical }
    }
}
impl ICodeAnalysisAggregate for MockCodeAnalysis {
    fn run_code_analysis(&self, _path: &str) -> LintResultList { self.results.clone() }
    fn run_code_analysis_dir(&self, _path: &str) -> LintResultList { self.results.clone() }
    fn run_code_analysis_path(&self, _path: &str) -> Vec<LintResult> { self.results.values.clone() }
    fn calc_score(&self, _results: &[LintResult]) -> f64 { self.score }
    fn check_critical(&self, _results: &[LintResult]) -> bool { self.critical }
    fn format_report(&self, _results: &LintResultList, _root: &str) -> String { "mock report".to_string() }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> { vec![] }
}

fn make_executor(mock: MockCodeAnalysis) -> LintExecutor { LintExecutor::new(Arc::new(mock)) }

#[test]
fn test_check_with_no_violations() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let flags = ActionFlags::default();
    let result = executor.check("/root", &flags);
    assert!(result.success);
    assert_eq!(result.violation_count, 0);
    assert!(result.output.contains("No violations found"));
}

#[test]
fn test_check_with_violations() {
    let executor = make_executor(MockCodeAnalysis::with_violations(3, false));
    let flags = ActionFlags::default();
    let result = executor.check("/root", &flags);
    assert!(result.success);
    assert_eq!(result.violation_count, 3);
    assert!(result.output.contains("3 violation"));
}

#[test]
fn test_scan() {
    let executor = make_executor(MockCodeAnalysis::with_violations(2, false));
    let result = executor.scan("/root");
    assert!(result.success);
    assert_eq!(result.violation_count, 2);
}

#[test]
fn test_fix_dry_run() {
    let executor = make_executor(MockCodeAnalysis::with_violations(5, false));
    let mut flags = ActionFlags::default();
    flags.dry_run = true;
    let result = executor.fix("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("DRY-RUN"));
}

#[test]
fn test_fix_live() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let flags = ActionFlags::default();
    let result = executor.fix("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("LIVE"));
}

#[test]
fn test_ci_pass() {
    let executor = make_executor(MockCodeAnalysis { score: 90.0, critical: false, ..MockCodeAnalysis::empty() });
    let mut flags = ActionFlags::default();
    flags.threshold = 80;
    let result = executor.ci("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("PASS"));
}

#[test]
fn test_ci_fail_low_score() {
    let executor = make_executor(MockCodeAnalysis { score: 50.0, critical: false, ..MockCodeAnalysis::empty() });
    let mut flags = ActionFlags::default();
    flags.threshold = 80;
    let result = executor.ci("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("FAIL"));
}

#[test]
fn test_ci_fail_critical() {
    let executor = make_executor(MockCodeAnalysis { score: 95.0, critical: true, ..MockCodeAnalysis::empty() });
    let mut flags = ActionFlags::default();
    flags.threshold = 80;
    let result = executor.ci("/root", &flags);
    assert!(result.success);
    assert!(result.output.contains("FAIL"));
}

#[test]
fn test_orphan() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.orphan("/root");
    assert!(result.success);
    assert!(result.output.contains("Orphan detection"));
}

#[test]
fn test_security() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.security("/root");
    assert!(result.success);
    assert!(result.output.contains("Security scan"));
}

#[test]
fn test_doctor() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.doctor();
    assert!(result.success);
    assert!(result.output.contains("Environment Diagnostics"));
}

#[test]
fn test_version() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let result = executor.version();
    assert!(result.success);
    assert!(result.output.contains("Lint Arwaky"));
}

#[test]
fn test_format_results_empty() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let results = LintResultList::new(vec![]);
    let output = executor.format_results(&results);
    assert_eq!(output, "No violations found.");
}

#[test]
fn test_format_results_with_violations() {
    let executor = make_executor(MockCodeAnalysis::empty());
    let results = LintResultList::new(vec![LintResult {
        file: FilePath::new("test.rs".to_string()).unwrap_or_default(),
        line: LineNumber::new(10), column: Default::default(),
        code: ErrorCode::raw("E001"), message: LintMessage::new("test message"),
        source: None, severity: Severity::LOW, enclosing_scope: None,
        related_locations: Default::default(),
    }]);
    let output = executor.format_results(&results);
    assert!(output.contains("1 violation"));
    assert!(output.contains("test.rs:10"));
    assert!(output.contains("test message"));
}
