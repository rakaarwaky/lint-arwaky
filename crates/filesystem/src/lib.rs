// filesystem crate — produces data for all feature crates.
//
// Each FR = capabilities (struct) + utility (functions).
// Capabilities CAN import utility. Utility CANNOT import capabilities.
//
// FR-001: File Discovery      → capabilities_file_walker (struct + functions)
// FR-002: AST Parsing         → capabilities_ast_parser (struct + functions)
// FR-003: Import Extraction    → utility_import_extraction (functions only)
// FR-004: Graph Construction   → capabilities_dependency_graph (struct + functions)
// FR-005: Workspace Detection  → utility_workspace_detection (functions only)
// FR-006: Tool Resolution      → utility_tool_resolution (functions only)
// FR-007: File Cache           → utility_file_cache (static cache + functions)

// ── Capabilities (stateful, produce structured data) ──
pub mod capabilities_ast_parser; // FR-002
pub mod capabilities_dependency_graph;
pub mod capabilities_file_walker; // FR-001 // FR-004

// ── Utility (stateless, technical mechanics) ──
pub mod utility_file_cache; // FR-007
pub mod utility_filesystem_io; // file I/O primitives
pub mod utility_import_extractor; // FR-003
pub mod utility_import_resolver;
pub mod utility_tool_resolution; // FR-006
pub mod utility_workspace_detection; // FR-005 // import path resolution helper

// ── Agent (orchestration) ──
pub mod agent_filesystem_orchestrator;
