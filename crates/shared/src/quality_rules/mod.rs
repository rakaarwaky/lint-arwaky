// quality-rules — taxonomy and contract types
pub mod contract_code_analysis_aggregate;
pub mod contract_quality_protocol;
pub mod taxonomy_analysis_vo;
pub use crate::common::taxonomy_code_analysis_vo;
pub use crate::common::taxonomy_operation_error;
pub mod taxonomy_code_analysis_request_vo;
pub mod taxonomy_violation_code_analysis_vo;

// ─── Re-exports ────────────────────────────────────────────
// Barrel re-export pattern: allows consumers to import directly

// ── Contract traits ──
pub use contract_code_analysis_aggregate::ICodeAnalysisAggregate;
pub use contract_quality_protocol::IBypassCheckerProtocol;
pub use contract_quality_protocol::ICodeMetricAnalyzerProtocol;
pub use contract_quality_protocol::IDeadInheritanceProtocol;
pub use contract_quality_protocol::ILineCheckerProtocol;
pub use contract_quality_protocol::IMandatoryClassProtocol;
pub use taxonomy_code_analysis_request_vo::{CodeAnalysisRequest, CodeAnalysisResponse};

// ── Taxonomy types ──
pub use taxonomy_analysis_vo::GraphAnalysisContext;
pub use taxonomy_analysis_vo::ImportGraph;
pub use taxonomy_analysis_vo::InboundLinkMap;
pub use taxonomy_analysis_vo::InheritanceMap;
pub use taxonomy_analysis_vo::OrphanIndicatorResult;
pub use taxonomy_analysis_vo::ReachabilityResult;
pub use taxonomy_code_analysis_vo::CodeAnalysisRuleVO;
pub use taxonomy_code_analysis_vo::MandatoryImportRuleVO;
pub use taxonomy_operation_error::LinterOperationError;
pub use taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
pub use taxonomy_violation_code_analysis_vo::Language;
pub use taxonomy_violation_code_analysis_vo::ViolationKind;
pub use taxonomy_violation_code_analysis_vo::WORD_PATTERN_TOKENS;
