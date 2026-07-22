use crate::utility_report_formatter::{
    format_config_result, format_dependency_report, format_doctor_report, format_results,
};
use shared::auto_fix::taxonomy_fix_vo::FixResult;
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_scan_request_vo::ScanRequest;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_manager_protocol::IHookManagerProtocol;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::project_setup::contract_maintenance_aggregate::MaintenanceCommandsAggregate;
use shared::project_setup::contract_setup_aggregate::SetupManagementAggregate;
use shared::project_setup::taxonomy_doctor_vo::DependencyReport;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared::tui::contract_lint_executor_protocol::ILintExecutorProtocol;
use shared::tui::taxonomy_action_flags_vo::ActionFlags;
use shared::tui::taxonomy_adapter_info_vo::AdapterInfo;
use shared::tui::taxonomy_lint_result_vo::LintExecutionResult;
use shared::tui::utility_tui_io as tui_io;
use std::sync::Arc;

// PURPOSE: Capabilities-layer lint executor — wraps ICodeAnalysisAggregate for the TUI.
// Implements ILintExecutorProtocol, providing all lint action methods (check, scan, fix, ci, etc.)
// with user-facing output formatting.

use shared::auto_fix::contract_fix_aggregate::LintFixOrchestratorAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct LintExecutor {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator: Option<Arc<dyn LintFixOrchestratorAggregate>>,
    setup_aggregate: Option<Arc<dyn SetupManagementAggregate>>,
    maintenance: Option<Arc<dyn MaintenanceCommandsAggregate>>,
    hook_port: Option<Arc<dyn IHookManagerProtocol>>,
    config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    external_lint: Option<Arc<dyn IExternalLintAggregate>>,
    orphan_aggregate: Option<Arc<dyn IOrphanAggregate>>,
    import_orchestrator: Option<Arc<dyn IImportRunnerAggregate>>,
    naming_orchestrator: Option<Arc<dyn INamingRunnerAggregate>>,
    role_orchestrator: Option<Arc<dyn IRoleRunnerAggregate>>,
    analysis_pipeline: Option<Arc<dyn IAnalysisPipelineAggregate>>,
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl ILintExecutorProtocol for LintExecutor {
    fn check(&self, path: &str, _flags: &ActionFlags) -> LintExecutionResult {
        let fp = shared::common::taxonomy_path_vo::FilePath::new(path).unwrap_or_default();
        let results = self.code_analysis.run_code_analysis(&fp);
        let count = results.len();
        let output = format_results(&results).to_string();
        LintExecutionResult {
            output,
            violation_count: count,
            success: count == 0,
        }
    }

    fn scan(&self, path: &str) -> LintExecutionResult {
        self.run_comprehensive_scan(path)
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
                let fp = shared::common::taxonomy_path_vo::FilePath::new(path).unwrap_or_default();
                let results = self.code_analysis.run_code_analysis(&fp);
                let count_before = results.len();
                let output = format!("[{}] Fix scan on {}\nViolations found: {}\nFix application requires FixOrchestrator aggregate.\nUse CLI `lint-arwaky-cli fix {}` for full fix pipeline.", mode, path, count_before, path);
                LintExecutionResult {
                    output,
                    violation_count: count_before,
                    success: false,
                }
            }
        }
    }

    fn ci(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult {
        let fp = shared::common::taxonomy_path_vo::FilePath::new(path).unwrap_or_default();
        let results = self.code_analysis.run_code_analysis(&fp);
        let score = self.code_analysis.calc_score(&results.values);
        let has_critical = self.code_analysis.check_critical(&results.values);
        let pass = score.value() >= flags.threshold as f64 && !has_critical;
        let status = if pass { "PASS" } else { "FAIL" };
        let output = format!("CI Report for {}\nScore: {:.1}/100 (threshold: {})\nViolations: {}\nCritical: {}\nStatus: {}", path, score, flags.threshold, results.len(), has_critical, status);
        if pass {
            LintExecutionResult::success(output, results.len())
        } else {
            LintExecutionResult {
                output,
                violation_count: results.len(),
                success: false,
            }
        }
    }

    fn orphan(&self, path: &str) -> LintExecutionResult {
        match &self.orphan_aggregate {
            Some(orphan_agg) => {
                // Resolve workspace root like CLI does
                let scan_root = shared::common::utility_file::find_workspace_root(path)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.to_string());
                let dir_path =
                    shared::common::taxonomy_path_vo::DirectoryPath::new(scan_root.clone())
                        .unwrap_or_default();
                let ignored = self
                    .config_orchestrator
                    .as_ref()
                    .map(|o| o.ignored_paths(&scan_root))
                    .unwrap_or_default();
                let source_files =
                    match shared::common::utility_file::scan_directory(&dir_path, &ignored)
                    {
                        Ok(list) => list.values,
                        Err(e) => {
                            return LintExecutionResult::failure(format!(
                                "Orphan detection for {}\nFailed to scan directory: {}",
                                path, e
                            ));
                        }
                    };
                let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
                if file_strs.is_empty() {
                    return LintExecutionResult::success(
                        format!(
                            "Orphan detection for {}\nNo source files found in {}.",
                            path, scan_root
                        ),
                        0,
                    );
                }
                let results = orphan_agg.check_orphans(&file_strs, &scan_root);
                let count = results.len();
                let mut output = format!(
                    "Orphan detection for {}\nScanned {} files in {}\n",
                    path,
                    file_strs.len(),
                    scan_root
                );
                if results.is_empty() {
                    output.push_str("No orphan files detected.\n");
                } else {
                    output.push_str(&format!("Found {} orphan(s):\n\n", count));
                    for (i, result) in results.iter().enumerate() {
                        output.push_str(&format!(
                            "{}. [{}] {} — {}\n   Code: {} | Severity: {}\n\n",
                            i + 1,
                            result
                                .source
                                .as_ref()
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| "unknown".to_string()),
                            result.file,
                            result.message,
                            result.code,
                            result.severity
                        ));
                    }
                }
                LintExecutionResult::success(output, count)
            }
            _ => {
                let output = format!("Orphan detection for {}\nUse CLI `lint-arwaky-cli orphan {}` for full orphan graph analysis.", path, path);
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn security(&self, path: &str) -> LintExecutionResult {
        match &self.external_lint {
            Some(ext_lint) => {
                let fp = shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
                    .unwrap_or_default();
                let rt = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(e) => {
                        return LintExecutionResult::failure(format!(
                            "Failed to create runtime for security scan: {}",
                            e
                        ));
                    }
                };
                let results = rt.block_on(ext_lint.scan_all(&fp));
                let security_names = ["cargo-audit", "bandit"];
                let security_results: LintResultList = LintResultList::new(
                    results
                        .values
                        .iter()
                        .filter(|r| {
                            r.source
                                .as_ref()
                                .is_some_and(|s| security_names.iter().any(|n| s.contains(n)))
                        })
                        .cloned()
                        .collect(),
                );
                let security_count = security_results.len();
                let output = if security_count == 0 {
                    format!("Security scan for {}\n{} total lint results, none from security adapters (cargo-audit, bandit).\nAdapters scanned: {}", path, results.len(), ext_lint.adapter_names().join(", "))
                } else {
                    let mut out = format!(
                        "Security scan for {}\nFound {} finding(s) from security adapters:\n\n",
                        path, security_count
                    );
                    for (i, result) in security_results.iter().enumerate() {
                        out.push_str(&format!(
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
                            result.severity
                        ));
                    }
                    out
                };
                LintExecutionResult::success(output, security_count)
            }
            None => {
                let output = format!("Security scan for {}\nUse CLI `lint-arwaky-cli security {}` for full vulnerability scan.", path, path);
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn duplicates(&self, path: &str) -> LintExecutionResult {
        let scan_root = shared::common::utility_file::find_workspace_root(path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string());

        let dir_path = shared::common::taxonomy_path_vo::DirectoryPath::new(scan_root.clone())
            .unwrap_or_default();
        let ignored = self
            .config_orchestrator
            .as_ref()
            .map(|o| o.ignored_paths(&scan_root))
            .unwrap_or_default();
        let source_files =
            match shared::common::utility_file::scan_directory(&dir_path, &ignored) {
                Ok(list) => list.values,
                Err(_) => Vec::new(),
            };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();

        let entries =
            shared::code_analysis::utility_code_duplication_detector::collect_file_entries(
                &file_strs,
            );
        let blocks =
            shared::code_analysis::utility_code_duplication_detector::scan_duplicate_blocks(
                entries, 10,
            );
        let violations = shared::code_analysis::utility_code_duplication_detector::build_violations(
            &blocks,
            file_strs.len() * 100,
            10,
        );
        let count = violations.len();
        let mut output = format!(
            "Duplication detection for {}\nScanned {} files\n",
            path,
            file_strs.len()
        );
        if violations.is_empty() {
            output.push_str("No significant code duplication detected.\n");
        } else {
            output.push_str(&format!("Found {} duplication violation(s):\n\n", count));
            for (i, v) in violations.iter().enumerate() {
                let msg = match v {
                    shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation::CodeDuplication { reason } => {
                        reason.as_ref().map(|r| r.value.clone()).unwrap_or_default()
                    }
                    _ => String::new(),
                };
                output.push_str(&format!("{}. {}\n\n", i + 1, msg));
            }
        }
        LintExecutionResult::success(output, count)
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
                let output = format!("Dependency scan for {}\nUse CLI `lint-arwaky-cli dependencies {}` for full report.", path, path);
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn doctor(&self) -> LintExecutionResult {
        match &self.maintenance {
            Some(maintenance) => {
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
                let output = "Environment Diagnostics:\nUse CLI `lint-arwaky-cli maintenance doctor` for full environment check.\nRequired: Rust toolchain, Python 3.8+, Node.js 18+".to_string();
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
                    "Adapter dependency installation.\nDetected language: {}\n",
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
                output.push_str(&format!("  {} Python (ruff, mypy, bandit)\n", py_icon));

                let js_result = rt.block_on(protocol.install_javascript_adapters(false));
                let js_icon = if js_result.value { "[OK]" } else { "[FAIL]" };
                let js_failed = !js_result.value;
                output.push_str(&format!(
                    "  {} JavaScript (eslint, prettier, typescript)\n",
                    js_icon
                ));

                if !py_result.value || js_failed {
                    output.push_str("\nSome adapter(s) failed to install.\n");
                    LintExecutionResult::failure(output)
                } else {
                    output.push_str("\nAll adapter dependencies installed.\n");
                    LintExecutionResult::success(output, 0)
                }
            }
            None => {
                let output = "Adapter dependency installation.\nUse CLI `lint-arwaky-cli setup install` to install all adapter dependencies.".to_string();
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn mcp_config(&self, flags: &ActionFlags) -> LintExecutionResult {
        match &self.setup_aggregate {
            Some(setup) => {
                let transport =
                    shared::cli_commands::taxonomy_protocol_vo::TransportProtocol::STDAggregate;
                let config_vo = match flags.mcp_client.as_str() {
                    "claude" => setup.mcp_config_claude(&transport),
                    "hermes" => setup.mcp_config_hermes(&transport),
                    "vscode" => setup.mcp_config_vscode(&transport),
                    _ => setup.generate_mcp_config(&transport),
                };
                let json = match serde_json::to_string_pretty(&config_vo.value) {
                    Ok(j) => j,
                    Err(e) => {
                        return LintExecutionResult::failure(format!(
                            "MCP config serialization failed: {}",
                            e
                        ));
                    }
                };
                let output = format!(
                    "MCP Configuration (client: {})\n  Transport: Stdio\n\n{}",
                    flags.mcp_client, json
                );
                LintExecutionResult::success(output, 0)
            }
            None => {
                let output = format!(
                    "MCP Configuration for client: {}.\n  Use CLI `lint-arwaky-cli setup mcp-config --client {}` to print config.",
                    flags.mcp_client, flags.mcp_client
                );
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn config_show(&self) -> LintExecutionResult {
        match &self.config_orchestrator {
            Some(orchestrator) => {
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
                let output = "Active Configuration\nSource: embedded (built-in defaults)\nNo config orchestrator configured. Use CLI `lint-arwaky-cli config-show`.".to_string();
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn install_hook(&self) -> LintExecutionResult {
        match &self.hook_port {
            Some(port) => {
                let exe_path_str = std::env::current_exe()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "lint-arwaky-cli".to_string());
                let exe_path = shared::common::taxonomy_path_vo::FilePath::new(exe_path_str)
                    .unwrap_or_default();
                match port.install_pre_commit(&exe_path) {
                    Ok(status) => {
                        if status.value {
                            LintExecutionResult::success("Git pre-commit hook installed successfully.".to_string(), 0)
                        } else {
                            LintExecutionResult::failure("Git pre-commit hook installation failed.\nNot a git repository? Run `git init` first.".to_string())
                        }
                    }
                    Err(e) => LintExecutionResult::failure(format!("Git pre-commit hook installation failed.\nError: {}", e)),
                }
            }
            None => LintExecutionResult::success("Git pre-commit hook installation.\nUse CLI `lint-arwaky-cli install-hook` to install.".to_string(), 0),
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
                        LintExecutionResult::success("No git pre-commit hook found (not a git repo or hook already removed).".to_string(), 0)
                    }
                }
                Err(e) => LintExecutionResult::failure(format!(
                    "Git pre-commit hook removal failed.\nError: {}",
                    e
                )),
            },
            None => LintExecutionResult::success(
                "Git pre-commit hook removal.\nUse CLI `lint-arwaky-cli uninstall-hook` to remove."
                    .to_string(),
                0,
            ),
        }
    }

    fn adapters(&self) -> LintExecutionResult {
        let adapters = Self::discover_adapters();
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl LintExecutor {
    pub fn new(code_analysis: Arc<dyn ICodeAnalysisAggregate>) -> Self {
        Self {
            code_analysis,
            fix_orchestrator: None,
            setup_aggregate: None,
            maintenance: None,
            hook_port: None,
            config_orchestrator: None,
            external_lint: None,
            orphan_aggregate: None,
            import_orchestrator: None,
            naming_orchestrator: None,
            role_orchestrator: None,
            analysis_pipeline: None,
        }
    }

    pub fn with_fix(mut self, fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>) -> Self {
        self.fix_orchestrator = Some(fix_orchestrator);
        self
    }

    pub fn with_setup(mut self, setup_aggregate: Arc<dyn SetupManagementAggregate>) -> Self {
        self.setup_aggregate = Some(setup_aggregate);
        self
    }

    pub fn with_maintenance(mut self, maintenance: Arc<dyn MaintenanceCommandsAggregate>) -> Self {
        self.maintenance = Some(maintenance);
        self
    }

    pub fn with_hook_port(mut self, hook_port: Arc<dyn IHookManagerProtocol>) -> Self {
        self.hook_port = Some(hook_port);
        self
    }

    pub fn with_config(
        mut self,
        config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    ) -> Self {
        self.config_orchestrator = Some(config_orchestrator);
        self
    }

    pub fn with_external_lint(mut self, external_lint: Arc<dyn IExternalLintAggregate>) -> Self {
        self.external_lint = Some(external_lint);
        self
    }

    pub fn with_orphan(mut self, orphan_aggregate: Arc<dyn IOrphanAggregate>) -> Self {
        self.orphan_aggregate = Some(orphan_aggregate);
        self
    }

    pub fn with_import_orchestrator(
        mut self,
        import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    ) -> Self {
        self.import_orchestrator = Some(import_orchestrator);
        self
    }

    pub fn with_naming_orchestrator(
        mut self,
        naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    ) -> Self {
        self.naming_orchestrator = Some(naming_orchestrator);
        self
    }

    pub fn with_role_orchestrator(
        mut self,
        role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    ) -> Self {
        self.role_orchestrator = Some(role_orchestrator);
        self
    }

    pub fn with_multi_project_orchestrator(
        mut self,
        multi_project_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    ) -> Self {
        self.config_orchestrator = Some(multi_project_orchestrator);
        self
    }

    pub fn with_analysis_pipeline(
        mut self,
        analysis_pipeline: Arc<dyn IAnalysisPipelineAggregate>,
    ) -> Self {
        self.analysis_pipeline = Some(analysis_pipeline);
        self
    }

    pub fn format_results(&self, results: &LintResultList) -> String {
        format_results(results).to_string()
    }

    fn format_doctor_report(
        diagnostics: &shared::project_setup::taxonomy_doctor_vo::ToolchainDiagnostics,
    ) -> LintExecutionResult {
        format_doctor_report(diagnostics)
    }

    fn run_init(&self) -> LintExecutionResult {
        match &self.setup_aggregate {
            Some(protocol) => {
                let languages = protocol.detect_languages();
                let mut created = Vec::new();
                let mut skipped = Vec::new();
                let mut errors = Vec::new();
                for lang in languages.iter() {
                    let lang_str = lang.value();
                    let config_path = format!("lint_arwaky.config.{}.yaml", lang_str);
                    if protocol.file_exists(&config_path) {
                        skipped.push(config_path);
                        continue;
                    }
                    let template = protocol.get_config_template(lang_str);
                    match protocol.write_config_file(&config_path, template) {
                        Ok(desc) => {
                            created
                                .push(format!("{} ({}) — {}", config_path, lang_str, desc.value));
                        }
                        Err(e) => {
                            errors.push(format!("{} — error: {}", config_path, e));
                        }
                    }
                }
                let mut output = String::from("Config initialization.\n");
                if !created.is_empty() {
                    output.push_str(&format!("Created:\n  {}\n", created.join("\n  ")));
                }
                if !skipped.is_empty() {
                    output.push_str(&format!("Already exist:\n  {}\n", skipped.join("\n  ")));
                }
                if !errors.is_empty() {
                    output.push_str(&format!("Errors:\n  {}\n", errors.join("\n  ")));
                    return LintExecutionResult::failure(output);
                }
                LintExecutionResult::success(output, 0)
            }
            None => {
                let output =
                    "Config initialization.\nUse CLI `lint-arwaky-cli init` to create configuration."
                        .to_string();
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn format_dependency_report(path: &str, report: &DependencyReport) -> LintExecutionResult {
        format_dependency_report(path, report)
    }

    fn format_config_result(
        result: &shared::config_system::taxonomy_source_vo::ConfigResult,
    ) -> LintExecutionResult {
        format_config_result(result)
    }

    /// Check if a binary is available in the system PATH.
    fn is_binary_available(b: &str) -> bool {
        tui_io::is_binary_available(b)
    }

    /// Discover available linter adapters and check binary availability.
    fn discover_adapters() -> Vec<AdapterInfo> {
        let mut list = vec![
            ("ast_rust_scanner", "Rust AST (built-in)", true),
            ("ast_py_scanner", "Python AST (built-in)", true),
            ("ast_js_scanner", "JS/TS AST (built-in)", true),
        ]
        .into_iter()
        .map(|(n, l, i)| AdapterInfo {
            name: n.into(),
            label: l.into(),
            installed: i,
        })
        .collect::<Vec<_>>();
        for (b, l) in [
            ("clippy", "Clippy (Rust)"),
            ("ruff", "Ruff (Python)"),
            ("mypy", "MyPy (Python)"),
            ("bandit", "Bandit (Python)"),
            ("radon", "Radon (Python metrics)"),
            ("eslint", "ESLint (JavaScript)"),
            ("prettier", "Prettier (JavaScript)"),
            ("tsc", "TypeScript Compiler"),
        ] {
            list.push(AdapterInfo {
                name: b.into(),
                label: l.into(),
                installed: Self::is_binary_available(b),
            });
        }
        list
    }
}

impl LintExecutor {
    fn run_comprehensive_scan(&self, path: &str) -> LintExecutionResult {
        // Delegate to AnalysisPipelineOrchestrator — centralizes all 6-linter pipeline logic.
        match &self.analysis_pipeline {
            Some(pipeline) => {
                let rt = match tokio::runtime::Runtime::new() {
                    Ok(rt) => rt,
                    Err(e) => {
                        return LintExecutionResult::failure(format!(
                            "Failed to create runtime: {}",
                            e
                        ));
                    }
                };

                // Use run_with_discovery for multi-workspace support.
                // Falls back to single-scan mode if no workspaces found.
                let request = ScanRequest::new(
                    shared::cli_commands::taxonomy_scan_request_vo::ScanTarget::new(
                        path.to_string(),
                    ),
                    shared::cli_commands::taxonomy_scan_request_vo::ScanMode::Scan,
                );

                match rt.block_on(pipeline.run(request)) {
                    Ok(report) => {
                        let count = report.results.len();
                        let results = LintResultList::new(report.results);
                        let output = self.format_results(&results);
                        LintExecutionResult::success(output, count)
                    }
                    Err(e) => LintExecutionResult::failure(format!("Pipeline error: {}", e)),
                }
            }
            None => {
                // Fallback: legacy inline pipeline (should never happen in production).
                self.run_legacy_scan(path)
            }
        }
    }

    /// Legacy inline pipeline — kept as fallback only. All callers should use analysis_pipeline.
    fn run_legacy_scan(&self, path: &str) -> LintExecutionResult {
        let mut all_results = Vec::new();

        // 1. AES code analysis
        let aes_fp =
            shared::common::taxonomy_path_vo::FilePath::new(path.to_string()).unwrap_or_default();
        let aes_results = self.code_analysis.run_code_analysis(&aes_fp);
        all_results.extend(aes_results.values);

        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return LintExecutionResult::failure(format!("Failed to create runtime: {}", e));
            }
        };

        // 2. Naming rules audit (AES101-102)
        if let Some(ref naming) = self.naming_orchestrator {
            let path_obj = shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
                .unwrap_or_default();
            let naming_results = rt.block_on(naming.run_audit(&path_obj)).unwrap_or_default();
            all_results.extend(naming_results);
        }

        // 3. Import rules audit (AES201-205, cycles)
        if let Some(ref import) = self.import_orchestrator {
            let path_obj = shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
                .unwrap_or_default();
            let import_results = rt.block_on(import.run_audit(&path_obj)).unwrap_or_default();
            all_results.extend(import_results);
        }

        // 4. External linter adapters (Clippy, Ruff, ESLint, etc.)
        if let Some(ref external_lint) = self.external_lint {
            let fp = shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
                .unwrap_or_default();
            let ext_results = rt.block_on(external_lint.scan_all(&fp));
            all_results.extend(ext_results.values);
        }

        // 5. Role rules audit (AES401-406)
        if let Some(ref role) = self.role_orchestrator {
            let path_obj = shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
                .unwrap_or_default();
            let role_results = rt.block_on(role.run_audit(&path_obj));
            all_results.extend(role_results);
        }

        // 6. Orphan detection (AES501-506)
        if let Some(ref orphan_agg) = self.orphan_aggregate {
            let dir_path = shared::common::taxonomy_path_vo::DirectoryPath::new(path.to_string())
                .unwrap_or_default();
            let ignored = self
                .config_orchestrator
                .as_ref()
                .map(|o| o.ignored_paths(path))
                .unwrap_or_default();
            let source_files =
                match shared::common::utility_file::scan_directory(&dir_path, &ignored) {
                    Ok(list) => list.values,
                    Err(_) => Vec::new(),
                };
            let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
            if !file_strs.is_empty() {
                let orphan_results = orphan_agg.check_orphans(&file_strs, path);
                all_results.extend(orphan_results);
            }
        }

        let count = all_results.len();
        let results = LintResultList::new(all_results);
        let output = self.format_results(&results);
        LintExecutionResult::success(output, count)
    }
}
