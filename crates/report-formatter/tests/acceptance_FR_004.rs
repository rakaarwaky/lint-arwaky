// FR-004 — JUnit XML Format Output
use report_formatter_lint_arwaky::capabilities_junit_formatter::JunitFormatter;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};

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
fn us1_normal_violations_have_failure_elements() {
    let report = ScanReport {
        results: vec![result("AES201", Severity::HIGH, "forbidden import")],
        diagnostics: vec![],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("<failure message=\"high: forbidden import\" type=\"high\">"));
    assert!(xml.contains("classname=\"AES201\""));
    assert!(xml.contains("name=\"src/surface.rs:14\""));
}

#[test]
fn us2_info_violations_have_no_failure_element() {
    let report = ScanReport {
        results: vec![result("AES401", Severity::INFO, "info")],
        diagnostics: vec![],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("classname=\"AES401\""));
    assert!(!xml.contains("<failure"));
}

#[test]
fn us3_parse_warn_diagnostics_become_skipped() {
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped: parse failure".to_string(),
            DiagnosticSeverity::Warning,
        )],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("<testcase classname=\"PARSE_WARN\" name=\"parser\">"));
    assert!(xml.contains("<skipped message=\"File skipped: parse failure\" />"));
}

#[test]
fn us4_special_characters_are_xml_escaped() {
    let report = ScanReport {
        results: vec![result("AES201", Severity::HIGH, "a < b & c > d \"e\" 'f'")],
        diagnostics: vec![],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("a &lt; b &amp; c &gt; d &quot;e&quot; &apos;f&apos;"));
}

#[test]
fn us5_test_and_failure_counts_match_results() {
    let report = ScanReport {
        results: vec![
            result("AES201", Severity::HIGH, "one"),
            result("AES301", Severity::INFO, "two"),
            result("AES401", Severity::LOW, "three"),
        ],
        diagnostics: vec![],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("tests=\"3\" failures=\"2\""));
}

#[test]
fn us6_empty_results_give_zero_tests_and_failures() {
    let xml = JunitFormatter::new()
        .format_junit_report(&ScanReport::new(vec![], vec![]))
        .value()
        .to_string();
    assert!(xml.contains("tests=\"0\" failures=\"0\""));
}

#[test]
fn us7_external_results_have_tool_native_classname() {
    let report = ScanReport {
        results: vec![LintResult {
            file: FilePath::new("src/lib.rs").unwrap(),
            line: LineNumber::new(5),
            code: ErrorCode::raw("eslint::no-unused-vars"),
            message: LintMessage::new("unused var"),
            source: Some(AdapterName::raw("eslint")),
            severity: Severity::MEDIUM,
            ..Default::default()
        }],
        diagnostics: vec![],
        score: None,
    };
    let xml = JunitFormatter::new()
        .format_junit_report(&report)
        .value()
        .to_string();
    assert!(xml.contains("classname=\"eslint::no-unused-vars\""));
}
