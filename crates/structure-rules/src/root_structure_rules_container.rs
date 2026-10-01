// PURPOSE: root container — structure-rules DI composition root
//
// Wires the three capability seams into the agent behind the contract trait.
// The container is a constructor and a one-liner facade that returns the
// orchestrator; no business logic lives here.
use crate::agent_structure_orchestrator::StructureOrchestrator;
use crate::capabilities_feature_health_auditor::FeatureHealthAuditor;
use crate::capabilities_shared_purity_auditor::SharedPurityAuditor;
use crate::capabilities_surface_purity_auditor::SurfacePurityAuditor;
use shared_structure_rules::contract_structure_aggregate::IStructureAggregate;
use std::sync::Arc;

/// Composition root for the structure-rules feature.
pub struct RootStructureRulesContainer;

impl RootStructureRulesContainer {
    /// Return a fresh orchestrator for this workspace.
    pub fn orchestrator() -> Arc<dyn IStructureAggregate> {
        Arc::new(StructureOrchestrator::new(
            Arc::new(SharedPurityAuditor {}),
            Arc::new(FeatureHealthAuditor {}),
            Arc::new(SurfacePurityAuditor {}),
        ))
    }
}
