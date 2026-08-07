// Unit tests — TUI report formatter utility tests.
use shared::maintenance::{ToolStatus, ToolchainDiagnostics};
use tui_lint_arwaky::utility_report_formatter;

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
