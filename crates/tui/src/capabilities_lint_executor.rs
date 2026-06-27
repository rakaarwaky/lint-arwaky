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

    /// Builder method: attach orphan detector aggregates for real orphan analysis.
    pub fn with_orphan(
        mut self,
        orphan_aggregate: Arc<dyn shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate>,
        layer_detector: Arc<dyn shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate>,
        scanner_provider: Arc<dyn shared::common::contract_scanner_provider_port::IScannerProviderPort>,
    ) -> Self {
        self.orphan_aggregate = Some(orphan_aggregate);
        self.layer_detector = Some(layer_detector);
        self.scanner_provider = Some(scanner_provider);
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
        output.push_str(&format!(
            "Naming word count: {}\n",
            config.naming.word_count.value
        ));

        if !config.layers.is_empty() {
            output.push_str("\nArchitecture Layers:\n");
            for (name, def) in config.layers.iter() {
                let policy = if def.naming.suffix_policy.value.is_empty() {
                    String::new()
                } else {
                    format!(" (policy: {})", def.naming.suffix_policy.value)
                };
                output.push_str(&format!("  - {}{}\n", name.value, policy));
            }
        }

        if !config.rules.is_empty() {
            output.push_str(&format!("\nRules ({}):\n", config.rules.len()));
            for (i, rule) in config.rules.iter().enumerate() {
                let desc = if rule.description.value.is_empty() {
                    String::new()
                } else if rule.description.value.len() > 60 {
                    format!(" — {}…", &rule.description.value[..60])
                } else {
                    format!(" — {}", rule.description.value)
                };
                output.push_str(&format!(
                    "  {}. {} [{}]{}\n",
                    i + 1,
                    rule.name.value,
                    rule.scope.value,
                    desc,
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
        match &self.setup_aggregate {
            Some(protocol) => {
                let language = protocol.detect_language();
                let lang_str = &language.value;
                let mut output = format!(
                    "Adapter dependency installation\nDetected language: {}\n\n",
                    lang_str
                );

                let rt = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(e) => {
                        return LintExecutionResult::failure(format!(
                            "Failed to create runtime for adapter installation: {}",
                            e
                        ));
                    }
                };

                let py_result = rt.block_on(protocol.install_python_adapters());
                let py_icon = if py_result.value { "[OK]" } else { "[FAIL]" };
                output.push_str(&format!(
                    "  {} Python (ruff, mypy, bandit)\n",
                    py_icon
                ));

                let is_js = lang_str.contains("javascript") || lang_str.contains("typescript");
                let mut js_failed = false;
                if is_js {
                    let js_result = rt.block_on(protocol.install_javascript_adapters(false));
                    let js_icon = if js_result.value { "[OK]" } else { "[FAIL]" };
                    if !js_result.value {
                        js_failed = true;
                    }
                    output.push_str(&format!(
                        "  {} JavaScript (eslint, prettier, typescript)\n",
                        js_icon
                    ));
                }

                if !py_result.value || js_failed {
                    output.push_str("\nSome adapter(s) failed to install.\n");
                    LintExecutionResult::failure(output)
                } else {
                    output.push_str("\nAll adapter dependencies installed.\n");
                    LintExecutionResult::success(output, 0)
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
                // Direct config reading fallback: search CWD and parents for config files
                // using the config-system crate, then parse and format the result.
                let cwd = std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string());
                let languages = ["rust", "python", "javascript"];
                let mut found_source: Option<
                    shared::config_system::taxonomy_source_vo::ConfigSource,
                > = None;

                for &lang in &languages {
                    let filename = format!("lint_arwaky.config.{}.yaml", lang);
                    let mut current = std::path::PathBuf::from(&cwd);
                    loop {
                        let candidate = current.join(&filename);
                        if candidate.exists() {
                            if let Ok(content) = std::fs::read_to_string(&candidate) {
                                found_source = Some(
                                    shared::config_system::taxonomy_source_vo::ConfigSource::new(
                                        lang,
                                        candidate.to_string_lossy().to_string(),
                                        content,
                                    ),
                                );
                                break;
                            }
                        }
                        if !current.pop() {
                            break;
                        }
                    }
                    if found_source.is_some() {
                        break;
                    }
                }

                match found_source {
                    Some(source) => {
                        let config =
                            shared::config_system::taxonomy_config_vo::parse_config_yaml(
                                &source.raw_content,
                            );
                        let result =
                            shared::config_system::taxonomy_source_vo::ConfigResult::new(
                                config,
                                source,
                                Vec::new(),
                            );
                        Self::format_config_result(&result)
                    }
                    None => {
                        let config =
                            shared::config_system::taxonomy_config_vo::default_aes_config();
                        let source =
                            shared::config_system::taxonomy_source_vo::ConfigSource::new(
                                "rust",
                                "embedded (built-in defaults)",
                                "",
                            );
                        let result =
                            shared::config_system::taxonomy_source_vo::ConfigResult::new(
                                config,
                                source,
                                vec![
                                    "No config file found, using built-in defaults"
                                        .to_string(),
                                ],
                            );
                        Self::format_config_result(&result)
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::*;
    use shared::cli_commands::taxonomy_result_vo::LintResult;
    use shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
    use shared::common::taxonomy_common_vo::BooleanVO;
    use shared::common::taxonomy_definition_vo::LayerDefinition;
    use shared::common::taxonomy_layer_vo::LayerNameVO;
    use shared::common::taxonomy_path_vo::FilePath;
    use shared::common::taxonomy_suggestion_vo::DescriptionVO;
    use shared::config_system::contract_orchestration_aggregate::IConfigOrchestrationAggregate;
    use shared::config_system::contract_reader_port::IConfigReaderPort;
    use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
    use shared::config_system::contract_workspace_detector_port::WorkspaceType;
    use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
    use shared::config_system::taxonomy_config_vo::ArchitectureRule;
    use shared::config_system::taxonomy_source_vo::ConfigResult;
    use shared::config_system::taxonomy_source_vo::ConfigSource;
    use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;

    // Minimal mock — config_show does not invoke code analysis.
    struct MockCodeAnalysis;
    impl ICodeAnalysisAggregate for MockCodeAnalysis {
        fn run_code_analysis(&self, _: &str) -> LintResultList { LintResultList(vec![]) }
        fn run_code_analysis_dir(&self, _: &str) -> LintResultList { LintResultList(vec![]) }
        fn run_code_analysis_path(&self, _: &str) -> Vec<LintResult> { vec![] }
        fn calc_score(&self, _: &[LintResult]) -> f64 { 100.0 }
        fn check_critical(&self, _: &[LintResult]) -> bool { false }
        fn format_report(&self, _: &LintResultList, _: &str) -> String { String::new() }
        fn active_rules(&self) -> Vec<CodeAnalysisRuleVO> { vec![] }
    }

    struct MockWorkspaceDetector;
    impl IWorkspaceDetectorPort for MockWorkspaceDetector {
        fn detect(&self, _: &FilePath) -> WorkspaceType { WorkspaceType::Rust }
        fn is_workspace(&self, _: &FilePath) -> bool { true }
    }

    struct MockConfigReader;
    #[async_trait::async_trait]
    impl IConfigReaderPort for MockConfigReader {
        async fn read_config(&self, _: &FilePath, _: &str) -> Option<ConfigSource> { None }
        async fn list_config_files(&self, _: &FilePath) -> Vec<(String, String)> { vec![] }
    }

    /// Orchestrator returning a config with 2 rules and 2 layers.
    struct MockConfigOrchestrator;
    #[async_trait::async_trait]
    impl IConfigOrchestrationAggregate for MockConfigOrchestrator {
        fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorPort> {
            Arc::new(MockWorkspaceDetector)
        }
        fn config_reader(&self) -> Arc<dyn IConfigReaderPort> {
            Arc::new(MockConfigReader)
        }
        async fn load_project_config(&self, _: &FilePath) -> ConfigResult {
            let mut layers = std::collections::HashMap::new();
            layers.insert(LayerNameVO::new("presentation"), LayerDefinition::default());
            layers.insert(LayerNameVO::new("domain"), LayerDefinition::default());
            let rules = vec![
                ArchitectureRule {
                    name: DescriptionVO::new("AES301 - No cross-layer imports"),
                    scope: LayerNameVO::new("presentation"),
                    ..Default::default()
                },
                ArchitectureRule {
                    name: DescriptionVO::new("AES302 - Domain isolation"),
                    scope: LayerNameVO::new("domain"),
                    ..Default::default()
                },
            ];
            let config = ArchitectureConfig {
                enabled: BooleanVO::new(true),
                layers,
                rules,
                ..Default::default()
            };
            let source = ConfigSource::new("rust", "/test/lint_arwaky.config.rust.yaml", "yaml content");
            ConfigResult::new(config, source, vec![])
        }
    }

    /// Orchestrator returning an empty config with a warning.
    struct MockEmptyConfigOrchestrator;
    #[async_trait::async_trait]
    impl IConfigOrchestrationAggregate for MockEmptyConfigOrchestrator {
        fn workspace_detector(&self) -> Arc<dyn IWorkspaceDetectorPort> {
            Arc::new(MockWorkspaceDetector)
        }
        fn config_reader(&self) -> Arc<dyn IConfigReaderPort> {
            Arc::new(MockConfigReader)
        }
        async fn load_project_config(&self, _: &FilePath) -> ConfigResult {
            ConfigResult::new(
                ArchitectureConfig::default(),
                ConfigSource::new("rust", "/empty.yaml", ""),
                vec!["No config found".to_string()],
            )
        }
    }

    #[test]
    fn config_show_returns_non_empty_with_rules_and_layers() {
        let executor = LintExecutor::new(Arc::new(MockCodeAnalysis))
            .with_config(Arc::new(MockConfigOrchestrator));
        let result: LintExecutionResult = executor.config_show();

        assert!(result.success, "config_show should succeed");
        assert!(!result.output.is_empty(), "output must not be empty");
        assert!(result.output.contains("Active Configuration"), "missing header");
        assert!(result.output.contains("Rules: 2"), "wrong rule count");
        assert!(result.output.contains("Layers: 2"), "wrong layer count");
        assert!(result.output.contains("Enabled: true"), "missing enabled flag");
        assert!(result.output.contains("AES301"), "missing rule AES301");
        assert!(result.output.contains("AES302"), "missing rule AES302");
        assert!(result.output.contains("scope: presentation"), "missing presentation scope");
        assert!(result.output.contains("scope: domain"), "missing domain scope");
        assert!(result.output.contains("Architecture Layers"), "missing layers section");
        assert!(result.output.contains("presentation"), "missing presentation layer");
        assert!(result.output.contains("domain"), "missing domain layer");
    }

    #[test]
    fn config_show_handles_empty_config_with_warnings() {
        let executor = LintExecutor::new(Arc::new(MockCodeAnalysis))
            .with_config(Arc::new(MockEmptyConfigOrchestrator));
        let result: LintExecutionResult = executor.config_show();

        assert!(result.success, "config_show should succeed even with empty config");
        assert!(result.output.contains("Active Configuration"), "missing header");
        assert!(result.output.contains("Rules: 0"), "wrong rule count");
        assert!(result.output.contains("Layers: 0"), "wrong layer count");
        assert!(result.output.contains("No config found"), "missing warning");
    }

    #[test]
    fn format_config_result_structural_validity() {
        let mut layers = std::collections::HashMap::new();
        layers.insert(LayerNameVO::new("api"), LayerDefinition::default());
        let rules = vec![
            ArchitectureRule {
                name: DescriptionVO::new("Rule One"),
                scope: LayerNameVO::new("api"),
                ..Default::default()
            },
        ];
        let config = ArchitectureConfig {
            enabled: BooleanVO::new(true),
            layers,
            rules,
            ..Default::default()
        };
        let source = ConfigSource::new("rust", "/test/config.yaml", "test content");
        let cr = ConfigResult::new(config, source, vec![]);

        let formatted = LintExecutor::format_config_result(&cr);
        assert!(formatted.success);
        assert!(formatted.output.contains("Source: /test/config.yaml (rust)"));
        assert!(formatted.output.contains("Rules: 1"));
        assert!(formatted.output.contains("1. Rule One"));
        assert!(formatted.output.contains("Architecture Layers"));
        assert!(formatted.output.contains("api"));
    }
}
