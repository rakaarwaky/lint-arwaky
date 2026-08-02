// import-rules — taxonomy and contract types
pub mod contract_cycle_import_protocol;
pub mod contract_dummy_import_protocol;
pub mod contract_import_forbidden_protocol;
pub mod contract_import_mandatory_protocol;
pub mod contract_import_runner_aggregate;
pub mod contract_unused_import_protocol;
pub mod taxonomy_dependency_edge_vo;
pub mod taxonomy_graph_color_vo;
pub mod taxonomy_import_constant;
pub mod taxonomy_import_error;
pub mod taxonomy_resolved_import_vo;
pub mod taxonomy_violation_import_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_cycle_import_protocol::ICycleImportProtocol;
pub use contract_dummy_import_protocol::IDummyImportCheckerProtocol;
pub use contract_import_forbidden_protocol::IImportForbiddenProtocol;
pub use contract_import_mandatory_protocol::IImportMandatoryProtocol;
pub use contract_import_runner_aggregate::IImportRunnerAggregate;
pub use contract_unused_import_protocol::IUnusedImportProtocol;

// ── Taxonomy types ──
pub use taxonomy_dependency_edge_vo::DependencyEdge;
pub use taxonomy_import_constant::DEFAULT_SKIP_DIRS;
pub use taxonomy_import_error::ImportError;
pub use taxonomy_violation_import_vo::AesImportViolation;
