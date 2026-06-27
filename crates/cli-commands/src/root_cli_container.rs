// PURPOSE: CliContainer — DI wiring for CLI binary aggregates
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::common::contract_scanner_provider_port::IScannerProviderPort;
use shared::config_system::contract_multi_project_orchestrator_aggregate::MultiProjectOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::git_hooks::contract_git_hooks_aggregate::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub struct CliContainer {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub scanner_provider: Arc<dyn IScannerProviderPort>,
    pub git_aggregate: Arc<dyn GitHooksAggregate>,
    pub multi_project_orchestrator: Arc<dyn MultiProjectOrchestratorAggregate>,
}

impl CliContainer {
    pub fn new_default() -> Self {
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_default();
        let analyzer = import_container.analyzer();

        let checker_container =
            code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(
                analyzer.clone(),
            );
        code_analysis::agent_code_analysis_orchestrator::init_global_checker(Arc::new(
            checker_container,
        ));

        let code_analysis_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new()
                .code_analysis_linter();
        let import_orchestrator = import_container.orchestrator();

        let role_container = role_rules::root_role_rules_container::RoleContainer::new();
        let role_orchestrator = role_container.orchestrator();

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::new(analyzer);
        let naming_orchestrator = naming_container.orchestrator();

        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = external_lint_container.aggregate();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let orphan_orchestrator = orphan_container.analyzer();
        let layer_detector = orphan_container.layer_detector();

        let scanner_provider: Arc<dyn IScannerProviderPort> = Arc::new(
            shared::common::infrastructure_file_collector_provider::FileCollectorProvider::new(),
        );

        let config_container =
            config_system::root_config_system_container::ConfigContainer::new();
        let multi_project_orchestrator = config_container.multi_project_orchestrator();

        let git_container = git_hooks::root_git_hooks_container::GitContainer::new_default();
        let git_aggregate = git_container.aggregate();

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            role_orchestrator,
            external_lint,
            orphan_orchestrator,
            layer_detector,
            scanner_provider,
            git_aggregate,
            multi_project_orchestrator,
        }
    }

    pub fn check_context(&self) -> crate::surface_check_command::CheckContext {
        crate::surface_check_command::CheckContext {
            code_analysis_linter: self.code_analysis_linter.clone(),
            import_orchestrator: self.import_orchestrator.clone(),
            naming_orchestrator: self.naming_orchestrator.clone(),
            external_lint: self.external_lint.clone(),
            role_orchestrator: self.role_orchestrator.clone(),
            scanner_provider: self.scanner_provider.clone(),
            orphan_orchestrator: self.orphan_orchestrator.clone(),
            layer_detector: self.layer_detector.clone(),
            language_detector: Arc::new(
                crate::infrastructure_language_detector::CliLanguageDetector::new(),
            ),
        }
    }
}
