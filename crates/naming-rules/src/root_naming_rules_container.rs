// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use crate::agent_naming_orchestrator::{NamingOrchestrator, NamingOrchestratorDeps};
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::naming_rules::INamingCheckerProtocol;
use shared::naming_rules::INamingRunnerAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct NamingContainer {
    naming_checker: Arc<dyn INamingCheckerProtocol>,
    orchestrator: Arc<dyn INamingRunnerAggregate>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl NamingContainer {
    pub fn new(config: Arc<ArchitectureConfig>, layer_map: Arc<LayerMapVO>) -> Self {
        let naming_checker: Arc<dyn INamingCheckerProtocol> =
            Arc::new(crate::capabilities_naming_checker::NamingChecker::new());

        let orchestrator = Arc::new(NamingOrchestrator::new(NamingOrchestratorDeps {
            naming_checker: naming_checker.clone(),
            config,
            layer_map,
        }));

        Self {
            naming_checker,
            orchestrator,
        }
    }

    pub fn naming_checker(&self) -> &Arc<dyn INamingCheckerProtocol> {
        &self.naming_checker
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        self.orchestrator.clone()
    }
}
