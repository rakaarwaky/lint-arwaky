// filesystem — taxonomy, contract, and aggregate types
// Organized by FR per FRD v3.0.0

pub mod contract_filesystem_aggregate;
pub mod contract_filesystem_protocol;
pub mod taxonomy_filesystem_request;
pub mod taxonomy_filesystem_response;
pub mod taxonomy_filesystem_vo;
pub mod utility_ast_python;
pub mod utility_ast_rust;
pub mod utility_ast_typescript;
pub mod utility_barrel_resolution;
pub mod utility_command_execution;
pub mod utility_container_wiring;
pub mod utility_filesystem_io;
pub mod utility_import_extractor;
pub mod utility_import_resolution;
pub mod utility_tool_resolution;
pub mod utility_workspace_detection;

pub use contract_filesystem_protocol::IFileSystemIOProtocol;
pub use contract_filesystem_protocol::IGraphProtocol;
pub use contract_filesystem_protocol::IParserProtocol;
pub use contract_filesystem_protocol::IToolResolutionProtocol;
pub use contract_filesystem_protocol::IWorkspaceProtocol;
pub use taxonomy_filesystem_request::FilesystemRequest;
pub use taxonomy_filesystem_response::FilesystemResponse;

// ─── Re-exports ────────────────────────────────────────────

// ── Taxonomy types ──
pub use taxonomy_filesystem_vo::DefinitionEntry;
pub use taxonomy_filesystem_vo::ExternalReferenceMap;
pub use taxonomy_filesystem_vo::FileEntry;
pub use taxonomy_filesystem_vo::FileNodeVO;
pub use taxonomy_filesystem_vo::GraphAnalysisContext;
pub use taxonomy_filesystem_vo::GraphData;
pub use taxonomy_filesystem_vo::ImplEntry;
pub use taxonomy_filesystem_vo::ImportEdgeVO;
pub use taxonomy_filesystem_vo::ImportEntry;
pub use taxonomy_filesystem_vo::ImportGraph;
pub use taxonomy_filesystem_vo::ImportType;
pub use taxonomy_filesystem_vo::InboundLinkMap;
pub use taxonomy_filesystem_vo::InheritanceMap;
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

// ── Aggregate ──
pub use contract_filesystem_aggregate::IFilesystemAggregate;
