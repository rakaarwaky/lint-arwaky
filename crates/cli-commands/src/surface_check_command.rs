// PURPOSE: Command: CLI surface for check/scan — runs AES analysis on target path
use std::collections::HashMap;
use std::sync::Arc;

use std::process::ExitCode;

use code_analysis::resolve_target;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use shared::source_parsing::taxonomy_path_vo::FilePath;

pub type OrchestratorFactory = Arc<
    dyn Fn(shared::config_system::taxonomy_config_vo::ArchitectureConfig) -> CheckContext
        + Send
        + Sync,
>;

pub struct CheckContext {
    pub arch_linter: Arc<dyn IArchLintProtocol>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
}

pub struct CheckCommandsSurface {
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub arch_linter: Arc<dyn IArchLintProtocol>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub factory: Option<OrchestratorFactory>,
}

impl CheckCommandsSurface {
    pub fn new(
        external_lint: Arc<dyn IExternalLintAggregate>,
        arch_linter: Arc<dyn IArchLintProtocol>,
        import_orchestrator: Arc<dyn IImportRunnerAggregate>,
        naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
        role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    ) -> Self {
        Self {
            external_lint,
            arch_linter,
            import_orchestrator,
            naming_orchestrator,
            role_orchestrator,
            factory: None,
        }
    }

    pub fn new_with_factory(
        external_lint: Arc<dyn IExternalLintAggregate>,
        arch_linter: Arc<dyn IArchLintProtocol>,
        import_orchestrator: Arc<dyn IImportRunnerAggregate>,
        naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
        role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
        factory: OrchestratorFactory,
    ) -> Self {
        Self {
            external_lint,
            arch_linter,
            import_orchestrator,
            naming_orchestrator,
            role_orchestrator,
            factory: Some(factory),
        }
    }

    /// Run AES analysis + external adapters on a target path.
    pub fn scan(&self, path: &str, filter: Option<&str>) {
        let path_obj = FilePath::new(path.to_string())
            .unwrap_or_else(|_| FilePath::new(".".to_string()).unwrap_or_default());
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
                use shared::config_system::contract_workspace_detector_port::IWorkspaceDetectorPort;
                let detector = config_system::WorkspaceDetector::new();
                let ws_type = detector.detect(&path_obj);
                let config = shared::config_system::taxonomy_config_vo::default_config_for_language(
                    ws_type.as_str(),
                );
                let ctx = factory(config);
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
        let path_obj2 = FilePath::new(path.to_string())
            .unwrap_or_else(|_| FilePath::new(".".to_string()).unwrap_or_default());
        let external_results = rt.block_on(self.external_lint.scan_all(&path_obj2));
        all_results.extend(external_results.values);

        // 4. Run role-rules audit (AES401, AES402, AES403, AES404, AES405, AES406)
        let role_results = rt.block_on(role_orchestrator.run_audit(&path_obj));
        all_results.extend(role_results);

        // 5. Run orphan detection
        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let orphan_analyzer = orphan_container.analyzer();
        let source_files = collect_source_files(path);
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        let orphan_results = orphan_analyzer.check_orphans(
            orphan_container.layer_detector().as_ref(),
            &file_strs,
            path,
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
        let root_dir = if path_obj.is_file() {
            path_obj
                .parent()
                .unwrap_or(path_obj)
                .to_string_lossy()
                .to_string()
        } else {
            file_path.to_string()
        };

        // Collect all source files from project root for graph building
        let source_files = collect_source_files(&root_dir);
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();

        // Normalize the target file path
        let target_path = if path_obj.is_absolute() {
            file_path.to_string()
        } else {
            let cwd = std::env::current_dir().unwrap_or_default();
            cwd.join(file_path).to_string_lossy().to_string()
        };

        // Run orphan detection
        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let orphan_analyzer = orphan_container.analyzer();
        let all_results = orphan_analyzer.check_orphans(
            orphan_container.layer_detector().as_ref(),
            &file_strs,
            &root_dir,
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

        let container = multi_project::root_multi_project_container::MultiProjectContainer::new();
        let orchestrator = container.aggregate();

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => {
                eprintln!("[error] failed to create tokio runtime");
                return;
            }
        };
        let workspaces = rt.block_on(orchestrator.discover_workspaces(&path_obj));

        if workspaces.len() <= 1 {
            self.scan(path, filter);
            return;
        }

        println!(
            "Lint Arwaky v{} (Multi-Workspace Mode)",
            env!("CARGO_PKG_VERSION")
        );
        println!("Found {} workspaces in {path}", workspaces.len());
        println!();

        let mut global_all_results = Vec::new();

        for ws in &workspaces {
            let ws_name = std::path::Path::new(&ws.path.value)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
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

            // Orphan detection per workspace
            let orphan_container =
                orphan_detector::root_orphan_detector_container::OrphanContainer::new();
            let orphan_analyzer = orphan_container.analyzer();
            let source_files = collect_source_files(&ws.path.value);
            let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
            let orphan_results = orphan_analyzer.check_orphans(
                orphan_container.layer_detector().as_ref(),
                &file_strs,
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
        println!("  Total Workspaces: {}", workspaces.len());
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

/// Collect all source files from a directory
fn collect_source_files(dir: &str) -> Vec<FilePath> {
    let mut files = Vec::new();
    let root = std::path::Path::new(dir);

    if root.is_dir() {
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                    if matches!(
                        dir_name.as_ref(),
                        "target"
                            | ".git"
                            | ".jj"
                            | ".opencode"
                            | "node_modules"
                            | ".github"
                            | "Graph-It-Live"
                            | "output"
                            | "scratch"
                            | "test-project-rust"
                            | "test-project-python"
                            | "test-project-javascript"
                            | "packages"
                    ) {
                        continue;
                    }
                    let sub_files = collect_source_files(&path.to_string_lossy());
                    files.extend(sub_files);
                } else if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy();
                    if matches!(ext_str.as_ref(), "rs" | "py" | "js" | "ts") {
                        if let Ok(fp) = FilePath::new(path.to_string_lossy().to_string()) {
                            files.push(fp);
                        }
                    }
                }
            }
        }
    }
    files
}

/// check = self-lint (AES analysis on current project, same algorithm as scan)
pub fn handle_check(
    path: Option<String>,
    _git_diff: bool,
    ctx: CheckContext,
    filter: Option<String>,
) -> ExitCode {
    let root = resolve_target(path);
    let surface = CheckCommandsSurface::new(
        ctx.external_lint,
        ctx.arch_linter,
        ctx.import_orchestrator,
        ctx.naming_orchestrator,
        ctx.role_orchestrator,
    );
    surface.scan(&root, filter.as_deref());
    ExitCode::SUCCESS
}

/// scan = AES analysis on external project + external adapters
pub fn handle_scan(
    path: Option<String>,
    arch_linter: Arc<dyn IArchLintProtocol>,
    import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    external_lint: Arc<dyn IExternalLintAggregate>,
    role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    factory: OrchestratorFactory,
    filter: Option<String>,
) -> ExitCode {
    let root = resolve_target(path);
    let surface = CheckCommandsSurface::new_with_factory(
        external_lint,
        arch_linter,
        import_orchestrator,
        naming_orchestrator,
        role_orchestrator,
        factory,
    );
    surface.scan_with_discovery(&root, filter.as_deref());
    ExitCode::SUCCESS
}
