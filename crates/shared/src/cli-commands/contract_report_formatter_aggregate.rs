// PURPOSE: IReportFormatterAggregate — aggregate trait for report formatting
// AES402: All primitive types replaced with taxonomy VOs.
//   * `String` return → `DisplayContent` (semantic formatted output)
//
// Surface layer depends on this aggregate to format ScanReport output.
// The aggregate delegates to the appropriate capabilities formatter
// (text, json, sarif, junit) based on the requested format.
use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_scan_report_vo::ScanReport;
use crate::common::taxonomy_display_content_vo::DisplayContent;

/// IReportFormatterAggregate — aggregate protocol for report formatting.
///
/// Implemented by ReportFormatterOrchestrator (agent layer).
/// Provides a single method for formatting a ScanReport into any supported format.
pub trait IReportFormatterAggregate: Send + Sync {
    /// Format the scan report into the specified output format.
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent;
}
