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
use std::process::ExitCode;
use std::sync::Arc;

use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

/// CheckContext — DI container struct holding all analysis subsystems.
/// Defined in the surfaces layer because surfaces are the primary consumers.
// ─── Block 1: Struct Definition ───────────────────────────
pub struct CheckContext {
    pub code_analysis_linter:
        Arc<dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub scanner_provider:
        Arc<dyn shared::common::contract_scanner_provider_port::IScannerProviderPort>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub language_detector:
        Arc<dyn shared::common::contract_language_detector_port::ILanguageDetectorPort>,
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

// ─── Block 3: Constructors & Helpers ──────────────────────
// ─── Block 2: Public Contract ─────────────────────────────
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
    ///
    /// **Note:** When `self.factory` is `None` (default), the `config` parameter
    /// is accepted but silently ignored — the same orchestrator instances are
    /// reused regardless of the passed `ArchitectureConfig`. Pass an explicit
    /// factory via `new_with_factory()` to make per-project config effective.
    pub fn scan(
        &self,
        path: &str,
        filter: Option<&str>,
        config: ArchitectureConfig,
        format: Format,
    ) -> ExitCode {
        let path_obj = crate::surface_common_command::resolve_file_path(path);
        let rt = match crate::surface_common_command::create_runtime() {
            Ok(r) => r,
            Err(_) => return ExitCode::FAILURE,
        };

        // Determine dynamic orchestrators based on detected language config
        // If no factory was provided, build a default one from the current orchestrators
        let effective_factory = self.factory.clone().unwrap_or_else(|| {
            let cal = self.code_analysis_linter.clone();
            let no = self.naming_orchestrator.clone();
            let io = self.import_orchestrator.clone();
            let ro = self.role_orchestrator.clone();
            let ext = self.external_lint.clone();
            let sp = self.scanner_provider.clone();
            let oo = self.orphan_orchestrator.clone();
            let ld = self.layer_detector.clone();
            Arc::new(move |_cfg: ArchitectureConfig| CheckContext {
                code_analysis_linter: cal.clone(),
                naming_orchestrator: no.clone(),
                import_orchestrator: io.clone(),
                role_orchestrator: ro.clone(),
                external_lint: ext.clone(),
                scanner_provider: sp.clone(),
                orphan_orchestrator: oo.clone(),
                layer_detector: ld.clone(),
                language_detector: Arc::new(
                    shared::common::taxonomy_language_detector_utility::LanguageDetector::new(),
                ),
            })
        });
        let ctx = effective_factory(config.clone());
        let (code_analysis_linter, naming_orchestrator, import_orchestrator, role_orchestrator) = (
            ctx.code_analysis_linter,
            ctx.naming_orchestrator,
            ctx.import_orchestrator,
            ctx.role_orchestrator,
        );

        let mut all_results = Vec::new();

        // 1. Run AES analysis (same algorithm for check and scan)
        let aes_results = code_analysis_linter.run_code_analysis(&path_obj);
        all_results.extend(aes_results.values);

        // 2-5. Run async linter groups concurrently
        let (naming_results, import_results, external_results, role_results) = rt.block_on(async {
            tokio::join!(
                naming_orchestrator.run_audit(&path_obj),
                import_orchestrator.run_audit(&path_obj),
                self.external_lint.scan_all(&path_obj),
                role_orchestrator.run_audit(&path_obj),
            )
        });
        all_results.extend(naming_results);
        all_results.extend(import_results);
        all_results.extend(external_results.values);
        all_results.extend(role_results);

        // 6. Run orphan detection (AES501-506: dead code via import graph)
        let orphan_results =
            self.run_orphan_detection_pass(path, &ctx.orphan_orchestrator, &ctx.layer_detector);
        all_results.extend(orphan_results);

        let violation_count = self.filter_and_display_results(
            all_results,
            path,
            filter,
            code_analysis_linter,
            &format,
        );
        if violation_count > 0 {
            ExitCode::from(1)
        } else {
            ExitCode::SUCCESS
        }
    }

    /// Run orphan detection pass — scans workspace for cross-folder import graph.
    ///
    /// Uses `code_analysis::collect_all_source_files_raw` to collect ALL source files
    /// from the workspace root, not just the scan directory. This is critical for
    /// cross-crate orphan detection: contracts defined in `shared/` may be implemented
    /// in `auto-fix/`, `git-hooks/`, `maintenance/`, etc., and the orphan graph must
    /// span all workspace members to correctly identify reachability.
    fn run_orphan_detection_pass(
        &self,
        path: &str,
        orphan_orchestrator: &Arc<dyn IOrphanAggregate>,
        layer_detector: &Arc<dyn ILayerDetectionAggregate>,
    ) -> Vec<shared::cli_commands::taxonomy_result_vo::LintResult> {
        let scan_root = crate::surface_check_action::find_workspace_root(path);
        let orphan_scan_root = scan_root.as_ref().and_then(|r| r.to_str()).unwrap_or(".");

        // Collect ALL source files from workspace root for cross-crate orphan detection
        // This fixes false positives where contracts in shared/ are implemented in
        // other crates (auto-fix, git-hooks, maintenance, etc.)
        let all_source_files: Vec<String> = if let Some(ref root) = scan_root {
            code_analysis::collect_all_source_files_raw(root)
                .iter()
                .map(|f| f.value.clone())
                .collect()
        } else {
            Vec::new()
        };

        let orphan_files: Vec<FilePath> = all_source_files
            .into_iter()
            .filter_map(|s| FilePath::new(s).ok())
            .collect();
        orphan_orchestrator.check_orphans(
            layer_detector.as_ref(),
            &orphan_files,
            orphan_scan_root,
        )
    }

    /// Filter results to the target path and display the report.
    fn filter_and_display_results(
        &self,
        all_results: Vec<shared::cli_commands::taxonomy_result_vo::LintResult>,
        path: &str,
        filter: Option<&str>,
        reporter: Arc<dyn ICodeAnalysisAggregate>,
        format: &Format,
    ) -> usize {
        let canonical_scan_path = crate::surface_common_command::canonicalize_path(path);
        let cwd = crate::surface_common_command::current_dir();
        let config = self.layer_detector.config();
        let filtered_results: Vec<_> = if let Some(code) = filter {
            all_results
                .into_iter()
                .filter(|r| {
                    let rule_code = r.code.to_string();
                    if config.ignored_rules.values.contains(&rule_code) {
                        return false;
                    }
                    let abs_path = cwd.join(&r.file.value);
                    r.code.code() == code
                        && abs_path.to_string_lossy().starts_with(&canonical_scan_path)
                })
                .collect()
        } else {
            all_results
                .into_iter()
                .filter(|r| {
                    let rule_code = r.code.to_string();
                    if config.ignored_rules.values.contains(&rule_code) {
                        return false;
                    }
                    let abs_path = cwd.join(&r.file.value);
                    abs_path.to_string_lossy().starts_with(&canonical_scan_path)
                })
                .collect()
        };
        let violation_count = filtered_results.len();
        match format {
            Format::Text => {
                let results_list = LintResultList::new(filtered_results);
                let path_fp = FilePath::new(path.to_string()).unwrap_or_default();
                println!("{}", reporter.format_report(&results_list, &path_fp));
            }
            Format::Json => {
                let json = serde_json::to_string_pretty(&filtered_results)
                    .unwrap_or_else(|_| "[]".to_string());
                println!("{json}");
            }
            Format::Sarif => {
                let sarif = self.format_sarif_output(&filtered_results);
                println!("{sarif}");
            }
            Format::Junit => {
                let junit = self.format_junit_output(&filtered_results);
                println!("{junit}");
            }
        }
        violation_count
    }

    /// Check if a single file is an orphan.
    /// Still needs to scan all files to build import graph for reachability analysis.
    pub fn check_orphan_single_file(&self, file_path: &str) {
        let path_obj = std::path::Path::new(file_path);

        // Find workspace root for cross-crate graph building
        let scan_root = match crate::surface_check_action::find_workspace_root(file_path) {
            Some(r) => r,
            None => std::path::PathBuf::from("."),
        };
        let all_files: Vec<String> = code_analysis::collect_all_source_files_raw(&scan_root)
            .iter()
            .map(|f| f.value.clone())
            .collect();

        // Normalize the target file path
        let target_path = if path_obj.is_absolute() {
            file_path.to_string()
        } else {
            let cwd = crate::surface_common_command::current_dir();
            cwd.join(file_path).to_string_lossy().to_string()
        };

        // Run orphan detection with workspace root
        let all_results = self.orphan_orchestrator.check_orphans(
            self.layer_detector.as_ref(),
            &all_files,
            &scan_root.to_string_lossy(),
        );

        // Filter results for the specific file — canonicalize for robust comparison
        let target_canonical = std::path::Path::new(&target_path).canonicalize().ok();
        let file_results: Vec<_> = all_results
            .into_iter()
            .filter(|r| {
                let r_canonical = std::path::Path::new(&r.file.value).canonicalize().ok();
                match (target_canonical.as_deref(), r_canonical.as_deref()) {
                    (Some(t), Some(r)) => t == r,
                    _ => r.file.value == target_path || r.file.value == file_path,
                }
            })
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
    /// If `member` is specified, only that workspace member is scanned.
    /// Cross-workspace orphan detection is important: contracts defined in
    /// `shared/` may be implemented in `import-rules/`, so the orphan graph
    /// must span all workspace members.
    pub fn scan_with_discovery(
        &self,
        path: &str,
        filter: Option<&str>,
        member: Option<&str>,
        format: Format,
    ) -> ExitCode {
        let path_obj = match FilePath::new(path.to_string()) {
            Ok(fp) => fp,
            Err(_) => {
                eprintln!("[error] invalid path: {path}");
                return ExitCode::from(1);
            }
        };

        let orchestrator = match self.multi_project_orchestrator.as_ref() {
            Some(o) => o.clone(),
            None => {
                eprintln!("[error] multi-project orchestrator not available");
                return ExitCode::from(1);
            }
        };

        let rt = match crate::surface_common_command::create_runtime() {
            Ok(r) => r,
            Err(_) => return ExitCode::from(1),
        };
        let workspaces = rt.block_on(orchestrator.discover_workspaces(&path_obj));
        let all_workspaces = workspaces.clone();

        if workspaces.is_empty() {
            // No workspaces discovered — fall back to single-scan mode and load config dynamically
            let config_container =
                config_system::root_config_system_container::ConfigContainer::new();
            let config_orchestrator = config_container.orchestrator();
            let config_result = rt.block_on(config_orchestrator.load_project_config(&path_obj));
            let loaded_config = config_result.config;
            return self.scan(path, filter, loaded_config, format);
        }

        // Filter to specific member if requested
        let workspaces = if let Some(member_name) = member {
            let filtered: Vec<_> = workspaces
                .into_iter()
                .filter(|ws| {
                    let ws_file = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    ws_file.contains(member_name) || ws.path.value.contains(member_name)
                })
                .collect();
            if filtered.is_empty() {
                eprintln!("[error] no workspace member matching '{member_name}'");
                eprintln!();
                eprintln!("Available members:");
                for ws in &all_workspaces {
                    let name = std::path::Path::new(&ws.path.value)
                        .file_name()
                        .map(|n| n.to_string_lossy())
                        .unwrap_or_default();
                    eprintln!("  - {} ({})", name, ws.workspace_type);
                }
                eprintln!();
                eprintln!("Usage: lint-arwaky-cli scan {path} --member <name>");
                return ExitCode::from(1);
            }
            filtered
        } else {
            workspaces
        };

        // Collect ALL source files from workspace root for cross-workspace orphan detection
        let scan_root = match crate::surface_check_action::find_workspace_root(path) {
            Some(r) => r,
            None => std::path::PathBuf::from(path),
        };
        let all_source_files: Vec<String> = code_analysis::collect_all_source_files_raw(&scan_root)
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
            let (
                code_analysis_linter,
                naming_orchestrator,
                import_orchestrator,
                role_orchestrator,
                orphan_orchestrator,
                layer_detector,
            ) = if let Some(ref factory) = self.factory {
                let ctx = factory(ws.config.clone());
                (
                    ctx.code_analysis_linter,
                    ctx.naming_orchestrator,
                    ctx.import_orchestrator,
                    ctx.role_orchestrator,
                    ctx.orphan_orchestrator,
                    ctx.layer_detector,
                )
            } else {
                (
                    self.code_analysis_linter.clone(),
                    self.naming_orchestrator.clone(),
                    self.import_orchestrator.clone(),
                    self.role_orchestrator.clone(),
                    self.orphan_orchestrator.clone(),
                    self.layer_detector.clone(),
                )
            };

            let aes_results = code_analysis_linter.run_code_analysis(&ws.path.value);
            all_results.extend(aes_results.values);

            let (naming_results, import_results, external_results, role_results) =
                rt.block_on(async {
                    tokio::join!(
                        naming_orchestrator.run_audit(&ws.path),
                        import_orchestrator.run_audit(&ws.path),
                        self.external_lint.scan_all(&ws.path),
                        role_orchestrator.run_audit(&ws.path),
                    )
                });
            all_results.extend(naming_results);
            all_results.extend(import_results);
            all_results.extend(external_results.values);
            all_results.extend(role_results);

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
                        let rule_code = r.code.to_string();
                        if ws.config.ignored_rules.values.contains(&rule_code) {
                            return false;
                        }
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        r.code.code() == code
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
                        let rule_code = r.code.to_string();
                        if ws.config.ignored_rules.values.contains(&rule_code) {
                            return false;
                        }
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(true)
                    })
                    .collect()
            };

            let member_orphans = orphan_orchestrator.check_orphans(
                layer_detector.as_ref(),
                &all_source_files,
                &scan_root.to_string_lossy(),
            );

            // Filter orphan results to this workspace member's path
            let filtered_orphans: Vec<_> = if let Some(code) = filter {
                member_orphans
                    .into_iter()
                    .filter(|r| {
                        let rule_code = r.code.to_string();
                        if ws.config.ignored_rules.values.contains(&rule_code) {
                            return false;
                        }
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        r.code.code() == code
                            && ws_canonical
                                .as_ref()
                                .map(|c| abs_path.starts_with(c))
                                .unwrap_or(true)
                    })
                    .collect()
            } else {
                member_orphans
                    .into_iter()
                    .filter(|r| {
                        let rule_code = r.code.to_string();
                        if ws.config.ignored_rules.values.contains(&rule_code) {
                            return false;
                        }
                        let abs_path = cwd_for_ws.join(&r.file.value);
                        ws_canonical
                            .as_ref()
                            .map(|c| abs_path.starts_with(c))
                            .unwrap_or(true)
                    })
                    .collect()
            };

            // Merge per-member results with filtered orphans for this workspace
            let mut member_results = filtered_results;
            member_results.extend(filtered_orphans);

            global_all_results.extend(member_results.clone());

            if multi {
                let total = member_results.len();
                println!("── [{ws_type}] {ws_name} — {total} violations ──");
                if !member_results.is_empty() {
                    let mut code_counts: std::collections::HashMap<String, usize> =
                        std::collections::HashMap::new();
                    for r in &member_results {
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
                // Single workspace — print full violation detail (respects --format)
                match format {
                    Format::Text => {
                        let results_list = LintResultList::new(member_results.clone());
                        print!(
                            "{}",
                            code_analysis_linter.format_report(&results_list, &ws.path.value)
                        );
                    }
                    Format::Json => {
                        let json = serde_json::to_string_pretty(&member_results)
                            .unwrap_or_else(|_| "[]".to_string());
                        println!("{json}");
                    }
                    Format::Sarif => {
                        let sarif = self.format_sarif_output(&member_results);
                        println!("{sarif}");
                    }
                    Format::Junit => {
                        let junit = self.format_junit_output(&member_results);
                        println!("{junit}");
                    }
                }
            }
        }

        if multi {
            match format {
                Format::Text => {
                    self.print_multi_workspace_summary(&global_all_results, &workspaces, member);
                }
                Format::Json => {
                    let json = serde_json::to_string_pretty(&global_all_results)
                        .unwrap_or_else(|_| "[]".to_string());
                    println!("{json}");
                }
                Format::Sarif => {
                    let sarif = self.format_sarif_output(&global_all_results);
                    println!("{sarif}");
                }
                Format::Junit => {
                    let junit = self.format_junit_output(&global_all_results);
                    println!("{junit}");
                }
            }
        }
        if global_all_results.is_empty() {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        }
    }

    /// Format results as a SARIF 2.1.0 JSON string.
    fn format_sarif_output(
        &self,
        results: &[shared::cli_commands::taxonomy_result_vo::LintResult],
    ) -> String {
        use shared::cli_commands::taxonomy_severity_vo::Severity;

        #[derive(serde::Serialize)]
        struct SarifLog {
            #[serde(rename = "$schema")]
            schema: &'static str,
            version: &'static str,
            runs: Vec<SarifRun>,
        }

        #[derive(serde::Serialize)]
        struct SarifRun {
            tool: SarifTool,
            results: Vec<SarifResult>,
        }

        #[derive(serde::Serialize)]
        struct SarifTool {
            driver: SarifDriver,
        }

        #[derive(serde::Serialize)]
        struct SarifDriver {
            name: &'static str,
            version: &'static str,
            information_uri: &'static str,
        }

        #[derive(serde::Serialize)]
        struct SarifResult {
            rule_id: String,
            level: String,
            message: SarifMessage,
            locations: Vec<SarifLocation>,
        }

        #[derive(serde::Serialize)]
        struct SarifMessage {
            text: String,
        }

        #[derive(serde::Serialize)]
        struct SarifLocation {
            physical_location: SarifPhysicalLocation,
        }

        #[derive(serde::Serialize)]
        struct SarifPhysicalLocation {
            artifact_location: SarifArtifactLocation,
            region: SarifRegion,
        }

        #[derive(serde::Serialize)]
        struct SarifArtifactLocation {
            uri: String,
        }

        #[derive(serde::Serialize)]
        struct SarifRegion {
            start_line: i64,
        }

        // Map Severity → SARIF level
        fn severity_to_sarif_level(sev: &Severity) -> &'static str {
            match sev {
                Severity::CRITICAL | Severity::HIGH => "error",
                Severity::MEDIUM => "warning",
                Severity::LOW | Severity::INFO => "note",
            }
        }

        let sarif_results: Vec<SarifResult> = results
            .iter()
            .map(|r| SarifResult {
                rule_id: r.code.to_string(),
                level: severity_to_sarif_level(&r.severity).to_string(),
                message: SarifMessage {
                    text: r.message.value.clone(),
                },
                locations: vec![SarifLocation {
                    physical_location: SarifPhysicalLocation {
                        artifact_location: SarifArtifactLocation {
                            uri: r.file.value.clone(),
                        },
                        region: SarifRegion {
                            start_line: std::cmp::max(1, r.line.value()),
                        },
                    },
                }],
            })
            .collect();

        let log = SarifLog {
            schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
            version: "2.1.0",
            runs: vec![SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "lint-arwaky",
                        version: env!("CARGO_PKG_VERSION"),
                        information_uri: "https://github.com/rakaarwaky/lint-arwaky",
                    },
                },
                results: sarif_results,
            }],
        };

        serde_json::to_string_pretty(&log).unwrap_or_else(|_| "{}".to_string())
    }

    /// Format results as JUnit XML.
    fn format_junit_output(
        &self,
        results: &[shared::cli_commands::taxonomy_result_vo::LintResult],
    ) -> String {
        let total = results.len();
        let failures: Vec<_> = results
            .iter()
            .filter(|r| {
                use shared::cli_commands::taxonomy_severity_vo::Severity;
                matches!(
                    r.severity,
                    Severity::CRITICAL | Severity::HIGH | Severity::MEDIUM | Severity::LOW
                )
            })
            .collect();
        let failure_count = failures.len();

        let mut xml = String::with_capacity(total.saturating_mul(256));
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
            "<testsuites name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));
        xml.push_str(&format!(
            "  <testsuite name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));

        for r in results {
            let classname = xml_escape(&r.code.to_string());
            let name = xml_escape(&format!("{}:{}", r.file.value, r.line.value()));
            let message = xml_escape(&r.message.value);
            let sev = r.severity.to_string();
            let is_info = r.severity == shared::cli_commands::taxonomy_severity_vo::Severity::INFO;

            xml.push_str(&format!(
                "    <testcase classname=\"{classname}\" name=\"{name}\">\n"
            ));
            if !is_info {
                xml.push_str(&format!(
                    "      <failure message=\"{sev}: {message}\" type=\"{sev}\">\n"
                ));
                xml.push_str(&format!("        {message}\n"));
                xml.push_str("      </failure>\n");
            }
            xml.push_str("    </testcase>\n");
        }

        xml.push_str("  </testsuite>\n");
        xml.push_str("</testsuites>\n");
        xml
    }

    /// Print multi-workspace text summary (extracted from scan_with_discovery).
    fn print_multi_workspace_summary(
        &self,
        global_all_results: &[shared::cli_commands::taxonomy_result_vo::LintResult],
        workspaces: &[shared::config_system::taxonomy_multi_project_workspace_info_vo::WorkspaceInfo],
        member: Option<&str>,
    ) {
        use std::collections::HashMap;

        let mut global_all_counts: HashMap<String, usize> = HashMap::new();
        for r in global_all_results {
            *global_all_counts.entry(r.code.to_string()).or_insert(0) += 1;
        }
        let global_total = global_all_results.len();
        let global_code_counts: HashMap<String, usize> = global_all_counts
            .iter()
            .filter(|(code, _)| code.starts_with("AES"))
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        let global_unique_codes = global_code_counts.len();
        let external_code_counts: HashMap<String, usize> = global_all_counts
            .iter()
            .filter(|(code, _)| !code.starts_with("AES"))
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        let global_unique_external = external_code_counts.len();

        println!("============================================================");
        println!("  Combined Multi-Workspace Report Summary");
        println!("============================================================");
        println!("  Total Workspace Members: {}", workspaces.len());
        println!("  Total Unique AES Codes: {global_unique_codes}");
        if global_unique_external > 0 {
            println!("  Total Unique External Codes: {global_unique_external}");
        }
        println!("  Total Violations: {global_total}");
        println!();
        let mut sorted: Vec<_> = global_code_counts.into_iter().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
        for (code, count) in &sorted {
            println!("  {code}: {count}");
        }
        if !external_code_counts.is_empty() {
            println!();
            println!("  ── External Lint Codes ──");
            let mut ext_sorted: Vec<_> = external_code_counts.into_iter().collect();
            ext_sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
            for (code, count) in &ext_sorted {
                println!("  {code}: {count}");
            }
        }

        if member.is_none() {
            println!();
            println!("============================================================");
            println!("  Scan Individual Members");
            println!("============================================================");
            println!("  To scan a specific workspace member:");
            println!("    lint-arwaky-cli scan . --member <name>");
            println!();
            println!("  Available members:");
            for ws in workspaces {
                let name = std::path::Path::new(&ws.path.value)
                    .file_name()
                    .map(|n| n.to_string_lossy())
                    .unwrap_or_default();
                println!("    - {} ({})", name, ws.workspace_type);
            }
            println!();
            println!("  Filter by AES rule code:");
            println!("    lint-arwaky-cli scan . --filter AES204");
        }
    }
}

/// XML-escape a string for safe inclusion in JUnit XML output.
fn xml_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
}
