// PURPOSE: Surface-layer lint executor — facade over dispatcher functions for the TUI.
// Provides all lint action methods (check, scan, fix, ci, etc.) with user-facing output
// formatting. Delegates to dispatcher crate for business logic; formats output as
// LintExecutionResult for the action handler.
// All methods are synchronous — consistent with dispatcher sync API.
use dispatcher::surface_check_action::{ScanOptions, collect_scan, collect_scan_with_progress};
use dispatcher::surface_ci_action::{CiScanDeps, collect_ci};
use dispatcher::surface_config_action::collect_config_show;
use dispatcher::surface_fix_action::collect_fix_direct;
use dispatcher::surface_git_action::{collect_install_hook, collect_uninstall_hook};
use dispatcher::surface_maintenance_action::{
    collect_dependencies, collect_doctor, collect_security,
};
use dispatcher::surface_orphan_action::{OrphanFactory, OrphanScanDeps, collect_orphan};
use dispatcher::surface_plugin_action::collect_adapters_detailed;
use dispatcher::surface_setup_action::{collect_init, collect_install, collect_mcp_config};
use dispatcher::surface_version_action::collect_version;

use crate::{ActionFlags, LintExecutionResult, LintOutcome};
use shared_auto_fix::IFixAggregate;
use shared_common::FilePath;
use shared_config_system::IConfigOrchestratorAggregate;
use shared_external_lint::IExternalLintAggregate;
use shared_filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared_git_hooks::IGitHooksAggregate;
use shared_import_rules::IImportRunnerAggregate;
use shared_maintenance::IMaintenanceAggregate;
use shared_naming_rules::INamingRunnerAggregate;
use shared_orphan_rules::IOrphanAggregate;
use shared_project_setup::ISetupAggregate;
use shared_quality_rules::ICodeAnalysisAggregate;
use shared_role_rules::IRoleRunnerAggregate;

use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct SurfaceLintExecutor {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
    fix_orchestrator: Option<Arc<dyn IFixAggregate>>,
    setup_aggregate: Option<Arc<dyn ISetupAggregate>>,
    maintenance: Option<Arc<dyn IMaintenanceAggregate>>,
    hook_port: Option<Arc<dyn IGitHooksAggregate>>,
    config_orchestrator: Option<Arc<dyn IConfigOrchestratorAggregate>>,
    external_lint: Option<Arc<dyn IExternalLintAggregate>>,
    orphan_aggregate: Option<Arc<dyn IOrphanAggregate>>,
    import_orchestrator: Option<Arc<dyn IImportRunnerAggregate>>,
    naming_orchestrator: Option<Arc<dyn INamingRunnerAggregate>>,
    role_orchestrator: Option<Arc<dyn IRoleRunnerAggregate>>,
    structure_orchestrator: Option<Arc<dyn shared_structure_rules::IStructureAggregate>>,
    doc_orchestrator: Option<Arc<dyn shared_doc_rules::IDocRunnerAggregate>>,
    filesystem: Arc<dyn IFilesystemAggregate>,
    filesystem_io: Arc<dyn shared_filesystem::IFileSystemIOProtocol>,
    filesystem_workspace: Arc<dyn shared_filesystem::IWorkspaceProtocol>,
    filesystem_tool_resolution: Arc<dyn shared_filesystem::IToolResolutionProtocol>,
    fs_seam: Arc<dispatcher::surface_check_action::FilesystemSeam>,
    fs_factory: Arc<dyn Fn() -> dispatcher::surface_check_action::FilesystemSeam + Send + Sync>,
    orphan_factory: Arc<OrphanFactory>,
}

/// Convert a UI path into the validated domain value exactly once at the
/// surface boundary. Never replace invalid input with `FilePath::default()`:
/// an empty path must be visible to the user as an action failure.
fn validated_path(path: &str) -> Result<FilePath, LintExecutionResult> {
    FilePath::new(path.to_string())
        .map_err(|_| LintExecutionResult::failure(format!("Invalid path: {path}")))
}

// ─── Block 2: Lint Action Methods ─────────────────────────

impl SurfaceLintExecutor {
    pub fn check(&self, path: &str, _flags: &ActionFlags) -> LintExecutionResult {
        let file_path = match validated_path(path) {
            Ok(path) => path,
            Err(error) => return error,
        };
        let opts = ScanOptions {
            path: Some(file_path),
            multi_project_orchestrator: self.config_orchestrator.clone(),
            filter: None,
            member: None,
            filesystem: Arc::new(self.fs_seam.as_ref().clone()),
            scan_aggregates: self.build_scan_aggregates(),
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
        let file_path = match validated_path(path) {
            Ok(path) => path,
            Err(error) => return error,
        };
        let opts = ScanOptions {
            path: Some(file_path),
            multi_project_orchestrator: self.config_orchestrator.clone(),
            filter: None,
            member: None,
            filesystem: Arc::new(self.fs_seam.as_ref().clone()),
            scan_aggregates: self.build_scan_aggregates(),
        };
        match collect_scan(opts) {
            Ok(violations) => {
                let total = violations.len();
                let output = format_violations(path, &violations);
                LintExecutionResult::success(output, total)
            }
            Err(e) => LintExecutionResult::failure(format!("Error: {e}")),
        }
    }

    /// Scan with dispatcher-owned progress milestones for the interactive TUI.
    pub fn scan_with_progress<F>(&self, path: &str, on_progress: F) -> LintExecutionResult
    where
        F: FnMut(String, usize, usize),
    {
        let file_path = match validated_path(path) {
            Ok(path) => path,
            Err(error) => return error,
        };
        let opts = ScanOptions {
            path: Some(file_path),
            multi_project_orchestrator: self.config_orchestrator.clone(),
            filter: None,
            member: None,
            filesystem: Arc::new(self.fs_seam.as_ref().clone()),
            scan_aggregates: self.build_scan_aggregates(),
        };
        match collect_scan_with_progress(opts, on_progress) {
            Ok(violations) => {
                let count = violations.len();
                LintExecutionResult::success(format_violations(path, &violations), count)
            }
            Err(error) => LintExecutionResult::failure(format!("Error: {error}")),
        }
    }

    /// Scan with a cooperative cancel token. Checks the flag between linter
    /// phases and stops early when set, returning a partial result with
    /// `cancelled = true` so callers can distinguish an early stop from a
    /// normal completion.
    pub fn scan_with_cancel<F>(
        &self,
        path: &str,
        cancel: &std::sync::atomic::AtomicBool,
        on_progress: F,
    ) -> LintExecutionResult
    where
        F: FnMut(String, usize, usize) + Send,
    {
        let file_path = match validated_path(path) {
            Ok(path) => path,
            Err(error) => return error,
        };
        let opts = ScanOptions {
            path: Some(file_path),
            multi_project_orchestrator: self.config_orchestrator.clone(),
            filter: None,
            member: None,
            filesystem: Arc::new(self.fs_seam.as_ref().clone()),
            scan_aggregates: self.build_scan_aggregates(),
        };
        match dispatcher::surface_check_action::collect_scan_with_cancel(opts, cancel, on_progress)
        {
            Ok(outcome) => {
                let count = outcome.violations.len();
                LintExecutionResult::success_cancelled(
                    format_violations(path, &outcome.violations),
                    count,
                    outcome.stopped_early,
                )
            }
            Err(error) => LintExecutionResult::failure(format!("Error: {error}")),
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
                return LintExecutionResult::unavailable(output);
            }
        };
        let fp = Some(match validated_path(path) {
            Ok(path) => path,
            Err(error) => return error,
        });
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
        let fp = Some(match validated_path(path) {
            Ok(path) => path,
            Err(error) => return error,
        });
        let threshold = shared_common::Threshold::new(flags.threshold);
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
                    outcome: if report.pass {
                        LintOutcome::Success
                    } else {
                        LintOutcome::Failure
                    },
                    success: report.pass,
                    cancelled: false,
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
                return LintExecutionResult::unavailable(output);
            }
        };
        let fp = Some(match validated_path(path) {
            Ok(path) => path,
            Err(error) => return error,
        });
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
                return LintExecutionResult::unavailable(output);
            }
        };
        let fp = Some(match validated_path(path) {
            Ok(path) => path,
            Err(error) => return error,
        });
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
                return LintExecutionResult::unavailable(output);
            }
        };
        let fp = Some(match validated_path(path) {
            Ok(path) => path,
            Err(error) => return error,
        });
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
                return LintExecutionResult::unavailable(
                    "Environment Diagnostics:\nUse CLI `lint-arwaky-cli maintenance doctor` for full environment check.\nRequired: Rust toolchain, Python 3.8+, Node.js 18+".to_string(),
                );
            }
        };
        let diagnostics = collect_doctor(maintenance);
        shared_tui::utility_report_formatter::format_doctor_report(&diagnostics)
    }

    pub fn init(&self, _flags: &ActionFlags) -> LintExecutionResult {
        let setup = match &self.setup_aggregate {
            Some(s) => s.clone(),
            None => {
                return LintExecutionResult::unavailable(
                    "Config initialization.\nUse CLI `lint-arwaky-cli init` to create configuration.".to_string(),
                );
            }
        };
        let items = collect_init(setup, self.filesystem_io.clone());
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
                return LintExecutionResult::unavailable(
                    "Adapter dependency installation.\nUse CLI `lint-arwaky-cli setup install` to install all adapter dependencies.".to_string(),
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
                return LintExecutionResult::unavailable(
                    "Active Configuration\nSource: embedded (built-in defaults)\nNo config orchestrator configured. Use CLI `lint-arwaky-cli config-show`.".to_string(),
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
                return LintExecutionResult::unavailable(
                    "Git pre-commit hook installation.\nUse CLI `lint-arwaky-cli install-hook` to install."
                        .to_string(),
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
                return LintExecutionResult::unavailable(
                    "Git pre-commit hook removal.\nUse CLI `lint-arwaky-cli uninstall-hook` to remove."
                        .to_string(),
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
        let adapters = collect_adapters_detailed(self.filesystem_tool_resolution.as_ref());
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
        filesystem_io: Arc<dyn shared_filesystem::IFileSystemIOProtocol>,
        filesystem_workspace: Arc<dyn shared_filesystem::IWorkspaceProtocol>,
        filesystem_tool_resolution: Arc<dyn shared_filesystem::IToolResolutionProtocol>,
        fs_seam: Arc<dispatcher::surface_check_action::FilesystemSeam>,
        fs_factory: Arc<dyn Fn() -> dispatcher::surface_check_action::FilesystemSeam + Send + Sync>,
        orphan_factory: Arc<OrphanFactory>,
    ) -> Self {
        Self {
            code_analysis,
            filesystem,
            filesystem_io,
            filesystem_workspace,
            filesystem_tool_resolution,
            fs_seam,
            fs_factory,
            orphan_factory,
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
            structure_orchestrator: None,
            doc_orchestrator: None,
        }
    }

    pub fn with_fix(mut self, fix_orchestrator: Arc<dyn IFixAggregate>) -> Self {
        self.fix_orchestrator = Some(fix_orchestrator);
        self
    }

    pub fn with_setup(mut self, setup_aggregate: Arc<dyn ISetupAggregate>) -> Self {
        self.setup_aggregate = Some(setup_aggregate);
        self
    }

    pub fn with_maintenance(mut self, maintenance: Arc<dyn IMaintenanceAggregate>) -> Self {
        self.maintenance = Some(maintenance);
        self
    }

    pub fn with_hook_port(mut self, hook_port: Arc<dyn IGitHooksAggregate>) -> Self {
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

    pub fn with_structure_orchestrator(
        mut self,
        structure_orchestrator: Arc<dyn shared_structure_rules::IStructureAggregate>,
    ) -> Self {
        self.structure_orchestrator = Some(structure_orchestrator);
        self
    }

    pub fn with_doc_orchestrator(
        mut self,
        doc_orchestrator: Arc<dyn shared_doc_rules::IDocRunnerAggregate>,
    ) -> Self {
        self.doc_orchestrator = Some(doc_orchestrator);
        self
    }

    fn build_scan_aggregates(&self) -> Option<dispatcher::surface_check_action::ScanAggregates> {
        Some(dispatcher::surface_check_action::ScanAggregates {
            quality: self.code_analysis.clone(),
            role: self.role_orchestrator.clone()?,
            import: self.import_orchestrator.clone()?,
            naming: self.naming_orchestrator.clone()?,
            external: self.external_lint.clone()?,
            orphan: self.orphan_aggregate.clone()?,
            config: self.config_orchestrator.clone()?,
            structure: self.structure_orchestrator.clone()?,
            doc: self.doc_orchestrator.clone()?,
            fs_seam: self.fs_seam.clone(),
        })
    }

    fn build_ci_deps(&self) -> Option<CiScanDeps> {
        Some(CiScanDeps {
            code_analysis_linter: self.code_analysis.clone(),
            import_orchestrator: self.import_orchestrator.clone()?,
            naming_orchestrator: self.naming_orchestrator.clone()?,
            config_orchestrator: self.config_orchestrator.clone()?,
            orphan_orchestrator: self.orphan_aggregate.clone()?,
            filesystem: self.filesystem.clone(),
            filesystem_io: self.filesystem_io.clone(),
        })
    }

    fn build_orphan_deps(&self) -> Option<OrphanScanDeps> {
        Some(OrphanScanDeps::new(
            self.orphan_aggregate.clone()?,
            self.config_orchestrator.clone()?,
            self.filesystem.clone(),
            self.filesystem_io.clone(),
            self.filesystem_workspace.clone(),
            self.fs_factory.clone(),
            self.orphan_factory.clone(),
        ))
    }
}

fn format_violations(path: &str, violations: &[shared_common::ViolationItem]) -> String {
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

#[cfg(test)]
mod tests {
    use super::validated_path;

    #[test]
    fn invalid_ui_path_is_a_visible_failure() {
        let error = validated_path("   ").expect_err("blank paths must be rejected");
        assert!(!error.success);
        assert!(error.output.contains("Invalid path"));
    }
}
