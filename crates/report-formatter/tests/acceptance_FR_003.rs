// FR-003 — SARIF 2.1.0 Format Output
use report_formatter_lint_arwaky::capabilities_sarif_formatter::SarifFormatter;
use shared::cli_commands::DiagnosticSeverity;
use shared::cli_commands::{LintResult, PipelineDiagnostic, ScanReport};
use shared::common::{AdapterName, ErrorCode, FilePath, LineNumber, LintMessage, Severity};

fn result(code: &str, sev: Severity, line: i64) -> LintResult {
    LintResult {
        file: FilePath::new("src/surface.rs").unwrap(),
        line: LineNumber::new(line),
        code: ErrorCode::raw(code),
        message: LintMessage::new(format!("message {code}")),
        source: Some(AdapterName::raw("architecture")),
        severity: sev,
        ..Default::default()
    }
}

fn parse(out: &str) -> serde_json::Value {
    let v: serde_json::Value = serde_json::from_str(out).expect("must be valid JSON");
    assert_eq!(v["version"], "2.1.0");
    v
}

#[test]
fn us1_normal_report_has_tool_metadata() {
    let report = ScanReport {
        results: vec![result("AES201", Severity::HIGH, 1)],
        diagnostics: vec![],
        score: None,
    };
    let v = parse(SarifFormatter::new().format_sarif_report(&report).value());
    let driver = &v["runs"][0]["tool"]["driver"];
    assert_eq!(driver["name"], "lint-arwaky");
    assert!(!driver["version"].as_str().unwrap().is_empty());
    assert!(
        driver["information_uri"]
            .as_str()
            .unwrap()
            .contains("rakaarwaky/lint-arwaky")
    );
}

#[test]
fn us2_critical_high_map_to_error_level() {
    let report = ScanReport {
        results: vec![
            result("AES101", Severity::CRITICAL, 1),
            result("AES102", Severity::HIGH, 2),
        ],
        diagnostics: vec![],
        score: None,
    };
    let v = parse(SarifFormatter::new().format_sarif_report(&report).value());
    let levels: Vec<&str> = v["runs"][0]["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["level"].as_str().unwrap())
        .collect();
    assert_eq!(levels, vec!["error", "error"]);
}

#[test]
fn us3_medium_maps_to_warning_level() {
    let report = ScanReport {
        results: vec![result("AES201", Severity::MEDIUM, 1)],
        diagnostics: vec![],
        score: None,
    };
    let v = parse(SarifFormatter::new().format_sarif_report(&report).value());
    assert_eq!(v["runs"][0]["results"][0]["level"], "warning");
}

#[test]
fn us4_low_info_map_to_note_level() {
    let report = ScanReport {
        results: vec![
            result("AES301", Severity::LOW, 1),
            result("AES401", Severity::INFO, 2),
        ],
        diagnostics: vec![],
        score: None,
    };
    let v = parse(SarifFormatter::new().format_sarif_report(&report).value());
    let levels: Vec<&str> = v["runs"][0]["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["level"].as_str().unwrap())
        .collect();
    assert_eq!(levels, vec!["note", "note"]);
}

#[test]
fn us5_parse_warn_diagnostic_is_note() {
    let report = ScanReport {
        results: vec![],
        diagnostics: vec![PipelineDiagnostic::new(
            "parser".to_string(),
            "File skipped".to_string(),
            DiagnosticSeverity::Warning,
        )],
        score: None,
    };
    let v = parse(SarifFormatter::new().format_sarif_report(&report).value());
    assert_eq!(v["runs"][0]["results"][0]["rule_id"], "PARSE_WARN");
    assert_eq!(v["runs"][0]["results"][0]["level"], "note");
}

#[test]
fn us6_line_zero_clamped_to_one() {
    let report = ScanReport {
        results: vec![result("AES201", Severity::LOW, 0)],
        diagnostics: vec![],
        score: None,
    };
    let v = parse(SarifFormatter::new().format_sarif_report(&report).value());
    let region = &v["runs"][0]["results"][0]["locations"][0]["physical_location"]["region"];
    assert_eq!(region["start_line"], 1);
}

#[test]
fn us7_empty_results_is_valid_sarif_with_empty_array() {
    let v = parse(
        SarifFormatter::new()
            .format_sarif_report(&ScanReport::new(vec![], vec![]))
            .value(),
    );
    assert!(v["runs"][0]["results"].as_array().unwrap().is_empty());
}

#[test]
fn us8_external_results_included_with_tool_native_id() {
    let report = ScanReport {
        results: vec![LintResult {
            file: FilePath::new("src/lib.rs").unwrap(),
            line: LineNumber::new(9),
            code: ErrorCode::raw("ruff::E501"),
            message: LintMessage::new("line too long"),
            source: Some(AdapterName::raw("ruff")),
            severity: Severity::MEDIUM,
            ..Default::default()
        }],
        diagnostics: vec![],
        score: None,
    };
    let v = parse(SarifFormatter::new().format_sarif_report(&report).value());
    assert_eq!(v["runs"][0]["results"][0]["rule_id"], "ruff::E501");
}
