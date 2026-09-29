// PURPOSE: StructureOrchestrator — routes audit requests to the three
// structural-auditor capability seams (AES701, AES702, AES703) and folds
// their findings into the aggregate response.
use shared::structure_rules::contract_structure_aggregate::IStructureAggregate;
use shared::structure_rules::contract_structure_protocol::{
    IStructureFeatureHealthProtocol, IStructureSharedPurityProtocol,
    IStructureSurfacePurityProtocol,
};
use shared::structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared::structure_rules::taxonomy_structure_rules_response::StructureResponse;
use shared::structure_rules::utility_structure_parsers::sorted;
use std::sync::Arc;

/// Orchestrates the folder-layout audit over three capability seams.
pub struct StructureOrchestrator {
    shared: Arc<dyn IStructureSharedPurityProtocol>,
    feature: Arc<dyn IStructureFeatureHealthProtocol>,
    surface: Arc<dyn IStructureSurfacePurityProtocol>,
}

impl IStructureAggregate for StructureOrchestrator {
    fn execute(&self, request: StructureRequest) -> StructureResponse {
        let s1 = self.shared.audit_shared(request.clone());
        let s2 = self.feature.audit_feature(request.clone());
        let s3 = self.surface.audit_surface(request);
        let mut all: Vec<StructureFinding> = Vec::new();
        let StructureResponse::Findings { findings } = s1;
        all.extend(findings);
        let StructureResponse::Findings { findings } = s2;
        all.extend(findings);
        let StructureResponse::Findings { findings } = s3;
        all.extend(findings);
        StructureResponse::Findings {
            findings: sorted(all),
        }
    }
}

impl StructureOrchestrator {
    /// Wrap three capability seams in a fresh orchestrator.
    pub fn new(
        shared: Arc<dyn IStructureSharedPurityProtocol>,
        feature: Arc<dyn IStructureFeatureHealthProtocol>,
        surface: Arc<dyn IStructureSurfacePurityProtocol>,
    ) -> Self {
        Self {
            shared,
            feature,
            surface,
        }
    }
}
