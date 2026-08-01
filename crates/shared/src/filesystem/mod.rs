// filesystem — taxonomy, contract, and aggregate types for file I/O and dependency graph

pub mod contract_filesystem_aggregate;
pub mod contract_filesystem_protocol;
pub mod taxonomy_filesystem_vo;
pub mod utility_filesystem_io;

// ─── Re-exports ────────────────────────────────────────────

// ── Taxonomy types ──
pub use taxonomy_filesystem_vo::CacheStatsVO;
pub use taxonomy_filesystem_vo::DefinitionEntry;
pub use taxonomy_filesystem_vo::FileEntry;
pub use taxonomy_filesystem_vo::FileNodeVO;
pub use taxonomy_filesystem_vo::FilesystemResult;
pub use taxonomy_filesystem_vo::GraphData;
pub use taxonomy_filesystem_vo::GraphStatsVO;
pub use taxonomy_filesystem_vo::ImplEntry;
pub use taxonomy_filesystem_vo::ImportEdgeVO;
pub use taxonomy_filesystem_vo::ImportEntry;
pub use taxonomy_filesystem_vo::ImportType;
pub use taxonomy_filesystem_vo::JavaScriptMetadata;
pub use taxonomy_filesystem_vo::Language;
pub use taxonomy_filesystem_vo::MAX_LINT_FILE_BYTES;
pub use taxonomy_filesystem_vo::MemoryBudgetVO;
pub use taxonomy_filesystem_vo::ParseMetadata;
pub use taxonomy_filesystem_vo::ParseWarning;
pub use taxonomy_filesystem_vo::PythonClassItem;
pub use taxonomy_filesystem_vo::PythonFnItem;
pub use taxonomy_filesystem_vo::PythonMetadata;
pub use taxonomy_filesystem_vo::RustFnItem;
pub use taxonomy_filesystem_vo::RustImplItem;
pub use taxonomy_filesystem_vo::RustMetadata;
pub use taxonomy_filesystem_vo::RustModItem;
pub use taxonomy_filesystem_vo::RustUseItem;
pub use taxonomy_filesystem_vo::ScanConfigVO;
pub use taxonomy_filesystem_vo::ScanStage;
pub use taxonomy_filesystem_vo::ScanTiming;
pub use taxonomy_filesystem_vo::TSClassItem;
pub use taxonomy_filesystem_vo::TSFnItem;
pub use taxonomy_filesystem_vo::TypeScriptMetadata;

// ── Contract traits ──
pub use contract_filesystem_protocol::IASTParserProtocol;
pub use contract_filesystem_protocol::IDependencyGraphProtocol;
pub use contract_filesystem_protocol::IFileWalkerProtocol;
pub use contract_filesystem_protocol::IFilesystemServiceProtocol;
pub use contract_filesystem_protocol::IImportExtractorProtocol;

// ── Aggregate ──
pub use contract_filesystem_aggregate::IFilesystemAggregate;
