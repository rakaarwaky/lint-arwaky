// PURPOSE: IStructureAggregate — single entry point over the structure-rules feature
///
/// The single door consumers knock on. The agent behind the aggregate
/// routes `StructureRequest::AuditAll` to the structural-auditor capability
/// and folds its findings into the response. Consumers never see the protocol.
use crate::structure_rules::taxonomy_structure_rules_request::StructureRequest;
use crate::structure_rules::taxonomy_structure_rules_response::StructureResponse;

/// Single entry point over the structure-rules feature.
pub trait IStructureAggregate: Send + Sync {
    /// Dispatch the request to the structural auditor and return the response.
    fn execute(&self, request: StructureRequest) -> StructureResponse;
}
