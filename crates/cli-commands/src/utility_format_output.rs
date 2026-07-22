// PURPOSE: Stateless formatting utilities for JUnit XML and generic escaping
//
// SARIF output delegates to report_formatter::SarifFormatter to avoid duplication.

use report_formatter::SarifFormatter;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;

/// Format lint results as a SARIF 2.1.0 JSON string.
///
/// Delegates to the shared SarifFormatter from report-formatter crate
/// to avoid code duplication (AES305).
pub fn format_sarif_output(results: &[LintResult]) -> String {
    let formatter = SarifFormatter::new();
    formatter.format_sarif(results)
}

/// Format lint results as JUnit XML.
pub fn format_junit_output(results: &[LintResult]) -> String {
    let total = results.len();
    let failures: Vec<_> = results
        .iter()
        .filter(|r| {
            matches!(
                r.severity,
                Severity::CRITICAL | Severity::HIGH | Severity::MEDIUM | Severity::LOW
            )
        })
        .collect();
    let failure_count = failures.len();

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
        let is_info = r.severity == Severity::INFO;

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

/// XML-escape a string for safe inclusion in JUnit XML output.
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
