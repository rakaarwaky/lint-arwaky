// PURPOSE: Module: orphan-detector module declarations and re-exports

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
use crate::output_report::taxonomy_severity_vo::Severity;
pub fn mk_orphan_result(
    file: &str,
    msg: &str,
    sev: Severity,
) -> crate::output_report::taxonomy_result_vo::LintResult {
    crate::output_report::taxonomy_result_vo::LintResult {
        file: crate::source_parsing::taxonomy_path_vo::FilePath::new(file.to_string())
            .unwrap_or_default(),
        line: crate::shared_common::taxonomy_common_vo::LineNumber::new(0),
        column: crate::shared_common::taxonomy_common_vo::ColumnNumber::new(0),
        code: crate::shared_common::taxonomy_error_vo::ErrorCode::raw("AES030"),
        message: crate::shared_common::taxonomy_message_vo::LintMessage::new(msg),
        source: Some(
            crate::shared_common::taxonomy_adapter_name_vo::AdapterName::raw("architecture"),
        ),
        severity: sev,
        enclosing_scope: None,
        related_locations: crate::shared_common::taxonomy_lint_vo::LocationList::new(),
    }
}
