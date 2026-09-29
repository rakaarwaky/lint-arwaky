// PURPOSE: NamingContainer — wiring for naming-rules feature (root layer, wiring only)
use crate::agent_naming_orchestrator::{NamingOrchestrator, NamingOrchestratorDeps};
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::config_system::taxonomy_config_system_vo::ArchitectureConfig;
use shared::naming_rules::INamingConventionProtocol;
use shared::naming_rules::INamingRunnerAggregate;
use shared::naming_rules::ISuffixPolicyProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct NamingContainer {
    naming_convention: Arc<dyn INamingConventionProtocol>,
    suffix_policy: Arc<dyn ISuffixPolicyProtocol>,
    orchestrator: Arc<dyn INamingRunnerAggregate>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

impl NamingContainer {
    pub fn new(config: Arc<ArchitectureConfig>, layer_map: Arc<LayerMapVO>) -> Self {
        let checker = Arc::new(crate::capabilities_naming_checker::NamingChecker::new());
        let naming_convention: Arc<dyn INamingConventionProtocol> = checker.clone();
        let suffix_policy: Arc<dyn ISuffixPolicyProtocol> = checker.clone();

        let orchestrator = Arc::new(NamingOrchestrator::new(NamingOrchestratorDeps {
            naming_convention: naming_convention.clone(),
            suffix_policy: suffix_policy.clone(),
            config,
            layer_map,
        }));

        Self {
            naming_convention,
            suffix_policy,
            orchestrator,
        }
    }

    /// The AES101 stem-shape capability, wired independently of AES102.
    pub fn naming_convention(&self) -> &Arc<dyn INamingConventionProtocol> {
        &self.naming_convention
    }

    /// The AES102 suffix/prefix policy capability, wired independently of AES101.
    pub fn suffix_policy(&self) -> &Arc<dyn ISuffixPolicyProtocol> {
        &self.suffix_policy
    }

    pub fn orchestrator(&self) -> Arc<dyn INamingRunnerAggregate> {
        self.orchestrator.clone()
    }
}
