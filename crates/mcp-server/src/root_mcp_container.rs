// PURPOSE: McpContainer — DI wiring for MCP server aggregates
use std::sync::Arc;

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

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            orphan_orchestrator,
            external_lint,
            role_orchestrator,
        }
    }
}
