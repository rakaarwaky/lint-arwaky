// PURPOSE: IReportFormatterProtocol — protocol for formatting ScanReport output
// AES402: All primitive types replaced with taxonomy VOs.
//   * `String` return → `DisplayContent` (semantic formatted output)
//
// Defines the contract that all report formatters must implement. Each formatter
// (text, json, sarif, junit) implements this trait to produce output in its
// respective format.
use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_scan_report_vo::ScanReport;
use crate::common::taxonomy_display_content_vo::DisplayContent;

/// IReportFormatterProtocol — protocol for formatting analysis results.
///
/// Implemented by TextFormatter, JsonFormatter, SarifFormatter, and JunitFormatter.
/// Each formatter converts a ScanReport into its respective output format.
pub trait IReportFormatterProtocol: Send + Sync {
    /// Format the scan report into the specified output format.
    ///
    /// # Arguments
    /// * `report` - The ScanReport to format
    /// * `format` - The desired output format
    ///
    /// # Returns
    /// Formatted output wrapped in DisplayContent VO.
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent;

    /// Return the supported format name (e.g., "text", "json").
    fn supported_format(&self) -> Format;
}
