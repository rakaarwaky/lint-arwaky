// Unit tests — Formatting utility tests: group_by_member, status_icon, output structure.
use shared_cli_commands::resolve_skill_hint_for_file;
use shared_cli_commands::utility_output_text_formatter::{
    code_summary_lines, extract_why_fix, group_by_member, status_icon,
};
use shared_common::ViolationItem;
use shared_common::{ColumnNumber, ErrorCode, FilePath, LineNumber, LintMessage, Severity};

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

/// The CI job that proves the external adapters ran parses this exact shape
/// with `grep -oP '^\s*\[\K[A-Za-z0-9_.\/-]+(?=\]\s+\d+\s+←)'`. Changing
/// the format without changing the workflow turns that step red with no local
/// failure, so the shape is pinned here.
#[test]
fn code_summary_lines_matches_the_shape_ci_parses() {
    let items = vec![
        violation("crates/foo/src/a.rs", "AES201", 1),
        violation("crates/foo/src/b.rs", "AES201", 9),
        violation("crates/foo/src/c.rs", "ruff.E501", 4),
    ];
    let refs: Vec<&ViolationItem> = items.iter().collect();

    let lines = code_summary_lines(&refs);
    assert_eq!(
        lines.len(),
        2,
        "one line per distinct code, however many files each touches; got {lines:#?}"
    );
    assert!(
        lines.iter().any(|l| l.starts_with("  [AES201] 2  ← ")),
        "a code touching two files counts 2; got {lines:#?}"
    );
    assert!(
        lines.iter().any(|l| l.starts_with("  [ruff.E501] 1  ← ")),
        "a tool-native code keeps its own name so CI can tell it from an AES code; got {lines:#?}"
    );
}

/// The shape CI uses, applied to a real line, must yield the code.
#[test]
fn code_summary_lines_satisfies_the_ci_regex() {
    let items = vec![violation("crates/foo/src/a.rs", "ruff.E501", 1)];
    let refs: Vec<&ViolationItem> = items.iter().collect();
    let line = code_summary_lines(&refs).remove(0);

    let parsed = regex_lite_capture(&line);
    assert_eq!(
        parsed.as_deref(),
        Some("ruff.E501"),
        "CI derives the code from `CODE] COUNT arrow`; the line {line:?} does not satisfy it"
    );
}

/// The subset of the CI regex that matters here: a code between `[` and `]`,
/// followed by a count, then the arrow.
fn regex_lite_capture(line: &str) -> Option<String> {
    let rest = line.trim_start().strip_prefix('[')?;
    let end = rest.find(']')?;
    let code = &rest[..end];
    let tail = rest[end + 1..].trim_start();
    let count_end = tail.find(char::is_whitespace)?;
    tail[count_end..]
        .trim_start()
        .starts_with('←')
        .then(|| code.to_string())
}

// ─── extract_why_fix tests ─────────────────────────────────────

#[test]
fn extract_why_fix_parses_aes_code_message() {
    let msg = "AES403 CAPABILITIES_PURITY: no orchestration allowed.\nWHY? Line 12 of foo.rs declares orchestration.\nHOW TO FIX? Move orchestration to agent layer.";
    let (why, fix) = extract_why_fix(msg);
    assert!(why.is_some(), "WHY should be extracted");
    assert!(fix.is_some(), "FIX should be extracted");
    assert!(why.unwrap().contains("orchestration"));
    assert!(fix.unwrap().contains("agent layer"));
}

#[test]
fn extract_why_fix_parses_orphan_code_message() {
    let msg = "AES505 AGENT_ORPHAN: 'foo' is not reachable.\nWHY? Agent file 'foo' is not reachable from any _entry file.\nFIX: Import 'foo' from a _entry file AND register it in a root_*_container.";
    let (why, fix) = extract_why_fix(msg);
    assert!(why.is_some());
    assert!(fix.is_some());
    assert!(why.unwrap().contains("not reachable"));
    assert!(fix.unwrap().contains("_entry file"));
}

#[test]
fn extract_why_fix_returns_none_for_external_code() {
    let msg = "F841 local variable assigned but never used";
    let (why, fix) = extract_why_fix(msg);
    assert!(why.is_none());
    assert!(fix.is_none());
}

#[test]
fn extract_why_fix_handles_why_colon_prefix() {
    let msg = "AES501 taxonomy file not reachable.\nWHY: file not wired.\nFIX: register in root container.";
    let (why, fix) = extract_why_fix(msg);
    assert_eq!(why.unwrap(), "file not wired.");
    assert_eq!(fix.unwrap(), "register in root container.");
}

#[test]
fn extract_why_fix_no_invented_text_for_plain_message() {
    let msg = "markdownlint_MD022 heading without blank line";
    let (why, fix) = extract_why_fix(msg);
    assert!(why.is_none());
    assert!(fix.is_none());
}
