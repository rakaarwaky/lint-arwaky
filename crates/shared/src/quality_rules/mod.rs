// quality-rules — taxonomy and contract types
pub mod contract_code_analysis_aggregate;
pub mod contract_quality_protocol;
pub mod taxonomy_quality_rules_vo;
pub use shared_common::taxonomy_code_analysis_vo;
pub use shared_common::taxonomy_operation_error;
pub mod taxonomy_quality_rules_request;
pub mod taxonomy_quality_rules_response;
pub mod utility_bypass_detector;
pub mod utility_code_duplication_detector;
pub mod utility_compliance_checker;
pub mod utility_compliance_score;
pub mod utility_language_mapper;
pub mod utility_mandatory_checker;
pub mod utility_violation_formatter;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_code_analysis_aggregate::ICodeAnalysisAggregate;
pub use contract_quality_protocol::IBypassCheckerProtocol;
pub use contract_quality_protocol::ICodeMetricAnalyzerProtocol;
pub use contract_quality_protocol::IDeadInheritanceProtocol;
pub use contract_quality_protocol::ILineCheckerProtocol;
pub use contract_quality_protocol::IMandatoryClassProtocol;
pub use taxonomy_quality_rules_request::CodeAnalysisRequest;
pub use taxonomy_quality_rules_response::CodeAnalysisResponse;

// ── Taxonomy types ──
pub use taxonomy_code_analysis_vo::CodeAnalysisRuleVO;
pub use taxonomy_code_analysis_vo::MandatoryImportRuleVO;
pub use taxonomy_operation_error::LinterOperationError;
pub use taxonomy_quality_rules_vo::AesCodeAnalysisViolation;
pub use taxonomy_quality_rules_vo::GraphAnalysisContext;
pub use taxonomy_quality_rules_vo::ImportGraph;
pub use taxonomy_quality_rules_vo::InboundLinkMap;
pub use taxonomy_quality_rules_vo::InheritanceMap;
pub use taxonomy_quality_rules_vo::Language;
pub use taxonomy_quality_rules_vo::OrphanIndicatorResult;
pub use taxonomy_quality_rules_vo::ReachabilityResult;
pub use taxonomy_quality_rules_vo::ViolationKind;
pub use taxonomy_quality_rules_vo::WORD_PATTERN_TOKENS;
