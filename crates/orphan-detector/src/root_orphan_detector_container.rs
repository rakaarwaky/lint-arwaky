// PURPOSE: OrphanContainer — wiring for orphan-detector feature (root layer, wiring only)
use crate::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use crate::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use std::sync::Arc;

pub struct OrphanContainer {
    analyzer: Arc<dyn IOrphanAggregate>,
    layer_detector: Arc<dyn ILayerDetectionAggregate>,
}

impl OrphanContainer {
    pub fn new() -> Self {
        Self::new_with_ignored(Vec::new())
    }

    pub fn new_with_ignored(_ignored_paths: Vec<String>) -> Self {
        let resolver: Arc<dyn IOrphanGraphResolverProtocol> = Arc::new(OrphanGraphResolver::new());
        let arch = Arc::new(ArchOrphanAnalyzer::new(
            resolver,
            Arc::new(crate::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_infrastructure_analyzer::InfrastructureOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer::new()),
        ));
        let layer: Arc<dyn ILayerDetectionAggregate> = arch.clone();
        Self {
            analyzer: arch.clone() as Arc<dyn IOrphanAggregate>,
            layer_detector: layer,
        }
    }

    pub fn analyzer(&self) -> Arc<dyn IOrphanAggregate> {
        self.analyzer.clone()
    }

    pub fn layer_detector(&self) -> Arc<dyn ILayerDetectionAggregate> {
        self.layer_detector.clone()
    }
}
impl Default for OrphanContainer {
    fn default() -> Self {
        Self::new()
    }
}
