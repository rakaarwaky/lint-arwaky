// PURPOSE: structure_rules — folder-layout auditors for AES701–AES704

pub mod contract_structure_aggregate;
pub mod contract_structure_protocol;
pub mod taxonomy_structure_rules_constant;
pub mod taxonomy_structure_rules_request;
pub mod taxonomy_structure_rules_response;
pub mod taxonomy_structure_rules_vo;
pub mod utility_structure_parsers;

// ─── Re-exports ────────────────────────────────────────────
pub use contract_structure_aggregate::IStructureAggregate;
pub use contract_structure_protocol::{
    IStructureFeatureHealthProtocol, IStructureSharedPurityProtocol,
    IStructureSurfacePurityProtocol, IStructureTestSuiteProtocol,
};
pub use taxonomy_structure_rules_constant::*;
pub use taxonomy_structure_rules_request::{StructureFinding, StructureRequest};
pub use taxonomy_structure_rules_response::StructureResponse;
pub use taxonomy_structure_rules_vo::{FolderInventory, LayerFile};
