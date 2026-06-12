// PURPOSE: Module declarations and re-exports for code-analysis (checkers, reporters, protocols, fixers, renamers, analyzers)

// Re-export shared code-analysis taxonomy and contract types
pub use shared::code_analysis::taxonomy_governance_entity::ArchitectureGovernanceEntity;
pub use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
pub use shared::code_analysis::contract_unused_protocol::IUnusedProtocol;
pub use shared::code_analysis::contract_lint_protocol::IArchLintProtocol;
pub use shared::code_analysis::contract_cycle_protocol;
pub use shared::code_analysis::contract_cycle_protocol::ICycleAnalysisProtocol;
pub use shared::code_analysis::contract_layer_detection_aggregate::ILayerDetectionAggregate;
pub use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
pub use shared::code_analysis::contract_analysis_protocol::IAnalysisProtocol;
pub use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
pub use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
pub use shared::code_analysis::contract_bypass_checker_protocol::IBypassCheckerProtocol;
pub use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
pub use shared::code_analysis::contract_inline_unused_protocol::IInlineUnusedProtocol;
pub use shared::code_analysis::contract_mandatory_inheritance_protocol::IMandatoryInheritanceProtocol;
pub use shared::code_analysis::taxonomy_analysis_vo::{
    FileDefinitionMap, FilePathSet, GraphAnalysisContext, ImportGraph, InboundLinkMap,
    InheritanceMap, ModuleToFileMap, OrphanIndicatorResult, ReachabilityResult,
};
pub use shared::code_analysis::contract_code_metric_analyzer_protocol::ICodeMetricAnalyzerProtocol;
pub use shared::code_analysis::contract_target_resolver_protocol::ITargetResolverProtocol;
pub use shared::code_analysis::taxonomy_import_source_vo;
pub use shared::code_analysis::contract_fix_runner_aggregate::IFixRunnerAggregate;

// Re-export shared output_report types
pub use shared::output_report::taxonomy_result_vo::LintResult;
pub use shared::output_report::taxonomy_result_vo::LintResultList;
pub use shared::output_report::taxonomy_severity_vo::Severity;
pub use shared::output_report::taxonomy_score_vo::compute_score as shared_compute_score;

// Re-export shared config_system types
pub use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
pub use shared::config_system::taxonomy_config_vo::default_aes_config;

// Re-export shared import_rules types
pub use shared::import_rules::contract_rule_protocol::IAnalyzer;

// Re-export shared role_rules contract types
pub use shared::role_rules::contract_role_aggregate::IRoleAggregate;
pub use shared::role_rules::contract_role_protocol::IContractRoleChecker;
pub use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;

// Local capability modules
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
pub use capabilities_project_target_resolver::ProjectTargetResolver;
pub mod capabilities_code_metric_analyzer;
pub use capabilities_code_metric_analyzer::CodeMetricAnalyzer;

// Local agent modules
pub mod agent_checking_orchestrator;
pub use agent_checking_orchestrator::LintCheckingOrchestrator;
pub mod agent_codebase_scan_orchestrator;
pub use agent_codebase_scan_orchestrator::{
    detect_source_dir, CodebaseScanOrchestrator, CodebaseScanPipelineOrchestrator,
};

// Local container
pub mod analysis_container;
pub use analysis_container::AnalysisContainer;
pub mod container;
pub use container::{CheckerContainer, RoleOrchestrator};

use once_cell::sync::Lazy;

static RESOLVER: Lazy<ProjectTargetResolver> = Lazy::new(ProjectTargetResolver::new);

pub fn resolve_target(path: Option<String>) -> String {
    RESOLVER.resolve_target(path)
}

pub fn walk_rs_files(dir: &std::path::Path, cb: &mut dyn FnMut(std::path::PathBuf)) {
    RESOLVER.walk_rs_files(dir, cb);
}

pub fn lint_path(path: &str) -> Vec<LintResult> {
    RESOLVER.lint_path(path)
}

pub fn compute_score(results: &[LintResult]) -> f64 {
    RESOLVER.compute_score(results)
}

pub fn count_loc(path: &str) -> usize {
    RESOLVER.count_loc(path)
}

pub fn has_critical(results: &[LintResult]) -> bool {
    RESOLVER.has_critical(results)
}

pub fn normalize_project_root(path: &str) -> String {
    RESOLVER.normalize_project_root(path)
}