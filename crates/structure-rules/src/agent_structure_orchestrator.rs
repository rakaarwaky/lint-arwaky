// PURPOSE: StructureOrchestrator — routes an audit request to the structural auditor
//
// The agent holds the capability seam behind the aggregate contract and
// returns its findings as the aggregate response, so consumers depend on the
// aggregate alone.
use shared::structure_rules::contract_structure_aggregate::IStructureAggregate;
use shared::structure_rules::contract_structure_protocol::IStructureAuditProtocol;
use shared::structure_rules::taxonomy_structure_request::StructureRequest;
use shared::structure_rules::taxonomy_structure_response::StructureResponse;
use std::sync::Arc;

/// Orchestrates the folder-layout audit over a capability seam.
pub struct StructureOrchestrator {
    /// The capability seam doing the invariant work.
    auditor: Arc<dyn IStructureAuditProtocol>,
}

impl IStructureAggregate for StructureOrchestrator {
    fn execute(&self, request: StructureRequest) -> StructureResponse {
        self.auditor.audit(request)
    }
}

impl StructureOrchestrator {
    /// Wrap a capability seam in a fresh orchestrator.
    pub fn new(auditor: Arc<dyn IStructureAuditProtocol>) -> Self {
        Self { auditor }
    }
}
