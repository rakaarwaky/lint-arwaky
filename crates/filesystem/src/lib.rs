// filesystem crate — produces data for all feature crates.
//
// Each FR = capabilities (struct) + utility (functions).
// Capabilities CAN import utility. Utility CANNOT import capabilities.
//
// FR-001: AST Parsing & Import Extraction  → capabilities_ast_parser + utility_ast_python + utility_ast_rust + utility_ast_typescript + utility_import_extractor + utility_tree_sitter_helpers
// FR-002: Dependency Graph Construction     → capabilities_dependency_graph
// FR-003: File I/O & Directory Operations   → capabilities_filesystem_io + utility_filesystem_io
// FR-004: Tool Resolution                   → capabilities_tool_resolution + utility_tool_resolution
// FR-005: Workspace Detection               → capabilities_workspace + utility_workspace_detection

// ── Capabilities (stateful, produce structured data) ──
pub mod capabilities_ast_parser; // FR-001
pub mod capabilities_dependency_graph; // FR-002
pub mod capabilities_filesystem_io; // FR-003
pub mod capabilities_tool_resolution; // FR-004
pub mod capabilities_workspace; // FR-005

// ── Utility (stateless, technical mechanics) ──
pub mod utility_ast_python; // FR-001
pub mod utility_ast_rust; // FR-001
pub mod utility_ast_typescript; // FR-001
pub mod utility_filesystem_io; // FR-003
pub mod utility_import_extractor; // FR-001
pub mod utility_tool_resolution; // FR-004
pub mod utility_tree_sitter_helpers; // FR-001
pub mod utility_workspace_detection; // FR-005

// ── Agent (orchestration) ──
pub mod agent_filesystem_orchestrator;

// ── Root (composition, wiring) ──
pub mod root_filesystem_container;
