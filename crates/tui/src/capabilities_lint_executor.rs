use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_action_flags_vo::ActionFlags;
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
use std::sync::Arc;

pub struct LintExecutor {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
}

impl LintExecutor {
    pub fn new(code_analysis: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self { code_analysis }
    }

    fn format_results(&self, results: &LintResultList) -> String {
        if results.is_empty() {
            return "No violations found.".to_string();
        }
        let mut output = format!("Found {} violation(s):\n\n", results.len());
        for (i, result) in results.iter().enumerate() {
            output.push_str(&format!(
                "{}. [{}] {}:{} — {}\n   Code: {} | Severity: {}\n\n",
                i + 1,
                result
                    .source
                    .as_ref()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                result.file,
                result.line.value,
                result.message,
                result.code,
                result.severity,
            ));
        }
        output
    }
}

impl ILintExecutorProtocol for LintExecutor {
    fn check(&self, path: &str, _flags: &ActionFlags) -> LintExecutionResult {
        let results = self.code_analysis.run_code_analysis(path);
        let output = self.format_results(&results);
        let count = results.len();
        LintExecutionResult::success(output, count)
    }

    fn scan(&self, path: &str) -> LintExecutionResult {
        let results = self.code_analysis.run_code_analysis(path);
        let output = self.format_results(&results);
        let count = results.len();
        LintExecutionResult::success(output, count)
    }

    fn fix(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult {
        let mode = if flags.dry_run { "DRY-RUN" } else { "LIVE" };
        let results = self.code_analysis.run_code_analysis(path);
        let count_before = results.len();
        let output = format!(
            "[{}] Fix scan on {}\nViolations found: {}\nFix application requires FixOrchestrator aggregate.\nUse CLI `lint-arwaky-cli fix {}` for full fix pipeline.",
            mode, path, count_before, path
        );
        LintExecutionResult::success(output, count_before)
    }

    fn ci(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult {
        let results = self.code_analysis.run_code_analysis(path);
        let score = self.code_analysis.calc_score(&results.values);
        let has_critical = self.code_analysis.check_critical(&results.values);
        let pass = score >= flags.threshold as f64 && !has_critical;
        let status = if pass { "PASS" } else { "FAIL" };
        let output = format!(
            "CI Report for {}\nScore: {:.1}/100 (threshold: {})\nViolations: {}\nCritical: {}\nStatus: {}",
            path,
            score,
            flags.threshold,
            results.len(),
            has_critical,
            status,
        );
        LintExecutionResult::success(output, results.len())
    }

    fn orphan(&self, path: &str) -> LintExecutionResult {
        let output = format!(
            "Orphan detection for {}\nUse CLI `lint-arwaky-cli orphan {}` for full orphan graph analysis.",
            path, path
        );
        LintExecutionResult::success(output, 0)
    }

    fn security(&self, path: &str) -> LintExecutionResult {
        let output = format!(
            "Security scan for {}\nUse CLI `lint-arwaky-cli security {}` for full vulnerability scan.",
            path, path
        );
        LintExecutionResult::success(output, 0)
    }

    fn duplicates(&self, path: &str) -> LintExecutionResult {
        let output = format!(
            "Duplication detection for {}\nUse CLI `lint-arwaky-cli duplicates {}` for full analysis.",
            path, path
        );
        LintExecutionResult::success(output, 0)
    }

    fn dependencies(&self, path: &str) -> LintExecutionResult {
        let output = format!(
            "Dependency scan for {}\nUse CLI `lint-arwaky-cli dependencies {}` for full report.",
            path, path
        );
        LintExecutionResult::success(output, 0)
    }

    fn doctor(&self) -> LintExecutionResult {
        let output = "Environment Diagnostics:\n\
            Use CLI `lint-arwaky-cli maintenance doctor` for full environment check.\n\
            Required: Rust toolchain, Python 3.8+, Node.js 18+"
            .to_string();
        LintExecutionResult::success(output, 0)
    }

    fn init(&self, _flags: &ActionFlags) -> LintExecutionResult {
        let output = "Config initialization.\nUse CLI `lint-arwaky-cli setup init` to create lint_arwaky.config.yaml".to_string();
        LintExecutionResult::success(output, 0)
    }

    fn install(&self, _flags: &ActionFlags) -> LintExecutionResult {
        let output = "Adapter dependency installation.\nUse CLI `lint-arwaky-cli setup install` to install all adapter dependencies.".to_string();
        LintExecutionResult::success(output, 0)
    }

    fn mcp_config(&self, flags: &ActionFlags) -> LintExecutionResult {
        let output = format!(
            "MCP Configuration for client: {}\nUse CLI `lint-arwaky-cli setup mcp-config --client {}` to print config.",
            flags.mcp_client, flags.mcp_client
        );
        LintExecutionResult::success(output, 0)
    }

    fn config_show(&self) -> LintExecutionResult {
        let output =
            "Active Configuration:\nUse CLI `lint-arwaky-cli config show` to display full config."
                .to_string();
        LintExecutionResult::success(output, 0)
    }

    fn install_hook(&self) -> LintExecutionResult {
        let output =
            "Git pre-commit hook installation.\nUse CLI `lint-arwaky-cli install-hook` to install."
                .to_string();
        LintExecutionResult::success(output, 0)
    }

    fn uninstall_hook(&self) -> LintExecutionResult {
        let output =
            "Git pre-commit hook removal.\nUse CLI `lint-arwaky-cli uninstall-hook` to remove."
                .to_string();
        LintExecutionResult::success(output, 0)
    }

    fn adapters(&self) -> LintExecutionResult {
        let output = "Active Linter Adapters:\n\
            1. ast_rust_scanner (Rust AST)\n\
            2. ast_py_scanner (Python AST)\n\
            3. ast_js_scanner (JS/TS AST)\n\
            4. rust_linter_adapter (Clippy)\n\
            5. python_ruff_adapter (Ruff)\n\
            6. python_mypy_adapter (MyPy)\n\
            7. python_bandit_adapter (Bandit)\n\
            8. python_metrics_adapter (Radon)\n\
            9. javascript_linter_adapter (ESLint/Prettier/TSC)"
            .to_string();
        LintExecutionResult::success(output, 0)
    }

    fn version(&self) -> LintExecutionResult {
        let output = format!(
            "Lint Arwaky v{} (AES Semantic Builder)",
            env!("CARGO_PKG_VERSION")
        );
        LintExecutionResult::success(output, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::cli_commands::taxonomy_result_vo::LintResult;
    use shared::cli_commands::taxonomy_severity_vo::Severity;
    use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
    use shared::common::taxonomy_common_vo::LineNumber;
    use shared::common::taxonomy_error_vo::ErrorCode;
    use shared::common::taxonomy_lint_vo::ScopeRef;
    use shared::common::taxonomy_message_vo::LintMessage;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::common::taxonomy_suggestion_vo::DescriptionVO;
    use std::sync::Arc;

    struct MockCodeAnalysis {
        results: LintResultList,
        score: f64,
        critical: bool,
    }

    impl MockCodeAnalysis {
        fn empty() -> Self {
            Self {
                results: LintResultList::new(vec![]),
                score: 95.0,
                critical: false,
            }
        }
        fn with_violations(count: usize, critical: bool) -> Self {
            let results: Vec<LintResult> = (0..count)
                .map(|i| LintResult {
                    file: FilePath::new(format!("file_{}.rs", i)).unwrap_or_default(),
                    line: LineNumber::new(1),
                    column: Default::default(),
                    code: ErrorCode::raw("TEST001"),
                    message: LintMessage::new(format!("violation {}", i)),
                    source: None,
                    severity: if critical && i == 0 {
                        Severity::CRITICAL
                    } else {
                        Severity::LOW
                    },
                    enclosing_scope: Some(ScopeRef {
                        name: DescriptionVO::new(String::new()),
                        kind: DescriptionVO::new(String::new()),
                        file: None,
                        start_line: None,
                        end_line: None,
                    }),
                    related_locations: Default::default(),
                })
                .collect();
            Self {
                results: LintResultList::new(results),
                score: if critical { 50.0 } else { 85.0 },
                critical,
            }
        }
    }

    impl ICodeAnalysisAggregate for MockCodeAnalysis {
        fn run_code_analysis(&self, _path: &str) -> LintResultList {
            self.results.clone()
        }
        fn run_code_analysis_dir(&self, _path: &str) -> LintResultList {
            self.results.clone()
        }
        fn run_code_analysis_path(&self, _path: &str) -> Vec<LintResult> {
            self.results.values.clone()
        }
        fn calc_score(&self, _results: &[LintResult]) -> f64 {
            self.score
        }
        fn check_critical(&self, _results: &[LintResult]) -> bool {
            self.critical
        }
        fn format_report(&self, _results: &LintResultList, _root: &str) -> String {
            "mock report".to_string()
        }
        fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> {
            vec![]
        }
    }

    fn make_executor(mock: MockCodeAnalysis) -> LintExecutor {
        LintExecutor::new(Arc::new(mock))
    }

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
        assert_eq!(result.violation_count, 5);
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
        let executor = make_executor(MockCodeAnalysis {
            score: 90.0,
            critical: false,
            ..MockCodeAnalysis::empty()
        });
        let mut flags = ActionFlags::default();
        flags.threshold = 80;
        let result = executor.ci("/root", &flags);
        assert!(result.success);
        assert!(result.output.contains("PASS"));
    }

    #[test]
    fn test_ci_fail_low_score() {
        let executor = make_executor(MockCodeAnalysis {
            score: 50.0,
            critical: false,
            ..MockCodeAnalysis::empty()
        });
        let mut flags = ActionFlags::default();
        flags.threshold = 80;
        let result = executor.ci("/root", &flags);
        assert!(result.success);
        assert!(result.output.contains("FAIL"));
    }

    #[test]
    fn test_ci_fail_critical() {
        let executor = make_executor(MockCodeAnalysis {
            score: 95.0,
            critical: true,
            ..MockCodeAnalysis::empty()
        });
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
    fn test_duplicates() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let result = executor.duplicates("/root");
        assert!(result.success);
        assert!(result.output.contains("Duplication detection"));
    }

    #[test]
    fn test_dependencies() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let result = executor.dependencies("/root");
        assert!(result.success);
        assert!(result.output.contains("Dependency scan"));
    }

    #[test]
    fn test_doctor() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let result = executor.doctor();
        assert!(result.success);
        assert!(result.output.contains("Environment Diagnostics"));
    }

    #[test]
    fn test_init() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let flags = ActionFlags::default();
        let result = executor.init(&flags);
        assert!(result.success);
        assert!(result.output.contains("Config initialization"));
    }

    #[test]
    fn test_install() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let flags = ActionFlags::default();
        let result = executor.install(&flags);
        assert!(result.success);
        assert!(result.output.contains("Adapter dependency"));
    }

    #[test]
    fn test_mcp_config() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let flags = ActionFlags::default();
        let result = executor.mcp_config(&flags);
        assert!(result.success);
        assert!(result.output.contains("MCP Configuration"));
    }

    #[test]
    fn test_config_show() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let result = executor.config_show();
        assert!(result.success);
        assert!(result.output.contains("Active Configuration"));
    }

    #[test]
    fn test_install_hook() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let result = executor.install_hook();
        assert!(result.success);
        assert!(result.output.contains("pre-commit hook"));
    }

    #[test]
    fn test_uninstall_hook() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let result = executor.uninstall_hook();
        assert!(result.success);
        assert!(result.output.contains("pre-commit hook"));
    }

    #[test]
    fn test_adapters() {
        let executor = make_executor(MockCodeAnalysis::empty());
        let result = executor.adapters();
        assert!(result.success);
        assert!(result.output.contains("Active Linter Adapters"));
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
            line: LineNumber::new(10),
            column: Default::default(),
            code: ErrorCode::raw("E001"),
            message: LintMessage::new("test message"),
            source: None,
            severity: Severity::LOW,
            enclosing_scope: None,
            related_locations: Default::default(),
        }]);
        let output = executor.format_results(&results);
        assert!(output.contains("1 violation"));
        assert!(output.contains("test.rs:10"));
        assert!(output.contains("test message"));
    }
}
