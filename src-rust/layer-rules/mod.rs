// PURPOSE: Module declarations and re-exports for layer-rules (all checkers, analyzers, protocols, orchestrators)
pub mod capabilities_naming_checker;
pub use capabilities_naming_checker::ArchNamingChecker;
pub mod agent_compliance_orchestrator;
pub mod capabilities_compliance_analyzer;
pub use capabilities_compliance_analyzer::ArchComplianceAnalyzer;
pub mod capabilities_layer_checker;
pub use capabilities_layer_checker::ArchLayerChecker;
pub mod capabilities_cycle_analyzer;
pub use capabilities_cycle_analyzer::{detect_cycle_edges, DependencyEdge};
pub mod capabilities_hierarchy_checker;
pub use capabilities_hierarchy_checker::SurfaceHierarchyChecker;
pub mod capabilities_import_forbidden_checker;
pub use capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
pub mod capabilities_import_mandatory_checker;
pub use capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
pub mod capabilities_import_utils;

pub mod contract_compliance_port;
pub mod contract_compliance_protocol;
pub use contract_compliance_protocol::IScopeBoundaryProtocol;
pub mod contract_rules_orchestrator;
pub mod contract_cycle_protocol;

pub mod contract_lint_protocol;
pub use contract_lint_protocol::IArchLintProtocol;

pub mod contract_rule_protocol;
pub use contract_rule_protocol::{IAnalyzer, INamingCheckerProtocol};
