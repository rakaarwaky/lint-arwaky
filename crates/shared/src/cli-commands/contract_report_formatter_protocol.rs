// PURPOSE: IReportFormatterProtocol — protocol for formatting ScanReport output
//
// Defines the contract that all report formatters must implement. Each formatter
// (text, json, sarif, junit) implements this trait to produce output in its
// respective format.
use crate::cli_commands::taxonomy_format_vo::Format;
use crate::cli_commands::taxonomy_scan_report_vo::ScanReport;

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
    /// A formatted string suitable for display or writing to a file.
    fn format(&self, report: &ScanReport, format: Format) -> String;

    /// Return the supported format name (e.g., "text", "json").
    fn supported_format(&self) -> Format;
}
