// PURPOSE: JsonFormatter — implements IJsonFormatProtocol and IFormatDelegationProtocol
// for JSON output (FR-002, FR-005)
use shared::cli_commands::{Format, ScanReport};
use shared::common::taxonomy_display_content_vo::DisplayContent;
use shared::report_formatter::contract_report_formatter_protocol::IFormatDelegationProtocol;
use shared::report_formatter::contract_report_formatter_protocol::IJsonFormatProtocol;
use shared::report_formatter::taxonomy_report_vo::{
    JsonDiagnostic, JsonReportDto, JsonSummary, JsonViolation,
};
use shared::report_formatter::utility_report_format::format_report_default;

/// JsonFormatter — produces structured pretty-printed JSON output from ScanReport.
pub struct JsonFormatter;

impl IJsonFormatProtocol for JsonFormatter {
    fn format_json(&self, report: &ScanReport) -> DisplayContent {
        render_json(report)
    }

    fn supported_format(&self) -> Format {
        Format::Json
    }
}

/// FR-005: the registered dispatch verb for this formatter.
impl IFormatDelegationProtocol for JsonFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Json {
            self.format_json(report)
        } else {
            DisplayContent::new(format_report_default(report))
        }
    }
}

impl JsonFormatter {
    /// Create a new JSON formatter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Free Functions (stateless renderers) ─────────────────

/// FR-002: render the scan report as pretty-printed JSON.
fn render_json(report: &ScanReport) -> DisplayContent {
    let mut crit = 0;
    let mut high = 0;
    let mut med = 0;
    let mut low = 0;

    // Separate AES violations from external lint results
    let mut violations = Vec::new();
    let mut external_results = Vec::new();

    for r in &report.results {
        let output = JsonViolation {
            file: r.file.value().to_string(),
            line: r.line.value(),
            code: r.code.to_string(),
            severity: r.severity.to_string(),
            message: r.message.value().to_string(),
        };

        match r.severity {
            shared::common::Severity::CRITICAL => crit += 1,
            shared::common::Severity::HIGH => high += 1,
            shared::common::Severity::MEDIUM => med += 1,
            shared::common::Severity::LOW => low += 1,
            _ => {}
        }

        if r.code.code().starts_with("AES") || r.code.code().starts_with("PARSE_") {
            violations.push(output);
        } else {
            external_results.push(output);
        }
    }

    let diagnostics: Vec<JsonDiagnostic> = report
        .diagnostics
        .iter()
        .map(|d| JsonDiagnostic {
            source: d.source.clone(),
            severity: format!("{:?}", d.severity),
            message: d.message.clone(),
        })
        .collect();

    let summary = JsonSummary {
        total_violations: crit + high + med + low,
        critical: crit,
        high,
        medium: med,
        low,
        score: report.score.as_ref().map(|s| s.value),
    };

    let dto = JsonReportDto {
        violations,
        external_results,
        diagnostics,
        summary,
    };

    DisplayContent::new(serde_json::to_string_pretty(&dto).unwrap_or_else(|_| "{}".to_string()))
}
