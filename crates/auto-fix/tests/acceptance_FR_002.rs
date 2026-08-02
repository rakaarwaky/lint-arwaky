// PURPOSE: Acceptance test — FR-002: Bypass Comment Removal (AES304)
// Requirement: Remove or replace invalid bypass comments (#[allow(...)], unwrap(), noqa, type: ignore, panic!).
// Scope: Python (ruff, mypy) and JavaScript/TypeScript (eslint), Rust (allow attrs).

use auto_fix_lint_arwaky::capabilities_fix_processor::LintFixProcessor;
use shared::auto_fix::IFixProtocol;
use shared::cli_commands::{LintResult, LintResultList};
use shared::code_analysis::{CodeAnalysisRuleVO, ICodeAnalysisAggregate};

use shared::common::{DisplayContent, FilePath};
use shared::common::{LineNumber, Score};

use std::io::Write;
use std::sync::Arc;
use tempfile::NamedTempFile;

use shared::common::BooleanVO;

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
    fn check_critical(&self, _: &[LintResult]) -> BooleanVO {
        BooleanVO::new(false)
    }
    fn format_report(&self, _: &LintResultList, _: &FilePath) -> DisplayContent {
        DisplayContent::new("")
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
    fn run_analysis_with_entries(
        &self,
        _files: &[shared::filesystem::taxonomy_filesystem_vo::FileEntry],
    ) -> Vec<shared::cli_commands::LintResult> {
        vec![]
    }

    }

    fn collect_file_entries(&self, _: &[String]) -> Vec<(std::path::PathBuf, String)> {
        vec![]
    }
}

fn sut() -> LintFixProcessor {
    LintFixProcessor::new(Arc::new(MockLinter { results: vec![] }))
}

/// FRD-BYPASS-01: Rust `#[allow(...)]` attribute removed.
#[test]
fn frd_rust_allow_attribute_removed() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "#[allow(unused_variables)]").unwrap();
    writeln!(tmp, "fn helper() -> i32 {{ 42 }}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let result = sut().fix_bypass_comments(&file_path, LineNumber::new(1));
    assert!(result.is_applied());

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("#[allow"));
    assert!(content.contains("fn helper()"));
}

/// FRD-BYPASS-02: Python `# noqa` comment line handled.
#[test]
fn frd_python_noqa_comment_handled() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "x = 1  # noqa").unwrap();
    writeln!(tmp, "y = 2").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let result = sut().fix_bypass_comments(&file_path, LineNumber::new(1));
    assert!(result.is_applied());
}

/// FRD-BYPASS-03: Python `# type: ignore` comment handled.
#[test]
fn frd_python_type_ignore_handled() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "result = func()  # type: ignore").unwrap();
    writeln!(tmp, "print(result)").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let result = sut().fix_bypass_comments(&file_path, LineNumber::new(1));
    assert!(result.is_applied());
}

/// FRD-BYPASS-04: `.unwrap()` replaced with `.expect("safe")`.
#[test]
fn frd_rust_unwrap_replaced_with_expect() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "let val = some_result.unwrap();").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let result = sut().fix_bypass_comments(&file_path, LineNumber::new(1));
    assert!(result.is_applied());

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("expect(\"safe\")"));
    assert!(!content.contains("unwrap()"));
}

/// FRD-BYPASS-05: Non-bypass lines are not modified.
#[test]
fn frd_non_bypass_line_untouched() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "let x = 42;").unwrap();
    writeln!(tmp, "println!(\"{{}}\", x);").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let result = sut().fix_bypass_comments(&file_path, LineNumber::new(1));
    assert!(!result.is_applied(), "Non-bypass line must not be modified");

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("let x = 42;"));
}

/// FRD-BYPASS-06: Fix does not break surrounding code.
#[test]
fn frd_bypass_fix_preserves_surrounding_code() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "fn setup() {{}}").unwrap();
    writeln!(tmp, "#[allow(dead_code)]").unwrap();
    writeln!(tmp, "fn helper() -> i32 {{ 42 }}").unwrap();
    writeln!(tmp, "fn main() {{ helper(); }}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let result = sut().fix_bypass_comments(&file_path, LineNumber::new(2));
    assert!(result.is_applied());

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("fn setup()"));
    assert!(content.contains("fn helper()"));
    assert!(content.contains("fn main()"));
    assert!(!content.contains("#[allow(dead_code)]"));
}
