// PURPOSE: RootReportFormatterContainer — wiring for report-formatter feature
// (root layer, wiring only). Composes the four formatter capabilities into
// the orchestrator behind IReportFormatterAggregate.
use crate::agent_report_formatter_orchestrator::{
    ReportFormatterDeps, ReportFormatterOrchestrator,
};
use crate::capabilities_json_formatter::JsonFormatter;
use crate::capabilities_junit_formatter::JunitFormatter;
use crate::capabilities_sarif_formatter::SarifFormatter;
use crate::capabilities_text_formatter::TextFormatter;
use shared_report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared_report_formatter::contract_report_formatter_protocol::{
    IJUnitFormatProtocol, IJsonFormatProtocol, ISarifFormatProtocol, ITextFormatProtocol,
};
use std::sync::Arc;

/// Composition root for the report-formatter feature.
#[derive(Clone)]
pub struct RootReportFormatterContainer;

impl RootReportFormatterContainer {
    /// Return a fresh aggregate with all four formatters wired in.
    pub fn aggregate() -> Arc<dyn IReportFormatterAggregate> {
        let deps = ReportFormatterDeps {
            text: Arc::new(TextFormatter::new()) as Arc<dyn ITextFormatProtocol>,
            json: Arc::new(JsonFormatter::new()) as Arc<dyn IJsonFormatProtocol>,
            sarif: Arc::new(SarifFormatter::new()) as Arc<dyn ISarifFormatProtocol>,
            junit: Arc::new(JunitFormatter::new()) as Arc<dyn IJUnitFormatProtocol>,
        };
        Arc::new(ReportFormatterOrchestrator::new(deps))
    }
}
