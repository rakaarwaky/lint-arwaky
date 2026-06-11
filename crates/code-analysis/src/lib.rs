// PURPOSE: Module declarations and re-exports for code-analysis (checkers, reporters, protocols, fixers, renamers, analyzers)
pub mod taxonomy_governance_entity;
pub use taxonomy_governance_entity::ArchitectureGovernanceEntity;
pub mod taxonomy_operation_error;
pub use taxonomy_operation_error::LinterOperationError;
pub mod capabilities_analysis_reporter;
pub use capabilities_analysis_reporter::AnalysisReporter;
pub mod capabilities_renamer_processor;
pub use capabilities_renamer_processor::SymbolRenamerProcessor;
pub mod capabilities_unused_checker;
pub use capabilities_unused_checker::UnusedImportRuleChecker;
pub mod contract_unused_protocol;
pub use contract_unused_protocol::IUnusedProtocol;
pub mod agent_checking_orchestrator;
pub use agent_checking_orchestrator::LintCheckingOrchestrator;

pub mod contract_lint_protocol;
pub use contract_lint_protocol::IArchLintProtocol;
pub mod capabilities_arch_self_lint_checker;
pub use capabilities_arch_self_lint_checker::ArchSelfLintChecker;
pub mod capabilities_cycle_analyzer;
pub mod contract_cycle_protocol;
pub use capabilities_cycle_analyzer::{
    detect_cycle_edges, DependencyCycleAnalyzer, DependencyEdge,
};

pub mod agent_codebase_scan_orchestrator;
pub use agent_codebase_scan_orchestrator::{
    detect_source_dir, CodebaseScanOrchestrator, CodebaseScanPipelineOrchestrator,
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

pub mod contract_layer_detection_aggregate;
pub use contract_layer_detection_aggregate::ILayerDetectionAggregate;
pub mod contract_adapter_port;
pub use contract_adapter_port::ILinterAdapterPort;
pub mod contract_analysis_protocol;
pub use contract_analysis_protocol::IAnalysisProtocol;
pub mod contract_class_protocol;
pub use contract_class_protocol::IMandatoryClassProtocol;
pub mod contract_line_protocol;
pub use contract_line_protocol::ILineCheckerProtocol;
pub mod contract_bypass_checker_protocol;
pub use contract_bypass_checker_protocol::IBypassCheckerProtocol;
pub mod contract_dead_inheritance_protocol;
pub use contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
pub mod contract_inline_unused_protocol;
pub use contract_inline_unused_protocol::IInlineUnusedProtocol;
pub mod contract_mandatory_inheritance_protocol;
pub use contract_mandatory_inheritance_protocol::IMandatoryInheritanceProtocol;
pub mod taxonomy_analysis_vo;
pub use taxonomy_analysis_vo::{
    FileDefinitionMap, FilePathSet, GraphAnalysisContext, ImportGraph, InboundLinkMap,
    InheritanceMap, ModuleToFileMap, OrphanIndicatorResult, ReachabilityResult,
};
pub mod capabilities_project_target_resolver;
pub use capabilities_project_target_resolver::ProjectTargetResolver;
pub mod contract_code_metric_analyzer_protocol;
pub mod contract_target_resolver_protocol;
pub mod taxonomy_import_source_vo;

pub mod capabilities_code_metric_analyzer;
pub use capabilities_code_metric_analyzer::CodeMetricAnalyzer;

pub mod contract_fix_runner_aggregate;
pub use contract_fix_runner_aggregate::IFixRunnerAggregate;

use code_analysis::contract_target_resolver_protocol::ITargetResolverProtocol;
use output_report::taxonomy_result_vo::LintResult;
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
pub mod analysis_container;
