// PURPOSE: Command: CLI surface for check/scan — runs AES analysis on target path
use std::collections::HashMap;
use std::sync::Arc;

use std::process::ExitCode;

use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared::source_parsing::taxonomy_path_vo::{DirectoryPath, FilePath};

pub type OrchestratorFactory = Arc<
    dyn Fn(shared::config_system::taxonomy_config_vo::ArchitectureConfig) -> CheckContext
        + Send
        + Sync,
>;

pub struct CheckContext {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider:
        Arc<dyn shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub language_detector:
        Arc<dyn shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort>,
}

impl CheckContext {
    pub fn new_default() -> Self {
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_default();
        let analyzer = import_container.analyzer();
        let import_orchestrator = import_container.orchestrator();

        let checker_container =
            code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(
                analyzer.clone(),
            );
        code_analysis::agent_code_analysis_orchestrator::init_global_checker(Arc::new(
            checker_container,
        ));

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::new(analyzer.clone());
        let naming_orchestrator = naming_container.orchestrator();
        let role_container = role_rules::root_role_rules_container::RoleContainer::new();
        let role_orchestrator = role_container.orchestrator();
        let code_analysis_container =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new_with_analyzer(
                analyzer,
            );
        let code_analysis_linter = code_analysis_container.code_analysis_linter();
        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = external_lint_container.aggregate();
        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let orphan_orchestrator = orphan_container.analyzer();
        let aes_config = shared::config_system::taxonomy_config_vo::default_aes_config();
        let fs: Arc<dyn shared::file_system::contract_system_port::IFileSystemPort> =
            Arc::new(import_rules::infrastructure_filesystem_adapter::OSFileSystemAdapter::new());
        let parser: Arc<dyn shared::source_parsing::contract_parser_port::ISourceParserPort> =
            Arc::new(import_rules::root_import_rules_container::NullSourceParser);
        let layer_detector: Arc<dyn ILayerDetectionAggregate> = Arc::new(
            import_rules::capabilities_layer_detection_analyzer::LayerDetectionAnalyzer::new(
                aes_config, fs, parser,
            ),
        );
        let scanner_provider: Arc<
            dyn shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort,
        > = Arc::new(shared::source_parsing::infrastructure_file_collector_provider::FileCollectorProvider::new());
        let language_detector: Arc<
            dyn shared::source_parsing::contract_language_detector_port::ILanguageDetectorPort,
        > = Arc::new(crate::infrastructure_language_detector::CliLanguageDetector::new());
        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            external_lint,
            role_orchestrator,
            scanner_provider,
            orphan_orchestrator,
            layer_detector,
            language_detector,
        }
    }
}

pub struct CheckCommandsSurface {
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider:
        Arc<dyn shared::source_parsing::contract_scanner_provider_port::IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
    pub factory: Option<OrchestratorFactory>,
}

impl CheckCommandsSurface {
    pub fn new(ctx: CheckContext) -> Self {
        Self {
            external_lint: ctx.external_lint,
            code_analysis_linter: ctx.code_analysis_linter,
            import_orchestrator: ctx.import_orchestrator,
            naming_orchestrator: ctx.naming_orchestrator,
            role_orchestrator: ctx.role_orchestrator,
            scanner_provider: ctx.scanner_provider,
            orphan_orchestrator: ctx.orphan_orchestrator,
            layer_detector: ctx.layer_detector,
            multi_project_orchestrator: None,
            factory: None,
        }
    }

    pub fn new_with_factory(
        ctx: CheckContext,
        multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
        factory: OrchestratorFactory,
    ) -> Self {
        Self {
            external_lint: ctx.external_lint,
            code_analysis_linter: ctx.code_analysis_linter,
            import_orchestrator: ctx.import_orchestrator,
            naming_orchestrator: ctx.naming_orchestrator,
            role_orchestrator: ctx.role_orchestrator,
            scanner_provider: ctx.scanner_provider,
            orphan_orchestrator: ctx.orphan_orchestrator,
            layer_detector: ctx.layer_detector,
            multi_project_orchestrator,
            factory: Some(factory),
        }
    }

    /// Run AES analysis + external adapters on a target path.
    pub fn scan(&self, path: &str, filter: Option<&str>, config: ArchitectureConfig) {
        let path_obj = FilePath::new(path.to_string()).unwrap_or_default();
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => {
                eprintln!("[error] failed to create tokio runtime");
                return;
            }
        };

        // Determine dynamic orchestrators based on detected language config
        let (code_analysis_linter, naming_orchestrator, import_orchestrator, role_orchestrator) =
            if let Some(ref factory) = self.factory {
                let ctx = factory(config.clone());
                (
                    ctx.code_analysis_linter,
                    ctx.naming_orchestrator,
                    ctx.import_orchestrator,
                    ctx.role_orchestrator,
                )
            } else {
                (
                    self.code_analysis_linter.clone(),
                    self.naming_orchestrator.clone(),
                    self.import_orchestrator.clone(),
                    self.role_orchestrator.clone(),
                )
            };

        let mut all_results = Vec::new();

        // 1. Run AES analysis (same algorithm for check and scan)
        let aes_results = code_analysis_linter.run_code_analysis(path);
        all_results.extend(aes_results.values);

        // 2. Run naming-rules audit (AES101, AES102)
        let naming_results = rt.block_on(naming_orchestrator.run_audit(&path_obj));
        all_results.extend(naming_results);

        // 3. Run import-rules audit (AES201, AES202, AES205, AES203, cycles)
        let import_results = rt.block_on(import_orchestrator.run_audit(&path_obj));
        all_results.extend(import_results);

        // 4. Run external linter adapters via aggregate
        let path_obj2 = FilePath::new(path.to_string()).unwrap_or_default();
        let external_results = rt.block_on(self.external_lint.scan_all(&path_obj2));
        all_results.extend(external_results.values);

        // 4. Run role-rules audit (AES401, AES402, AES403, AES404, AES405, AES406)
        let role_results = rt.block_on(role_orchestrator.run_audit(&path_obj));
        all_results.extend(role_results);

        // 5. Run orphan detection — always scan entire workspace for cross-folder import graph.
        // Scan from workspace root so cross-crate imports can be resolved.
        let scan_root = find_workspace_root(path);
        let orphan_scan_root = scan_root.as_ref().and_then(|r| r.to_str()).unwrap_or(".");
        let dir_path = DirectoryPath::new(orphan_scan_root.to_string()).unwrap_or_default();
        let source_files = match self.scanner_provider.scan_directory(&dir_path) {
            Ok(list) => list.values,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        let orphan_results = self.orphan_orchestrator.check_orphans(
            self.layer_detector.as_ref(),
            &file_strs,
            orphan_scan_root,
        );
        all_results.extend(orphan_results);

        let canonical_scan_path = match std::path::Path::new(path).canonicalize() {
            Ok(p) => p,
            Err(_) => std::path::PathBuf::from(path),
        }
        .to_string_lossy()
        .to_string();
        let cwd = match std::env::current_dir() {
            Ok(d) => d,
            Err(_) => std::path::PathBuf::new(),
        };
        let filtered_results: Vec<_> = if let Some(code) = filter {
            all_results
                .into_iter()
                .filter(|r| {
                    let abs_path = cwd.join(&r.file.value);
                    r.code.to_string().contains(code)
                        && abs_path.to_string_lossy().starts_with(&canonical_scan_path)
                })
                .collect()
        } else {
            all_results
                .into_iter()
                .filter(|r| {
                    let abs_path = cwd.join(&r.file.value);
                    abs_path.to_string_lossy().starts_with(&canonical_scan_path)
                })
                .collect()
        };
        let results_list = LintResultList::new(filtered_results);
        println!(
            "{}",
            code_analysis_linter.format_report(&results_list, path)
        );
    }

    /// Check if a single file is an orphan.
    /// Still needs to scan all files to build import graph for reachability analysis.
    pub fn check_orphan_single_file(&self, file_path: &str) {
        let path_obj = std::path::Path::new(file_path);

        // Collect all source files from workspace root for cross-folder graph building
        let dir_path = DirectoryPath::new(".".to_string()).unwrap_or_default();
        let source_files = match self.scanner_provider.scan_directory(&dir_path) {
            Ok(list) => list.values,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();

        // Normalize the target file path
        let target_path = if path_obj.is_absolute() {
            file_path.to_string()
        } else {
            let cwd = match std::env::current_dir() {
                Ok(d) => d,
                Err(_) => std::path::PathBuf::new(),
            };
            cwd.join(file_path).to_string_lossy().to_string()
        };

        // Run orphan detection
        let all_results =
            self.orphan_orchestrator
                .check_orphans(self.layer_detector.as_ref(), &file_strs, ".");

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

        if workspaces.is_empty() {
            // No workspaces discovered — treat path as a standalone scan
            let default_config = ArchitectureConfig::default();
            self.scan(path, filter, default_config);
            return;
        }

        // Collect ALL source files from workspace root for cross-workspace orphan detection
        let scan_root = match find_workspace_root(path) {
            Some(r) => r,
            None => std::path::PathBuf::from(path),
        };
        let all_source_files: Vec<String> =
            shared::source_parsing::collect_all_source_files(&scan_root)
                .iter()
                .map(|f| f.value.clone())
                .collect();

        let multi = workspaces.len() > 1;
        if multi {
            println!(
                "Lint Arwaky v{} (Multi-Workspace Mode)",
                env!("CARGO_PKG_VERSION")
            );
            println!("Found {} workspaces in {path}", workspaces.len());
            println!();
        }

        let mut global_all_results = Vec::new();

        for ws in &workspaces {
            let ws_name = match std::path::Path::new(&ws.path.value).file_name() {
                Some(name) => name.to_string_lossy(),
                None => std::borrow::Cow::Borrowed(""),
            };
            let ws_type = &ws.workspace_type;

            let mut all_results = Vec::new();

            // Determine dynamic orchestrators based on detected language config
            let (code_analysis_linter, naming_orchestrator, import_orchestrator, role_orchestrator) =
                if let Some(ref factory) = self.factory {
                    let ctx = factory(ws.config.clone());
                    (
                        ctx.code_analysis_linter,
                        ctx.naming_orchestrator,
                        ctx.import_orchestrator,
                        ctx.role_orchestrator,
                    )
                } else {
                    (
                        self.code_analysis_linter.clone(),
                        self.naming_orchestrator.clone(),
                        self.import_orchestrator.clone(),
                        self.role_orchestrator.clone(),
                    )
                };

            let aes_results = code_analysis_linter.run_code_analysis(&ws.path.value);
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

            // Filter results to only those in this workspace member's path
            let ws_canonical = std::path::Path::new(&ws.path.value).canonicalize().ok();
            let cwd_for_ws = match std::env::current_dir() {
                Ok(d) => d,
                Err(_) => std::path::PathBuf::new(),
            };
            let filtered_results: Vec<_> = if let Some(code) = filter {
                all_results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        let matches_path = ws_canonical.as_ref().is_none_or(|c| {
                            abs_path
                                .to_string_lossy()
                                .starts_with(c.to_string_lossy().as_ref())
                        });
                        r.code.to_string().contains(code) && matches_path
                    })
                    .collect()
            } else if let Some(ref canonical) = ws_canonical {
                all_results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        abs_path
                            .to_string_lossy()
                            .starts_with(canonical.to_string_lossy().as_ref())
                    })
                    .collect()
            } else {
                all_results
            };

            global_all_results.extend(filtered_results.clone());

            if multi {
                let total = filtered_results.len();
                println!("── [{ws_type}] {ws_name} — {total} violations ──");
                if !filtered_results.is_empty() {
                    let mut code_counts: HashMap<String, usize> = HashMap::new();
                    for r in &filtered_results {
                        *code_counts.entry(r.code.to_string()).or_insert(0) += 1;
                    }
                    let mut sorted: Vec<_> = code_counts.into_iter().collect();
                    sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
                    for (code, count) in &sorted {
                        println!("       {code}: {count}");
                    }
                } else {
                    println!("   (clean)");
                }
                println!();
            } else {
                // Single workspace — print full violation detail
                let results_list = LintResultList::new(filtered_results);
                print!(
                    "{}",
                    code_analysis_linter.format_report(&results_list, &ws.path.value)
                );
            }
        }

        if multi {
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
}

/// Walk up from `path` to find the workspace root (parent of `crates/`, `packages/`, or `modules/`).
fn find_workspace_root(path: &str) -> Option<std::path::PathBuf> {
    let mut dir = std::path::Path::new(path).to_path_buf();
    if !dir.is_absolute() {
        dir = std::env::current_dir().ok()?.join(&dir);
    }
    loop {
        if dir.join("Cargo.toml").exists()
            || dir.join("crates").is_dir()
            || dir.join("packages").is_dir()
            || dir.join("modules").is_dir()
        {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
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
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
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
            ctx.code_analysis_linter.clone(),
            ctx.language_detector.clone(),
            "HEAD".to_string(),
        ))
    } else {
        let surface = CheckCommandsSurface::new(ctx);
        surface.scan(&root, filter.as_deref(), config);
        ExitCode::SUCCESS
    }
}

/// scan = AES analysis on external project + external adapters
pub fn handle_scan(
    path: Option<String>,
    ctx: CheckContext,
    multi_project_orchestrator: Option<Arc<dyn MultiProjectOrchestratorAggregate>>,
    factory: OrchestratorFactory,
    filter: Option<String>,
) -> ExitCode {
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let surface = CheckCommandsSurface::new_with_factory(ctx, multi_project_orchestrator, factory);
    surface.scan_with_discovery(&root, filter.as_deref());
    ExitCode::SUCCESS
}

pub fn handle_ci(
    code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    path: Option<String>,
    threshold: u32,
) -> ExitCode {
    use shared::cli_commands::taxonomy_severity_vo::Severity;
    let root = match path {
        Some(p) => p,
        None => ".".to_string(),
    };
    let results = code_analysis_linter.run_code_analysis_path(&root);
    let score = code_analysis_linter.calc_score(&results);
    let effective_threshold = if threshold == 80 { 70 } else { threshold };

    let has_crit = code_analysis_linter.check_critical(&results);
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
