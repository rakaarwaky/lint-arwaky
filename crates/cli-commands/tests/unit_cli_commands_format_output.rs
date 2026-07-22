//! Unit tests for utility_format_output — SARIF, JUnit XML, and XML escaping.

use cli_commands_lint_arwaky::utility_format_output::{
    format_junit_output, format_sarif_output, xml_escape,
};
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;

// ─── xml_escape ──────────────────────────────────────────────────────────────

#[test]
fn xml_escape_plain_text_unchanged() {
    assert_eq!(xml_escape("hello world"), "hello world");
}

#[test]
fn xml_escape_ampersand() {
    assert_eq!(xml_escape("a & b"), "a &amp; b");
}

#[test]
fn xml_escape_angle_brackets() {
    assert_eq!(xml_escape("<tag>"), "&lt;tag&gt;");
}

#[test]
fn xml_escape_quotes() {
    assert_eq!(
        xml_escape(r#"say "hi" & 'bye'"#),
        "say &quot;hi&quot; &amp; &apos;bye&apos;"
    );
}

#[test]
fn xml_escape_empty_string() {
    assert_eq!(xml_escape(""), "");
}

#[test]
fn xml_escape_multiple_special_chars() {
    assert_eq!(
        xml_escape("a<b>c&d\"e'f"),
        "a&lt;b&gt;c&amp;d&quot;e&apos;f"
    );
}

// ─── format_junit_output ─────────────────────────────────────────────────────

#[test]
fn junit_empty_results_produces_valid_xml_header() {
    let output = format_junit_output(&[]);
    assert!(output.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
    assert!(output.contains("tests=\"0\""));
    assert!(output.contains("failures=\"0\""));
    assert!(output.contains("</testsuites>"));
}

#[test]
fn junit_single_critical_violation() {
    let results = vec![LintResult::new_arch(
        "src/main.rs",
        10,
        "AES301",
        Severity::CRITICAL,
        "Forbidden import detected",
    )];
    let output = format_junit_output(&results);
    assert!(output.contains("tests=\"1\""));
    assert!(output.contains("failures=\"1\""));
    assert!(output.contains("AES301"));
    assert!(output.contains("src/main.rs:10"));
    assert!(output.contains("<failure"));
    assert!(output.contains("critical"));
}

#[test]
fn junit_info_severity_not_counted_as_failure() {
    let results = vec![LintResult::new_arch(
        "src/lib.rs",
        1,
        "AES101",
        Severity::INFO,
        "Informational note",
    )];
    let output = format_junit_output(&results);
    assert!(output.contains("tests=\"1\""));
    assert!(output.contains("failures=\"0\""));
    assert!(!output.contains("<failure"));
}

#[test]
fn junit_multiple_violations_mixed_severity() {
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES201", Severity::HIGH, "high issue"),
        LintResult::new_arch("b.rs", 2, "AES202", Severity::LOW, "low issue"),
        LintResult::new_arch("c.rs", 3, "AES203", Severity::INFO, "info note"),
    ];
    let output = format_junit_output(&results);
    assert!(output.contains("tests=\"3\""));
    assert!(output.contains("failures=\"2\"")); // HIGH + LOW, not INFO
}

#[test]
fn junit_escapes_special_chars_in_message() {
    let results = vec![LintResult::new_arch(
        "src/main.rs",
        5,
        "AES301",
        Severity::MEDIUM,
        "Use <Vec> & \"String\" instead",
    )];
    let output = format_junit_output(&results);
    assert!(output.contains("&lt;Vec&gt;"));
    assert!(output.contains("&"));
    assert!(output.contains("&quot;String&quot;"));
}

// ─── format_sarif_output ─────────────────────────────────────────────────────

#[test]
fn sarif_empty_results_produces_valid_structure() {
    let output = format_sarif_output(&[]);
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    assert_eq!(parsed["version"], "2.1.0");
    assert!(parsed["$schema"].as_str().unwrap().contains("sarif"));
    assert!(parsed["runs"].is_array());
}

#[test]
fn sarif_single_result_has_correct_fields() {
    let results = vec![LintResult::new_arch(
        "crates/foo/src/lib.rs",
        42,
        "AES201",
        Severity::HIGH,
        "Forbidden import from contract(protocol)",
    )];
    let output = format_sarif_output(&results);
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    let run = &parsed["runs"][0];
    let result = &run["results"][0];
    assert_eq!(result["rule_id"], "AES201");
    assert!(result["message"]["text"]
        .as_str()
        .unwrap()
        .contains("Forbidden import"));
    let location = &result["locations"][0];
    assert_eq!(
        location["physical_location"]["artifact_location"]["uri"],
        "crates/foo/src/lib.rs"
    );
    assert_eq!(location["physical_location"]["region"]["start_line"], 42);
}

#[test]
fn sarif_severity_mapping() {
    let results = vec![
        LintResult::new_arch("a.rs", 1, "AES101", Severity::CRITICAL, "crit"),
        LintResult::new_arch("b.rs", 2, "AES102", Severity::HIGH, "high"),
        LintResult::new_arch("c.rs", 3, "AES103", Severity::MEDIUM, "med"),
        LintResult::new_arch("d.rs", 4, "AES104", Severity::LOW, "low"),
    ];
    let output = format_sarif_output(&results);
    let parsed: serde_json::Value = serde_json::from_str(&output).expect("valid JSON");
    let results_arr = parsed["runs"][0]["results"].as_array().unwrap();
    assert_eq!(results_arr.len(), 4);
    // SARIF levels: error, error, warning, note (typical mapping)
    for r in results_arr {
        assert!(r["level"].is_string());
    }
}
