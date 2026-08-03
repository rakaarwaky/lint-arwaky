// Unit tests — JunitFormatter (FR-004): JUnit XML output and xml_escape (FR-007).
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use report_formatter_lint_arwaky::capabilities_junit_formatter::xml_escape;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{Format, LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};
use shared::report_formatter::IReportFormatterProtocol;

fn result(code: &str, sev: Severity, message: &str) -> LintResult {
    LintResult {
        file: FilePath::new("src/surface.rs").unwrap(),
        line: LineNumber::new(14),
        code: ErrorCode::raw(code),
        message: LintMessage::new(message.to_string()),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        ..Default::default()
    }
}

#[test]
fn empty_report_produces_valid_xml_skeleton() {
    let out = JunitFormatter::new().format_junit_report(&ScanReport::new(vec![], vec![]));
    let xml = out.value();
    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(
        xml.contains("<testsuites name=\"lint-arwaky\" tests=\"0\" failures=\"0\" skipped=\"0\">")
    );
    assert!(xml.contains("<testsuite"));
    assert!(xml.ends_with("</testsuites>\n"));
}

#[test]
fn non_info_violations_produce_failure_elements() {
    let report = ScanReport {
        results: vec![
            result("AES201", Severity::CRITICAL, "forbidden import"),
            result("AES301", Severity::MEDIUM, "bypass found"),
        ],
        diagnostics: vec![],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("tests=\"2\" failures=\"2\""));
    assert!(xml.contains("classname=\"AES201\""));
    assert!(xml.contains("name=\"src/surface.rs:14\""));
    assert!(xml.contains("<failure message=\"critical: forbidden import\" type=\"critical\">"));
    assert!(xml.contains("classname=\"AES301\""));
}

#[test]
fn info_violations_produce_clean_testcase() {
    let report = ScanReport {
        results: vec![result("AES401", Severity::INFO, "info only")],
        diagnostics: vec![],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("tests=\"1\" failures=\"0\""));
    assert!(xml.contains("classname=\"AES401\""));
    assert!(!xml.contains("<failure"));
}

#[test]
fn parse_warn_diagnostics_become_skipped() {
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped".to_string(),
            DiagnosticSeverity::Warning,
        )],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("tests=\"1\" failures=\"0\" skipped=\"1\""));
    assert!(xml.contains("classname=\"PARSE_WARN\""));
    assert!(xml.contains("<skipped message=\"File skipped\" />"));
}

#[test]
fn special_characters_are_escaped() {
    let report = ScanReport {
        results: vec![result(
            "AES201",
            Severity::HIGH,
            "bad \"quote\" <tag> & 'apos'",
        )],
        diagnostics: vec![],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("bad &quot;quote&quot; &lt;tag&gt; &amp; &apos;apos&apos;"));
    assert!(!xml.contains("<tag>"));
}

#[test]
fn external_results_use_tool_native_classname() {
    let report = ScanReport {
        results: vec![LintResult {
            file: FilePath::new("src/lib.rs").unwrap(),
            line: LineNumber::new(5),
            code: ErrorCode::raw("ruff::E501"),
            message: LintMessage::new("line too long"),
            source: Some(AdapterName::raw("ruff")),
            severity: Severity::LOW,
            ..Default::default()
        }],
        diagnostics: vec![],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("classname=\"ruff::E501\""));
    assert!(xml.contains("<failure"));
}

#[test]
fn direct_format_junit_slice_works() {
    let results = vec![result("AES101", Severity::LOW, "minor")];
    let out = JunitFormatter::new().format_junit(&results);
    assert!(out.value().contains("tests=\"1\""));
    assert!(out.value().contains("classname=\"AES101\""));
}

#[test]
fn mismatched_format_falls_back_to_default() {
    let report = ScanReport::new(vec![], vec![]);
    let out = JunitFormatter::new().format(&report, Format::Json);
    assert!(out.value().contains("Violations: 0"));
}

#[test]
fn junit_formatter_supports_junit_format() {
    assert_eq!(JunitFormatter::new().supported_format(), Format::Junit);
}

// ── FR-007: xml_escape utility ────────────────────────────

#[test]
fn xml_escape_handles_all_five_entities() {
    assert_eq!(xml_escape("&<>\""), "&amp;&lt;&gt;&quot;");
    assert_eq!(xml_escape("it's"), "it&apos;s");
}

#[test]
fn xml_escape_empty_string() {
    assert_eq!(xml_escape(""), "");
}

#[test]
fn xml_escape_normal_text_unchanged() {
    assert_eq!(xml_escape("plain text 123"), "plain text 123");
}

#[test]
fn xml_escape_multiple_special_chars_all_escaped() {
    assert_eq!(
        xml_escape("a & b < c > d \" e ' f"),
        "a &amp; b &lt; c &gt; d &quot; e &apos; f"
    );
}
