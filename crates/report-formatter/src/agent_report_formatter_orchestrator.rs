// PURPOSE: ReportFormatterOrchestrator — implements IReportFormatterAggregate
//! ReportFormatterOrchestrator — agent layer that coordinates report formatting.
//! Implements IReportFormatterAggregate by delegating to the appropriate
//! capabilities formatter through the FR-005 delegation seam.
use shared::cli_commands::{Format, ScanReport};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::report_formatter::contract_report_formatter_protocol::IFormatDelegationProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ReportFormatterDeps {
    pub text: Arc<dyn IFormatDelegationProtocol>,
    pub json: Arc<dyn IFormatDelegationProtocol>,
    pub sarif: Arc<dyn IFormatDelegationProtocol>,
    pub junit: Arc<dyn IFormatDelegationProtocol>,
}

pub struct ReportFormatterOrchestrator {
    deps: ReportFormatterDeps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IReportFormatterAggregate for ReportFormatterOrchestrator {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        let formatter: &dyn IFormatDelegationProtocol = match format {
            Format::Text => self.deps.text.as_ref(),
            Format::Json => self.deps.json.as_ref(),
            Format::Sarif => self.deps.sarif.as_ref(),
            Format::Junit => self.deps.junit.as_ref(),
        };
        formatter.format(report, format)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ReportFormatterOrchestrator {
    pub fn new(deps: ReportFormatterDeps) -> Self {
        Self { deps }
    }
}
