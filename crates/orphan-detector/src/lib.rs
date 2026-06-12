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
pub mod capabilities_orphan_infrastructure_analyzer;
pub mod capabilities_orphan_surfaces_analyzer;
pub mod capabilities_orphan_taxonomy_analyzer;
pub use agent_orphan_orchestrator::{ArchOrphanAnalyzer, OrphanGraphResolver};

use output_report::taxonomy_result_vo::LintResult;
use output_report::taxonomy_severity_vo::Severity;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_message_vo::LintMessage;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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
pub mod root_orphan_container;
