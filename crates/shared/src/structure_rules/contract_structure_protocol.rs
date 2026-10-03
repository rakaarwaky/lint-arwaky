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

/// Test-suite coverage: every feature folder that owns source holds at least one
/// file for each of the seven test types in `tests/`, and at least one `bench_`
/// file in `benches/` (AES704).
/// AES705 — reports layer files sitting at a member dir root instead of
/// inside the folder that owns them.
pub trait IStructureMemberRootProtocol: Send + Sync {
    /// Report every misplaced layer file at every member dir root.
    fn audit_member_root(&self, request: StructureRequest) -> StructureResponse;
}

/// AES704 — reports missing test categories across every feature folder.
pub trait IStructureTestSuiteProtocol: Send + Sync {
    /// Report every missing test category across every feature folder.
    fn audit_test_suite(&self, request: StructureRequest) -> StructureResponse;
}
