// PURPOSE: Surface-layer lint executor — facade over dispatcher functions for the TUI.
// Provides all lint action methods (check, scan, fix, ci, etc.) with user-facing output
// formatting. Delegates to dispatcher crate for business logic; formats output as
// LintExecutionResult for the action handler.
// All methods are synchronous — consistent with dispatcher sync API.
use dispatcher::surface_check_action::{ScanOptions, collect_scan};
use dispatcher::surface_ci_action::{CiScanDeps, collect_ci};
use dispatcher::surface_config_action::collect_config_show;
use dispatcher::surface_fix_action::collect_fix_direct;
use dispatcher::surface_git_action::{collect_install_hook, collect_uninstall_hook};
use dispatcher::surface_maintenance_action::{
    collect_dependencies, collect_doctor, collect_security,
};
use dispatcher::surface_orphan_action::{OrphanScanDeps, collect_orphan};
use dispatcher::surface_plugin_action::collect_adapters_detailed;
use dispatcher::surface_setup_action::{collect_init, collect_install, collect_mcp_config};
use dispatcher::surface_version_action::collect_version;

use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::common::FilePath;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::project_setup::SetupManagementAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::role_rules::IRoleRunnerAggregate;
use shared::tui::{ActionFlags, LintExecutionResult};

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SurfaceLintExecutor {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator: Option<Arc<dyn LintFixOrchestratorAggregate>>,
    setup_aggregate: Option<Arc<dyn SetupManagementAggregate>>,
    maintenance: Option<Arc<dyn MaintenanceCommandsAggregate>>,
    hook_port: Option<Arc<dyn GitHooksAggregate>>,
    config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    external_lint: Option<Arc<dyn IExternalLintAggregate>>,
    orphan_aggregate: Option<Arc<dyn IOrphanAggregate>>,
    import_orchestrator: Option<Arc<dyn IImportRunnerAggregate>>,
    naming_orchestrator: Option<Arc<dyn INamingRunnerAggregate>>,
    role_orchestrator: Option<Arc<dyn IRoleRunnerAggregate>>,
    filesystem: Arc<dyn IFilesystemAggregate>,
}

// ─── Block 2: Lint Action Methods ─────────────────────────

impl SurfaceLintExecutor {
    pub fn check(&self, path: &str, _flags: &ActionFlags) -> LintExecutionResult {
        let opts = ScanOptions {
            path: Some(FilePath::new(path.to_string()).unwrap_or_default()),
            multi_project_orchestrator: self.config_orchestrator.clone(),
            filter: None,
            member: None,
            filesystem: self.filesystem.clone(),
        };
        match collect_scan(opts) {
            Ok(violations) => {
                let count = violations.len();
                let output = format_violations(path, &violations);
                LintExecutionResult::success(output, count)
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    pub fn scan(&self, path: &str) -> LintExecutionResult {
        let opts = ScanOptions {
            path: Some(FilePath::new(path.to_string()).unwrap_or_default()),
            multi_project_orchestrator: self.config_orchestrator.clone(),
            filter: None,
            member: None,
            filesystem: self.filesystem.clone(),
        };
        match collect_scan(opts) {
            Ok(violations) => {
                let total = violations.len();
                let mut output = format!(
                    "Comprehensive scan for {}\nViolations found: {}\n\n",
                    path, total
                );
                for (i, v) in violations.iter().enumerate() {
                    output.push_str(&format!(
                        "{}. [{}] {}:{} — {}\n   Code: {} | Severity: {}\n\n",
                        i + 1,
                        v.severity,
                        v.file,
                        v.line.value,
                        v.message,
                        v.code,
                        v.severity
                    ));
                }
                LintExecutionResult::success(output, total)
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    pub fn fix(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult {
        let fix_orch = match &self.fix_orchestrator {
            Some(o) => o.clone(),
            None => {
                let output = format!(
                    "[{}] Fix scan on {}\nFix application requires FixOrchestrator aggregate.\nUse CLI `lint-arwaky-cli fix {}` for full fix pipeline.",
                    if flags.dry_run { "DRY-RUN" } else { "LIVE" },
                    path,
                    path
                );
                return LintExecutionResult::failure(output);
            }
        };
        let fp = Some(FilePath::new(path.to_string()).unwrap_or_default());
        match collect_fix_direct(fp, flags.dry_run, self.code_analysis.clone(), fix_orch) {
            Ok(report) => {
                let mode = if report.dry_run { "DRY-RUN" } else { "LIVE" };
                let output = format!("[{}] {}", mode, report.output);
                if report.success {
                    LintExecutionResult::success(output, report.fixed_count)
                } else {
                    LintExecutionResult::failure(output)
                }
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    pub fn ci(&self, path: &str, flags: &ActionFlags) -> LintExecutionResult {
        let deps = match self.build_ci_deps() {
            Some(d) => d,
            None => {
                return LintExecutionResult::failure(
                    "CI validation requires quality, import, naming, orphan, and config aggregates."
                        .to_string(),
                );
            }
        };
        let fp = Some(FilePath::new(path.to_string()).unwrap_or_default());
        let threshold = shared::common::Threshold::new(flags.threshold);
        match collect_ci(deps, fp, threshold) {
            Ok(report) => {
                let status = if report.pass { "PASS" } else { "FAIL" };
                let mut output = format!(
                    "CI Report for {}\nScore: {:.1}/100 (threshold: {})\nViolations: {}\nCritical: {}\nStatus: {}",
                    path,
                    report.score,
                    report.threshold,
                    report.total_violations,
                    report.critical,
                    status
                );
                if !report.reasons.is_empty() {
                    output.push_str("\nReasons:\n");
                    for r in &report.reasons {
                        output.push_str(&format!("  - {}\n", r));
                    }
                }
                LintExecutionResult {
                    output,
                    violation_count: report.total_violations,
                    success: report.pass,
                }
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    pub fn orphan(&self, path: &str) -> LintExecutionResult {
        let deps = match self.build_orphan_deps() {
            Some(d) => d,
            None => {
                let output = format!(
                    "Orphan detection for {}\nUse CLI `lint-arwaky-cli orphan {}` for full orphan graph analysis.",
                    path, path
                );
                return LintExecutionResult::success(output, 0);
            }
        };
        let fp = Some(FilePath::new(path.to_string()).unwrap_or_default());
        match collect_orphan(fp, None, deps, None) {
            Ok(violations) => {
                let count = violations.len();
                let mut output = format!("Orphan detection for {}\n", path);
                if violations.is_empty() {
                    output.push_str("No orphan files detected.\n");
                } else {
                    output.push_str(&format!("Found {} orphan(s):\n\n", count));
                    for (i, v) in violations.iter().enumerate() {
                        output.push_str(&format!(
                            "{}. [{}] {} — {}\n   Code: {} | Severity: {}\n\n",
                            i + 1,
                            v.severity,
                            v.file,
                            v.message,
                            v.code,
                            v.severity
                        ));
                    }
                }
                LintExecutionResult::success(output, count)
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    pub fn security(&self, path: &str) -> LintExecutionResult {
        let maintenance = match &self.maintenance {
            Some(m) => m.clone(),
            None => {
                let output = format!(
                    "Security scan for {}\nUse CLI `lint-arwaky-cli security {}` for full vulnerability scan.",
                    path, path
                );
                return LintExecutionResult::success(output, 0);
            }
        };
        let fp = Some(FilePath::new(path.to_string()).unwrap_or_default());
        match collect_security(maintenance, fp) {
            Ok(report) => {
                let count = report.findings.len();
                let mut output = format!(
                    "Security scan for {}\nTool: {} (installed: {})\nLanguage: {}\n\n",
                    path, report.tool_name, report.tool_installed, report.language
                );
                if report.findings.is_empty() {
                    output.push_str("No security findings.\n");
                } else {
                    output.push_str(&format!("Found {} finding(s):\n\n", count));
                    for (i, f) in report.findings.iter().enumerate() {
                        output.push_str(&format!(
                            "{}. [{}] {} — {}\n   File: {}:{}\n\n",
                            i + 1,
                            f.severity,
                            f.test_id,
                            f.issue,
                            f.file,
                            f.line
                        ));
                    }
                }
                LintExecutionResult::success(output, count)
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    pub fn dependencies(&self, path: &str) -> LintExecutionResult {
        let maintenance = match &self.maintenance {
            Some(m) => m.clone(),
            None => {
                let output = format!(
                    "Dependency scan for {}\nUse CLI `lint-arwaky-cli dependencies {}` for full report.",
                    path, path
                );
                return LintExecutionResult::success(output, 0);
            }
        };
        let fp = Some(FilePath::new(path.to_string()).unwrap_or_default());
        match collect_dependencies(maintenance, fp) {
            Ok(report) => {
                let count = report.dependencies.len();
                let mut output = format!(
                    "Dependency scan for {}\nLanguage: {}\nTotal: {}\n",
                    path, report.language, count
                );
                for dep in report.dependencies.iter().take(30) {
                    output.push_str(&format!("  {} {}\n", dep.name, dep.version));
                }
                if count > 30 {
                    output.push_str(&format!("  ... and {} more\n", count - 30));
                }
                LintExecutionResult::success(output, count)
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    pub fn doctor(&self) -> LintExecutionResult {
        let maintenance = match &self.maintenance {
            Some(m) => m.clone(),
            None => {
                return LintExecutionResult::success(
                    "Environment Diagnostics:\nUse CLI `lint-arwaky-cli maintenance doctor` for full environment check.\nRequired: Rust toolchain, Python 3.8+, Node.js 18+".to_string(),
                    0,
                );
            }
        };
        let diagnostics = collect_doctor(maintenance);
        crate::utility_report_formatter::format_doctor_report(&diagnostics)
    }

    pub fn init(&self, _flags: &ActionFlags) -> LintExecutionResult {
        let setup = match &self.setup_aggregate {
            Some(s) => s.clone(),
            None => {
                return LintExecutionResult::success(
                    "Config initialization.\nUse CLI `lint-arwaky-cli init` to create configuration.".to_string(),
                    0,
                );
            }
        };
        let items = collect_init(setup, self.filesystem.clone());
        let mut output = String::from("Config initialization.\n");
        let mut has_errors = false;
        for item in &items {
            if item.ok {
                output.push_str(&format!("  {}\n", item.message));
            } else {
                output.push_str(&format!("  [ERROR] {}\n", item.message));
                has_errors = true;
            }
        }
        if has_errors {
            LintExecutionResult::failure(output)
        } else {
            LintExecutionResult::success(output, 0)
        }
    }

    pub fn install(&self, _flags: &ActionFlags) -> LintExecutionResult {
        let setup = match &self.setup_aggregate {
            Some(s) => s.clone(),
            None => {
                return LintExecutionResult::success(
                    "Adapter dependency installation.\nUse CLI `lint-arwaky-cli setup install` to install all adapter dependencies.".to_string(),
                    0,
                );
            }
        };
        let report = collect_install(setup, false);
        let mut output = String::from("Adapter dependency installation.\n");
        output.push_str(&format!(
            "  Python adapters: {}\n",
            if report.py_ok { "OK" } else { "FAILED" }
        ));
        output.push_str(&format!(
            "  JS/TS adapters: {}\n",
            if report.js_ok { "OK" } else { "FAILED" }
        ));
        LintExecutionResult::success(output, 0)
    }

    pub fn mcp_config(&self, flags: &ActionFlags) -> LintExecutionResult {
        let report = collect_mcp_config(&flags.mcp_client);
        let output = format!(
            "MCP Configuration (client: {})\n  Binary: {}\n\n{}",
            report.client, report.binary, report.config_json
        );
        LintExecutionResult::success(output, 0)
    }

    pub fn config_show(&self) -> LintExecutionResult {
        let orchestrator = match &self.config_orchestrator {
            Some(o) => o.clone(),
            None => {
                return LintExecutionResult::success(
                    "Active Configuration\nSource: embedded (built-in defaults)\nNo config orchestrator configured. Use CLI `lint-arwaky-cli config-show`.".to_string(),
                    0,
                );
            }
        };
        let report = collect_config_show(orchestrator);
        let mut output = String::from("Active Configuration\n");
        for entry in &report.entries {
            output.push_str(&format!(
                "\n== {} ({}) ==\n{}\n",
                entry.language, entry.path, entry.content
            ));
        }
        if !report.warnings.is_empty() {
            output.push_str("\nWarnings:\n");
            for w in &report.warnings {
                output.push_str(&format!("  {}\n", w));
            }
        }
        LintExecutionResult::success(output, 0)
    }

    pub fn install_hook(&self) -> LintExecutionResult {
        let hooks = match &self.hook_port {
            Some(h) => h.clone(),
            None => {
                return LintExecutionResult::success(
                    "Git pre-commit hook installation.\nUse CLI `lint-arwaky-cli install-hook` to install."
                        .to_string(),
                    0,
                );
            }
        };
        let exe_path = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "lint-arwaky-cli".to_string());
        let fp = FilePath::new(exe_path).unwrap_or_default();
        match collect_install_hook(hooks, &fp) {
            Ok(report) => {
                if report.success {
                    LintExecutionResult::success(report.message, 0)
                } else {
                    LintExecutionResult::failure(report.message)
                }
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    pub fn uninstall_hook(&self) -> LintExecutionResult {
        let hooks = match &self.hook_port {
            Some(h) => h.clone(),
            None => {
                return LintExecutionResult::success(
                    "Git pre-commit hook removal.\nUse CLI `lint-arwaky-cli uninstall-hook` to remove."
                        .to_string(),
                    0,
                );
            }
        };
        match collect_uninstall_hook(hooks) {
            Ok(report) => {
                if report.success {
                    LintExecutionResult::success(report.message, 0)
                } else {
                    LintExecutionResult::failure(report.message)
                }
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    pub fn adapters(&self) -> LintExecutionResult {
        let adapters = collect_adapters_detailed(self.filesystem.as_ref());
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

    pub fn version(&self) -> LintExecutionResult {
        let report = collect_version();
        let output = format!("Lint Arwaky v{} (AES Semantic Builder)", report.version);
        LintExecutionResult::success(output, 0)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl SurfaceLintExecutor {
    pub fn new(
        code_analysis: Arc<dyn ICodeAnalysisAggregate>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            code_analysis,
            filesystem,
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

    pub fn with_hook_port(mut self, hook_port: Arc<dyn GitHooksAggregate>) -> Self {
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

    fn build_ci_deps(&self) -> Option<CiScanDeps> {
        Some(CiScanDeps {
            code_analysis_linter: self.code_analysis.clone(),
            import_orchestrator: self.import_orchestrator.clone()?,
            naming_orchestrator: self.naming_orchestrator.clone()?,
            config_orchestrator: self.config_orchestrator.clone()?,
            orphan_orchestrator: self.orphan_aggregate.clone()?,
            filesystem: self.filesystem.clone(),
        })
    }

    fn build_orphan_deps(&self) -> Option<OrphanScanDeps> {
        Some(OrphanScanDeps::new(
            self.orphan_aggregate.clone()?,
            self.config_orchestrator.clone()?,
            self.filesystem.clone(),
            Arc::new(|| {
                filesystem::root_filesystem_container::FilesystemContainer::new().orchestrator()
            }),
            Arc::new(|config, fs| {
                orphan_rules::root_orphan_detector_container::OrphanContainer::new_with_config(
                    config, fs,
                )
                .analyzer()
            }),
        ))
    }
}

fn format_violations(
    path: &str,
    violations: &[dispatcher::surface_output_component::ViolationItem],
) -> String {
    if violations.is_empty() {
        return format!("No violations found for {}.", path);
    }
    let mut output = format!("Found {} violation(s) for {}:\n\n", violations.len(), path);
    for (i, v) in violations.iter().enumerate() {
        output.push_str(&format!(
            "{}. [{}] {}:{} — {}\n   Code: {} | Severity: {}\n\n",
            i + 1,
            v.severity,
            v.file,
            v.line.value,
            v.message,
            v.code,
            v.severity
        ));
    }
    output
}
