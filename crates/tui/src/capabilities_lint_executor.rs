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
