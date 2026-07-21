// PURPOSE: JunitFormatter — implements IReportFormatterProtocol for JUnit XML output
//
// Formats ScanReport into JUnit XML format.
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;

// ─── Block 1: Struct Definition ───────────────────────────
/// JunitFormatter — produces JUnit XML output from ScanReport.
pub struct JunitFormatter;

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for JunitFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> String {
        if format == Format::Junit {
            self.format_junit(&report.results)
        } else {
            format_report_default(report)
        }
    }

    fn supported_format(&self) -> Format {
        Format::Junit
    }
}

impl JunitFormatter {
    /// Format results as JUnit XML.
    pub fn format_junit(&self, results: &[LintResult]) -> String {
        let total = results.len();
        let failure_count = results
            .iter()
            .filter(|r| {
                matches!(
                    r.severity,
                    shared::cli_commands::taxonomy_severity_vo::Severity::CRITICAL
                        | shared::cli_commands::taxonomy_severity_vo::Severity::HIGH
                        | shared::cli_commands::taxonomy_severity_vo::Severity::MEDIUM
                        | shared::cli_commands::taxonomy_severity_vo::Severity::LOW
                )
            })
            .count();

        let mut xml = String::with_capacity(total.saturating_mul(256));
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
            "<testsuites name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));
        xml.push_str(&format!(
            "  <testsuite name=\"lint-arwaky\" tests=\"{total}\" failures=\"{failure_count}\">\n"
        ));

        for r in results {
            let classname = xml_escape(&r.code.to_string());
            let name = xml_escape(&format!("{}:{}", r.file.value, r.line.value()));
            let message = xml_escape(&r.message.value);
            let sev = r.severity.to_string();
            let is_info = r.severity == shared::cli_commands::taxonomy_severity_vo::Severity::INFO;

            xml.push_str(&format!(
                "    <testcase classname=\"{classname}\" name=\"{name}\">\n"
            ));
            if !is_info {
                xml.push_str(&format!(
                    "      <failure message=\"{sev}: {message}\" type=\"{sev}\">\n"
                ));
                xml.push_str(&format!("        {message}\n"));
                xml.push_str("      </failure>\n");
            }
            xml.push_str("    </testcase>\n");
        }

        xml.push_str("  </testsuite>\n");
        xml.push_str("</testsuites>\n");
        xml
    }
}

/// XML-escape a string for safe inclusion in JUnit XML output.
fn xml_escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            other => escaped.push(other),
        }
    }
    escaped
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl JunitFormatter {
    /// Create a new JUnit formatter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for JunitFormatter {
    fn default() -> Self {
        Self
    }
}
