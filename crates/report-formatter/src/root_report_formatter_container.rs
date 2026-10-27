// PURPOSE: Root-layer composition container for the report-formatter feature.

use crate::agent_report_formatter_orchestrator::{
    ReportFormatterDeps, ReportFormatterOrchestrator,
};
use crate::capabilities_json_formatter::JsonFormatter;
use crate::capabilities_junit_formatter::JunitFormatter;
use crate::capabilities_sarif_formatter::SarifFormatter;
use crate::capabilities_text_formatter::TextFormatter;
use shared_report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;

use std::sync::Arc;

/// The AES-named root container for the `report_formatter` feature module.
/// It composes the four formatter capabilities behind the
/// `IReportFormatterAggregate` contract that the CLI consumes.
pub struct ReportFormatterContainer {
    orchestrator: Arc<dyn IReportFormatterAggregate>,
}

impl ReportFormatterContainer {
    /// Create with the default formatter capabilities wired in.
    pub fn new() -> Self {
        Self {
            orchestrator: Arc::new(ReportFormatterOrchestrator::new(ReportFormatterDeps {
                text: Arc::new(TextFormatter::new()),
                json: Arc::new(JsonFormatter::new()),
                sarif: Arc::new(SarifFormatter::new()),
                junit: Arc::new(JunitFormatter::new()),
            })),
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn IReportFormatterAggregate> {
        self.orchestrator.clone()
    }
}

impl Default for ReportFormatterContainer {
    fn default() -> Self {
        Self::new()
    }
}
