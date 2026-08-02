pub mod contract_orphan_aggregate;
pub mod contract_orphan_graph_resolver_protocol;
pub mod contract_orphan_parser_protocol;
pub mod contract_orphan_protocol;
pub mod taxonomy_orphan_contract_vo;
pub mod taxonomy_orphan_parse_result_vo;
pub mod taxonomy_violation_orphan_vo;

// ─── Re-exports ────────────────────────────────────────────
pub use contract_orphan_aggregate::IOrphanAggregate;
pub use contract_orphan_graph_resolver_protocol::IOrphanGraphResolverProtocol;
pub use contract_orphan_parser_protocol::IOrphanParserProtocol;
pub use contract_orphan_protocol::IAgentOrphanProtocol;
pub use contract_orphan_protocol::ICapabilitiesOrphanProtocol;
pub use contract_orphan_protocol::IContractOrphanProtocol;
pub use contract_orphan_protocol::ISurfacesOrphanProtocol;
pub use contract_orphan_protocol::ITaxonomyOrphanProtocol;
pub use contract_orphan_protocol::IUtilityOrphanProtocol;

pub use taxonomy_orphan_contract_vo::OrphanEntryPatternListVO;
pub use taxonomy_orphan_contract_vo::OrphanFileListVO;
pub use taxonomy_orphan_parse_result_vo::{
    AstImportVO, AstModDeclVO, AstStructDefVO, AstTraitDefVO, AstTraitImplVO, FileParseResultVO,
    PythonParseResultVO, RustParseResultVO, TsParseResultVO,
};
pub use taxonomy_violation_orphan_vo::AesOrphanViolation;
