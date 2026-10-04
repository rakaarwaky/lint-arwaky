// Unit tests — Formatting utility tests: group_by_member, status_icon, output structure.
use shared_cli_commands::resolve_skill_hint_for_file;
use shared_cli_commands::utility_output_text_formatter::{
    group_by_member, hierarchy_key, status_icon,
};
use shared_cli_commands::{ScanScope, classify_scan_scope};
use shared_common::ViolationItem;
use shared_common::{
    ColumnNumber, ErrorCode, FilePath, LineNumber, LintMessage, LintResult, Severity,
};

fn violation(file: &str, code: &str, line: i64) -> ViolationItem {
    ViolationItem {
        code: ErrorCode::raw(code),
        file: FilePath::new(file.to_string()).unwrap(),
        line: LineNumber::new(line),
        column: ColumnNumber::new(0),
        message: LintMessage::new(format!("{code} at {file}:{line}")),
        severity: Severity::HIGH,
        violation_name: String::new(),
        why: String::new(),
        fix: String::new(),
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
// ─── violation message parser ────────────────────────────────
//
// The parser is the backward-compat path: capabilities now populate the
// structured `violation_name` / `why` / `fix` fields, and anything that has
// not been converted yet still carries the text embedded in `message`.

/// A capability message keeps its name when the structured field is empty.
#[test]
fn from_lint_result_recovers_name_why_fix_from_the_message() {
    let result = LintResult::new_arch(
        "crates/foo/src/a.rs",
        12,
        "AES403",
        Severity::HIGH,
        "AES403 CAPABILITIES_PURITY: orchestration in a capability.\n\
         WHY: line 12 declares orchestration.\n\
         FIX: move orchestration to the agent layer.",
    );
    let item = ViolationItem::from_lint_result(&result);

    assert_eq!(item.violation_name, "CAPABILITIES_PURITY");
    assert_eq!(item.why, "line 12 declares orchestration.");
    assert_eq!(item.fix, "move orchestration to the agent layer.");
}

/// An explicit field wins over whatever the message happens to contain.
#[test]
fn from_lint_result_prefers_the_structured_fields() {
    let result = LintResult::new_arch_with_name(
        "crates/foo/src/a.rs",
        12,
        "AES403",
        Severity::HIGH,
        "AES403 WRONG_NAME: something.\nWHY: stale why.\nFIX: stale fix.",
        "CAPABILITIES_PURITY",
        "the real reason",
        "the real fix",
    );
    let item = ViolationItem::from_lint_result(&result);

    assert_eq!(item.violation_name, "CAPABILITIES_PURITY");
    assert_eq!(item.why, "the real reason");
    assert_eq!(item.fix, "the real fix");
}

/// The legacy `WHY?` marker still parses, so an unconverted capability keeps
/// its guidance.
#[test]
fn from_lint_result_accepts_the_legacy_why_marker() {
    let result = LintResult::new_arch(
        "crates/foo/src/a.rs",
        3,
        "AES505",
        Severity::HIGH,
        "AES505 AGENT_ORPHAN: 'foo' is not reachable.\n\
         WHY? not reachable from any entry file.\n\
         HOW TO FIX? import it from an entry file.",
    );
    let item = ViolationItem::from_lint_result(&result);

    assert_eq!(item.violation_name, "AGENT_ORPHAN");
    assert_eq!(item.why, "not reachable from any entry file.");
    assert_eq!(item.fix, "import it from an entry file.");
}

/// A tool-native message gets no invented name — the code alone is the label.
#[test]
fn from_lint_result_invents_nothing_for_a_tool_message() {
    let result = LintResult::new_arch(
        "packages/app/src/a.py",
        7,
        "F841",
        Severity::HIGH,
        "local variable 'x' is assigned but never used",
    );
    let item = ViolationItem::from_lint_result(&result);

    assert!(item.violation_name.is_empty(), "no name to invent");
    assert!(item.why.is_empty(), "no WHY to invent");
    assert!(item.fix.is_empty(), "no FIX to invent");
}

/// A tool code that itself contains a colon must not leak message prose into
/// the name slot: `markdownlint::MD022` splits on its own prefix.
#[test]
fn a_colon_in_the_tool_code_does_not_leak_into_the_name() {
    let result = LintResult::new_arch(
        "crates/doc/README.md",
        1,
        "markdownlint::MD022",
        Severity::HIGH,
        "Headings should be surrounded by blank lines [Expected: 1; Actual: 2]",
    );
    let item = ViolationItem::from_lint_result(&result);

    assert!(
        item.violation_name.is_empty(),
        "message prose must not be read as a name; got {:?}",
        item.violation_name
    );
}

/// Lowercase prose is a description, never a violation name.
#[test]
fn lowercase_prose_is_not_a_violation_name() {
    let result = LintResult::new_arch(
        "crates/foo/src/a.rs",
        1,
        "AES101",
        Severity::HIGH,
        "AES101 the file is too short to review",
    );
    let item = ViolationItem::from_lint_result(&result);

    assert!(item.violation_name.is_empty());
}

// ─── report hierarchy ────────────────────────────────────────
//
// Each level has its own bracket — `{top}`, `[member]`, `(file)` — so a
// reader can tell a member folder from a file by shape alone. A file landing
// in the member slot would print `[report.md]` and read as a folder that does
// not exist.

/// The normal shape: member dir, member, file path.
#[test]
fn a_member_file_nests_under_its_member() {
    let (top, member, file) = hierarchy_key("crates/shared_common/src/agent_foo.rs");
    assert_eq!(
        (top.as_str(), member.as_str(), file.as_str()),
        ("crates", "shared_common", "src/agent_foo.rs")
    );
}

/// A file sitting directly in a member dir has no member level of its own.
///
/// `crates/root_violation.rs` is a file; `crates/shared_common` is a folder.
#[test]
fn a_flat_member_file_gets_no_member_heading() {
    let (top, member, file) = hierarchy_key("crates/root_violation.rs");
    assert_eq!(top, "crates");
    assert_eq!(member, "", "a file must not fill the member slot");
    assert_eq!(file, "root_violation.rs");
}

/// A finding about a member folder itself keeps that folder as the member.
#[test]
fn a_member_folder_finding_keeps_the_member_heading() {
    let (top, member, file) = hierarchy_key("crates/shared_common");
    assert_eq!(top, "crates");
    assert_eq!(member, "shared_common");
    assert_eq!(file, "shared_common");
}

/// A flat document at the target root reports under `root`, with no member.
#[test]
fn a_target_root_file_reports_under_root() {
    let (top, member, file) = hierarchy_key("BACKLOG.md");
    assert_eq!(top, "root");
    assert_eq!(member, "", "a flat document must not fill the member slot");
    assert_eq!(file, "BACKLOG.md");
}

/// An empty relative path still yields the three slots.
#[test]
fn an_empty_path_yields_placeholder_levels() {
    assert_eq!(
        hierarchy_key(""),
        ("root".to_string(), "root".to_string(), String::new())
    );
}

// ─── scan scope ──────────────────────────────────────────────

/// The four scopes the CLI resolves a user path into.
#[test]
fn a_workspace_root_scans_everything() {
    let scope = classify_scan_scope("workspaces-bad", "workspaces-bad");
    assert_eq!(scope, ScanScope::Workspace);
    assert!(scope.filter_prefix().is_none(), "nothing to narrow");
}

/// `crates` alone means that member dir only.
#[test]
fn a_member_dir_scans_only_that_member() {
    let scope = classify_scan_scope("workspaces-bad", "workspaces-bad/crates");
    assert_eq!(
        scope,
        ScanScope::TopLevel {
            member: "crates".to_string()
        }
    );
    assert!(
        scope.filter_prefix().is_none(),
        "the member dir is the scope"
    );
}

/// A subfolder widens the scan to the member dir and narrows the report.
#[test]
fn a_subfolder_scans_the_whole_member_but_reports_the_subfolder() {
    let scope = classify_scan_scope("workspaces-bad", "workspaces-bad/crates/import-rules");
    assert_eq!(
        scope,
        ScanScope::Subfolder {
            scan_root: "workspaces-bad/crates".to_string(),
            filter_to: "workspaces-bad/crates/import-rules".to_string(),
        }
    );
    assert_eq!(
        scope.scan_root(),
        "workspaces-bad/crates",
        "the sibling files the architecture rules read must still be scanned"
    );
    assert_eq!(
        scope.filter_prefix(),
        Some("workspaces-bad/crates/import-rules")
    );
}

/// A single file widens the scan to the member dir and narrows to that file.
#[test]
fn a_single_file_scans_the_whole_member_but_reports_one_file() {
    // Classification reads the filesystem, so the fixture must exist.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../workspaces-bad")
        .canonicalize()
        .expect("the bad-workspace fixture is part of the repo");
    let file = root.join("crates/utility_orphan_test/src/utility_dead_code.rs");
    let scope = classify_scan_scope(&root.to_string_lossy(), &file.to_string_lossy());
    match &scope {
        ScanScope::SingleFile {
            scan_root,
            filter_to,
        } => {
            assert_eq!(
                std::path::Path::new(scan_root),
                root.join("crates"),
                "scan covers the whole member dir, not just the subfolder"
            );
            assert_eq!(
                std::path::Path::new(filter_to),
                file,
                "report narrows to the one file"
            );
        }
        other => panic!("expected a single-file scope, got {other:?}"),
    }
}
