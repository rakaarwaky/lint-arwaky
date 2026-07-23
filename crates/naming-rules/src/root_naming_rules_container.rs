// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use crate::agent_naming_orchestrator::{NamingOrchestrator, NamingOrchestratorDeps};
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::contract_naming_checker_protocol::{
    INamingConventionChecker, ISuffixPrefixChecker,
};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::taxonomy_definition_vo::LayerMapVO;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct NamingContainer {
    naming_convention_checker: Arc<dyn INamingConventionChecker>,
    suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker>,
    config: Arc<ArchitectureConfig>,
    layer_map: Arc<LayerMapVO>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl NamingContainer {
    pub fn new(config: Arc<ArchitectureConfig>, layer_map: Arc<LayerMapVO>) -> Self {
        let naming_convention_checker: Arc<dyn INamingConventionChecker> =
            Arc::new(crate::capabilities_naming_convention_checker::NamingConventionChecker::new());
        let suffix_prefix_checker: Arc<dyn ISuffixPrefixChecker> =
            Arc::new(crate::capabilities_suffix_prefix_checker::SuffixPrefixChecker::new());
        Self {
            naming_convention_checker,
            suffix_prefix_checker,
            config,
            layer_map,
        }
    }

    /// Create from config orchestrator — the canonical way per AES architecture.
    pub fn from_orchestrator(
        orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
        project_root: &str,
    ) -> Self {
        let config = Arc::new(orchestrator.load_config_sync(project_root));
        let layer_map = Arc::new(LayerMapVO::new(config.layers.clone()));
        Self::new(config, layer_map)
    }

    pub fn naming_convention_checker(&self) -> &Arc<dyn INamingConventionChecker> {
        &self.naming_convention_checker
    }

    pub fn suffix_prefix_checker(&self) -> &Arc<dyn ISuffixPrefixChecker> {
        &self.suffix_prefix_checker
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(NamingOrchestrator::new(NamingOrchestratorDeps {
            naming_convention_checker: self.naming_convention_checker.clone(),
            suffix_prefix_checker: self.suffix_prefix_checker.clone(),
            config: self.config.clone(),
            layer_map: self.layer_map.clone(),
        }))
    }
}