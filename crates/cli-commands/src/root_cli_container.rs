// PURPOSE: CliContainer — DI wiring for CLI binary aggregates
use std::sync::Arc;

use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
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
    pub git_aggregate: Arc<dyn GitHooksAggregate>,
    pub multi_project_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub report_formatter: Arc<dyn IReportFormatterAggregate>,
}

fn make_layer_map() -> (
    shared::config_system::taxonomy_config_vo::ArchitectureConfig,
    shared::taxonomy_definition_vo::LayerMapVO,
) {
    let aes_config = shared::config_system::utility_config_defaults::default_aes_config();
    let (merged_layers, _) =
        shared::config_system::utility_config_merger::merge_config(&aes_config);
    let mut config = aes_config;
    config.layers = merged_layers;
    let layer_map = shared::taxonomy_definition_vo::LayerMapVO::new(config.layers.clone());
    (config, layer_map)
}

impl CliContainer {
    pub fn new_default() -> Self {
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_default();

        let (config, layer_map) = make_layer_map();

        let code_analysis_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new_with_config(
                config, layer_map,
            )
            .code_analysis_linter();
        let import_orchestrator = import_container.orchestrator();

        let role_container = role_rules::root_role_rules_container::RoleContainer::new();
        let role_orchestrator = role_container.orchestrator();

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::default();
        let naming_orchestrator = naming_container.orchestrator();

        let external_lint_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = external_lint_container.aggregate();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let orphan_orchestrator = orphan_container.analyzer();

        let config_container = config_system::root_config_system_container::ConfigContainer::new();
        let multi_project_orchestrator = config_container.orchestrator();

        let git_container = git_hooks::root_git_hooks_container::GitContainer::new_default();
        let git_aggregate = git_container.aggregate();

        // Wire up report formatter capabilities → aggregate
        let text_formatter: Arc<
            dyn shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol,
        > = Arc::new(crate::capabilities_text_formatter::TextFormatter::new(
            code_analysis_linter.clone(),
        ));
        let json_formatter: Arc<
            dyn shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol,
        > = Arc::new(crate::capabilities_json_formatter::JsonFormatter::new());
        let sarif_formatter: Arc<
            dyn shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol,
        > = Arc::new(crate::capabilities_sarif_formatter::SarifFormatter::new());
        let junit_formatter: Arc<
            dyn shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol,
        > = Arc::new(crate::capabilities_junit_formatter::JunitFormatter::new());
        let report_formatter: Arc<dyn IReportFormatterAggregate> = Arc::new(
            crate::agent_report_formatter_orchestrator::ReportFormatterOrchestrator::new(
                text_formatter,
                json_formatter,
                sarif_formatter,
                junit_formatter,
            ),
        );

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            role_orchestrator,
            external_lint,
            orphan_orchestrator,
            git_aggregate,
            multi_project_orchestrator,
            report_formatter,
        }
    }

    pub fn check_context(&self) -> crate::surface_check_command::CheckContext {
        crate::surface_check_command::CheckContext {
            code_analysis_linter: self.code_analysis_linter.clone(),
            import_orchestrator: self.import_orchestrator.clone(),
            naming_orchestrator: self.naming_orchestrator.clone(),
            external_lint: self.external_lint.clone(),
            role_orchestrator: self.role_orchestrator.clone(),
            orphan_orchestrator: self.orphan_orchestrator.clone(),
            report_formatter: self.report_formatter.clone(),
        }
    }
}
