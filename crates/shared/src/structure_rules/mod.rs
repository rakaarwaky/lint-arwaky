// PURPOSE: structure_rules — folder-layout auditors for AES701–AES703

pub mod contract_structure_aggregate;
pub mod contract_structure_protocol;
pub mod taxonomy_structure_constant;
pub mod taxonomy_structure_request;
pub mod taxonomy_structure_response;
pub mod taxonomy_structure_vo;

// ─── Re-exports ────────────────────────────────────────────
pub use contract_structure_aggregate::IStructureAggregate;
pub use contract_structure_protocol::IStructureAuditProtocol;
pub use taxonomy_structure_constant::*;
pub use taxonomy_structure_request::{StructureFinding, StructureRequest};
pub use taxonomy_structure_response::StructureResponse;
pub use taxonomy_structure_vo::{FolderInventory, LayerFile};
