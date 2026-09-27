// orphan-rules — contract and taxonomy types
pub mod contract_orphan_aggregate;
pub mod contract_orphan_protocol;
pub mod taxonomy_orphan_request;
pub mod taxonomy_orphan_response;
pub mod taxonomy_orphan_vo;

// ─── Re-exports ────────────────────────────────────────────
pub use contract_orphan_aggregate::IOrphanAggregate;
pub use contract_orphan_protocol::IAgentOrphanProtocol;
pub use contract_orphan_protocol::ICapabilitiesOrphanProtocol;
pub use contract_orphan_protocol::IContractOrphanProtocol;
pub use contract_orphan_protocol::IOrphanParserProtocol;
pub use contract_orphan_protocol::ISurfacesOrphanProtocol;
pub use contract_orphan_protocol::ITaxonomyOrphanProtocol;
pub use contract_orphan_protocol::IUtilityOrphanProtocol;
pub use taxonomy_orphan_request::OrphanRequest;
pub use taxonomy_orphan_response::OrphanResponse;
pub use taxonomy_orphan_vo::AesOrphanViolation;
pub use taxonomy_orphan_vo::AstImportVO;
pub use taxonomy_orphan_vo::AstModDeclVO;
pub use taxonomy_orphan_vo::AstStructDefVO;
pub use taxonomy_orphan_vo::AstTraitDefVO;
pub use taxonomy_orphan_vo::AstTraitImplVO;
pub use taxonomy_orphan_vo::FileParseResultVO;
pub use taxonomy_orphan_vo::FilePathSet;
pub use taxonomy_orphan_vo::IdentifierVisitor;
pub use taxonomy_orphan_vo::OrphanEntryPatternListVO;
pub use taxonomy_orphan_vo::OrphanFileListVO;
pub use taxonomy_orphan_vo::PythonParseResultVO;
pub use taxonomy_orphan_vo::RustParseResultVO;
pub use taxonomy_orphan_vo::TsParseResultVO;
pub use taxonomy_orphan_vo::extract_idents_from_stream;
