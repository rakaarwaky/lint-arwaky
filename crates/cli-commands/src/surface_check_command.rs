// PURPOSE: CheckCommandsSurface — CLI surface for check/scan commands
use std::collections::HashMap;
use std::sync::Arc;

use std::process::ExitCode;

use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

use crate::infrastructure_check_context::CheckContext;

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

        // 4. Run role-rules audit (AES401, AES402, AES403, AES404, AES405, AES406)
        let role_results = rt.block_on(role_orchestrator.run_audit(&path_obj));
        all_results.extend(role_results);

        // 5. Run orphan detection
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
        let scan_root = crate::surface_check_main::find_workspace_root(path);
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
}
