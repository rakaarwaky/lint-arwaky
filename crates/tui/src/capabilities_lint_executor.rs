// PURPOSE: Capabilities-layer lint executor — wraps ICodeAnalysisAggregate for the TUI.
// Implements ILintExecutorProtocol, providing all lint action methods (check, scan, fix, ci, etc.)
// with user-facing output formatting. Many actions delegate to the CLI for full pipeline execution.

use orphan_detector::root_orphan_detector_container::OrphanContainer;
use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::taxonomy_doctor_vo::SecurityScanReport;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_action_flags_vo::ActionFlags;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_action_flags_vo::ActionFlags;
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
use std::sync::Arc;

/// Adapter descriptor — name, display label, and whether it is installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterInfo {
    pub name: String,
    pub label: String,
    pub installed: bool,
}

/// Check whether a binary is reachable on PATH via `which`.
fn is_binary_available(binary: &str) -> bool {
    Command::new("which")
        .arg(binary)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Discover all known linter adapters and report their install status.
/// Built-in AST scanners are always available; external tools are checked via `which`.
pub fn discover_adapters() -> Vec<AdapterInfo> {
    let builtins = [
        ("ast_rust_scanner", "Rust AST (built-in)"),
        ("ast_py_scanner", "Python AST (built-in)"),
        ("ast_js_scanner", "JS/TS AST (built-in)"),
    ];
    let externals = [
        ("clippy", "Clippy (Rust)"),
        ("ruff", "Ruff (Python)"),
        ("mypy", "MyPy (Python)"),
        ("bandit", "Bandit (Python)"),
        ("radon", "Radon (Python metrics)"),
        ("eslint", "ESLint (JavaScript)"),
        ("prettier", "Prettier (JavaScript)"),
        ("tsc", "TypeScript Compiler"),
    ];

    let mut adapters: Vec<AdapterInfo> = builtins
        .iter()
        .map(|(name, label)| AdapterInfo {
            name: name.to_string(),
            label: label.to_string(),
            installed: true,
        })
        .collect();

    for (binary, label) in externals {
        adapters.push(AdapterInfo {
            name: binary.to_string(),
            label: label.to_string(),
            installed: is_binary_available(binary),
        });
    }

    adapters
}

/// LintExecutor — TUI-facing lint action provider.
/// Delegates code analysis to ICodeAnalysisAggregate and formats results for display.
/// Optionally holds a LintFixOrchestratorAggregate for real auto-fix execution.
/// Optionally holds a SetupManagementAggregate for real adapter installation.
pub struct LintExecutor {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator: Option<Arc<dyn LintFixOrchestratorAggregate>>,
    orphan_container: Option<OrphanContainer>,
    setup_aggregate: Option<Arc<dyn SetupManagementAggregate>>,
    maintenance: Option<Arc<dyn MaintenanceCommandsAggregate>>,
}

impl LintExecutor {
    pub fn new(code_analysis: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self {
            code_analysis,
            fix_orchestrator: None,
            orphan_container: None,
            setup_aggregate: None,
            maintenance: None,
        }
    }

    pub fn new_with_fix(
        code_analysis: Arc<dyn ICodeAnalysisAggregate>,
        fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
    ) -> Self {
        Self {
            code_analysis,
            fix_orchestrator: Some(fix_orchestrator),
            orphan_container: None,
            setup_aggregate: None,
            maintenance: None,
        }
    }

    pub fn new_with_fix_and_orphan(
        code_analysis: Arc<dyn ICodeAnalysisAggregate>,
        fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
        orphan_container: OrphanContainer,
    ) -> Self {
        Self {
            code_analysis,
            fix_orchestrator: Some(fix_orchestrator),
            orphan_container: Some(orphan_container),
            setup_aggregate: None,
            maintenance: None,
        }
    }

    pub fn new_with_fix_and_setup(
        code_analysis: Arc<dyn ICodeAnalysisAggregate>,
        fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
        orphan_container: OrphanContainer,
        setup_aggregate: Arc<dyn SetupManagementAggregate>,
    ) -> Self {
        Self {
            code_analysis,
            fix_orchestrator: Some(fix_orchestrator),
            orphan_container: Some(orphan_container),
            setup_aggregate: Some(setup_aggregate),
            maintenance: None,
        }
    }

    pub fn with_maintenance(mut self, maintenance: Arc<dyn MaintenanceCommandsAggregate>) -> Self {
        self.maintenance = Some(maintenance);
        self
    }

    /// Format lint results into a human-readable numbered list for TUI preview panel.
    pub fn format_results(&self, results: &LintResultList) -> String {
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

/// ILintExecutorProtocol implementation — each method calls code_analysis then formats output.
/// Methods like orphan/security/duplicates that require specialized analysis delegate to CLI.
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

        match &self.fix_orchestrator {
            Some(orchestrator) => {
                let file_path =
                    shared::common::taxonomy_path_vo::FilePath::new(path).unwrap_or_default();
                let fix_result: FixResult = orchestrator.execute(&file_path);
                let output = format!("[{}] {}", mode, fix_result.output);
                if fix_result.is_success() {
                    LintExecutionResult::success(output, 0)
                } else {
                    LintExecutionResult::failure(output)
                }
            }
            None => {
                let results = self.code_analysis.run_code_analysis(path);
                let count_before = results.len();
                let output = format!(
                    "[{}] Fix scan on {}\nViolations found: {}\nFix application requires FixOrchestrator aggregate.\nUse CLI `lint-arwaky-cli fix {}` for full fix pipeline.",
                    mode, path, count_before, path
                );
                LintExecutionResult::success(output, count_before)
            }
        }
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
        match &self.orphan_container {
            Some(container) => {
                let scan_root = std::path::Path::new(path);
                let source_files = shared::common::collect_all_source_files(scan_root);
                if source_files.is_empty() {
                    let output = format!("No source files found in {}", path);
                    return LintExecutionResult::success(output, 0);
                }
                let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
                let analyzer = container.analyzer();
                let layer_detector = container.layer_detector();
                let results = analyzer.check_orphans(layer_detector.as_ref(), &file_strs, path);
                let count = results.len();
                if count == 0 {
                    let output = format!("Orphan detection for {}\nNo orphan files found.", path);
                    LintExecutionResult::success(output, 0)
                } else {
                    let mut output = format!(
                        "Orphan detection for {}\nFound {} orphan(s):\n\n",
                        path, count
                    );
                    for (i, r) in results.iter().enumerate() {
                        output.push_str(&format!(
                            "{}. [{}] {} — {}\n   Code: {} | Severity: {}\n\n",
                            i + 1,
                            r.source
                                .as_ref()
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "unknown".to_string()),
                            r.file,
                            r.message,
                            r.code,
                            r.severity,
                        ));
                    }
                    LintExecutionResult::success(output, count)
                }
            }
            None => {
                let output = format!(
                    "Orphan detection for {}\nUse CLI `lint-arwaky-cli orphan {}` for full orphan graph analysis.",
                    path, path
                );
                LintExecutionResult::success(output, 0)
            }
        }
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
        let output =
            "Config initialization.\nUse CLI `lint-arwaky-cli setup init` to create lint_arwaky.config.yaml"
                .to_string();
        LintExecutionResult::success(output, 0)
    }

    fn install(&self, flags: &ActionFlags) -> LintExecutionResult {
        match &self.setup_aggregate {
            Some(aggregate) => {
                if flags.dry_run {
                    // Dry-run: detect language and show what would be installed
                    let lang = aggregate.detect_language();
                    let mut output = format!(
                        "[DRY-RUN] Adapter installation for project language: {}\n\n",
                        lang.value
                    );
                    if lang.value == "rust" {
                        output.push_str("Rust projects use built-in AST scanners.\n");
                        output.push_str("No Python/JS adapter installation needed.\n");
                    } else {
                        let adapters = discover_adapters();
                        let externals: Vec<&AdapterInfo> = adapters
                            .iter()
                            .filter(|a| !a.name.starts_with("ast_"))
                            .collect();
                        output.push_str("Would install:\n");
                        for adapter in &externals {
                            let status = if adapter.installed {
                                "[already installed]"
                            } else {
                                "[not installed]"
                            };
                            output.push_str(&format!(
                                "  - {} ({}) {}\n",
                                adapter.label, adapter.name, status
                            ));
                        }
                    }
                    LintExecutionResult::success(output, 0)
                } else {
                    // Real installation via the setup aggregate
                    let rt =
                        tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
                    let lang = aggregate.detect_language();
                    let mut output = format!(
                        "Adapter installation for project language: {}\n\n",
                        lang.value
                    );
                    let mut failures = 0u32;

                    if lang.value != "rust" {
                        // Install Python adapters
                        async fn install_py(
                            agg: &dyn SetupManagementAggregate,
                        ) -> bool {
                            let status = agg.install_python_adapters().await;
                            *status
                        }
                        let py_ok = rt.block_on(install_py(aggregate.as_ref()));
                        let py_label = if py_ok { "OK" } else { "FAILED" };
                        output.push_str(&format!("Python adapters: {}\n", py_label));
                        if !py_ok {
                            failures += 1;
                        }

                        // Install JS adapters (use sudo from flags)
                        async fn install_js(
                            agg: &dyn SetupManagementAggregate,
                            sudo: bool,
                        ) -> bool {
                            let status = agg.install_javascript_adapters(sudo).await;
                            *status
                        }
                        let js_ok =
                            rt.block_on(install_js(aggregate.as_ref(), flags.use_sudo));
                        let js_label = if js_ok { "OK" } else { "FAILED" };
                        output.push_str(&format!("JS adapters: {}\n", js_label));
                        if !js_ok {
                            failures += 1;
                        }
                    } else {
                        output.push_str(
                            "Rust projects use built-in AST scanners (Clippy via rustup).\n",
                        );
                        output.push_str("No additional adapter installation needed.\n");
                    }

                    if failures == 0 {
                        LintExecutionResult::success(output, 0)
                    } else {
                        LintExecutionResult::failure(format!(
                            "{}\n\n{} adapter group(s) failed. Run `lint-arwaky-cli setup install` for verbose output.",
                            output, failures
                        ))
                    }
                }
            }
            None => {
                let output =
                    "Adapter dependency installation.\nUse CLI `lint-arwaky-cli setup install` to install all adapter dependencies."
                        .to_string();
                LintExecutionResult::success(output, 0)
            }
        }
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
        let adapters = discover_adapters();
        let mut output = String::from("Active Linter Adapters:\n");
        for (i, adapter) in adapters.iter().enumerate() {
            let status = if adapter.installed { "[+]" } else { "[-]" };
            output.push_str(&format!(
                "  {}. [{}] {} ({})\n",
                i + 1,
                status,
                adapter.label,
                adapter.name
            ));
        }
        let installed = adapters.iter().filter(|a| a.installed).count();
        let total = adapters.len();
        output.push_str(&format!("\n{} of {} adapters available", installed, total));
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
