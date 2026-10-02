// PURPOSE: StructureOrchestrator — routes audit requests to the three
// structural-auditor capability seams (AES701, AES702, AES703) and folds
// their findings into the aggregate response.
use shared_structure_rules::contract_structure_aggregate::IStructureAggregate;
use shared_structure_rules::contract_structure_protocol::{
    IStructureFeatureHealthProtocol, IStructureSharedPurityProtocol,
    IStructureSurfacePurityProtocol, IStructureTestSuiteProtocol,
};
use shared_structure_rules::taxonomy_structure_rules_request::{
    StructureFinding, StructureRequest,
};
use shared_structure_rules::taxonomy_structure_rules_response::StructureResponse;
use shared_structure_rules::utility_structure_parsers::sorted;
use std::sync::Arc;

/// Orchestrates the folder-layout audit over three capability seams.
pub struct StructureOrchestrator {
    shared: Arc<dyn IStructureSharedPurityProtocol>,
    feature: Arc<dyn IStructureFeatureHealthProtocol>,
    surface: Arc<dyn IStructureSurfacePurityProtocol>,
    test_suite: Arc<dyn IStructureTestSuiteProtocol>,
}

impl IStructureAggregate for StructureOrchestrator {
    fn execute(&self, request: StructureRequest) -> StructureResponse {
        let s1 = self.shared.audit_shared(request.clone());
        let s2 = self.feature.audit_feature(request.clone());
        let s3 = self.surface.audit_surface(request.clone());
        let s4 = self.test_suite.audit_test_suite(request);
        let mut all: Vec<StructureFinding> = Vec::new();
        for response in [s1, s2, s3, s4] {
            let StructureResponse::Findings { findings } = response;
            all.extend(findings);
        }
        StructureResponse::Findings {
            findings: sorted(all),
        }
    }
}

impl StructureOrchestrator {
    /// Wrap four capability seams in a fresh orchestrator.
    pub fn new(
        shared: Arc<dyn IStructureSharedPurityProtocol>,
        feature: Arc<dyn IStructureFeatureHealthProtocol>,
        surface: Arc<dyn IStructureSurfacePurityProtocol>,
        test_suite: Arc<dyn IStructureTestSuiteProtocol>,
    ) -> Self {
        Self {
            shared,
            feature,
            surface,
            test_suite,
        }
    }
}
