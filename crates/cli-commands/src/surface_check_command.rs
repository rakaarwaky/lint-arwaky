// PURPOSE: Command: CLI surface for check/scan — runs AES analysis on target path
use std::collections::HashMap;
use std::sync::Arc;

use std::process::ExitCode;

use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::code_analysis::contract_lint_aggregate::IArchLintAggregate;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort;
use shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort;
use shared::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};

pub type OrchestratorFactory = Arc<
    dyn Fn(shared::config_system::taxonomy_config_vo::ArchitectureConfig) -> CheckContext
        + Send
        + Sync,
>;

pub struct CheckContext {
    pub arch_linter: Arc<dyn IArchLintAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub language_detector: Arc<dyn ILanguageDetectorPort>,
}

pub struct CheckCommandsSurface {
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub arch_linter: Arc<dyn IArchLintAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
    pub factory: Option<OrchestratorFactory>,
}

impl CheckCommandsSurface {
    pub fn new(
        external_lint: Arc<dyn IExternalLintAggregate>,
        arch_linter: Arc<dyn IArchLintAggregate>,
        import_orchestrator: Arc<dyn IImportRunnerAggregate>,
        naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
        role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
        scanner_provider: Arc<dyn IScannerProviderPort>,
        orphan_orchestrator: Arc<dyn IOrphanAggregate>,
        layer_detector: Arc<dyn ILayerDetectionAggregate>,
    ) -> Self {
        Self {
            external_lint,
            arch_linter,
            import_orchestrator,
            naming_orchestrator,
            role_orchestrator,
            scanner_provider,
            orphan_orchestrator,
            layer_detector,
            multi_project_orchestrator: None,
            factory: None,
        }
    }

    pub fn new_with_factory(
        external_lint: Arc<dyn IExternalLintAggregate>,
        arch_linter: Arc<dyn IArchLintAggregate>,
        import_orchestrator: Arc<dyn IImportRunnerAggregate>,
        naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
        role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
        scanner_provider: Arc<dyn IScannerProviderPort>,
        orphan_orchestrator: Arc<dyn IOrphanAggregate>,
        layer_detector: Arc<dyn ILayerDetectionAggregate>,
        multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
        factory: OrchestratorFactory,
    ) -> Self {
        Self {
            external_lint,
            arch_linter,
            import_orchestrator,
            naming_orchestrator,
            role_orchestrator,
            scanner_provider,
            orphan_orchestrator,
            layer_detector,
            multi_project_orchestrator,
            factory: Some(factory),
        }
    }

    /// Run AES analysis + external adapters on a target path.
    pub fn scan(&self, path: &str, filter: Option<&str>, config: ArchitectureConfig) {
        let path_obj = match FilePath::new(path.to_string()) {
            Ok(fp) => fp,
            Err(_) => match FilePath::new(".".to_string()) {
                Ok(fp) => fp,
                Err(_) => FilePath::default(),
            },
        };
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => {
                eprintln!("[error] failed to create tokio runtime");
                return;
            }
        };

        // Determine dynamic orchestrators based on detected language config
        let (arch_linter, naming_orchestrator, import_orchestrator, role_orchestrator) =
            if let Some(ref factory) = self.factory {
                let ctx = factory(config.clone());
                (
                    ctx.arch_linter,
                    ctx.naming_orchestrator,
                    ctx.import_orchestrator,
                    ctx.role_orchestrator,
                )
            } else {
                (
                    self.arch_linter.clone(),
                    self.naming_orchestrator.clone(),
                    self.import_orchestrator.clone(),
                    self.role_orchestrator.clone(),
                )
            };

        let mut all_results = Vec::new();

        // 1. Run AES analysis (same algorithm for check and scan)
        let aes_results = arch_linter.run_self_lint(path);
        all_results.extend(aes_results.values);

        // 2. Run naming-rules audit (AES101, AES102)
        let naming_results = rt.block_on(naming_orchestrator.run_audit(&path_obj));
        all_results.extend(naming_results);

        // 3. Run import-rules audit (AES201, AES202, AES205, AES203, cycles)
        let import_results = rt.block_on(import_orchestrator.run_audit(&path_obj));
        all_results.extend(import_results);

        // 4. Run external linter adapters via aggregate
        let path_obj2 = match FilePath::new(path.to_string()) {
            Ok(fp) => fp,
            Err(_) => match FilePath::new(".".to_string()) {
                Ok(fp) => fp,
                Err(_) => FilePath::default(),
            },
        };
        let external_results = rt.block_on(self.external_lint.scan_all(&path_obj2));
        all_results.extend(external_results.values);

        // 4. Run role-rules audit (AES401, AES402, AES403, AES404, AES405, AES406)
        let role_results = rt.block_on(role_orchestrator.run_audit(&path_obj));
        all_results.extend(role_results);

        // 5. Run orphan detection — always scan entire workspace for cross-folder import graph
        let dir_path = match DirectoryPath::new(".".to_string()) {
            Ok(dp) => dp,
            Err(_) => DirectoryPath::default(),
        };
        let source_files = match self
            .scanner_provider
            .scan_directory(&dir_path)
            .map(|list| list.values)
        {
            Ok(files) => files,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        let orphan_results = self.orphan_orchestrator.check_orphans(
            self.layer_detector.as_ref(),
            &file_strs,
            ".",
        );
        all_results.extend(orphan_results);

        let filtered_results: Vec<_> = if let Some(code) = filter {
            all_results
                .into_iter()
                .filter(|r| r.code.to_string().contains(code))
                .collect()
        } else {
            all_results
        };
        let results_list = LintResultList::new(filtered_results);
        println!("{}", arch_linter.format_report(&results_list, path));
    }

    /// Check if a single file is an orphan.
    /// Still needs to scan all files to build import graph for reachability analysis.
    pub fn check_orphan_single_file(&self, file_path: &str) {
        let path_obj = std::path::Path::new(file_path);

        // Collect all source files from workspace root for cross-folder graph building
        let dir_path = match DirectoryPath::new(".".to_string()) {
            Ok(dp) => dp,
            Err(_) => DirectoryPath::default(),
        };
        let source_files = match self
            .scanner_provider
            .scan_directory(&dir_path)
            .map(|list| list.values)
        {
            Ok(files) => files,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();

        // Normalize the target file path
        let target_path = if path_obj.is_absolute() {
            file_path.to_string()
        } else {
            let cwd = match std::env::current_dir() {
                Ok(p) => p,
                Err(_) => std::path::PathBuf::default(),
            };
            cwd.join(file_path).to_string_lossy().to_string()
        };

        // Run orphan detection
        let all_results = self.orphan_orchestrator.check_orphans(
            self.layer_detector.as_ref(),
            &file_strs,
            ".",
        );

        // Filter results for the specific file
        let file_results: Vec<_> = all_results
            .into_iter()
            .filter(|r| r.file.value == target_path || r.file.value == file_path)
            .collect();

        if file_results.is_empty() {
            println!(
                "  {} is NOT an orphan (reachable from entry point)",
                file_path
            );
        } else {
            println!("  {} is an ORPHAN:", file_path);
            for r in &file_results {
                println!("    [{}] {}", r.code, r.message);
            }
        }
    }

    /// Scan with multi-workspace discovery.
    /// If >1 workspaces found, show summary per workspace with violations grouped by code.
    pub fn scan_with_discovery(&self, path: &str, filter: Option<&str>) {
        let path_obj = match FilePath::new(path.to_string()) {
            Ok(fp) => fp,
            Err(_) => {
                eprintln!("[error] invalid path: {path}");
                return;
            }
        };

        let orchestrator = match self.multi_project_orchestrator.as_ref() {
            Some(o) => o.clone(),
            None => {
                eprintln!("[error] multi-project orchestrator not available");
                return;
            }
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => {
                eprintln!("[error] failed to create tokio runtime");
                return;
            }
        };
        let workspaces = rt.block_on(orchestrator.discover_workspaces(&path_obj));

        if workspaces.len() <= 1 {
            let default_config = ArchitectureConfig::default();
            self.scan(path, filter, default_config);
            return;
        }

        println!(
            "Lint Arwaky v{} (Multi-Workspace Mode)",
            env!("CARGO_PKG_VERSION")
        );
        println!("Found {} workspaces in {path}", workspaces.len());
        println!();

        let mut global_all_results = Vec::new();

        // Collect ALL source files from scan root for cross-workspace orphan detection
        let all_source_files: Vec<String> =
            shared::source_parsing::collect_all_source_files(std::path::Path::new(path))
                .iter()
                .map(|f| f.value.clone())
                .collect();

        for ws in &workspaces {
            let ws_name = match std::path::Path::new(&ws.path.value).file_name() {
                Some(name) => name.to_string_lossy(),
                None => std::borrow::Cow::Borrowed(""),
            };
            let ws_type = &ws.workspace_type;

            let mut all_results = Vec::new();

            // Determine dynamic orchestrators based on detected language config
            let (arch_linter, naming_orchestrator, import_orchestrator, role_orchestrator) =
                if let Some(ref factory) = self.factory {
                    let ctx = factory(ws.config.clone());
                    (
                        ctx.arch_linter,
                        ctx.naming_orchestrator,
                        ctx.import_orchestrator,
                        ctx.role_orchestrator,
                    )
                } else {
                    (
                        self.arch_linter.clone(),
                        self.naming_orchestrator.clone(),
                        self.import_orchestrator.clone(),
                        self.role_orchestrator.clone(),
                    )
                };

            let aes_results = arch_linter.run_self_lint(&ws.path.value);
            all_results.extend(aes_results.values);

            let naming_results = rt.block_on(naming_orchestrator.run_audit(&ws.path));
            all_results.extend(naming_results);

            let import_results = rt.block_on(import_orchestrator.run_audit(&ws.path));
            all_results.extend(import_results);

            let external_results = rt.block_on(self.external_lint.scan_all(&ws.path));
            all_results.extend(external_results.values);

            // Role-rules per workspace (AES401, AES402, AES403, AES404, AES405, AES406)
            let role_results = rt.block_on(role_orchestrator.run_audit(&ws.path));
            all_results.extend(role_results);

            // Orphan detection — scan across ALL workspaces so contracts in shared/
            // can find their implementations in other crates
            let orphan_results = self.orphan_orchestrator.check_orphans(
                self.layer_detector.as_ref(),
                &all_source_files,
                &ws.path.value,
            );
            all_results.extend(orphan_results);

            let filtered_results: Vec<_> = if let Some(code) = filter {
                all_results
                    .into_iter()
                    .filter(|r| r.code.to_string().contains(code))
                    .collect()
            } else {
                all_results
            };

            global_all_results.extend(filtered_results.clone());

            let mut code_counts: HashMap<String, usize> = HashMap::new();
            for r in &filtered_results {
                *code_counts.entry(r.code.to_string()).or_insert(0) += 1;
            }
            let total = filtered_results.len();

            println!("── [{ws_type}] {ws_name} — {total} violations ──");
            if !code_counts.is_empty() {
                let mut sorted: Vec<_> = code_counts.into_iter().collect();
                sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
                for (code, count) in &sorted {
                    println!("   {code}: {count}");
                }
            } else {
                println!("   (clean)");
            }
            println!();
        }

        // Print combined summary
        let mut global_code_counts: HashMap<String, usize> = HashMap::new();
        for r in &global_all_results {
            *global_code_counts.entry(r.code.to_string()).or_insert(0) += 1;
        }
        let global_total = global_all_results.len();
        let global_unique_codes = global_code_counts.len();

        println!("============================================================");
        println!("  Combined Multi-Workspace Report Summary");
        println!("============================================================");
        println!("  Total Workspace Members: {}", workspaces.len());
        println!("  Total Unique AES Codes: {}", global_unique_codes);
        println!("  Total Violations: {}", global_total);
        if !global_code_counts.is_empty() {
            println!("------------------------------------------------------------");
            let mut sorted: Vec<_> = global_code_counts.into_iter().collect();
            sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
            for (code, count) in &sorted {
                println!("  {code}: {count}");
            }
        }
        println!("============================================================");
        println!();

        println!("To scan a specific workspace:");
        for ws in &workspaces {
            println!("  scan {}", ws.path.value);
        }
    }
}

/// check = self-lint (AES analysis on current project, same algorithm as scan)
pub fn handle_check(
    path: Option<String>,
    git_diff: bool,
    ctx: CheckContext,
    filter: Option<String>,
    git_aggregate: Option<Arc<dyn GitHooksAggregate>>,
    config: ArchitectureConfig,
) -> ExitCode {
    let root = path.unwrap_or_else(|| ".".to_string());
    if git_diff {
        let git_agg = match git_aggregate {
            Some(g) => g,
            None => {
                eprintln!("[error] git hooks not available");
                return ExitCode::FAILURE;
            }
        };
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r,
            Err(_) => {
                eprintln!("[error] failed to create tokio runtime");
                return ExitCode::FAILURE;
            }
        };
        rt.block_on(crate::surface_git_command::handle_git_diff(
            git_agg,
            ctx.arch_linter.clone(),
            ctx.language_detector.clone(),
            "HEAD".to_string(),
        ))
    } else {
        let surface = CheckCommandsSurface::new(
            ctx.external_lint,
            ctx.arch_linter,
            ctx.import_orchestrator,
            ctx.naming_orchestrator,
            ctx.role_orchestrator,
            ctx.scanner_provider,
            ctx.orphan_orchestrator,
            ctx.layer_detector,
        );
        surface.scan(&root, filter.as_deref(), config);
        ExitCode::SUCCESS
    }
}

/// scan = AES analysis on external project + external adapters
#[allow(clippy::too_many_arguments)]
pub fn handle_scan(
    path: Option<String>,
    arch_linter: Arc<dyn IArchLintAggregate>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    scanner_provider: Arc<dyn IScannerProviderPort>,
    orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    layer_detector: Arc<dyn ILayerDetectionAggregate>,
    multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
    factory: OrchestratorFactory,
    filter: Option<String>,
) -> ExitCode {
    let root = path.unwrap_or_else(|| ".".to_string());
    let surface = CheckCommandsSurface::new_with_factory(
        external_lint,
        arch_linter,
        import_orchestrator,
        naming_orchestrator,
        role_orchestrator,
        scanner_provider,
        orphan_orchestrator,
        layer_detector,
        multi_project_orchestrator,
        factory,
    );
    surface.scan_with_discovery(&root, filter.as_deref());
    ExitCode::SUCCESS
}

pub fn handle_ci(
    arch_linter: Arc<dyn IArchLintAggregate>,
    path: Option<String>,
    threshold: u32,
) -> ExitCode {
    use shared::cli_commands::taxonomy_severity_vo::Severity;
    let root = path.unwrap_or_else(|| ".".to_string());
    let results = arch_linter.run_lint(&root);
    let score = arch_linter.calc_score(&results);
    let effective_threshold = if threshold == 80 { 70 } else { threshold };

    let has_crit = arch_linter.check_critical(&results);
    let below_threshold = (score as u32) < effective_threshold;

    println!("Architecture Compliance CI");
    println!("Score: {:.1} / 100", score);
    println!("Threshold: {}", effective_threshold);
    println!();

    let mut reasons: Vec<String> = Vec::new();
    if has_crit {
        reasons.push("CRITICAL violation(s) detected — auto-fail triggered".to_string());
    }
    if below_threshold {
        reasons.push(format!(
            "Score below threshold ({:.1} < {})",
            score, effective_threshold
        ));
    }

    let critical_count = results
        .iter()
        .filter(|r| r.severity == Severity::CRITICAL)
        .count();
    let high_count = results
        .iter()
        .filter(|r| r.severity == Severity::HIGH)
        .count();
    let medium_count = results
        .iter()
        .filter(|r| r.severity == Severity::MEDIUM)
        .count();
    let low_count = results
        .iter()
        .filter(|r| r.severity == Severity::LOW)
        .count();

    println!(
        "CRITICAL: {} | HIGH: {} | MEDIUM: {} | LOW: {}",
        critical_count, high_count, medium_count, low_count
    );
    println!();

    if reasons.is_empty() {
        println!("Result: PASS (exit code 0)");
        ExitCode::SUCCESS
    } else {
        for r in &reasons {
            println!("  {}", r);
        }
        println!("Result: FAIL (exit code 1)");
        ExitCode::from(1)
    }
}
