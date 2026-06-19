// PURPOSE: Module declarations for orphan-detector (orchestrator, analyzers, container)
pub use shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate;
pub use shared::orphan_detector::contract_orphan_protocol::{
    IAgentOrphanProtocol, ICapabilitiesOrphanProtocol, IContractOrphanProtocol,
    IInfrastructureOrphanProtocol, ISurfacesOrphanProtocol, ITaxonomyOrphanProtocol,
};
pub mod agent_orphan_orchestrator;
pub mod capabilities_orphan_agent_analyzer;
pub mod capabilities_orphan_capabilities_analyzer;
pub mod capabilities_orphan_contract_analyzer;
pub mod capabilities_orphan_graph_resolver;
pub mod capabilities_orphan_infrastructure_analyzer;
pub mod capabilities_orphan_surfaces_analyzer;
pub mod capabilities_orphan_taxonomy_analyzer;
pub use agent_orphan_orchestrator::ArchOrphanAnalyzer;
pub use capabilities_orphan_graph_resolver::OrphanGraphResolver;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::LintMessage;
pub fn mk_orphan_result(file: &str, msg: &str, sev: Severity, code: &str) -> LintResult {
    LintResult {
        file: FilePath::new(file.to_string()).unwrap_or_default(),
        line: LineNumber::new(0),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}
pub mod root_orphan_detector_container;
