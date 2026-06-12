// PURPOSE: Module declarations for code-analysis (checkers, reporters, analyzers, container)

pub mod capabilities_analysis_reporter;
pub use capabilities_analysis_reporter::AnalysisReporter;
pub mod capabilities_renamer_processor;
pub use capabilities_renamer_processor::SymbolRenamerProcessor;
pub mod capabilities_unused_checker;
pub use capabilities_unused_checker::UnusedImportRuleChecker;
pub mod capabilities_arch_self_lint_checker;
pub use capabilities_arch_self_lint_checker::ArchSelfLintChecker;
pub mod capabilities_cycle_analyzer;
pub use capabilities_cycle_analyzer::{
    detect_cycle_edges, DependencyCycleAnalyzer, DependencyEdge,
};
pub mod capabilities_class_checker;
pub use capabilities_class_checker::ArchClassChecker;
pub mod capabilities_line_checker;
pub use capabilities_line_checker::ArchLineChecker;
pub mod capabilities_check_bypass_checker;
pub use capabilities_check_bypass_checker::BypassChecker;
pub mod capabilities_dead_inheritance_checker;
pub use capabilities_dead_inheritance_checker::DeadInheritanceChecker;
pub mod capabilities_inline_unused_checker;
pub use capabilities_inline_unused_checker::InlineUnusedChecker;
pub mod capabilities_mandatory_inheritance_checker;
pub use capabilities_mandatory_inheritance_checker::MandatoryInheritanceChecker;
pub mod capabilities_project_target_resolver;
pub use capabilities_project_target_resolver::{
    compute_score, count_loc, has_critical, lint_path, normalize_project_root, resolve_target,
    walk_rs_files, ProjectTargetResolver,
};
pub mod capabilities_code_metric_analyzer;
pub use capabilities_code_metric_analyzer::CodeMetricAnalyzer;
pub mod agent_checking_orchestrator;
pub use agent_checking_orchestrator::LintCheckingOrchestrator;
pub mod agent_codebase_scan_orchestrator;
pub use agent_codebase_scan_orchestrator::{
    detect_source_dir, CodebaseScanOrchestrator, CodebaseScanPipelineOrchestrator,
};
pub mod root_code_analysis_container;
pub use root_code_analysis_container::{AnalysisContainer, CheckerContainer, RoleOrchestrator};
