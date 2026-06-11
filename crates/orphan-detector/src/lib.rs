// PURPOSE: Module declarations and re-exports for orphan-detector (orchestrator, analyzers, protocols, aggregate)
pub mod taxonomy_layer_names_constant;
pub use taxonomy_layer_names_constant::{
    LAYER_AGENT, LAYER_CAPABILITIES, LAYER_CONTRACT, LAYER_GLOBAL, LAYER_INFRASTRUCTURE,
    LAYER_ROOT, LAYER_SURFACES, LAYER_TAXONOMY,
};

pub mod agent_orphan_orchestrator;
pub mod capabilities_orphan_agent_analyzer;
pub mod capabilities_orphan_capabilities_analyzer;
pub mod capabilities_orphan_contract_analyzer;
pub mod capabilities_orphan_infrastructure_analyzer;
pub mod capabilities_orphan_surfaces_analyzer;
pub mod capabilities_orphan_taxonomy_analyzer;
pub mod contract_orphan_aggregate;
pub mod contract_orphan_protocol;

// Re-export aggregate contract
pub use contract_orphan_aggregate::IOrphanAggregate;

// Re-export orchestrator components
pub use agent_orphan_orchestrator::{ArchOrphanAnalyzer, OrphanGraphResolver};

// Re-export protocols
pub use contract_orphan_protocol::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    IInfrastructureOrphanProtocol, ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol,
};

// Shared helper to build a lint result
use output_report::taxonomy_result_vo::LintResult;
use output_report::taxonomy_severity_vo::Severity;
use shared_common::taxonomy_adapter_name_vo::AdapterName;
use shared_common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_common::taxonomy_error_vo::ErrorCode;
use shared_common::taxonomy_lint_vo::LocationList;
use shared_common::taxonomy_message_vo::LintMessage;
use source_parsing::taxonomy_path_vo::FilePath;
pub fn mk_orphan_result(file: &str, msg: &str, sev: Severity) -> LintResult {
    LintResult {
        file: FilePath::new(file.to_string()).unwrap_or_default(),
        line: LineNumber::new(0),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw("AES030"),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}
pub mod orphan_container;
