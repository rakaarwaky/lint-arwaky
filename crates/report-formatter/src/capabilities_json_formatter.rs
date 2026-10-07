// PURPOSE: JsonFormatter — implements IJsonFormatProtocol for JSON output (FR-002).
use shared_cli_commands::{Format, ScanReport};
use shared_common::taxonomy_display_content_vo::DisplayContent;
use shared_report_formatter::contract_report_formatter_protocol::IJsonFormatProtocol;
use shared_report_formatter::taxonomy_report_formatter_vo::{
    JsonDiagnostic, JsonReportDto, JsonSummary, JsonViolation,
};

// ─── Block 1: Struct Definition ────────────────────────────

/// JsonFormatter — produces structured pretty-printed JSON output from ScanReport.
pub struct JsonFormatter;

// ─── Block 2: Protocol Trait Implementation ────────────────

impl IJsonFormatProtocol for JsonFormatter {
    fn format_json(&self, report: &ScanReport) -> DisplayContent {
        render_json(report)
    }

    fn supported_format(&self) -> Format {
        Format::Json
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────────────

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
        match r.severity {
            shared_common::Severity::CRITICAL => crit += 1,
            shared_common::Severity::HIGH => high += 1,
            shared_common::Severity::MEDIUM => med += 1,
            shared_common::Severity::LOW => low += 1,
            _ => {}
        }

        // INFO results are excluded from the summary, so they must not land
        // in the violations array either — otherwise total_violations could
        // not equal violations.len().
        if r.severity == shared_common::Severity::INFO {
            continue;
        }

        let output = JsonViolation {
            file: r.file.value().to_string(),
            line: r.line.value(),
            code: r.code.to_string(),
            severity: r.severity.to_string(),
            message: r.message.value().to_string(),
        };

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
        // Must always equal violations.len() — counting crit+high+med+low would
        // include external_results and silently drop INFO severities that land
        // in the violations array.
        total_violations: violations.len(),
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

    // A serialization failure must never render as a bare `{}`: an empty object
    // is indistinguishable from a clean scan for a consumer reading
    // `.summary.total`. Emit an explicitly marked error document instead.
    DisplayContent::new(serde_json::to_string_pretty(&dto).unwrap_or_else(|e| {
        serde_json::json!({
            "formatter_error": true,
            "format": "json",
            "reason": e.to_string(),
        })
        .to_string()
    }))
}
