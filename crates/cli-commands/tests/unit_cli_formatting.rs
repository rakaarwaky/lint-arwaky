// Unit tests — Formatting utility tests: group_by_member, status_icon, output structure.
use cli_commands::utility_output_text_formatter::{group_by_member, status_icon};
use shared_common::{ColumnNumber, ErrorCode, FilePath, LineNumber, LintMessage, Severity};
use shared_common::{ViolationItem, resolve_skill_hint_for_file};

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

#[test]
fn skill_hint_available_for_every_violation_in_a_scan() {
    let violations = vec![
        violation("crates/foo/contract_scan_protocol.rs", "AES201", 1),
        violation("crates/foo/agent_scan_orchestrator.rs", "AES403", 2),
        violation("crates/foo/utility_path_resolver.rs", "AES304", 3),
        violation("crates/foo/taxonomy_project_setup_vo.rs", "AES101", 4),
    ];

    for v in &violations {
        let hint = resolve_skill_hint_for_file(v.code.code(), &v.file.value);
        assert!(
            !hint.guidance().is_empty(),
            "empty guidance for {}",
            v.code.code()
        );
    }
}

#[test]
fn skill_hint_follows_file_layer_not_aes_code() {
    // AES201 in a utility-layer file routes to aes-utility, not aes-contract.
    let hint = resolve_skill_hint_for_file("AES201", "crates/foo/utility_path_resolver.rs");
    assert_eq!(hint.skill, Some("aes-utility"));
    assert_eq!(
        hint.guidance(),
        "[run cli \"lint-arwaky-cli skill read aes-utility\"]"
    );
}

#[test]
fn aes403_in_agent_file_routes_to_agent_skill() {
    let hint = resolve_skill_hint_for_file("AES403", "crates/foo/agent_scan_orchestrator.rs");
    assert_eq!(hint.skill, Some("aes-agent"));
    assert_eq!(
        hint.guidance(),
        "[run cli \"lint-arwaky-cli skill read aes-agent\"]"
    );
}
