// quality-rules — taxonomy and contract types
pub mod contract_bypass_checker_protocol;
pub mod contract_class_protocol;
pub mod contract_code_analysis_aggregate;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_dead_inheritance_protocol;
pub mod contract_line_protocol;
pub mod taxonomy_analysis_vo;
pub mod taxonomy_code_analysis_rule_vo;
pub mod taxonomy_operation_error;
pub mod taxonomy_violation_code_analysis_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_bypass_checker_protocol::IBypassCheckerProtocol;
pub use contract_class_protocol::IMandatoryClassProtocol;
pub use contract_code_analysis_aggregate::ICodeAnalysisAggregate;
pub use contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
pub use contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
pub use contract_line_protocol::ILineCheckerProtocol;

// ── Taxonomy types ──
pub use taxonomy_analysis_vo::GraphAnalysisContext;
pub use taxonomy_analysis_vo::ImportGraph;
pub use taxonomy_analysis_vo::InboundLinkMap;
pub use taxonomy_analysis_vo::InheritanceMap;
pub use taxonomy_analysis_vo::OrphanIndicatorResult;
pub use taxonomy_analysis_vo::ReachabilityResult;
pub use taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO;
pub use taxonomy_code_analysis_rule_vo::MandatoryImportRuleVO;
pub use taxonomy_operation_error::LinterOperationError;
pub use taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
pub use taxonomy_violation_code_analysis_vo::Language;
pub use taxonomy_violation_code_analysis_vo::ViolationKind;
pub use taxonomy_violation_code_analysis_vo::WORD_PATTERN_TOKENS;
pub use taxonomy_violation_code_analysis_vo::format_code_analysis_violation;
