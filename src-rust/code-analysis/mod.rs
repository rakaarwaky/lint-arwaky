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

pub mod contract_checker_aggregate;
pub use contract_checker_aggregate::ICheckerAggregate;
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
pub mod contract_reporting_protocol;
pub use contract_reporting_protocol::ILintReportingProtocol;
pub mod taxonomy_analysis_vo;
pub use taxonomy_analysis_vo::{
    FileDefinitionMap, FilePathSet, GraphAnalysisContext, ImportGraph, InboundLinkMap,
    InheritanceMap, ModuleToFileMap, OrphanIndicatorResult, ReachabilityResult,
};
pub mod taxonomy_import_source_vo;
