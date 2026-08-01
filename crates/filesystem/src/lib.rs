// filesystem crate — centralized file I/O, AST parsing, and dependency graph
// Implements FR-001 through FR-005 from the filesystem FRD.

// ── Capabilities (protocol implementations) ──
pub mod capabilities_ast_parser;
pub mod capabilities_dependency_graph;
pub mod capabilities_file_walker;
pub mod capabilities_import_extractor;

// ── Utility ──
pub mod utility_import_extractor;

// ── Agent (orchestration) ──
pub mod agent_filesystem_orchestrator;

// ── Re-exports ──
pub use agent_filesystem_orchestrator::FilesystemOrchestrator;
pub use capabilities_ast_parser::ASTParser;
pub use capabilities_dependency_graph::DependencyGraph;
pub use capabilities_file_walker::FileWalker;
pub use capabilities_import_extractor::ImportExtractor;
