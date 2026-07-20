// PURPOSE: McpContainer — DI wiring for MCP server aggregates
use std::sync::Arc;

use shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
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
    pub layer_detector: Arc<dyn ILayerDetectionAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
}

impl McpContainer {
    pub fn new_default() -> Self {
        let import_container =
            import_rules::root_import_rules_container::ImportContainer::new_default();

        let (config, layer_map) = {
            let aes_config = shared::config_system::taxonomy_config_vo::default_aes_config();
            let (merged, _) = shared::config_system::utility_config_merger::merge_config(&aes_config);
            let mut c = aes_config;
            c.layers = merged;
            let lm = shared::taxonomy_definition_vo::LayerMapVO::new(c.layers.clone());
            (c, lm)
        };

        let checker_container =
            code_analysis::root_code_analysis_container::CodeAnalysisCheckerContainer::new(
                config,
                layer_map,
            );
        code_analysis::agent_code_analysis_orchestrator::init_global_checker(Arc::new(
            checker_container,
        ));

        let code_analysis_linter =
            code_analysis::root_code_analysis_container::CodeAnalysisContainer::new()
                .code_analysis_linter();

        let import_orchestrator = import_container.orchestrator();

        let naming_container =
            naming_rules::root_naming_rules_container::NamingContainer::new_default();
        let naming_orchestrator = naming_container.orchestrator();

        let orphan_container =
            orphan_detector::root_orphan_detector_container::OrphanContainer::new();
        let orphan_orchestrator = orphan_container.analyzer();
        let layer_detector = orphan_container.layer_detector();

        let ext_container =
            external_lint::root_external_lint_container::ExternalLintContainer::new_default();
        let external_lint = ext_container.aggregate();

        let role_container = role_rules::root_role_rules_container::RoleContainer::new();
        let role_orchestrator = role_container.orchestrator();

        Self {
            code_analysis_linter,
            import_orchestrator,
            naming_orchestrator,
            orphan_orchestrator,
            layer_detector,
            external_lint,
            role_orchestrator,
        }
    }
}
