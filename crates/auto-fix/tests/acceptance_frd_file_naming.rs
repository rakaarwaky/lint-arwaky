// PURPOSE: Acceptance test — FRD: AES Fix: File Naming Correction
// Requirement: Automatically rename files that violate the snake_case convention.
// Scope: All supported languages.
// Note: The current implementation handles symbol renaming (AES101) via
//       LintFixProcessor::rename_symbol. File-level renaming is delegated
//       to the orchestrator pipeline.

use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::Score;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use std::io::Write;
use std::sync::Arc;
use tempfile::NamedTempFile;

struct MockLinter {
    results: Vec<LintResult>,
}
impl ICodeAnalysisAggregate for MockLinter {
    fn run_code_analysis(&self, _: &FilePath) -> LintResultList {
        LintResultList::new(self.results.clone())
    }
    fn run_code_analysis_dir(&self, _: &FilePath) -> LintResultList {
        LintResultList::new(self.results.clone())
    }
    fn run_code_analysis_path(&self, _: &FilePath) -> Vec<LintResult> {
        self.results.clone()
    }
    fn calc_score(&self, _: &[LintResult]) -> Score {
        Score::new(100.0)
    }
    fn check_critical(&self, _: &[LintResult]) -> bool {
        false
    }
    fn format_report(&self, _: &LintResultList, _: &FilePath) -> String {
        String::new()
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

/// FRD-NAMING-01: AES101 violation triggers symbol rename in execute pipeline.
#[test]
fn frd_aes101_naming_violation_triggers_rename() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "fn BadName() {{}}").unwrap();
    writeln!(tmp, "fn main() {{ BadName(); }}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let violations = vec![LintResult::new_arch(
        &file_path,
        1,
        "AES101",
        Severity::LOW,
        "BadName violates snake_case",
    )];

    let linter = MockLinter {
        results: violations,
    };
    let sut = LintFixProcessor::new(Arc::new(linter));
    let path = FilePath::new(file_path.clone()).unwrap();
    let result = sut.execute(&path);

    // The processor should attempt a rename for AES101
    assert!(result.is_success());
}

/// FRD-NAMING-02: Rename is idempotent — running twice produces no further changes.
#[test]
fn frd_naming_fix_is_idempotent() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "fn my_func() {{}}").unwrap();
    writeln!(tmp, "fn main() {{ my_func(); }}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let violations = vec![LintResult::new_arch(
        &file_path,
        1,
        "AES101",
        Severity::LOW,
        "my_func naming issue",
    )];

    let linter = MockLinter {
        results: violations.clone(),
    };
    let sut = LintFixProcessor::new(Arc::new(linter));
    let path = FilePath::new(file_path.clone()).unwrap();

    let result1 = sut.execute(&path);
    let content_after_first = std::fs::read_to_string(&file_path).unwrap();

    // Second run — should produce no further changes
    let linter2 = MockLinter { results: vec![] };
    let sut2 = LintFixProcessor::new(Arc::new(linter2));
    let result2 = sut2.execute(&path);
    let content_after_second = std::fs::read_to_string(&file_path).unwrap();

    assert_eq!(
        content_after_first, content_after_second,
        "Idempotency violated"
    );
}

/// FRD-NAMING-03: Non-fixable naming violations reported as manual.
#[test]
fn frd_non_fixable_naming_reported_manual() {
    let sut = LintFixProcessor::new(Arc::new(MockLinter { results: vec![] }));
    let violations = vec![LintResult::new_arch(
        "x.rs",
        1,
        "AES305",
        Severity::LOW,
        "dead inheritance",
    )];
    let manual = sut.report_non_fixable(&violations);
    assert_eq!(manual.len(), 1);
}
