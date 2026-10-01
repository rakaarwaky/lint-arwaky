// PURPOSE: structure-rules capability contracts — three seams, one per rule group
///
/// IStructureSharedPurityProtocol  → AES701 (shared/kernel folder rules)
/// IStructureFeatureHealthProtocol → AES702 (feature folder health + docs)
/// IStructureSurfacePurityProtocol → AES703 (surface folder purity + docs)
use crate::taxonomy_structure_rules_request::StructureRequest;
use crate::taxonomy_structure_rules_response::StructureResponse;

/// Shared (kernel) folder purity: no forbidden layers, no doc pair.
pub trait IStructureSharedPurityProtocol: Send + Sync {
    fn audit_shared(&self, request: StructureRequest) -> StructureResponse;
}

/// Feature folder health: capabilities + orchestrator required; no foreign files;
/// FRD.md + BACKLOG.md required; reverse check: doc pair implies orchestrator.
pub trait IStructureFeatureHealthProtocol: Send + Sync {
    fn audit_feature(&self, request: StructureRequest) -> StructureResponse;
}

/// Surface folder purity: no misplaced capabilities or agents; DESIGN.md required.
pub trait IStructureSurfacePurityProtocol: Send + Sync {
    fn audit_surface(&self, request: StructureRequest) -> StructureResponse;
}
