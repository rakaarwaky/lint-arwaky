// PURPOSE: JunitFormatter — implements IReportFormatterProtocol for JUnit XML output per FR-004
use shared::cli_commands::{Format, LintResult, ScanReport};
use shared::common::DisplayContent;
use shared::report_formatter::IReportFormatterProtocol;
use crate::utility_report_format::format_report_default;

// ─── Block 1: Struct Definition ───────────────────────────
/// JunitFormatter — produces JUnit XML output from ScanReport.
pub struct JunitFormatter;

// ─── Block 2: Protocol Trait Implementation ───────────────
#[async_trait::async_trait]
impl IReportFormatterProtocol for JunitFormatter {
    fn format(&self, report: &ScanReport, format: Format) -> DisplayContent {
        if format == Format::Junit {
            self.format_junit_report(report)
        } else {
        }
    }

    fn supported_format(&self) -> Format {
        Format::Junit
    }
}

impl JunitFormatter {
    /// Format ScanReport as JUnit XML wrapped in DisplayContent.
    pub fn format_junit_report(&self, report: &ScanReport) -> DisplayContent {
        let total_tests = report.results.len() + report.diagnostics.len();

        let failure_count = report
            .results
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

        let skip_count = report.diagnostics.len();

        let mut xml = String::with_capacity(total_tests.saturating_mul(256));
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
        "<testsuites name=\"lint-arwaky\" tests=\"{total_tests}\" failures=\"{failure_count}\" skipped=\"{skip_count}\">\n"
        ));
        xml.push_str(&format!(
        "  <testsuite name=\"lint-arwaky\" tests=\"{total_tests}\" failures=\"{failure_count}\" skipped=\"{skip_count}\">\n"
        ));

        // 1. Violations
        for r in &report.results {
            append_lint_result_testcase(&mut xml, r);
        }

        // 2. Diagnostics (PARSE_WARN -> <skipped>)
        for d in &report.diagnostics {
            let classname = xml_escape("PARSE_WARN");
            let name = xml_escape(&d.source);
            let message = xml_escape(&d.message);

            xml.push_str(&format!(
                "    <testcase classname=\"{classname}\" name=\"{name}\">\n"
            ));
            xml.push_str(&format!("      <skipped message=\"{message}\" />\n"));
            xml.push_str("    </testcase>\n");
        }

        xml.push_str("  </testsuite>\n");
        xml.push_str("</testsuites>\n");
        DisplayContent::new(xml)
    }

    /// Format results array directly as JUnit XML (backward compatibility).
    pub fn format_junit(&self, results: &[LintResult]) -> DisplayContent {
        let dummy_report = ScanReport {
            results: results.to_vec(),
            diagnostics: vec![],
            score: None,
        };
use crate::utility_report_format::format_report_default;
        self.format_junit_report(&dummy_report)
    }
}

fn append_lint_result_testcase(xml: &mut String, r: &LintResult) {
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

/// XML-escape a string for safe inclusion in JUnit XML output (FR-007).
pub fn xml_escape(s: &str) -> String {
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
