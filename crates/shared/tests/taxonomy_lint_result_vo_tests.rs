use shared_lint_arwaky::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared_lint_arwaky::cli_commands::taxonomy_severity_vo::Severity;
use shared_lint_arwaky::common::taxonomy_adapter_name_vo::AdapterName;
use shared_lint_arwaky::common::taxonomy_common_vo::{ColumnNumber, LineNumber};
use shared_lint_arwaky::common::taxonomy_error_vo::ErrorCode;
use shared_lint_arwaky::common::taxonomy_lint_vo::LocationList;
use shared_lint_arwaky::common::taxonomy_message_vo::LintMessage;
use shared_lint_arwaky::common::taxonomy_path_vo::FilePath;

fn sample_result() -> LintResult {
    LintResult {
        file: FilePath::new("test.rs").unwrap_or_default(),
        line: LineNumber::new(10),
        column: ColumnNumber::new(5),
        code: ErrorCode::raw("E001"),
        message: LintMessage::new("test error"),
        source: Some(AdapterName::raw("test")),
        severity: Severity::HIGH,
        enclosing_scope: None,
        related_locations: LocationList::new(),
    }
}

// ---------------------------------------------------------------------------
// LintResult
// ---------------------------------------------------------------------------

#[test]
fn lint_result_has_expected_fields() {
    let r = sample_result();
    assert_eq!(r.file.value, "test.rs");
    assert_eq!(r.line.value(), 10);
    assert_eq!(r.column.value(), 5);
}

#[test]
fn lint_result_new_arch_creates_result() {
    let r = LintResult::new_arch(
        "src/main.rs",
        42,
        "AES101",
        Severity::HIGH,
        "violation detected",
    );
    assert_eq!(r.file.value, "src/main.rs");
    assert_eq!(r.line.value(), 42);
    assert_eq!(r.code.code(), "AES101");
    assert_eq!(r.source.as_ref().unwrap().value(), "architecture");
}

#[test]
fn lint_result_position() {
    let r = sample_result();
    let pos = r.position();
    assert_eq!(pos.line.value(), 10);
    assert_eq!(pos.column.value(), 5);
}

#[test]
fn lint_result_identity() {
    let r = sample_result();
    let id = r.identity();
    assert!(id.value().contains("test.rs"));
    assert!(id.value().contains("E001"));
}

#[test]
fn lint_result_clone() {
    let r = sample_result();
    let c = r.clone();
    assert_eq!(r.file.value, c.file.value);
    assert_eq!(r.line.value(), c.line.value());
}

// ---------------------------------------------------------------------------
// LintResultList
// ---------------------------------------------------------------------------

#[test]
fn lint_result_list_new_empty() {
    let list = LintResultList::new(vec![]);
    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn lint_result_list_new_with_values() {
    let list = LintResultList::new(vec![sample_result()]);
    assert!(!list.is_empty());
    assert_eq!(list.len(), 1);
}

#[test]
fn lint_result_list_push_and_iterate() {
    let mut list = LintResultList::new(vec![]);
    list.push(sample_result());
    list.push(sample_result());
    assert_eq!(list.len(), 2);
    assert_eq!(list.iter().count(), 2);
}

#[test]
fn lint_result_list_append() {
    let mut list = LintResultList::new(vec![]);
    list.append(sample_result());
    assert_eq!(list.len(), 1);
}

#[test]
fn lint_result_list_default() {
    let list = LintResultList::default();
    assert!(list.is_empty());
}

// ---------------------------------------------------------------------------
// Severity
// ---------------------------------------------------------------------------

#[test]
fn severity_values_correct_order() {
    assert_eq!(Severity::LOW as i32, 1);
    assert_eq!(Severity::MEDIUM as i32, 2);
    assert_eq!(Severity::HIGH as i32, 3);
    assert_eq!(Severity::CRITICAL as i32, 4);
}

#[test]
fn severity_display() {
    assert_eq!(format!("{}", Severity::LOW), "low");
    assert_eq!(format!("{}", Severity::MEDIUM), "medium");
    assert_eq!(format!("{}", Severity::HIGH), "high");
}

#[test]
fn severity_clone() {
    let s = Severity::HIGH;
    assert_eq!(s.clone() as i32, Severity::HIGH as i32);
}

// ---------------------------------------------------------------------------
// ErrorCode
// ---------------------------------------------------------------------------

#[test]
fn error_code_raw() {
    let code = ErrorCode::raw("AES101");
    assert_eq!(code.code(), "AES101");
}

#[test]
fn error_code_display() {
    let code = ErrorCode::raw("clippy::pedantic");
    assert_eq!(format!("{}", code), "clippy::pedantic");
}

// ---------------------------------------------------------------------------
// AdapterName
// ---------------------------------------------------------------------------

#[test]
fn adapter_name_raw() {
    let name = AdapterName::raw("ruff");
    assert_eq!(name.value(), "ruff");
}

#[test]
fn adapter_name_display() {
    let name = AdapterName::raw("eslint");
    assert_eq!(format!("{}", name), "eslint");
}
