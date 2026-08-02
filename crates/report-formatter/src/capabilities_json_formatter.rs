// PURPOSE: JsonFormatter — implements IReportFormatterProtocol for JSON output per FR-002
use shared::cli_commands::{Format, ScanReport};
use shared::common::DisplayContent;
use shared::report_formatter::{
    IReportFormatterProtocol, JsonDiagnostic, JsonReportDto, JsonSummary, JsonViolation,
};
use crate::utility_report_format::format_report_default;

/// JsonFormatter — produces structured pretty-printed JSON output from ScanReport.
pub struct JsonFormatter;

#[async_trait::async_trait]
impl IReportFormatterProtocol for JsonFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Json {
            self.format_json(report)
        } else {
            DisplayContent::new(format_report_default(report))
        }
    }

    fn supported_format(&self) -> Format {
        Format::Json
    }
}

impl JsonFormatter {
    /// Create a new JSON formatter.
    pub fn new() -> Self {
        Self
    }

    /// Format scan report as structured pretty-printed JSON.
    pub fn format_json(&self, report: &ScanReport) -> DisplayContent {
        let mut crit = 0;
        let mut high = 0;
        let mut med = 0;
        let mut low = 0;

        // Separate AES violations from external lint results
        let mut violations = Vec::new();
        let mut external_results = Vec::new();

        for r in &report.results {
            let output = JsonViolation {
                file: r.file.value.clone(),
                line: r.line.value(),
                code: r.code.to_string(),
                severity: r.severity.to_string(),
                message: r.message.value.clone(),
            };

            match r.severity {
                shared::common::taxonomy_severity_vo::Severity::CRITICAL => crit += 1,
                shared::common::taxonomy_severity_vo::Severity::HIGH => high += 1,
                shared::common::taxonomy_severity_vo::Severity::MEDIUM => med += 1,
                shared::common::taxonomy_severity_vo::Severity::LOW => low += 1,
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
            total_violations: report.results.len(),
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
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}
