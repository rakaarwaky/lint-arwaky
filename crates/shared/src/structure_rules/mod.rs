// PURPOSE: structure_rules — folder-layout auditors for AES701–AES704

pub mod contract_structure_aggregate;
pub mod contract_structure_protocol;
pub mod taxonomy_structure_rules_constant;
pub mod taxonomy_structure_rules_request;
pub mod taxonomy_structure_rules_response;
pub mod taxonomy_structure_rules_vo;
pub mod utility_structure_parsers;

// #570: shared-structure-rules hosts the dispatcher surface orchestrators
// (surface_*_action). It must never depend on the structure-rules linter
// crate — that would be a cycle.
#[test]
fn shared_structure_rules_has_no_linter_cycle() {
    let toml = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .expect("shared structure_rules Cargo.toml");
    assert!(
        !toml.contains("structure-rules.workspace"),
        "cycle: shared-structure-rules must not depend on the structure-rules linter crate"
    );
}

// ─── #570: Dispatcher surface orchestrators (moved from crates/dispatcher) ───
// Surface crates must not depend on the dispatcher crate (ARCHITECTURE.md §10
// Surface Groups). The orchestrator logic lives here; dispatcher re-exports
// these for backward compatibility.
pub mod surface_check_action;
pub mod surface_ci_action;
pub mod surface_config_action;
pub mod surface_docs_action;
pub mod surface_external_action;
pub mod surface_fix_action;
pub mod surface_git_action;
pub mod surface_import_action;
pub mod surface_layer_scan_action;
pub mod surface_maintenance_action;
pub mod surface_naming_action;
pub mod surface_orphan_action;
pub mod surface_plugin_action;
pub mod surface_quality_action;
pub mod surface_role_action;
pub mod surface_setup_action;
pub mod surface_structure_action;
pub mod surface_test_entries;
pub mod surface_version_action;
pub mod surface_watch_action;

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
