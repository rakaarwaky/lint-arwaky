// PURPOSE: E2E tests — full fix pipeline on real temp files.
// Exercises: container → orchestrator → processor → file I/O → result.

use auto_fix_lint_arwaky::root_auto_fix_container::AutoFixContainer;
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::common::taxonomy_common_vo::Score;
use shared::common::taxonomy_path_vo::FilePath;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use std::io::Write;
use std::sync::Arc;
use tempfile::NamedTempFile;

// ─── Configurable mock linter ─────────────────────────────

struct ConfigurableLinter {
    results: Vec<LintResult>,
}

impl ICodeAnalysisAggregate for ConfigurableLinter {
    fn run_code_analysis(&self, _: &FilePath) -> LintResultList {
        LintResultList::new(self.results.clone())
    }
    fn run_code_analysis_dir(&self, _: &FilePath) -> LintResultList {
        LintResultList::new(self.results.clone())
    }
    fn run_code_analysis_path(&self, _: &FilePath) -> Vec<LintResult> {
        self.results.clone()
    }
    fn calc_score(&self, _: &[LintResult]) -> Score { Score::new(80.0) }
    fn check_critical(&self, _: &[LintResult]) -> bool { false }
    fn format_report(&self, _: &LintResultList, _: &FilePath) -> String { String::new() }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> { vec![] }
}

// ─── E2E: Unused import removal ───────────────────────────

#[test]
fn e2e_unused_import_removed_from_file() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    writeln!(tmp, "use std::fs;").unwrap();
    writeln!(tmp, "").unwrap();
    writeln!(tmp, "fn main() {{").unwrap();
    writeln!(tmp, "    let _ = std::fs::read(\"x\");").unwrap();
    writeln!(tmp, "}}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let violations = vec![
        LintResult::new_arch(&file_path, 1, "AES203", Severity::Warning, "unused import std::io"),
    ];

    let linter = ConfigurableLinter { results: violations };
    let container = AutoFixContainer::new(Arc::new(linter));
    let orch = container.orchestrator(false);

    let path = FilePath::new(file_path.clone()).unwrap();
    let result = orch.execute(&path);

    // Verify fix was applied
    assert!(result.output.value().contains("Fixed"));
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("use std::io;"));
    assert!(content.contains("use std::fs;"));
    assert!(content.contains("fn main()"));
}

// ─── E2E: Bypass comment removal ──────────────────────────

#[test]
fn e2e_bypass_comment_removed_from_file() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "#[allow(unused_variables)]").unwrap();
    writeln!(tmp, "fn main() {{").unwrap();
    writeln!(tmp, "    let x = 42;").unwrap();
    writeln!(tmp, "}}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let violations = vec![
        LintResult::new_arch(&file_path, 1, "AES304", Severity::Warning, "bypass comment detected"),
    ];

    let linter = ConfigurableLinter { results: violations };
    let container = AutoFixContainer::new(Arc::new(linter));
    let orch = container.orchestrator(false);

    let path = FilePath::new(file_path.clone()).unwrap();
    let result = orch.execute(&path);

    assert!(result.output.value().contains("Fixed"));
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("#[allow(unused_variables)]"));
    assert!(content.contains("fn main()"));
}

// ─── E2E: Dry-run does not modify ─────────────────────────

#[test]
fn e2e_dry_run_preserves_file_content() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let original_content = std::fs::read_to_string(&file_path).unwrap();

    let violations = vec![
        LintResult::new_arch(&file_path, 1, "AES203", Severity::Warning, "unused import"),
    ];

    let linter = ConfigurableLinter { results: violations };
    let container = AutoFixContainer::new(Arc::new(linter));
    let orch = container.orchestrator(true); // dry_run = true

    let path = FilePath::new(file_path.clone()).unwrap();
    let result = orch.execute(&path);

    assert!(result.output.value().contains("Dry-run"));
    let after_content = std::fs::read_to_string(&file_path).unwrap();
    assert_eq!(original_content, after_content);
}

// ─── E2E: Multiple violation types in one pass ────────────

#[test]
fn e2e_multiple_violations_fixed_in_single_pass() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "use std::io;").unwrap();
    writeln!(tmp, "#[allow(dead_code)]").unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let violations = vec![
        LintResult::new_arch(&file_path, 1, "AES203", Severity::Warning, "unused import"),
        LintResult::new_arch(&file_path, 2, "AES304", Severity::Warning, "bypass comment"),
    ];

    let linter = ConfigurableLinter { results: violations };
    let container = AutoFixContainer::new(Arc::new(linter));
    let orch = container.orchestrator(false);

    let path = FilePath::new(file_path.clone()).unwrap();
    let result = orch.execute(&path);

    assert!(result.output.value().contains("Fixed"));
    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(!content.contains("use std::io;"));
    assert!(!content.contains("#[allow(dead_code)]"));
    assert!(content.contains("fn main()"));
}

// ─── E2E: Non-fixable violations reported ─────────────────

#[test]
fn e2e_non_fixable_violations_included_in_report() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "fn main() {{}}").unwrap();
    tmp.flush().unwrap();

    let file_path = tmp.path().to_str().unwrap().to_string();
    let violations = vec![
        LintResult::new_arch(&file_path, 1, "AES305", Severity::Warning, "dead inheritance"),
    ];

    let linter = ConfigurableLinter { results: violations };
    let container = AutoFixContainer::new(Arc::new(linter));
    let orch = container.orchestrator(false);

    let path = FilePath::new(file_path.clone()).unwrap();
    let result = orch.execute(&path);

    assert!(result.output.value().contains("No automatic fixes"));
    assert!(result.output.value().contains("AES305"));
}
