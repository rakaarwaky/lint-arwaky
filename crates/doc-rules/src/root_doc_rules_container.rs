// PURPOSE: root container — doc-rules DI composition root
//
// Wires the ten per-doc capability seams into the agent behind the contract
// trait. The container is where concrete capability types are named — the
// agent and the capabilities never see each other, only the protocols. No
// business logic lives here.
use crate::agent_doc_orchestrator::DocOrchestrator;
use crate::capabilities_doc_agents::AgentsCapability;
use crate::capabilities_doc_architecture::ArchitectureCapability;
use crate::capabilities_doc_backlog::BacklogCapability;
use crate::capabilities_doc_contributing::ContributingCapability;
use crate::capabilities_doc_data::DataCapability;
use crate::capabilities_doc_design::DesignCapability;
use crate::capabilities_doc_frd::FrdCapability;
use crate::capabilities_doc_prd::PrdCapability;
use crate::capabilities_doc_readme::ReadmeCapability;
use crate::capabilities_doc_roadmap::RoadmapCapability;
use shared_doc_rules::contract_doc_aggregate::IDocRunnerAggregate;
use shared_doc_rules::contract_doc_protocol::{
    IAgentsDocProtocol, IArchitectureDocProtocol, IBacklogDocProtocol, IContributingDocProtocol,
    IDataDocProtocol, IDesignDocProtocol, IFrdDocProtocol, IPrdDocProtocol, IReadmeDocProtocol,
    IRoadmapDocProtocol,
};
use std::sync::Arc;

/// Composition root for the doc-rules feature.
pub struct RootDocRulesContainer;

impl RootDocRulesContainer {
    /// Return a fresh orchestrator for this workspace.
    pub fn orchestrator() -> Arc<dyn IDocRunnerAggregate> {
        let agents: Arc<dyn IAgentsDocProtocol> = Arc::new(AgentsCapability::new());
        let architecture: Arc<dyn IArchitectureDocProtocol> = Arc::new(ArchitectureCapability::new());
        let backlog: Arc<dyn IBacklogDocProtocol> = Arc::new(BacklogCapability::new());
        let contributing: Arc<dyn IContributingDocProtocol> = Arc::new(ContributingCapability::new());
        let data: Arc<dyn IDataDocProtocol> = Arc::new(DataCapability::new());
        let design: Arc<dyn IDesignDocProtocol> = Arc::new(DesignCapability::new());
        let frd: Arc<dyn IFrdDocProtocol> = Arc::new(FrdCapability::new());
        let prd: Arc<dyn IPrdDocProtocol> = Arc::new(PrdCapability::new());
        let readme: Arc<dyn IReadmeDocProtocol> = Arc::new(ReadmeCapability::new());
        let roadmap: Arc<dyn IRoadmapDocProtocol> = Arc::new(RoadmapCapability::new());
        Arc::new(DocOrchestrator::new(
            agents,
            architecture,
            backlog,
            contributing,
            data,
            design,
            frd,
            prd,
            readme,
            roadmap,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RootDocRulesContainer::orchestrator creates a valid aggregate without panic.
    #[test]
    fn orchestrator_instantiation_succeeds() {
        let _orch = RootDocRulesContainer::orchestrator();
    }
}
