// PURPOSE: Module declarations and re-exports for layer-rules (all checkers, analyzers, protocols, orchestrators)
pub mod capabilities_naming_checker;
pub use capabilities_naming_checker::ArchNamingChecker;

pub mod capabilities_layer_detection_analyzer;
pub use capabilities_layer_detection_analyzer::LayerDetectionAnalyzer;
pub mod capabilities_cycle_analyzer;
pub use capabilities_cycle_analyzer::{detect_cycle_edges, DependencyEdge};
pub mod capabilities_import_forbidden_checker;
pub use capabilities_import_forbidden_checker::ArchImportForbiddenChecker;
pub mod capabilities_import_mandatory_checker;
pub use capabilities_import_mandatory_checker::ArchImportMandatoryChecker;
pub mod contract_import_parser_port;
pub use contract_import_parser_port::ImportParser;

pub mod contract_cycle_protocol;

pub mod contract_lint_protocol;
pub use contract_lint_protocol::IArchLintProtocol;

pub mod contract_rule_protocol;
pub use contract_rule_protocol::{IAnalyzer, INamingCheckerProtocol};
