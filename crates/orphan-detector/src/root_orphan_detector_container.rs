use crate::agent_orphan_orchestrator::{ArchOrphanAnalyzer, ArchOrphanDeps};
use crate::capabilities_orphan_graph_resolver::OrphanGraphResolver;
use crate::capabilities_orphan_parser_dispatcher::OrphanParserDispatcher;
use shared::common::FilePath;
use shared::config_system::{ArchitectureConfig, IConfigOrchestratorAggregate};

use shared::orphan_detector::{
    IOrphanAggregate, IOrphanGraphResolverProtocol, IOrphanParserProtocol,
};

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
        let parser_dispatcher: Arc<dyn IOrphanParserProtocol> =
            Arc::new(OrphanParserDispatcher::new());
        let resolver: Arc<dyn IOrphanGraphResolverProtocol> =
            Arc::new(OrphanGraphResolver::new(parser_dispatcher.clone()));

        let arch = Arc::new(ArchOrphanAnalyzer::new(
            ArchOrphanDeps {
                resolver,
                taxonomy_analyzer: Arc::new(
                    crate::capabilities_orphan_taxonomy_analyzer::TaxonomyOrphanAnalyzer::new(),
                ),
                contract_analyzer: Arc::new(
                    crate::capabilities_orphan_contract_analyzer::ContractOrphanAnalyzer::new(
                        parser_dispatcher.clone(),
                    ),
                ),
                capabilities_analyzer: Arc::new(
                    crate::capabilities_orphan_capabilities_analyzer::CapabilitiesOrphanAnalyzer::new(
                        parser_dispatcher.clone(),
                    ),
                ),
                utility_analyzer: Arc::new(
                    crate::capabilities_orphan_utility_analyzer::UtilityOrphanAnalyzer::new(
                        parser_dispatcher.clone(),
                    ),
                ),
                agent_analyzer: Arc::new(
                    crate::capabilities_orphan_agent_analyzer::AgentOrphanAnalyzer::new(
                        parser_dispatcher.clone(),
                    ),
                ),
                surfaces_analyzer: Arc::new(
                    crate::capabilities_orphan_surfaces_analyzer::SurfacesOrphanAnalyzer::new(),
                ),
            },
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
        let fp = FilePath::new(project_root.to_string()).unwrap_or_default();
        let config = orchestrator.load_config_sync(&fp);
        Self::new_with_config(config)
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
