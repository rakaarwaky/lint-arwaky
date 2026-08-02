use crate::utility_report_formatter::{format_config_result, format_results};
use shared::auto_fix::FixResult;
use shared::cli_commands::LintResultList;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;

use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::project_setup::SetupManagementAggregate;
use shared::role_rules::IRoleRunnerAggregate;
use shared::tui::{ActionFlags, AdapterInfo, ILintExecutorProtocol, LintExecutionResult};

use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::filesystem::taxonomy_filesystem_vo::ToolName;
use std::sync::Arc;

// PURPOSE: Capabilities-layer lint executor — wraps ICodeAnalysisAggregate for the TUI.
// Implements ILintExecutorProtocol, providing all lint action methods (check, scan, fix, ci, etc.)
// with user-facing output formatting.
// All methods are synchronous — async aggregates fall back to CLI suggestions.

use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::file_watch::IWatchAggregate;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct LintExecutor {
    code_analysis: Arc<dyn ICodeAnalysisAggregate>,
    watch_aggregate: Option<Arc<dyn IWatchAggregate>>,
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
                let output = format!(
                    "[{}] Fix scan on {}\nViolations found: {}\nFix application requires FixOrchestrator aggregate.\nUse CLI `lint-arwaky-cli fix {}` for full fix pipeline.",
                    mode, path, count_before, path
                );
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
        let pass = score.value() >= flags.threshold as f64 && !has_critical.value();
        let status = if pass { "PASS" } else { "FAIL" };
        let output = format!(
            "CI Report for {}\nScore: {:.1}/100 (threshold: {})\nViolations: {}\nCritical: {}\nStatus: {}",
            path,
            score,
            flags.threshold,
            results.len(),
            has_critical.value(),
            status
        );
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
                let scan_root = self
                    .filesystem
                    .workspace_root(
                        &shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
                            .unwrap_or_default(),
                    )
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.to_string());
                let root_fp = shared::common::taxonomy_path_vo::FilePath::new(scan_root.clone())
                    .unwrap_or_default();
                let dir_path =
                    shared::common::taxonomy_path_vo::DirectoryPath::new(scan_root.clone())
                        .unwrap_or_default();
                let ignored = self
                    .config_orchestrator
                    .as_ref()
                    .map(|o| o.ignored_paths(&root_fp))
                    .unwrap_or_default();
                let source_files = self.filesystem.discover_source_files(
                    std::path::Path::new(dir_path.value()),
                    ignored.values(),
                );
                let file_strs: Vec<String> = source_files;
                if file_strs.is_empty() {
                    return LintExecutionResult::success(
                        format!(
                            "Orphan detection for {}\nNo source files found in {}.",
                            path, scan_root
                        ),
                        0,
                    );
                }
                let files_vo =
                    shared::orphan_rules::taxonomy_orphan_contract_vo::OrphanFileListVO::new(
                        file_strs.clone(),
                    );
                let results = orphan_agg.check_orphans(&files_vo, &root_fp);
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
                let output = format!(
                    "Orphan detection for {}\nUse CLI `lint-arwaky-cli orphan {}` for full orphan graph analysis.",
                    path, path
                );
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn security(&self, path: &str) -> LintExecutionResult {
        match &self.external_lint {
            Some(ext_lint) => {
                // External lint scan_all is async — suggest CLI for full security scan
                let adapter_list = ext_lint
                    .adapter_names()
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let output = format!(
                    "Security scan for {}\nSecurity adapters available: {}\n\nNote: Security scanning requires async runtime.\nUse CLI: `lint-arwaky-cli security {}`",
                    path, adapter_list, path
                );
                LintExecutionResult::success(output, 0)
            }
            None => {
                let output = format!(
                    "Security scan for {}\nUse CLI `lint-arwaky-cli security {}` for full vulnerability scan.",
                    path, path
                );
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn duplicates(&self, path: &str) -> LintExecutionResult {
        let scan_root = self
            .filesystem
            .workspace_root(
                &shared::common::taxonomy_path_vo::FilePath::new(path.to_string())
                    .unwrap_or_default(),
            )
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string());

        let root_fp =
            shared::common::taxonomy_path_vo::FilePath::new(scan_root.clone()).unwrap_or_default();
        let dir_path = shared::common::taxonomy_path_vo::DirectoryPath::new(scan_root.clone())
            .unwrap_or_default();
        let ignored = self
            .config_orchestrator
            .as_ref()
            .map(|o| o.ignored_paths(&root_fp))
            .unwrap_or_default();
        let source_files = self
            .filesystem
            .discover_source_files(std::path::Path::new(dir_path.value()), ignored.values());
        let file_strs: Vec<String> = source_files;

        let entries = self.code_analysis.collect_file_entries(&file_strs);
        let blocks = self.code_analysis.scan_duplicate_blocks(entries, 10);
        let violations = self
            .code_analysis
            .build_violations(&blocks, file_strs.len() * 100, 10);
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
                    shared::quality_rules::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation::CodeDuplication { reason } => {
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
            Some(_maintenance) => {
                // MaintenanceCommandsAggregate methods are async — suggest CLI
                let output = format!(
                    "Dependency scan for {}\nUse CLI `lint-arwaky-cli dependencies {}` for full report.",
                    path, path
                );
                LintExecutionResult::success(output, 0)
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
            Some(_maintenance) => {
                // MaintenanceCommandsAggregate::diagnose_toolchain is async — suggest CLI
                let output = "Environment Diagnostics:\nUse CLI `lint-arwaky-cli maintenance doctor` for full environment check.\nRequired: Rust toolchain, Python 3.8+, Node.js 18+".to_string();
                LintExecutionResult::success(output, 0)
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
                let output = format!(
                    "Adapter dependency installation.\nDetected language: {}\n\nNote: Adapter installation requires async runtime.\nUse CLI: `lint-arwaky-cli setup install`",
                    lang_str
                );
                LintExecutionResult::success(output, 0)
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
                let cwd = std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string());
                let project_root =
                    shared::common::taxonomy_path_vo::FilePath::new(cwd).unwrap_or_default();
                let result = orchestrator.load_project_config(&project_root);
                format_config_result(&result)
            }
            None => {
                let output = "Active Configuration\nSource: embedded (built-in defaults)\nNo config orchestrator configured. Use CLI `lint-arwaky-cli config-show`.".to_string();
                LintExecutionResult::success(output, 0)
            }
        }
    }

    fn install_hook(&self) -> LintExecutionResult {
        match &self.hook_port {
            Some(_hook) => {
                // GitHooksAggregate methods are async — suggest CLI
                LintExecutionResult::success(
                    "Git pre-commit hook installation.\nUse CLI `lint-arwaky-cli install-hook` to install."
                        .to_string(),
                    0,
                )
            }
            None => LintExecutionResult::success(
                "Git pre-commit hook installation.\nUse CLI `lint-arwaky-cli install-hook` to install."
                    .to_string(),
                0,
            ),
        }
    }

    fn uninstall_hook(&self) -> LintExecutionResult {
        match &self.hook_port {
            Some(_hook) => {
                // GitHooksAggregate methods are async — suggest CLI
                LintExecutionResult::success(
                    "Git pre-commit hook removal.\nUse CLI `lint-arwaky-cli uninstall-hook` to remove."
                        .to_string(),
                    0,
                )
            }
            None => LintExecutionResult::success(
                "Git pre-commit hook removal.\nUse CLI `lint-arwaky-cli uninstall-hook` to remove."
                    .to_string(),
                0,
            ),
        }
    }

    fn adapters(&self) -> LintExecutionResult {
        let adapters = Self::discover_adapters(self.filesystem.as_ref());
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

    fn watch(
        &self,
        path: &str,
    ) -> (
        LintExecutionResult,
        std::sync::mpsc::Receiver<shared::tui::taxonomy_watch_message_vo::WatchMessage>,
    ) {
        // Watch mode requires async runtime — suggest CLI
        let (tx, rx) = std::sync::mpsc::channel();
        let _ = tx.send(shared::tui::taxonomy_watch_message_vo::WatchMessage::new(
            "Watch mode requires async runtime. Use CLI: lint-arwaky-cli watch".to_string(),
        ));
        let output = format!(
            "Watch mode for {}\nUse CLI `lint-arwaky-cli watch {}` for file watching.",
            path, path
        );
        let result = LintExecutionResult::success(output, 0);
        (result, rx)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl LintExecutor {
    pub fn new(
        code_analysis: Arc<dyn ICodeAnalysisAggregate>,
        watch_aggregate: Option<Arc<dyn IWatchAggregate>>,
        filesystem: Arc<dyn IFilesystemAggregate>,
    ) -> Self {
        Self {
            code_analysis,
            watch_aggregate,
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

    pub fn with_watch_aggregate(mut self, watch_aggregate: Arc<dyn IWatchAggregate>) -> Self {
        self.watch_aggregate = Some(watch_aggregate);
        self
    }

    pub fn with_multi_project_orchestrator(
        mut self,
        multi_project_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    ) -> Self {
        self.config_orchestrator = Some(multi_project_orchestrator);
        self
    }

    pub fn format_results(&self, results: &LintResultList) -> String {
        format_results(results).to_string()
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

    /// Discover available linter adapters and check binary availability.
    fn discover_adapters(filesystem: &dyn IFilesystemAggregate) -> Vec<AdapterInfo> {
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
                installed: filesystem.is_binary_available(&ToolName {
                    value: b.to_string(),
                }),
            });
        }
        list
    }
}

impl LintExecutor {
    fn run_comprehensive_scan(&self, path: &str) -> LintExecutionResult {
        self.run_legacy_scan(path)
    }

    /// Parallel scan — runs core linters concurrently via thread::scope.
    /// Uses filesystem aggregate for file discovery instead of direct crate imports.
    fn run_legacy_scan(&self, path: &str) -> LintExecutionResult {
        let path_string = path.to_string();

        // Use filesystem aggregate: workspace_root + discover_source_files
        let scan_root = self
            .filesystem
            .workspace_root(
                &shared::common::taxonomy_path_vo::FilePath::new(path_string.clone())
                    .unwrap_or_default(),
            )
            .unwrap_or_else(|| std::path::PathBuf::from(&path_string));
        let root_fp = shared::common::taxonomy_path_vo::FilePath::new(path_string.clone())
            .unwrap_or_default();
        let ignored = self
            .config_orchestrator
            .as_ref()
            .map(|o| o.ignored_paths(&root_fp))
            .unwrap_or_default();

        let file_strs: Vec<String> = self
            .filesystem
            .discover_source_files(&scan_root, ignored.values());

        // Pre-compute shared data for linter threads
        let aes_fp = shared::common::taxonomy_path_vo::FilePath::new(path_string.clone())
            .unwrap_or_default();

        // Clone Arcs for thread ownership
        let code_analysis = self.code_analysis.clone();
        let naming = self.naming_orchestrator.clone();
        let import = self.import_orchestrator.clone();
        let orphan_agg = self.orphan_aggregate.clone();

        // Clone path_string for each thread
        let path_string_n = path_string.clone();
        let path_string_i = path_string.clone();

        std::thread::scope(|s| {
            // 1. AES code analysis (sync)
            let h1 = s.spawn(move || code_analysis.run_code_analysis(&aes_fp).values);

            // 2. Naming rules audit — AES101-102 (sync, no tokio needed)
            let h2 = s.spawn(move || {
                naming
                    .map(|n| {
                        let p = shared::common::taxonomy_path_vo::FilePath::new(path_string_n)
                            .unwrap_or_default();
                        n.run_audit(&p).unwrap_or_default()
                    })
                    .unwrap_or_default()
            });

            // 3. Import rules audit — AES201-205, cycles (sync, no tokio needed)
            let h3 = s.spawn(move || {
                import
                    .map(|i| {
                        let p = shared::common::taxonomy_path_vo::FilePath::new(path_string_i)
                            .unwrap_or_default();
                        i.run_audit(&p).unwrap_or_default()
                    })
                    .unwrap_or_default()
            });

            // 4. Orphan detection — AES501-506 (sync)
            let h4 = s.spawn(move || {
                if file_strs.is_empty() {
                    return Vec::new();
                }
                let files_vo =
                    shared::orphan_rules::taxonomy_orphan_contract_vo::OrphanFileListVO::new(
                        file_strs,
                    );
                orphan_agg
                    .map(|o| o.check_orphans(&files_vo, &root_fp))
                    .unwrap_or_default()
            });

            // Collect all results
            let mut all_results = Vec::new();
            all_results.extend(h1.join().unwrap_or_default());
            all_results.extend(h2.join().unwrap_or_default());
            all_results.extend(h3.join().unwrap_or_default());
            all_results.extend(h4.join().unwrap_or_default());

            let count = all_results.len();
            let results = LintResultList::new(all_results);
            let output = self.format_results(&results);
            LintExecutionResult::success(output, count)
        })
    }
}
