// Unit tests — ViolationItem construction and fields from various sources.
use dispatcher_lint_arwaky::surface_output_component::ViolationItem;
use shared::cli_commands::LintResult;
use shared::common::{ColumnNumber, ErrorCode, FilePath, LineNumber, LintMessage, Severity};

fn make_lint_result(
    file: &str,
    line: i64,
    code: &str,
    severity: Severity,
    msg: &str,
) -> LintResult {
    LintResult {
        file: FilePath::new(file.to_string()).unwrap(),
        line: LineNumber::new(line),
        column: ColumnNumber::new(0),
        code: ErrorCode::raw(code),
        message: LintMessage::new(msg),
        source: None,
        severity,
        enclosing_scope: None,
        related_locations: vec![],
    }
}

#[test]
fn violation_item_from_lint_result_preserves_fields() {
    let lr = make_lint_result("src/lib.rs", 42, "AES201", Severity::HIGH, "test message");
    let item = ViolationItem::from_lint_result(&lr);

    assert_eq!(item.file.value, "src/lib.rs");
    assert_eq!(item.line.value(), 42);
    assert_eq!(item.code.code(), "AES201");
    assert_eq!(item.message.value, "test message");
    assert!(matches!(item.severity, Severity::HIGH));
}

#[test]
fn violation_item_from_json_obj_valid() {
    let json = serde_json::json!({
        "file": "src/main.rs",
        "line": 10,
        "column": 5,
        "code": "AES301",
        "message": "duplication",
        "severity": "MEDIUM"
    });
    let item = ViolationItem::from_json_obj(&json).unwrap();

    assert_eq!(item.file.value, "src/main.rs");
    assert_eq!(item.line.value(), 10);
    assert_eq!(item.column.value(), 5);
    assert_eq!(item.code.code(), "AES301");
    assert_eq!(item.message.value, "duplication");
    assert!(matches!(item.severity, Severity::MEDIUM));
}

#[test]
fn violation_item_from_json_obj_missing_field_returns_none() {
    let json = serde_json::json!({
        "file": "src/main.rs",
        "line": 10
        // Missing "code" and "message"
    });
    assert!(ViolationItem::from_json_obj(&json).is_none());
}

#[test]
fn violation_item_from_json_obj_defaults() {
    let json = serde_json::json!({
        "file": "src/lib.rs",
        "code": "AES101",
        "message": "test"
        // Missing "line", "column", "severity"
    });
    let item = ViolationItem::from_json_obj(&json).unwrap();
    assert_eq!(item.line.value(), 0); // default
    assert_eq!(item.column.value(), 0); // default
    assert!(matches!(item.severity, Severity::INFO)); // default
}

#[test]
fn severity_level_ordering() {
    let critical = make_lint_result("f.rs", 1, "AES101", Severity::CRITICAL, "c");
    let high = make_lint_result("f.rs", 1, "AES101", Severity::HIGH, "h");
    let medium = make_lint_result("f.rs", 1, "AES101", Severity::MEDIUM, "m");
    let low = make_lint_result("f.rs", 1, "AES101", Severity::LOW, "l");
    let info = make_lint_result("f.rs", 1, "AES101", Severity::INFO, "i");

    assert_eq!(
        ViolationItem::from_lint_result(&critical).severity_level(),
        4
    );
    assert_eq!(ViolationItem::from_lint_result(&high).severity_level(), 3);
    assert_eq!(ViolationItem::from_lint_result(&medium).severity_level(), 2);
    assert_eq!(ViolationItem::from_lint_result(&low).severity_level(), 1);
    assert_eq!(ViolationItem::from_lint_result(&info).severity_level(), 0);
}

#[test]
fn violation_item_is_clone() {
    let lr = make_lint_result("src/lib.rs", 1, "AES201", Severity::HIGH, "test");
    let item = ViolationItem::from_lint_result(&lr);
    let cloned = item.clone();
    assert_eq!(item.code.code(), cloned.code.code());
    assert_eq!(item.file.value, cloned.file.value);
}
