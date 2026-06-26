// PURPOSE: CheckCommandsSurface — CLI surface for check/scan commands
//
// This is the primary surface that coordinates the full lint pipeline.
// The scan() method runs ALL linters in sequence:
//   1. Code analysis (AES301-305)
//   2. Naming rules (AES101-102)
//   3. Import rules (AES201-205)
//   4. External linters (Clippy, Ruff, ESLint, etc.)
//   5. Role rules (AES401-406)
//   6. Orphan detection (AES501-506)
//
// The OrchestratorFactory type enables the `scan` command to create
// fresh per-project DI containers for each workspace member, so that
// each member gets its own language-specific configuration.
use std::sync::Arc;

use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

/// CheckContext — DI container struct holding all analysis subsystems.
/// Defined in the surfaces layer because surfaces are the primary consumers.
pub struct CheckContext {
    pub code_analysis_linter: Arc<dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider: Arc<dyn shared::common::contract_scanner_provider_port::IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub language_detector: Arc<dyn shared::common::contract_language_detector_port::ILanguageDetectorPort>,
}

pub type OrchestratorFactory = Arc<
    dyn Fn(shared::config_system::taxonomy_config_vo::ArchitectureConfig) -> CheckContext
        + Send
        + Sync,
>;

pub struct CheckCommandsSurface {
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider:
        Arc<dyn shared::common::contract_scanner_provider_port::IScannerProviderPort>,
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
    ///
    /// This is the core scan pipeline. It runs all 6 linter groups in the
    /// same order every time:
    ///   1. code-analysis (AES301-305) — file lines, bypass, mandatory defs
    ///   2. naming (AES101-102) — suffix/prefix conventions
    ///   3. imports (AES201-205) — mandatory, forbidden, unused, cycles
    ///   4. external (Clippy, Ruff, ESLint) — subprocess-based linting
    ///   5. roles (AES401-406) — layer-role violations
    ///   6. orphans (AES501-506) — dead code detection via import graph
    ///
    /// If a factory is provided, per-project containers are created for
    /// each workspace member (used by scan, not check).
    pub fn scan(&self, path: &str, filter: Option<&str>, config: ArchitectureConfig) {
        let path_obj = crate::surface_common_command::resolve_file_path(path);
        let rt = match crate::surface_common_command::create_runtime() {
            Ok(r) => r,
            Err(_) => return,
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

        // 5. Run role-rules audit (AES401-406: layer-role violations)
        let role_results = rt.block_on(role_orchestrator.run_audit(&path_obj));
        all_results.extend(role_results);

        // 6. Run orphan detection (AES501-506: dead code via import graph)
        let orphan_results = self.run_orphan_detection_pass(
            path,
            &self.scanner_provider,
            &self.orphan_orchestrator,
            &self.layer_detector,
        );
        all_results.extend(orphan_results);

        self.filter_and_display_results(all_results, path, filter, code_analysis_linter);
    }

    /// Run orphan detection pass — scans workspace for cross-folder import graph.
    fn run_orphan_detection_pass(
        &self,
        path: &str,
        scanner_provider: &Arc<
            dyn shared::common::contract_scanner_provider_port::IScannerProviderPort,
        >,
        orphan_orchestrator: &Arc<dyn IOrphanAggregate>,
        layer_detector: &Arc<dyn ILayerDetectionAggregate>,
    ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        let scan_root = crate::surface_check_action::find_workspace_root(path);
        let orphan_scan_root = scan_root.as_ref().and_then(|r| r.to_str()).unwrap_or(".");
        let dir_path = DirectoryPath::new(orphan_scan_root.to_string()).unwrap_or_default();
        let source_files = match scanner_provider.scan_directory(&dir_path) {
            Ok(list) => list.values,
            Err(_) => Vec::new(),
        };
        let file_strs: Vec<String> = source_files.iter().map(|f| f.value.clone()).collect();
        orphan_orchestrator.check_orphans(layer_detector.as_ref(), &file_strs, orphan_scan_root)
    }

    /// Filter results to the target path and display the report.
    fn filter_and_display_results(
        &self,
        all_results: Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
        path: &str,
        filter: Option<&str>,
        reporter: Arc<dyn ICodeAnalysisAggregate>,
    ) {
        let canonical_scan_path = crate::surface_common_command::canonicalize_path(path);
        let cwd = crate::surface_common_command::current_dir();
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
        println!("{}", reporter.format_report(&results_list, path));
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
            let cwd = crate::surface_common_command::current_dir();
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
    ///
    /// For each discovered workspace member (Cargo.toml member, pyproject.toml
    /// module, package.json workspace):
    ///   1. Create per-project DI containers via OrchestratorFactory
    ///   2. Run all 6 linter groups on the member
    ///   3. Run orphan detection across ALL source files (cross-workspace)
    ///   4. Filter results to that member's path
    ///   5. Aggregate into global results
    ///
    /// Cross-workspace orphan detection is important: contracts defined in
    /// `shared/` may be implemented in `import-rules/`, so the orphan graph
    /// must span all workspace members.
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

        let rt = match crate::surface_common_command::create_runtime() {
            Ok(r) => r,
            Err(_) => return,
        };
        let workspaces = rt.block_on(orchestrator.discover_workspaces(&path_obj));

        if workspaces.is_empty() {
            // No workspaces discovered — treat path as a standalone scan
            let default_config = ArchitectureConfig::default();
            self.scan(path, filter, default_config);
            return;
        }

        // Collect ALL source files from workspace root for cross-workspace orphan detection
        let scan_root = match crate::surface_check_action::find_workspace_root(path) {
            Some(r) => r,
            None => std::path::PathBuf::from(path),
        };
        let all_source_files: Vec<String> = shared::common::collect_all_source_files(&scan_root)
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
                        r.code.to_string().contains(code)
                            && ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(true)
                    })
                    .collect()
            } else {
                all_results
                    .into_iter()
                    .filter(|r| {
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(true)
                    })
                    .collect()
            };

            global_all_results.extend(filtered_results);

            if multi {
                let result_list = LintResultList::new(global_all_results.clone());
                let report = code_analysis_linter.format_report(&result_list, &ws.path.value);
                println!("\n--- {} ({}) ---", ws_name, ws_type);
                println!("{report}");
            }
        }

        if workspaces.len() > 1 {
            println!("\n=== Combined Summary ===");
            let global_list = LintResultList::new(global_all_results);
            println!(
                "{}",
                self.code_analysis_linter
                    .format_report(&global_list, path)
            );
        }
    }
}
