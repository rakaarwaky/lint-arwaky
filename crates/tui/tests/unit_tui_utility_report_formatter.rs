// Unit tests — TUI report formatter utility tests.
use shared::cli_commands::{LintResult, LintResultList};
use shared::common::{
    AdapterName, ColumnNumber, ErrorCode, FilePath, LineNumber, LintMessage, LocationList, Severity,
};
use shared::maintenance::{DependencyInfo, DependencyReport, ToolStatus, ToolchainDiagnostics};
use tui_lint_arwaky::utility_report_formatter;

fn make_lint_result(code: &str, severity: Severity, msg: &str) -> LintResult {
    LintResult {
        file: FilePath::new("src/lib.rs".to_string()).unwrap(),
        line: LineNumber::new(1),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg),
        source: Some(AdapterName::raw("clippy")),
        severity,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}

#[test]
fn format_results_empty_returns_no_violations() {
    let list = LintResultList::new(vec![]);
    let result = utility_report_formatter::format_results(&list);
    assert!(result.value.contains("No violations found."));
}

#[test]
fn format_results_with_violations() {
    let list = LintResultList::new(vec![
        make_lint_result("AES201", Severity::HIGH, "import forbidden"),
        make_lint_result("AES301", Severity::MEDIUM, "duplication"),
    ]);
    let result = utility_report_formatter::format_results(&list);
    assert!(result.value.contains("Found 2 violation(s)"));
    assert!(result.value.contains("AES201"));
    assert!(result.value.contains("AES301"));
    assert!(result.value.contains("import forbidden"));
}

#[test]
fn format_results_shows_source() {
    let list = LintResultList::new(vec![make_lint_result("AES201", Severity::HIGH, "test")]);
    let result = utility_report_formatter::format_results(&list);
    assert!(result.value.contains("[clippy]"));
}

#[test]
fn format_doctor_report_all_ok() {
    let diag = ToolchainDiagnostics {
        rust_tools: vec![ToolStatus {
            name: "cargo".to_string(),
            status: "OK".to_string(),
            version: "1.75.0".to_string(),
        }],
        python_tools: vec![ToolStatus {
            name: "python".to_string(),
            status: "OK".to_string(),
            version: "3.11.0".to_string(),
        }],
        js_tools: vec![],
        vcs_tools: vec![ToolStatus {
            name: "git".to_string(),
            status: "OK".to_string(),
            version: "2.40.0".to_string(),
        }],
        binary_path: "/usr/bin/lint-arwaky".to_string(),
    };
    let result = utility_report_formatter::format_doctor_report(&diag);
    assert!(result.output.contains("All required tools OK."));
    assert!(result.output.contains("cargo"));
    assert!(result.violation_count == 0);
    assert!(result.success);
}

#[test]
fn format_doctor_report_with_failures() {
    let diag = ToolchainDiagnostics {
        rust_tools: vec![ToolStatus {
            name: "cargo".to_string(),
            status: "FAIL".to_string(),
            version: "".to_string(),
        }],
        python_tools: vec![],
        js_tools: vec![],
        vcs_tools: vec![],
        binary_path: "/usr/bin/lint-arwaky".to_string(),
    };
    let result = utility_report_formatter::format_doctor_report(&diag);
    assert!(result.output.contains("1 required tool(s) missing!"));
    assert!(result.violation_count == 1);
}

#[test]
fn format_doctor_report_with_warnings() {
    let diag = ToolchainDiagnostics {
        rust_tools: vec![ToolStatus {
            name: "clippy".to_string(),
            status: "WARN".to_string(),
            version: "".to_string(),
        }],
        python_tools: vec![],
        js_tools: vec![],
        vcs_tools: vec![],
        binary_path: "/usr/bin/lint-arwaky".to_string(),
    };
    let result = utility_report_formatter::format_doctor_report(&diag);
    assert!(result.output.contains("(optional)"));
    // WARN is not a required tool failure
    assert!(result.violation_count == 0);
}

#[test]
fn format_dependency_report() {
    let report = DependencyReport {
        language: "rust".to_string(),
        dependencies: vec![
            DependencyInfo {
                name: "serde".to_string(),
                version: "1.0".to_string(),
                dep_type: "direct".to_string(),
            },
            DependencyInfo {
                name: "tokio".to_string(),
                version: "1.0".to_string(),
                dep_type: "direct".to_string(),
            },
            DependencyInfo {
                name: "serde_json".to_string(),
                version: "1.0".to_string(),
                dep_type: "transitive".to_string(),
            },
        ],
    };
    let result = utility_report_formatter::format_dependency_report("my_project", &report);
    assert!(result.output.contains("Dependency scan for my_project"));
    assert!(result.output.contains("Language: rust"));
    assert!(result.output.contains("Total dependencies: 3"));
    assert!(result.output.contains("Direct"));
    assert!(result.output.contains("Transitive"));
}

#[test]
fn format_dependency_report_empty() {
    let report = DependencyReport {
        language: "python".to_string(),
        dependencies: vec![],
    };
    let result = utility_report_formatter::format_dependency_report("empty_project", &report);
    assert!(result.output.contains("Total dependencies: 0"));
    assert!(result.success);
}
