// PURPOSE: Unit tests for LintFixProcessor — auto-fix capability.
// Covers: execute, fix_bypass_comments, fix_unused_import, emit_fix_event,
//         report_non_fixable, is_fixable, fixable_codes.

use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::contract_fix_protocol::IFixProtocol;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::common::taxonomy_common_vo::{BooleanVO, Count, LineNumber, Score};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::common::taxonomy_error_vo::ErrorCode;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use std::io::Write;
use std::sync::Arc;
use tempfile::NamedTempFile;

// ─── Mock ICodeAnalysisAggregate ──────────────────────────

struct MockLinter {
    results: LintResultList,
}

impl MockLinter {
    fn empty() -> Self {
        Self {
            results: LintResultList::new(vec![]),
        }
    }

    fn with_violations(results: Vec<LintResult>) -> Self {
        Self {
            results: LintResultList::new(results),
        }
    }
}

impl ICodeAnalysisAggregate for MockLinter {
    fn run_code_analysis(&self, _project_root: &FilePath) -> LintResultList {
        self.results.clone()
    }

    fn run_code_analysis_dir(&self, _src_dir: &FilePath) -> LintResultList {
        self.results.clone()
    }

    fn run_code_analysis_path(&self, _path: &FilePath) -> Vec<LintResult> {
        self.results.values.clone()
    }

    fn calc_score(&self, _results: &[LintResult]) -> Score {
        Score::new(100.0)
    }

    fn check_critical(&self, _results: &[LintResult]) -> BooleanVO {
        BooleanVO::new(false)
    }

    fn format_report(&self, _results: &LintResultList, _project_root: &FilePath) -> DisplayContent {
        DisplayContent::new("mock report")
    }

    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

fn make_violation(file: &str, line: usize, code: &str, msg: &str) -> LintResult {
    LintResult::new_arch(file, line, code, Severity::LOW, msg)
}

fn sut_with_linter(linter: MockLinter) -> LintFixProcessor {
    LintFixProcessor::new(Arc::new(linter))
}

fn sut_dry_run(linter: MockLinter) -> LintFixProcessor {
    LintFixProcessor::with_dry_run(true, Arc::new(linter))
}

// ─── fixable_codes ────────────────────────────────────────

#[test]
fn fixable_codes_returns_aes101_aes304_aes203() {
    let sut = sut_with_linter(MockLinter::empty());
    let codes = sut.fixable_codes();
    let code_strs: Vec<&str> = codes.iter().map(|c| c.code()).collect();
    assert!(code_strs.contains(&"AES101"));
    assert!(code_strs.contains(&"AES304"));
    assert!(code_strs.contains(&"AES203"));
    assert_eq!(codes.len(), 3);
}

// ─── is_fixable ───────────────────────────────────────────

#[test]
fn is_fixable_true_for_aes203() {
    let sut = sut_with_linter(MockLinter::empty());
    let v = make_violation("test.rs", 1, "AES203", "unused import");
    assert!(sut.is_fixable(&v));
}

#[test]
fn is_fixable_true_for_aes304() {
    let sut = sut_with_linter(MockLinter::empty());
    let v = make_violation("test.rs", 1, "AES304", "bypass comment");
    assert!(sut.is_fixable(&v));
}

#[test]
fn is_fixable_true_for_aes101() {
    let sut = sut_with_linter(MockLinter::empty());
    let v = make_violation("test.rs", 1, "AES101", "naming violation");
    assert!(sut.is_fixable(&v));
}

#[test]
fn is_fixable_false_for_aes305() {
    let sut = sut_with_linter(MockLinter::empty());
    let v = make_violation("test.rs", 1, "AES305", "dead inheritance");
    assert!(!sut.is_fixable(&v));
}

#[test]
fn is_fixable_false_for_unknown_code() {
    let sut = sut_with_linter(MockLinter::empty());
    let v = make_violation("test.rs", 1, "XYZ999", "unknown");
    assert!(!sut.is_fixable(&v));
}

// ─── report_non_fixable ───────────────────────────────────

#[test]
fn report_non_fixable_filters_fixable_codes() {
    let sut = sut_with_linter(MockLinter::empty());
    let violations = vec![
        make_violation("a.rs", 1, "AES203", "unused import"),
        make_violation("b.rs", 5, "AES305", "dead inheritance"),
        make_violation("c.rs", 10, "AES304", "bypass"),
    ];
    let manual = sut.report_non_fixable(&violations);
    assert_eq!(manual.len(), 1);
    assert!(manual[0].value().contains("AES305"));
}

#[test]
fn report_non_fixable_empty_when_all_fixable() {
    let sut = sut_with_linter(MockLinter::empty());
    let violations = vec![
        make_violation("a.rs", 1, "AES203", "unused"),
        make_violation("b.rs", 2, "AES304", "bypass"),
    ];
    let manual = sut.report_non_fixable(&violations);
    assert!(manual.is_empty());
}

#[test]
fn report_non_fixable_all_manual_when_none_fixable() {
    let sut = sut_with_linter(MockLinter::empty());
    let violations = vec![
        make_violation("a.rs", 1, "AES305", "dead"),
        make_violation("b.rs", 2, "AES402", "primitive"),
    ];
    let manual = sut.report_non_fixable(&violations);
    assert_eq!(manual.len(), 2);
}

// ─── emit_fix_event ───────────────────────────────────────

#[test]
fn emit_fix_event_returns_fix_applied_with_correct_fields() {
    let sut = sut_with_linter(MockLinter::empty());
    let path = FilePath::new("src/main.rs".to_string()).unwrap();
    let code = ErrorCode::raw("AES203");
    let count = Count::new(3);

    let event = sut.emit_fix_event(&path, code, count);
    assert_eq!(event.path, path);
    assert_eq!(event.error_code, ErrorCode::raw("AES203"));
    assert_eq!(event.changes_count, Count::new(3));
    assert_eq!(event.adapter.value(), "lint-fix-orchestrator");
}

// ─── fix_unused_import ────────────────────────────────────

#[test]
fn fix_unused_import_removes_use_line() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    writeln!(tmp, "use std::fs;").unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    tmp.flush().unwrap();

    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_unused_import(tmp.path().to_str().unwrap(), LineNumber::new(1));
    assert!(result);

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert!(!content.contains("use std::io;"));
    assert!(content.contains("use std::fs;"));
    assert!(content.contains("fn main()"));
}

#[test]
fn fix_unused_import_removes_python_import_line() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "import os").unwrap();
    writeln!(tmp, "import sys").unwrap();
    writeln!(tmp, "print('hello')").unwrap();
    tmp.flush().unwrap();

    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_unused_import(tmp.path().to_str().unwrap(), LineNumber::new(1));
    assert!(result);

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert!(!content.contains("import os"));
    assert!(content.contains("import sys"));
}

#[test]
fn fix_unused_import_returns_false_for_non_import_line() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    tmp.flush().unwrap();

    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_unused_import(tmp.path().to_str().unwrap(), LineNumber::new(1));
    assert!(!result);
}

#[test]
fn fix_unused_import_returns_false_for_nonexistent_file() {
    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_unused_import("/nonexistent/file.rs", LineNumber::new(1));
    assert!(!result);
}

#[test]
fn fix_unused_import_returns_false_for_line_zero() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    tmp.flush().unwrap();

    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_unused_import(tmp.path().to_str().unwrap(), LineNumber::new(0));
    assert!(!result);
}

#[test]
fn fix_unused_import_returns_false_for_out_of_bounds_line() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    tmp.flush().unwrap();

    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_unused_import(tmp.path().to_str().unwrap(), LineNumber::new(999));
    assert!(!result);
}

// ─── fix_bypass_comments ──────────────────────────────────

#[test]
fn fix_bypass_removes_allow_attribute_line() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "#[allow(unused)]").unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    tmp.flush().unwrap();

    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_bypass_comments(tmp.path().to_str().unwrap(), LineNumber::new(1));
    assert!(result);

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert!(!content.contains("#[allow(unused)]"));
    assert!(content.contains("fn main()"));
}

#[test]
fn fix_bypass_removes_noqa_comment() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "x = 1  # noqa").unwrap();
    writeln!(tmp, "y = 2").unwrap();
    tmp.flush().unwrap();

    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_bypass_comments(tmp.path().to_str().unwrap(), LineNumber::new(1));
    assert!(result);
}

#[test]
fn fix_bypass_returns_false_for_non_bypass_line() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    tmp.flush().unwrap();

    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_bypass_comments(tmp.path().to_str().unwrap(), LineNumber::new(1));
    assert!(!result);
}

#[test]
fn fix_bypass_returns_false_for_nonexistent_file() {
    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_bypass_comments("/nonexistent/file.rs", LineNumber::new(1));
    assert!(!result);
}

#[test]
fn fix_bypass_replaces_unwrap_with_expect() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "let x = val.unwrap();").unwrap();
    tmp.flush().unwrap();

    let sut = sut_with_linter(MockLinter::empty());
    let result = sut.fix_bypass_comments(tmp.path().to_str().unwrap(), LineNumber::new(1));
    assert!(result);

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert!(content.contains("expect(\"safe\")"));
    assert!(!content.contains("unwrap()"));
}

// ─── execute (dry_run) ────────────────────────────────────

#[test]
fn execute_dry_run_reports_without_modifying() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let violations = vec![make_violation(
        &file_path,
        1,
        "AES203",
        "unused import std::io",
    )];
    let linter = MockLinter::with_violations(violations);
    let sut = sut_dry_run(linter);

    let path = FilePath::new(file_path.clone()).unwrap();
    let result = sut.execute(&path);

    assert!(result.output.value().contains("Dry-run"));
    // File should NOT be modified
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("use std::io;"));
}

// ─── execute (no violations) ──────────────────────────────

#[test]
fn execute_no_violations_reports_no_fixes() {
    let linter = MockLinter::empty();
    let sut = sut_with_linter(linter);

    let path = FilePath::new("src/main.rs".to_string()).unwrap();
    let result = sut.execute(&path);

    assert!(result.output.value().contains("No automatic fixes applied"));
    assert!(result.is_success());
}

// ─── execute (with unused import fix) ─────────────────────

#[test]
fn execute_fixes_unused_import() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let violations = vec![make_violation(
        &file_path,
        1,
        "AES203",
        "unused import std::io",
    )];
    let linter = MockLinter::with_violations(violations);
    let sut = sut_with_linter(linter);

    let path = FilePath::new(file_path.clone()).unwrap();
    let result = sut.execute(&path);

    assert!(result.output.value().contains("Fixed"));
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("use std::io;"));
}

// ─── with_dry_run constructor ─────────────────────────────

#[test]
fn with_dry_run_true_does_not_modify_files() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let violations = vec![make_violation(&file_path, 1, "AES203", "unused")];
    let linter = MockLinter::with_violations(violations);
    let sut = LintFixProcessor::with_dry_run(true, Arc::new(linter));

    let path = FilePath::new(file_path.clone()).unwrap();
    let _ = sut.execute(&path);

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("use std::io;"));
}
