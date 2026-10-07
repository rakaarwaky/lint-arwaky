// PURPOSE: ReportFormatterOrchestrator — implements IReportFormatterAggregate
//! ReportFormatterOrchestrator — agent layer that coordinates report formatting.
//! Implements IReportFormatterAggregate by routing to the specific protocol
//! of the formatter selected for the requested Format.
use shared_cli_commands::{Format, ScanReport};
use shared_common::taxonomy_message_vo::DisplayContent;
use shared_report_formatter::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared_report_formatter::contract_report_formatter_protocol::IJUnitFormatProtocol;
use shared_report_formatter::contract_report_formatter_protocol::IJsonFormatProtocol;
use shared_report_formatter::contract_report_formatter_protocol::ISarifFormatProtocol;
use shared_report_formatter::contract_report_formatter_protocol::ITextFormatProtocol;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ReportFormatterDeps {
    pub text: Arc<dyn ITextFormatProtocol>,
    pub json: Arc<dyn IJsonFormatProtocol>,
    pub sarif: Arc<dyn ISarifFormatProtocol>,
    pub junit: Arc<dyn IJUnitFormatProtocol>,
}

pub struct ReportFormatterOrchestrator {
    deps: ReportFormatterDeps,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IReportFormatterAggregate for ReportFormatterOrchestrator {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        match format {
            Format::Text => self.deps.text.format_text(report),
            Format::Json => self.deps.json.format_json(report),
            Format::Sarif => self.deps.sarif.format_sarif(report),
            Format::Junit => self.deps.junit.format_junit(report),
        }
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ReportFormatterOrchestrator {
    pub fn new(deps: ReportFormatterDeps) -> Self {
        Self { deps }
    }
}
