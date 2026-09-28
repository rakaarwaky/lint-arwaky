// PURPOSE: root container — structure-rules DI composition root
//
// Wires the capability into the agent behind the contract trait. The
// container is a constructor and a one-liner facade that returns the
// orchestrator; no business logic lives here.
use crate::agent_structure_orchestrator::StructureOrchestrator;
use crate::capabilities_structure_auditor::StructureAuditor;
use shared::structure_rules::contract_structure_aggregate::IStructureAggregate;
use shared::structure_rules::contract_structure_protocol::IStructureAuditProtocol;
use std::sync::Arc;

/// Composition root for the structure-rules feature.
pub struct RootStructureRulesContainer;

impl RootStructureRulesContainer {
    /// Return a fresh orchestrator for this workspace.
    pub fn orchestrator() -> Arc<dyn IStructureAggregate> {
        let auditor: Arc<dyn IStructureAuditProtocol> = Arc::new(StructureAuditor {});
        Arc::new(StructureOrchestrator::new(auditor))
    }
}
