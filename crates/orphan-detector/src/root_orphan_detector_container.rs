use crate::agent_orphan_orchestrator::ArchOrphanAnalyzer;
use crate::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
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
        let config = ArchitectureConfig {
            ignored_paths: shared::common::taxonomy_paths_vo::FilePathList::new(
                ignored_paths
                    .into_iter()
                    .filter_map(|p| shared::common::taxonomy_path_vo::FilePath::new(p).ok())
                    .collect(),
            ),
            ..Default::default()
        };
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

    /// Create from config orchestrator — the canonical way per AES architecture.
    pub fn from_orchestrator(
        orchestrator: &Arc<dyn IConfigOrchestratorAggregate>,
        project_root: &str,
    ) -> Self {
        let config = orchestrator.load_config_sync(project_root);
        Self::new_with_config(config)
    }

    pub fn analyzer(&self) -> Arc<dyn IOrphanAggregate> {
        self.analyzer.clone()
    }
}
