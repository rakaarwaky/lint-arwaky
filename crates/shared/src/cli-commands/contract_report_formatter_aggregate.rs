// PURPOSE: IReportFormatterAggregate — aggregate trait for report formatting
//
// Surface layer depends on this aggregate to format ScanReport output.
// The aggregate delegates to the appropriate capabilities formatter
// (text, json, sarif, junit) based on the requested format.
use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_scan_report_vo::ScanReport;

/// IReportFormatterAggregate — aggregate port for report formatting.
///
/// Implemented by ReportFormatterOrchestrator (agent layer).
/// Provides a single method for formatting a ScanReport into any supported format.
pub trait IReportFormatterAggregate: Send + Sync {
    /// Format the scan report into the specified output format.
    fn format(&self, report: &ScanReport, format: Format) -> String;
}
