// PURPOSE: Module declarations for code-analysis (checkers, container, orchestrator)

pub mod capabilities_mandatory_definition_checker;
pub use capabilities_mandatory_definition_checker::MandatoryDefinitionChecker;
pub mod capabilities_line_checker;
pub use capabilities_line_checker::ArchLineChecker;
pub mod capabilities_check_bypass_checker;
pub use capabilities_check_bypass_checker::BypassChecker;
pub mod capabilities_code_duplication_analyzer;
pub use capabilities_code_duplication_analyzer::CodeDuplicationAnalyzer;
pub mod agent_code_analysis_orchestrator;
pub use agent_code_analysis_orchestrator::{
    has_critical, lint_path, CodeAnalysisOrchestrator,
};
// Re-export for CLI surfaces backward compatibility
pub use shared::cli_commands::taxonomy_score_vo::compute_score;
pub mod root_code_analysis_container;
pub use root_code_analysis_container::{CodeAnalysisCheckerContainer, CodeAnalysisContainer};
