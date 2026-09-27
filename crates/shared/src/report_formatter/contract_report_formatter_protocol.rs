// PURPOSE: report-formatter-domain capability contracts (AES102 `_protocol`).
//
// AES402: All primitive types replaced with taxonomy VOs.
//   * `String` return → `DisplayContent` (semantic formatted output)
//
// One file for the report-formatter feature. Each trait below is one
// capability seam mapped to exactly one `FR-ReportFormatter-NNN` heading in
// `crates/report-formatter/FRD.md`, so no trait covers a requirement that
// does not exist and no requirement lacks a trait.
use crate::cli_commands::taxonomy_command_vo::ScanReport;
use crate::cli_commands::taxonomy_format_vo::Format;
use crate::common::taxonomy_display_content_vo::DisplayContent;

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

    /// Escape the five XML entities required by the JUnit schema.
    fn xml_escape(&self, text: &str) -> DisplayContent;
}

/// FR-ReportFormatter-005: route a request to the formatter for its `Format`.
///
/// Implemented by every formatter, and by the orchestrator over the whole
/// registered set. The orchestrator holds one `Arc<dyn IFormatDelegationProtocol>`
/// per format and dispatches through this verb.
pub trait IFormatDelegationProtocol: Send + Sync {
    /// Render the scan report in `format`, falling back to the default
    /// summary when this formatter is not registered for that format.
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent;
}

/// FR-ReportFormatter-006: simple text summary used when no format matches.
///
/// Implemented by TextFormatter.
pub trait IDefaultReportFallbackProtocol: Send + Sync {
    /// Render the plain-text summary with counts by code and diagnostics.
    fn format_default(&self, report: &ScanReport) -> DisplayContent;
}

/// FR-ReportFormatter-007: XML entity escaping utility.
///
/// Implemented by JunitFormatter, the only JUnit XML producer.
pub trait IXmlEscapeProtocol: Send + Sync {
    /// `&`, `<`, `>`, `"`, `'` → named entities; every other char passes through.
    fn xml_escape(&self, text: &str) -> DisplayContent;
}
