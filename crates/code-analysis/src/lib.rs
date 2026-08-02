// PURPOSE: Module declarations for code-analysis (checkers, container, orchestrator)

pub mod capabilities_mandatory_definition_checker;
pub mod utility_code_duplication_detector;
pub use capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
pub mod capabilities_line_checker;
pub use capabilities_line_checker::ArchLineChecker;
pub mod capabilities_check_bypass_checker;
pub use capabilities_check_bypass_checker::BypassChecker;
pub mod capabilities_code_duplication_analyzer;
pub use capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
pub mod agent_code_analysis_orchestrator;
pub use agent_code_analysis_orchestrator::{CodeAnalysisOrchestrator, has_critical};
// Re-export for CLI surfaces backward compatibility
pub use shared::common::utility_compliance_score::compute_score;
pub mod root_code_analysis_container;
pub mod utility_bypass_detector;
pub mod utility_column_index;
pub mod utility_language_mapper;
pub mod utility_mandatory_checker;
pub use root_code_analysis_container::CodeAnalysisContainer;
