/// PURPOSE: Module declarations for orphan-detector (orchestrator, analyzers, container)
pub mod agent_orphan_orchestrator;
pub mod capabilities_orphan_agent_analyzer;
pub mod capabilities_orphan_capabilities_analyzer;
pub mod capabilities_orphan_contract_analyzer;
pub mod capabilities_orphan_graph_resolver;
pub mod capabilities_orphan_surfaces_analyzer;
pub mod capabilities_orphan_taxonomy_analyzer;
pub mod capabilities_orphan_utility_analyzer;
pub mod root_orphan_detector_container;

// Taxonomy — shared parse result VOs
pub mod taxonomy_orphan_parse_result_vo;

// Utility — language-specific parsers (1 utility = 1 bahasa)
pub mod utility_orphan_parser_dispatch;
pub mod utility_orphan_python_parser;
pub mod utility_orphan_rust_parser;
pub mod utility_orphan_ts_parser;

// Utility — graph helpers (delegated to shared crate)
pub use shared::orphan_detector::utility_orphan_graph_resolver;

// Deprecated (delegated to shared crate)
// DEPRECATED: utility_orphan_regex_patterns removed in v1.12.0 (replaced by AST parsing).
