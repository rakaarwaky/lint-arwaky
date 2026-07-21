// PURPOSE: JsonFormatter — implements IReportFormatterProtocol for JSON output
//
// Formats ScanReport into pretty-printed JSON output.
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;

/// JsonFormatter — produces JSON output from ScanReport.
pub struct JsonFormatter;

impl JsonFormatter {
    /// Create a new JSON formatter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl IReportFormatterProtocol for JsonFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> String {
        if format == Format::Json {
            serde_json::to_string_pretty(&report.results).unwrap_or_else(|_| "[]".to_string())
        } else {
            format_report_default(report)
        }
    }

    fn supported_format(&self) -> Format {
        Format::Json
    }
}

/// Default report formatter — produces a simple text summary.
pub fn format_report_default(report: &ScanReport) -> String {
    let mut output = String::new();
    output.push_str("Lint Arwaky Report\n");
    output.push_str(&format!("Violations: {}\n", report.results.len()));
    output.push_str(&format!("Diagnostics: {}\n", report.diagnostics.len()));

    // Group violations by code
    let mut code_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for r in &report.results {
        *code_counts.entry(r.code.to_string()).or_insert(0) += 1;
    }
    if !code_counts.is_empty() {
        output.push_str("\nViolations by code:\n");
        let mut sorted: Vec<_> = code_counts.into_iter().collect();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
        for (code, count) in &sorted {
            output.push_str(&format!("  {code}: {count}\n"));
        }
    }

    // Show diagnostics
    if !report.diagnostics.is_empty() {
        output.push_str("\nDiagnostics:\n");
        for d in &report.diagnostics {
            output.push_str(&format!(
                "  [{}/:{:?}] {}\n",
                d.source, d.severity, d.message
            ));
        }
    }

    output
}
