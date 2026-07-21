// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
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

    pub fn naming_convention_checker(&self) -> &Arc<dyn INamingConventionChecker> {
        &self.naming_convention_checker
    }

    pub fn suffix_prefix_checker(&self) -> &Arc<dyn ISuffixPrefixChecker> {
        &self.suffix_prefix_checker
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        Arc::new(crate::agent_naming_orchestrator::NamingOrchestrator::new(
            self.naming_convention_checker.clone(),
            self.suffix_prefix_checker.clone(),
            self.config.clone(),
            self.layer_map.clone(),
        ))
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for NamingContainer {
    fn default() -> Self {
        let config = Arc::new(shared::config_system::utility_config_defaults::default_aes_config());
        let layer_map = Arc::new(LayerMapVO::new(std::collections::HashMap::new()));
        Self::new(config, layer_map)
    }
}
