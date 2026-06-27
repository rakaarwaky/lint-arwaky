// PURPOSE: Capabilities-layer lint executor — wraps ICodeAnalysisAggregate for the TUI.
// Implements ILintExecutorProtocol, providing all lint action methods (check, scan, fix, ci, etc.)
// with user-facing output formatting.

use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
use shared::git_hooks::contract_manager_port::IHookManagerPort;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::taxonomy_doctor_vo::DependencyReport;
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
    std::process::Command::new("which")
        .arg(binary)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Discover all known linter adapters and report their install status.
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
/// Optionally holds a SetupManagementAggregate for real config initialization,
/// a MaintenanceCommandsAggregate for real environment diagnostics (doctor),
/// an IHookManagerPort for real git hook operations, and an
/// IConfigOrchestrationAggregate for real config display.
pub struct LintExecutor {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator: Option<Arc<dyn LintFixOrchestratorAggregate>>,
    setup_aggregate: Option<Arc<dyn SetupManagementAggregate>>,
    maintenance: Option<Arc<dyn MaintenanceCommandsAggregate>>,
    hook_port: Option<Arc<dyn IHookManagerPort>>,
    config_orchestrator: Option<Arc<dyn IConfigOrchestrationAggregate>>,
}

impl LintExecutor {
    pub fn new(code_analysis: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self {
            code_analysis,
            fix_orchestrator: None,
            setup_aggregate: None,
            maintenance: None,
            hook_port: None,
            config_orchestrator: None,
        }
    }

    pub fn new_with_fix(
        code_analysis: Arc<dyn ICodeAnalysisAggregate>,
        fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
    ) -> Self {
        Self {
            code_analysis,
            fix_orchestrator: Some(fix_orchestrator),
            setup_aggregate: None,
            maintenance: None,
            hook_port: None,
            config_orchestrator: None,
        }
    }

    pub fn new_with_setup(
        code_analysis: Arc<dyn ICodeAnalysisAggregate>,
        fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
        setup_aggregate: Arc<dyn SetupManagementAggregate>,
    ) -> Self {
        Self {
            code_analysis,
            fix_orchestrator: Some(fix_orchestrator),
            setup_aggregate: Some(setup_aggregate),
            maintenance: None,
            hook_port: None,
            config_orchestrator: None,
        }
    }

    /// Builder method: attach a maintenance aggregate for real doctor diagnostics.
    pub fn with_maintenance(mut self, maintenance: Arc<dyn MaintenanceCommandsAggregate>) -> Self {
        self.maintenance = Some(maintenance);
        self
    }

    /// Builder method: attach a hook manager port for real git hook operations.
    pub fn with_hook_port(mut self, hook_port: Arc<dyn IHookManagerPort>) -> Self {
        self.hook_port = Some(hook_port);
        self
    }

    /// Builder method: attach a config orchestrator for real config display.
    pub fn with_config(
        mut self,
        config_orchestrator: Arc<dyn IConfigOrchestrationAggregate>,
    ) -> Self {
        self.config_orchestrator = Some(config_orchestrator);
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

    /// Format ToolchainDiagnostics into a human-readable health report.
    fn format_doctor_report(
        diagnostics: &shared::project_setup::taxonomy_doctor_vo::ToolchainDiagnostics,
    ) -> LintExecutionResult {
        let mut output = String::from("Environment Diagnostics\n");
        output.push_str(&format!("Binary: {}\n\n", diagnostics.binary_path));

        let format_section = |name: &str,
                              tools: &[shared::project_setup::taxonomy_doctor_vo::ToolStatus]|
         -> String {
            let mut section = format!("== {} ==\n", name);
            for tool in tools {
                let icon = match tool.status.as_str() {
                    "OK" => "\u{2713}",
                    "WARN" => "\u{26A0}",
                    "FAIL" => "\u{2717}",
                    _ => "?",
                };
                let note = match tool.status.as_str() {
                    "WARN" => " (optional)",
                    "FAIL" => " (required)",
                    _ => "",
                };
                section.push_str(&format!(
                    "  {} {} {}{}\n",
                    icon, tool.name, tool.version, note
                ));
            }
            section.push('\n');
            section
        };

        output.push_str(&format_section("Rust Tools", &diagnostics.rust_tools));
        output.push_str(&format_section("Python Tools", &diagnostics.python_tools));
        output.push_str(&format_section("JS/TS Tools", &diagnostics.js_tools));
        output.push_str(&format_section("VCS Tools", &diagnostics.vcs_tools));

        let fail_count = diagnostics
            .rust_tools
            .iter()
            .chain(diagnostics.python_tools.iter())
            .chain(diagnostics.js_tools.iter())
            .chain(diagnostics.vcs_tools.iter())
            .filter(|t| t.status == "FAIL")
            .count();

        if fail_count == 0 {
            output.push_str("All required tools OK.\n");
        } else {
            output.push_str(&format!("{} required tool(s) missing!\n", fail_count));
        }

        LintExecutionResult::success(output, fail_count)
    }

    /// Run real config initialization using SetupManagementAggregate.
    fn run_init(&self) -> LintExecutionResult {
        match &self.setup_aggregate {
            Some(protocol) => {
                let language = protocol.detect_language();
                let lang_str = &language.value;
                let config_path = "lint_arwaky.config.yaml";

                if protocol.file_exists(config_path) {
                    let output = format!(
                        "Config initialization.\nlint_arwaky.config.yaml already exists.\nDetected language: {}",
                        lang_str
                    );
                    return LintExecutionResult::success(output, 0);
                }

                let template = protocol.get_config_template(lang_str);
                match protocol.write_config_file(config_path, template) {
                    Ok(desc) => {
                        let output = format!(
                            "Config initialization.\n{}\nDetected language: {}",
                            desc.value, lang_str
                        );
                        LintExecutionResult::success(output, 0)
                    }
                    Err(e) => {
                        let output = format!(
                            "Config initialization failed.\nError: {}\n\nUse CLI `lint-arwaky-cli setup init` as fallback.",
                            e
                        );
                        LintExecutionResult::failure(output)
                    }
                }
            }
            None => {
                let output =
                    "Config initialization.\nUse CLI `lint-arwaky-cli setup init` to create lint_arwaky.config.yaml"
                        .to_string();
                LintExecutionResult::success(output, 0)
            }
        }
    }

    /// Format DependencyReport into a human-readable list for the TUI.
    fn format_dependency_report(
        path: &str,
        report: &DependencyReport,
    ) -> LintExecutionResult {
        let count = report.dependencies.len();
        let mut output = format!(
            "Dependency scan for {}\nLanguage: {}\nTotal dependencies: {}\n",
            path, report.language, count
        );

        let direct: Vec<_> = report
            .dependencies
            .iter()
            .filter(|d| d.dep_type == "direct")
            .collect();
        let transitive: Vec<_> = report
            .dependencies
            .iter()
            .filter(|d| d.dep_type == "transitive")
            .collect();

        if !direct.is_empty() {
            output.push_str(&format!("\nDirect ({}) [top 30]:\n", direct.len()));
            for dep in direct.iter().take(30) {
                output.push_str(&format!("  {} {}\n", dep.name, dep.version));
            }
            if direct.len() > 30 {
                output.push_str(&format!("  ... and {} more\n", direct.len() - 30));
            }
        }

        if !transitive.is_empty() {
            output.push_str(&format!(
                "\nTransitive ({}) [top 30]:\n",
                transitive.len()
            ));
            for dep in transitive.iter().take(30) {
                output.push_str(&format!("  {} {}\n", dep.name, dep.version));
            }
            if transitive.len() > 30 {
                output.push_str(&format!("  ... and {} more\n", transitive.len() - 30));
            }
        }

        LintExecutionResult::success(output, count)
    }

    /// Format a ConfigResult into a human-readable display for the TUI config_show action.
    fn format_config_result(
        result: &shared::config_system::taxonomy_source_vo::ConfigResult,
    ) -> LintExecutionResult {
        let mut output = String::from("Active Configuration\n");
        output.push_str(&format!(
            "Source: {} ({})\n",
            result.source.path.value, result.source.language
        ));

        if !result.warnings.is_empty() {
            output.push_str("\nWarnings:\n");
            for w in &result.warnings {
                output.push_str(&format!("  - {}\n", w));
            }
        }

        let config = &result.config;
        output.push_str(&format!("\nEnabled: {}\n", config.enabled.value));
        output.push_str(&format!("Layers: {}\n", config.layers.len()));
        output.push_str(&format!("Rules: {}\n", config.rules.len()));
        output.push_str(&format!(
            "Ignored paths: {}\n",
            config.ignored_paths.values.len()
        ));
        output.push_str(&format!(
            "Mandatory class definition: {}\n",
            config.mandatory_class_definition.value
        ));

        if !config.layers.is_empty() {
            output.push_str("\nArchitecture Layers:\n");
            for name in config.layers.keys() {
                output.push_str(&format!("  - {}\n", name.value));
            }
        }

        if !config.rules.is_empty() {
            output.push_str("\nRules:\n");
            for (i, rule) in config.rules.iter().enumerate() {
                output.push_str(&format!(
                    "  {}. {} (scope: {})\n",
                    i + 1,
                    rule.name.value,
                    rule.scope.value,
                ));
            }
        }

        LintExecutionResult::success(output, 0)
    }
}

/// ILintExecutorProtocol implementation.
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
            path, score, flags.threshold, results.len(), has_critical, status,
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
        match &self.maintenance {
            Some(maintenance) => {
                let fp = shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
                    .unwrap_or_default();
                let rt = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(e) => {
                        return LintExecutionResult::failure(format!(
                            "Failed to create runtime for dependency report: {}",
                            e
                        ));
                    }
                };
                match rt.block_on(maintenance.run_dependency_report(&fp)) {
                    Ok(report) => Self::format_dependency_report(path, &report),
                    Err(e) => LintExecutionResult::failure(format!(
                        "Dependency scan for {}\nError: {}",
                        path, e
                    )),
                }
            }
            None => {
                let output = format!(
                    "Dependency scan for {}\nUse CLI `lint-arwaky-cli dependencies {}` for full report.",
                    path, path
                );
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn doctor(&self) -> LintExecutionResult {
        match &self.maintenance {
            Some(maintenance) => {
                // Bridge async->sync: create a short-lived tokio runtime to run
                // the async diagnose_toolchain() from this sync trait method.
                let rt = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(e) => {
                        return LintExecutionResult::failure(format!(
                            "Failed to create runtime for diagnostics: {}",
                            e
                        ));
                    }
                };
                let diagnostics = rt.block_on(maintenance.diagnose_toolchain());
                Self::format_doctor_report(&diagnostics)
            }
            None => {
                let output = "Environment Diagnostics:\n\
                    Use CLI `lint-arwaky-cli maintenance doctor` for full environment check.\n\
                    Required: Rust toolchain, Python 3.8+, Node.js 18+"
                    .to_string();
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn init(&self, _flags: &ActionFlags) -> LintExecutionResult {
        self.run_init()
    }

    fn install(&self, _flags: &ActionFlags) -> LintExecutionResult {
        let output =
            "Adapter dependency installation.\nUse CLI `lint-arwaky-cli setup install` to install all adapter dependencies."
                .to_string();
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
        match &self.config_orchestrator {
            Some(orchestrator) => {
                // Bridge async->sync: create a short-lived tokio runtime.
                let rt = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(e) => {
                        return LintExecutionResult::failure(format!(
                            "Failed to create runtime for config display: {}",
                            e
                        ));
                    }
                };
                let cwd = std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string());
                let project_root =
                    shared::common::taxonomy_path_vo::FilePath::new(cwd).unwrap_or_default();
                let result = rt.block_on(orchestrator.load_project_config(&project_root));
                Self::format_config_result(&result)
            }
            None => {
                let output =
                    "Active Configuration:\nUse CLI `lint-arwaky-cli config show` to display full config."
                        .to_string();
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn install_hook(&self) -> LintExecutionResult {
        match &self.hook_port {
            Some(port) => {
                let exe_path = shared::common::taxonomy_path_vo::FilePath::default();
                match port.install_pre_commit(&exe_path) {
                    Ok(status) => {
                        if status.value {
                            LintExecutionResult::success(
                                "Git pre-commit hook installed successfully.".to_string(),
                                0,
                            )
                        } else {
                            LintExecutionResult::failure(
                                "Git pre-commit hook installation failed.\n\
                                 Not a git repository? Run `git init` first."
                                    .to_string(),
                            )
                        }
                    }
                    Err(e) => LintExecutionResult::failure(format!(
                        "Git pre-commit hook installation failed.\nError: {}",
                        e
                    )),
                }
            }
            None => LintExecutionResult::success(
                "Git pre-commit hook installation.\n\
                 Use CLI `lint-arwaky-cli install-hook` to install."
                    .to_string(),
                0,
            ),
        }
    }

    fn uninstall_hook(&self) -> LintExecutionResult {
        match &self.hook_port {
            Some(port) => match port.uninstall_pre_commit() {
                Ok(status) => {
                    if status.value {
                        LintExecutionResult::success(
                            "Git pre-commit hook removed successfully.".to_string(),
                            0,
                        )
                    } else {
                        LintExecutionResult::success(
                            "No git pre-commit hook found \
                             (not a git repo or hook already removed)."
                                .to_string(),
                            0,
                        )
                    }
                }
                Err(e) => LintExecutionResult::failure(format!(
                    "Git pre-commit hook removal failed.\nError: {}",
                    e
                )),
            },
            None => LintExecutionResult::success(
                "Git pre-commit hook removal.\n\
                 Use CLI `lint-arwaky-cli uninstall-hook` to remove."
                    .to_string(),
                0,
            ),
        }
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
