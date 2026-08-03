// Unit tests — SarifFormatter (FR-003): SARIF 2.1.0 JSON output.
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{Format, LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};
use shared::report_formatter::IReportFormatterProtocol;

fn result(code: &str, sev: Severity, line: i64) -> LintResult {
    LintResult {
        file: FilePath::new("src/surface.rs").unwrap(),
        line: LineNumber::new(line),
        code: ErrorCode::raw(code),
        message: LintMessage::new(format!("message for {code}")),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        ..Default::default()
    }
}

fn parse_sarif(out: &str) -> serde_json::Value {
    let v: serde_json::Value = serde_json::from_str(out).expect("must be valid JSON");
    assert_eq!(
        v["$schema"],
        "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json"
    );
    assert_eq!(v["version"], "2.1.0");
    v
}

#[test]
fn empty_report_is_valid_sarif() {
    let out = SarifFormatter::new().format_sarif_report(&ScanReport::new(vec![], vec![]));
    let v = parse_sarif(out.value());
    let runs = v["runs"].as_array().unwrap();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0]["tool"]["driver"]["name"], "lint-arwaky");
    assert!(runs[0]["results"].as_array().unwrap().is_empty());
}

#[test]
fn severity_mapping_matches_sarif_levels() {
    let report = ScanReport {
        results: vec![
            result("AES101", Severity::CRITICAL, 1),
            result("AES102", Severity::HIGH, 2),
            result("AES201", Severity::MEDIUM, 3),
            result("AES301", Severity::LOW, 4),
            result("AES401", Severity::INFO, 5),
        ],
        diagnostics: vec![],
        score: None,
    };
    let out = SarifFormatter::new().format_sarif_report(&report);
    let v = parse_sarif(out.value());
    let results = v["runs"][0]["results"].as_array().unwrap();
    let levels: Vec<&str> = results
        .iter()
        .map(|r| r["level"].as_str().unwrap())
        .collect();
    assert_eq!(levels, vec!["error", "error", "warning", "note", "note"]);
}

#[test]
fn result_carries_rule_id_message_and_location() {
    let report = ScanReport {
        results: vec![result("AES201", Severity::HIGH, 14)],
        diagnostics: vec![],
        score: None,
    };
    let out = SarifFormatter::new().format_sarif_report(&report);
    let v = parse_sarif(out.value());
    let r = &v["runs"][0]["results"][0];
    assert_eq!(r["rule_id"], "AES201");
    assert_eq!(r["message"]["text"], "message for AES201");
    assert_eq!(
        r["locations"][0]["physical_location"]["artifact_location"]["uri"],
        "src/surface.rs"
    );
    assert_eq!(
        r["locations"][0]["physical_location"]["region"]["start_line"],
        14
    );
}

#[test]
fn line_zero_is_clamped_to_one() {
    let report = ScanReport {
        results: vec![result("AES201", Severity::LOW, 0)],
        diagnostics: vec![],
        score: None,
    };
    let out = SarifFormatter::new().format_sarif_report(&report);
    let v = parse_sarif(out.value());
    assert_eq!(
        v["runs"][0]["results"][0]["locations"][0]["physical_location"]["region"]["start_line"],
        1
    );
}

#[test]
fn parse_warn_diagnostics_become_note_results() {
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped".to_string(),
            DiagnosticSeverity::Warning,
        )],
        score: None,
    };
    let out = SarifFormatter::new().format_sarif_report(&report);
    let v = parse_sarif(out.value());
    let results = v["runs"][0]["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0]["rule_id"], "PARSE_WARN");
    assert_eq!(results[0]["level"], "note");
}

#[test]
fn external_results_included_with_tool_native_rule_id() {
    let report = ScanReport {
        results: vec![LintResult {
            file: FilePath::new("src/lib.rs").unwrap(),
            line: LineNumber::new(9),
            code: ErrorCode::raw("clippy::needless_return"),
            message: LintMessage::new("needless return"),
            source: Some(AdapterName::raw("clippy")),
            severity: Severity::MEDIUM,
            ..Default::default()
        }],
        diagnostics: vec![],
        score: None,
    };
    let out = SarifFormatter::new().format_sarif_report(&report);
    let v = parse_sarif(out.value());
    let r = &v["runs"][0]["results"][0];
    assert_eq!(r["rule_id"], "clippy::needless_return");
    assert_eq!(r["level"], "warning");
}

#[test]
fn rules_metadata_contains_all_rule_ids() {
    let report = ScanReport {
        results: vec![
            result("AES101", Severity::HIGH, 1),
            result("AES201", Severity::LOW, 2),
        ],
        diagnostics: vec![],
        score: None,
    };
    let out = SarifFormatter::new().format_sarif_report(&report);
    let v = parse_sarif(out.value());
    let rules = v["runs"][0]["rules"].as_array().unwrap();
    let ids: Vec<&str> = rules.iter().map(|r| r["id"].as_str().unwrap()).collect();
    assert!(ids.contains(&"AES101"));
    assert!(ids.contains(&"AES201"));
    assert_eq!(rules[0]["defaultConfiguration"], "error");
}

#[test]
fn direct_format_sarif_slice_works() {
    let results = vec![result("AES301", Severity::MEDIUM, 3)];
    let out = SarifFormatter::new().format_sarif(&results);
    let v = parse_sarif(out.value());
    assert_eq!(v["runs"][0]["results"].as_array().unwrap().len(), 1);
}

#[test]
fn mismatched_format_falls_back_to_default() {
    let report = ScanReport::new(vec![], vec![]);
    let out = SarifFormatter::new().format(&report, Format::Text);
    assert!(out.value().contains("Violations: 0"));
}

#[test]
fn sarif_formatter_supports_sarif_format() {
    assert_eq!(SarifFormatter::new().supported_format(), Format::Sarif);
}
