// filesystem — taxonomy, contract, and aggregate types for file I/O and dependency graph

pub mod taxonomy_filesystem_vo;
pub mod contract_filesystem_protocol;
pub mod contract_filesystem_aggregate;

// ─── Re-exports ────────────────────────────────────────────

// ── Taxonomy types ──
pub use taxonomy_filesystem_vo::FileEntry;
pub use taxonomy_filesystem_vo::ImportEntry;
pub use taxonomy_filesystem_vo::ImportType;
pub use taxonomy_filesystem_vo::Language;
pub use taxonomy_filesystem_vo::ScanTiming;
pub use taxonomy_filesystem_vo::FilesystemResult;

// ── Contract traits ──
pub use contract_filesystem_protocol::IFileWalkerProtocol;
pub use contract_filesystem_protocol::IFileCacheProtocol;
pub use contract_filesystem_protocol::IASTParserProtocol;
pub use contract_filesystem_protocol::IImportExtractorProtocol;
pub use contract_filesystem_protocol::IDependencyGraphProtocol;
pub use contract_filesystem_protocol::IFilesystemServiceProtocol;

// ── Aggregate ──
pub use contract_filesystem_aggregate::IFilesystemAggregate;
