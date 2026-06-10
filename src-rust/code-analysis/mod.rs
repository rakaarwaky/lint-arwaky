// PURPOSE: Module declarations and re-exports for code-analysis (checkers, reporters, protocols, fixers, renamers, analyzers)
pub mod capabilities_analysis_reporter;
pub use capabilities_analysis_reporter::AnalysisReporter;
pub mod capabilities_renamer_processor;
pub use capabilities_renamer_processor::SymbolRenamerProcessor;
pub mod capabilities_unused_checker;
pub use capabilities_unused_checker::UnusedImportRuleChecker;
pub mod contract_type_protocol;
pub use contract_type_protocol::IDomainTypeProtocol;
pub mod contract_unused_protocol;
pub use contract_unused_protocol::IUnusedProtocol;
pub mod agent_checking_orchestrator;
pub use agent_checking_orchestrator::LintCheckingOrchestrator;

pub mod capabilities_fix_processor;
pub use capabilities_fix_processor::LintFixProcessor;
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
pub mod capabilities_single_bottleneck_checker;
pub use capabilities_single_bottleneck_checker::SingleBottleneckChecker;
pub mod capabilities_missing_vo_checker;
pub use capabilities_missing_vo_checker::MissingVoChecker;

pub mod contract_layer_detection_aggregate;
pub use contract_layer_detection_aggregate::ILayerDetectionAggregate;
pub mod contract_adapter_port;
pub use contract_adapter_port::ILinterAdapterPort;
pub mod contract_analysis_protocol;
pub use contract_analysis_protocol::IAnalysisProtocol;
pub mod contract_class_protocol;
pub use contract_class_protocol::IMandatoryClassProtocol;
pub mod contract_fix_aggregate;
pub use contract_fix_aggregate::LintFixOrchestratorAggregate;
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
pub mod contract_single_bottleneck_protocol;
pub use contract_single_bottleneck_protocol::ISingleBottleneckProtocol;
pub mod contract_missing_vo_protocol;
pub use contract_missing_vo_protocol::IMissingVoProtocol;
pub mod contract_reporting_protocol;
pub use contract_reporting_protocol::ILintReportingProtocol;
pub mod taxonomy_analysis_vo;
pub use taxonomy_analysis_vo::{
    FileDefinitionMap, FilePathSet, GraphAnalysisContext, ImportGraph, InboundLinkMap,
    InheritanceMap, ModuleToFileMap, OrphanIndicatorResult, ReachabilityResult,
};
pub mod capabilities_project_target_resolver;
pub mod taxonomy_import_source_vo;
pub use capabilities_project_target_resolver::{
    compute_score, count_loc, has_critical, lint_path, normalize_project_root, resolve_target,
    walk_rs_files, ProjectTargetResolver,
};
