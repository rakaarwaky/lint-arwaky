// PURPOSE: report-formatter-domain capability contracts (AES102 `_protocol`).
//
// AES402: All primitive types replaced with taxonomy VOs.
//   * `String` return → `DisplayContent` (semantic formatted output)
//
// One file for the report-formatter feature. Four traits, one per capability
// (FR-001 through FR-004). No delegation trait — the orchestrator routes
// directly through the specific protocol of the selected formatter.
use shared_cli_commands::taxonomy_cli_commands_vo::ScanReport;
use shared_cli_commands::taxonomy_format_vo::Format;
use shared_common::taxonomy_message_vo::DisplayContent;

/// FR-ReportFormatter-001: human-readable text output.
///
/// Implemented by TextFormatter.
pub trait ITextFormatProtocol: Send + Sync {
    /// Render the scan report as human-readable text.
    fn format_text(&self, report: &ScanReport) -> DisplayContent;

    /// The single `Format` this formatter is registered for.
    fn supported_format(&self) -> Format;
}

/// FR-ReportFormatter-002: pretty-printed JSON output for CI/CD consumers.
///
/// Implemented by JsonFormatter.
pub trait IJsonFormatProtocol: Send + Sync {
    /// Render the scan report as pretty-printed JSON.
    fn format_json(&self, report: &ScanReport) -> DisplayContent;

    /// The single `Format` this formatter is registered for.
    fn supported_format(&self) -> Format;
}

/// FR-ReportFormatter-003: SARIF 2.1.0 output for IDE and Code Scanning.
///
/// Implemented by SarifFormatter.
pub trait ISarifFormatProtocol: Send + Sync {
    /// Render the scan report as a SARIF 2.1.0 document.
    fn format_sarif(&self, report: &ScanReport) -> DisplayContent;

    /// The single `Format` this formatter is registered for.
    fn supported_format(&self) -> Format;
}

/// FR-ReportFormatter-004: JUnit XML output for CI/CD test report parsers.
///
/// Implemented by JunitFormatter.
pub trait IJUnitFormatProtocol: Send + Sync {
    /// Render the scan report as JUnit XML.
    fn format_junit(&self, report: &ScanReport) -> DisplayContent;

    /// The single `Format` this formatter is registered for.
    fn supported_format(&self) -> Format;
}
