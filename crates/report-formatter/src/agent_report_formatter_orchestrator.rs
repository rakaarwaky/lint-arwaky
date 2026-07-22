// PURPOSE: ReportFormatterOrchestrator — implements IReportFormatterAggregate
// AES402: All primitive types replaced with taxonomy VOs.
//   * `String` return → `DisplayContent` (semantic formatted output)
//
// Agent layer that delegates formatting to the appropriate capabilities
// formatter (text, json, sarif, junit) based on the requested format.
use shared::cli_commands::contract_report_formatter_aggregate::IReportFormatterAggregate;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_display_content_vo::DisplayContent;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
/// ReportFormatterOrchestrator — agent layer that coordinates report formatting.
///
/// Implements IReportFormatterAggregate by delegating to the appropriate
/// capabilities formatter based on the requested format.
pub struct ReportFormatterOrchestrator {
    text: Arc<dyn IReportFormatterProtocol>,
    json: Arc<dyn IReportFormatterProtocol>,
    sarif: Arc<dyn IReportFormatterProtocol>,
    junit: Arc<dyn IReportFormatterProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IReportFormatterAggregate for ReportFormatterOrchestrator {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        let formatter: &dyn IReportFormatterProtocol = match format {
            Format::Text => self.text.as_ref(),
            Format::Json => self.json.as_ref(),
            Format::Sarif => self.sarif.as_ref(),
            Format::Junit => self.junit.as_ref(),
        };
        formatter.format(report, format)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl ReportFormatterOrchestrator {
    pub fn new(
        text: Arc<dyn IReportFormatterProtocol>,
        json: Arc<dyn IReportFormatterProtocol>,
        sarif: Arc<dyn IReportFormatterProtocol>,
        junit: Arc<dyn IReportFormatterProtocol>,
    ) -> Self {
        Self {
            text,
            json,
            sarif,
            junit,
        }
    }
}
