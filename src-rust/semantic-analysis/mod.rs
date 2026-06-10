// PURPOSE: Module declarations and re-exports for semantic-analysis (analyzers, detectors, resolvers, protocols)
pub mod capabilities_scope_bounds_detector;
pub use capabilities_scope_bounds_detector::ScopeBoundaryAnalyzer;
pub mod capabilities_scope_enclosing_resolver;
pub use capabilities_scope_enclosing_resolver::ScopeBoundaryResolver;
pub mod capabilities_data_flow_analyzer;
pub use capabilities_data_flow_analyzer::{DataFlowAnalyzer, DataFlowEntry};
pub mod capabilities_semantic_scope_analyzer;
pub use capabilities_semantic_scope_analyzer::SemanticScopeAnalyzer;
pub mod capabilities_call_chain_analyzer;
pub use capabilities_call_chain_analyzer::CallChainAnalyzer;
pub mod contract_data_flow_protocol;
pub use contract_data_flow_protocol::IDataFlowProtocol;
pub mod contract_semantic_tracer_port;
pub use contract_semantic_tracer_port::ISemanticTracerPort;
pub mod contract_semantic_tracer_protocol;
pub use contract_semantic_tracer_protocol::ISemanticTracerProtocol;
pub mod taxonomy_semantic_error;
pub use taxonomy_semantic_error::{CallChainError, ScopeResolutionError, SemanticError};
