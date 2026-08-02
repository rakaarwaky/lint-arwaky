// filesystem — taxonomy, contract, and aggregate types
// Organized by FR per FRD v3.0.0

pub mod contract_filesystem_aggregate;
pub mod contract_parser_protocol;
pub mod contract_graph_protocol;
pub mod contract_workspace_protocol;
pub mod contract_tool_resolution_protocol;
pub mod contract_filesystem_io_protocol;
pub mod taxonomy_filesystem_vo;

// ─── Re-exports ────────────────────────────────────────────

// ── Taxonomy types ──
pub use taxonomy_filesystem_vo::DefinitionEntry;
pub use taxonomy_filesystem_vo::FileEntry;
pub use taxonomy_filesystem_vo::FileNodeVO;
pub use taxonomy_filesystem_vo::GraphData;
pub use taxonomy_filesystem_vo::ImplEntry;
pub use taxonomy_filesystem_vo::ImportEdgeVO;
pub use taxonomy_filesystem_vo::ImportEntry;
pub use taxonomy_filesystem_vo::ImportType;
pub use taxonomy_filesystem_vo::JavaScriptMetadata;
pub use taxonomy_filesystem_vo::Language;
pub use taxonomy_filesystem_vo::MAX_LINT_FILE_BYTES;
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
pub use taxonomy_filesystem_vo::ScanTiming;
pub use taxonomy_filesystem_vo::TSClassItem;
pub use taxonomy_filesystem_vo::TSFnItem;
pub use taxonomy_filesystem_vo::TypeScriptMetadata;

// ── Focused protocol traits ──
pub use contract_parser_protocol::IParserProtocol;
pub use contract_graph_protocol::IGraphProtocol;
pub use contract_workspace_protocol::IWorkspaceProtocol;
pub use contract_tool_resolution_protocol::IToolResolutionProtocol;
pub use contract_filesystem_io_protocol::IFileSystemIOProtocol;

// ── Aggregate ──
pub use contract_filesystem_aggregate::IFilesystemAggregate;
