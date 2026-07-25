pub mod contract_orphan_aggregate;
pub mod contract_orphan_graph_resolver_protocol;
pub mod contract_orphan_protocol;
pub mod taxonomy_orphan_contract_vo;
pub mod taxonomy_violation_orphan_vo;
pub mod utility_file_cache;
pub mod utility_orphan_detector;
pub mod utility_orphan_filename;
pub mod utility_orphan_io;
pub mod utility_orphan_path;
pub mod utility_workspace_scanner;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_orphan_aggregate::IOrphanAggregate;
pub use contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
pub use contract_orphan_protocol::ITaxonomyOrphanProtocol;
pub use contract_orphan_protocol::IContractOrphanProtocol;
pub use contract_orphan_protocol::ICapabilitiesOrphanProtocol;
pub use contract_orphan_protocol::IUtilityOrphanProtocol;
pub use contract_orphan_protocol::IAgentOrphanProtocol;
pub use contract_orphan_protocol::ISurfacesOrphanProtocol;

// ── Taxonomy types ──
pub use taxonomy_orphan_contract_vo::OrphanFileListVO;
pub use taxonomy_orphan_contract_vo::OrphanEntryPatternListVO;
pub use taxonomy_violation_orphan_vo::AesOrphanViolation;
