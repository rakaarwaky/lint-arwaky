// PURPOSE: Module declarations and re-exports for layer-rules (all checkers, analyzers, protocols, orchestrators)
pub mod agent_compliance_orchestrator;
pub use agent_compliance_orchestrator::{
    ArchComplianceOrchestrator, ArchitectureOrchestrator, InfrastructureMixinContainer,
    OrchestratorMixinContainer, WatchCommandsOrchestrator, WatchExecutionOrchestrator,
};
pub mod capabilities_compliance_analyzer;
pub use capabilities_compliance_analyzer::ArchComplianceAnalyzer;
pub mod capabilities_layer_checker;
pub use capabilities_layer_checker::ArchLayerChecker;
pub mod capabilities_cycle_analyzer;
pub use capabilities_cycle_analyzer::{
    detect_cycle_edges, DependencyCycleAnalyzer, DependencyEdge,
};
pub mod capabilities_hierarchy_checker;
pub use capabilities_hierarchy_checker::SurfaceHierarchyChecker;
pub mod capabilities_import_forbidden_checker;
pub use capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
pub mod capabilities_import_mandatory_checker;
pub use capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
pub mod capabilities_import_utils;

pub mod contract_compliance_port;
pub use contract_compliance_port::IArchCompliancePort;
pub mod contract_compliance_protocol;
pub use contract_compliance_protocol::{IArchComplianceProtocol, IScopeBoundaryProtocol};
pub mod contract_rules_orchestrator;
pub use contract_rules_orchestrator::ArchRulesOrchestratorAggregate;
pub mod contract_cycle_protocol;
pub use contract_cycle_protocol::{DefaultCycleAnalysisProtocol, ICycleAnalysisProtocol};
pub mod contract_import_protocol;
pub use contract_import_protocol::{DefaultArchImportProtocol, IArchImportProtocol};
pub mod contract_inheritance_protocol;
pub use contract_inheritance_protocol::{DefaultArchInheritanceProtocol, IArchInheritanceProtocol};
pub mod contract_lint_protocol;
pub use contract_lint_protocol::IArchLintProtocol;
pub mod contract_orchestrator_aggregate;
pub use contract_orchestrator_aggregate::ArchitectureOrchestratorAggregate;
pub mod contract_rule_protocol;
pub use contract_rule_protocol::{
    IAnalyzer, IArchRuleProtocol, IArchStructureProtocol, IInternalCheckerProtocol,
    IMetricCheckerProtocol, INamingCheckerProtocol, INamingRuleProtocol,
};
