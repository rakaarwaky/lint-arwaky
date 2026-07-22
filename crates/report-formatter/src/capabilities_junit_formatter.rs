// PURPOSE: JunitFormatter — implements IReportFormatterProtocol for JUnit XML output
//
// Formats ScanReport into JUnit XML format.
use super::utility_report_format::format_report_default;
use shared::cli_commands::contract_report_formatter_protocol::IReportFormatterProtocol;
use shared::cli_commands::taxonomy_format_vo::Format;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_scan_report_vo::ScanReport;
use shared::common::taxonomy_display_content_vo::DisplayContent;

// ─── Block 1: Struct Definition ───────────────────────────
/// JunitFormatter — produces JUnit XML output from ScanReport.
pub struct JunitFormatter;

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for JunitFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Junit {
            self.format_junit(&report.results)
        } else {
            DisplayContent::new(format_report_default(report))
        }
    }

    fn supported_format(&self) -> Format {
        Format::Junit
    }
}

impl JunitFormatter {
    /// Format results as JUnit XML wrapped in DisplayContent.
    pub fn format_junit(&self, results: &[LintResult]) -> DisplayContent {
        let total = results.len();
        let failure_count = results
            .iter()
            .filter(|r| {
                matches!(
                    r.severity,
                    shared::common::taxonomy_severity_vo::Severity::CRITICAL
                        | shared::common::taxonomy_severity_vo::Severity::HIGH
                        | shared::common::taxonomy_severity_vo::Severity::MEDIUM
                        | shared::common::taxonomy_severity_vo::Severity::LOW
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
            let is_info = r.severity == shared::common::taxonomy_severity_vo::Severity::INFO;

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
        DisplayContent::new(xml)
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
