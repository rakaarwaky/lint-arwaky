use crate::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use crate::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
use shared::orphan_detector::contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
use std::sync::Arc;

pub struct OrphanContainer {
    analyzer: Arc<dyn IOrphanAggregate>,
}

impl OrphanContainer {
    pub fn new() -> Self {
        Self::new_with_config(ArchitectureConfig::default())
    }

    pub fn new_with_ignored(ignored_paths: Vec<String>) -> Self {
        let mut config = ArchitectureConfig::default();
        config.ignored_paths = shared::common::taxonomy_paths_vo::FilePathList::new(
            ignored_paths
                .into_iter()
                .filter_map(|p| shared::common::taxonomy_path_vo::FilePath::new(p).ok())
                .collect(),
        );
        Self::new_with_config(config)
    }

    pub fn new_with_config(config: ArchitectureConfig) -> Self {
        let resolver: Arc<dyn IOrphanGraphResolverProtocol> = Arc::new(OrphanGraphResolver::new());
        let arch = Arc::new(ArchOrphanAnalyzer::new(
            resolver,
            Arc::new(crate::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer::new()),
            Arc::new(
                crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new(),
            ),
            Arc::new(crate::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer::new()),
            Arc::new(crate::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer::new()),
            config,
        ));
        Self {
            analyzer: arch.clone() as Arc<dyn IOrphanAggregate>,
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
