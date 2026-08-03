// Unit tests — Formatting utility tests: group_by_member, status_icon, output structure.
use cli_commands::surface_formatting::{group_by_member, status_icon};
use dispatcher_lint_arwaky::surface_output_component::ViolationItem;
use shared::common::{ColumnNumber, ErrorCode, FilePath, LineNumber, LintMessage, Severity};

fn violation(file: &str, code: &str, line: i64) -> ViolationItem {
    ViolationItem {
        code: ErrorCode::raw(code),
        file: FilePath::new(file.to_string()).unwrap(),
        line: LineNumber::new(line),
        column: ColumnNumber::new(0),
        message: LintMessage::new(format!("{code} at {file}:{line}")),
        severity: Severity::HIGH,
    }
}

#[test]
fn group_by_member_groups_correctly() {
    let violations = vec![
        violation("crates/foo/src/lib.rs", "AES201", 1),
        violation("crates/foo/src/main.rs", "AES202", 2),
        violation("crates/bar/src/lib.rs", "AES301", 3),
    ];

    let grouped = group_by_member(&violations, "crates", None);
    // Should have at least 2 groups (foo and bar)
    assert!(
        grouped.len() >= 2,
        "Expected at least 2 groups, got {}",
        grouped.len()
    );
}

#[test]
fn group_by_member_with_force_member() {
    let violations = vec![
        violation("crates/foo/src/lib.rs", "AES201", 1),
        violation("crates/bar/src/lib.rs", "AES301", 2),
    ];

    let grouped = group_by_member(&violations, "crates", Some("forced"));
    assert_eq!(grouped.len(), 1);
    assert!(grouped.contains_key("forced"));
    assert_eq!(grouped["forced"].len(), 2);
}

#[test]
fn group_by_member_empty_violations() {
    let violations: Vec<ViolationItem> = vec![];
    let grouped = group_by_member(&violations, ".", None);
    assert!(grouped.is_empty());
}

#[test]
fn status_icon_ok_returns_checkmark() {
    let icon = status_icon(true);
    // When NO_COLOR is not set, should return unicode checkmark
    if std::env::var_os("NO_COLOR").is_some() {
        assert!(icon.contains("OK"));
    } else {
        assert_eq!(icon, "\u{2713}");
    }
}

#[test]
fn status_icon_fail_returns_cross() {
    let icon = status_icon(false);
    if std::env::var_os("NO_COLOR").is_some() {
        assert!(icon.contains("FAIL"));
    } else {
        assert_eq!(icon, "\u{2717}");
    }
}

#[test]
fn group_by_member_single_file_path() {
    let violations = vec![violation("src/main.rs", "AES201", 1)];

    let grouped = group_by_member(&violations, "src/main.rs", None);
    // Should have at least one group
    assert!(!grouped.is_empty());
}
