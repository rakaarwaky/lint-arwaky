// filesystem — taxonomy, contract, and aggregate types for file I/O and dependency graph

pub mod contract_filesystem_aggregate;
pub mod contract_filesystem_protocol;
pub mod taxonomy_filesystem_vo;

// ─── Re-exports ────────────────────────────────────────────

// ── Taxonomy types ──
pub use taxonomy_filesystem_vo::CacheStatsVO;
pub use taxonomy_filesystem_vo::FileEntry;
pub use taxonomy_filesystem_vo::FileNodeVO;
pub use taxonomy_filesystem_vo::FilesystemResult;
pub use taxonomy_filesystem_vo::GraphStatsVO;
pub use taxonomy_filesystem_vo::ImportEdgeVO;
pub use taxonomy_filesystem_vo::ImportEntry;
pub use taxonomy_filesystem_vo::ImportType;
pub use taxonomy_filesystem_vo::Language;
pub use taxonomy_filesystem_vo::MAX_LINT_FILE_BYTES;
pub use taxonomy_filesystem_vo::MemoryBudgetVO;
pub use taxonomy_filesystem_vo::ScanConfigVO;
pub use taxonomy_filesystem_vo::ScanTiming;

// ── Contract traits ──
pub use contract_filesystem_protocol::IASTParserProtocol;
pub use contract_filesystem_protocol::IDependencyGraphProtocol;
pub use contract_filesystem_protocol::IFileWalkerProtocol;
pub use contract_filesystem_protocol::IFilesystemServiceProtocol;
pub use contract_filesystem_protocol::IImportExtractorProtocol;

// ── Aggregate ──
pub use contract_filesystem_aggregate::IFilesystemAggregate;
