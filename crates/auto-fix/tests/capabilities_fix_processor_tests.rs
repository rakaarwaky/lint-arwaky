use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::LintMessage;
use std::sync::Arc;

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

struct MockLinter;
impl ICodeAnalysisAggregate for MockLinter {
    fn run_code_analysis(&self, _path: &FilePath) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_dir(&self, _src_dir: &FilePath) -> LintResultList {
        LintResultList::default()
    }
    fn run_code_analysis_path(&self, _path: &FilePath) -> Vec<LintResult> {
        vec![]
    }
    fn calc_score(&self, _results: &[LintResult]) -> f64 {
        0.0
    }
    fn check_critical(&self, _results: &[LintResult]) -> bool {
        false
    }
    fn format_report(&self, _results: &LintResultList, _project_root: &FilePath) -> String {
        String::new()
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

#[test]
fn fixable_codes_includes_expected() {
    let processor = LintFixProcessor::with_dry_run(
        true,
        Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>,
    );
    let codes = processor.fixable_codes();
    let strings: Vec<String> = codes.iter().map(|c| c.code().to_string()).collect();
    assert!(strings.contains(&"AES101".to_string()));
    assert!(strings.contains(&"AES304".to_string()));
    assert!(strings.contains(&"AES203".to_string()));
}

#[test]
fn is_fixable_true_for_known_codes() {
    let processor = LintFixProcessor::with_dry_run(
        true,
        Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>,
    );
    assert!(processor.is_fixable(&make_result("AES101")));
    assert!(!processor.is_fixable(&make_result("AES102")));
}

#[test]
fn report_non_fixable_filters_correctly() {
    let processor = LintFixProcessor::with_dry_run(
        true,
        Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>,
    );
    let violations = vec![
        make_result("AES101"),
        make_result("AES500"),
        make_result("AES203"),
    ];
    let manual = processor.report_non_fixable(&violations);
    assert_eq!(manual.len(), 1);
    assert!(manual[0].to_string().contains("AES500"));
}

#[test]
fn is_fixable_false_for_unknown_code() {
    let processor = LintFixProcessor::with_dry_run(
        true,
        Arc::new(MockLinter) as Arc<dyn ICodeAnalysisAggregate>,
    );
    assert!(!processor.is_fixable(&make_result("AES999")));
}
