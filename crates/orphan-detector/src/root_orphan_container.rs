// PURPOSE: OrphanContainer — wiring for orphan-detector feature (root layer, wiring only)
use std::sync::Arc;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;

pub struct OrphanContainer {
    analyzer: Arc<dyn IOrphanAggregate>,
}

impl OrphanContainer {
    pub fn new() -> Self {
        Self {
            analyzer: Arc::new(
                crate::agent_orphan_orchestrator::ArchOrphanAnalyzer::new(
                    Arc::new(crate::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer::new()),
                    Arc::new(crate::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer::new()),
                    Arc::new(crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new()),
                    Arc::new(crate::capabilities_orphan_infrastructure_analyzer::InfrastructureOrphanAnalyzer::new()),
                    Arc::new(crate::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer::new()),
                    Arc::new(crate::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer::new()),
                ),
            ),
        }
    }

    pub fn analyzer(&self) -> Arc<dyn IOrphanAggregate> {
        self.analyzer.clone()
    }
}
impl Default for OrphanContainer {
    fn default() -> Self {
        Self::new()
    }
}

