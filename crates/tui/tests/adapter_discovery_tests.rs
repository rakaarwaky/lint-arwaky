use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use std::sync::Arc;
use tui_lint_arwaky::capabilities_lint_executor::{discover_adapters, LintExecutor};
use tui_lint_arwaky::capabilities_report_formatter::ReportFormatterHelper;

struct MockCodeAnalysis;
impl ICodeAnalysisAggregate for MockCodeAnalysis {
    fn run_code_analysis(&self, _path: &str) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn run_code_analysis_dir(&self, _path: &str) -> LintResultList {
        LintResultList::new(vec![])
    }
    fn run_code_analysis_path(&self, _path: &str) -> Vec<LintResult> {
        vec![]
    }
    fn calc_score(&self, _results: &[LintResult]) -> f64 {
        95.0
    }
    fn check_critical(&self, _results: &[LintResult]) -> bool {
        false
    }
    fn format_report(&self, _results: &LintResultList, _root: &str) -> String {
        "mock".into()
    }
    fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
        vec![]
    }
}

#[test]
fn test_discover_adapters_returns_builtins_and_externals() {
    let adapters = discover_adapters();

    // Must return at least the 3 built-in scanners + 8 external tools
    assert!(
        adapters.len() >= 11,
        "Expected at least 11 adapters, got {}",
        adapters.len()
    );

    // Built-in adapters are always installed
    let installed: Vec<_> = adapters.iter().filter(|a| a.installed).collect();
    assert!(
        installed.len() >= 3,
        "Expected at least 3 installed built-in adapters, got {}",
        installed.len()
    );

    // Verify known built-in names are present
    let names: Vec<&str> = adapters.iter().map(|a| a.name.as_str()).collect();
    assert!(names.contains(&"ast_rust_scanner"));
    assert!(names.contains(&"ast_py_scanner"));
    assert!(names.contains(&"ast_js_scanner"));

    // Verify external adapter names are present
    assert!(names.contains(&"clippy"));
    assert!(names.contains(&"ruff"));
    assert!(names.contains(&"mypy"));
    assert!(names.contains(&"eslint"));

    // Each adapter has a non-empty label
    for adapter in &adapters {
        assert!(
            !adapter.label.is_empty(),
            "Adapter '{}' has empty label",
            adapter.name
        );
    }
}

#[test]
fn test_discover_adapters_builtin_always_installed() {
    let adapters = discover_adapters();
    // Built-in scanners must always report installed = true
    let builtin_names = ["ast_rust_scanner", "ast_py_scanner", "ast_js_scanner"];
    for name in &builtin_names {
        let adapter = adapters
            .iter()
            .find(|a| a.name == *name)
            .unwrap_or_else(|| panic!("missing built-in adapter: {}", name));
        assert!(
            adapter.installed,
            "built-in '{}' should always be installed",
            name
        );
    }
}

#[test]
fn test_discover_adapters_deterministic() {
    // Calling twice should return the same results
    let first = discover_adapters();
    let second = discover_adapters();
    assert_eq!(first.len(), second.len());
    for (a, b) in first.iter().zip(second.iter()) {
        assert_eq!(a.name, b.name);
        assert_eq!(a.label, b.label);
        assert_eq!(a.installed, b.installed);
    }
}

#[test]
fn test_adapters_method_formats_output() {
    let executor = LintExecutor::new(Arc::new(MockCodeAnalysis), Arc::new(ReportFormatterHelper));
    let result = executor.adapters();
    assert!(result.success);
    assert!(result.output.contains("Active Linter Adapters"));
    assert!(result.output.contains("of"));
    assert!(result.output.contains("adapters available"));
}
