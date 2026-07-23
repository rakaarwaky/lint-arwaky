// PURPOSE: McpContainer — DI wiring for MCP server aggregates
use std::sync::Arc;

use cli_commands::agent_analysis_pipeline_orchestrator::{
    AnalysisPipelineDeps, AnalysisPipelineOrchestrator,
};
use shared::cli_commands::contract_analysis_pipeline_aggregate::IAnalysisPipelineAggregate;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

pub struct McpContainer {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub analysis_pipeline: Arc<dyn IAnalysisPipelineAggregate>,
}

impl McpContainer {
    pub fn new_default() -> Self {
        // Create config orchestrator — single source of truth for config
        let config_container = config_system::root_config_system_container::ConfigContainer::new();
        let orchestrator = config_container.orchestrator();

        // All containers get config from orchestrator
        let code_analysis_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::from_orchestrator(
                &orchestrator,
                ".",
            )
            .code_analysis_linter();

        let import_container =
            import_rules::root_import_rules_container::ImportContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let import_orchestrator = import_container.orchestrator();

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let naming_orchestrator = naming_container.orchestrator();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let orphan_orchestrator = orphan_container.analyzer();

        let ext_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = ext_container.aggregate();

        let role_container =
            role_rules::root_role_rules_container::RoleContainer::from_orchestrator(
                &orchestrator,
                ".",
            );
        let role_orchestrator = role_container.orchestrator();

        // Wire analysis pipeline orchestrator — same deps as CLI, just reused for MCP
        let analysis_pipeline: Arc<dyn IAnalysisPipelineAggregate> =
            Arc::new(AnalysisPipelineOrchestrator::new(AnalysisPipelineDeps {
                code_analysis_linter: code_analysis_linter.clone(),
                naming_orchestrator: naming_orchestrator.clone(),
                import_orchestrator: import_orchestrator.clone(),
                external_lint: external_lint.clone(),
                role_orchestrator: role_orchestrator.clone(),
                orphan_orchestrator: orphan_orchestrator.clone(),
                config_orchestrator: orchestrator.clone(),
                format: Format::Text,
            }));

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            orphan_orchestrator,
            external_lint,
            role_orchestrator,
            config_orchestrator: orchestrator,
            analysis_pipeline,
        }
    }
}
