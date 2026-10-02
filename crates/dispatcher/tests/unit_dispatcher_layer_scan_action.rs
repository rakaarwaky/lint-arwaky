// Unit tests — layer-scoped scan narrowing.
//
// A layer command (`taxonomy`, `contract`, `capabilities`, `utility`, `agents`,
// `surface`) must report the violations whose file carries that layer's AES
// name prefix and nothing else. The filter is the whole contract, so it is
// asserted directly against synthetic violations — one per layer, plus the
// findings that belong to no layer at all (a folder-level structure finding, a
// doc invariant) and the `root` layer, which has no subcommand.

use dispatcher_lint_arwaky::surface_layer_scan_action::retain_violations_for_layer;
use shared_common::taxonomy_violation_item_vo::ViolationItem;
use shared_common::{ColumnNumber, ErrorCode, FilePath, LineNumber, LintMessage, Severity};

fn violation(code: &str, file: &str) -> ViolationItem {
    ViolationItem {
        code: ErrorCode::raw(code.to_string()),
        file: FilePath::new(file.to_string()).expect("non-empty path"),
        line: LineNumber::new(1),
        column: ColumnNumber::new(1),
        message: LintMessage::new("finding"),
        severity: Severity::HIGH,
    }
}

fn codes(violations: &[ViolationItem]) -> Vec<String> {
    violations
        .iter()
        .map(|v| v.code.code().to_string())
        .collect()
}

/// One violation per layer, each in a file whose name carries that layer's
/// prefix — the shape a full scan produces.
fn one_per_layer() -> Vec<ViolationItem> {
    vec![
        violation("AES101", "crates/shared/src/common/taxonomy_path_vo.rs"),
        violation(
            "AES201",
            "crates/shared/src/role_rules/contract_role_protocol.rs",
        ),
        violation(
            "AES301",
            "crates/quality-rules/src/capabilities_bypass_checker.rs",
        ),
        violation(
            "AES401",
            "crates/shared/src/role_rules/utility_role_checker.rs",
        ),
        violation(
            "AES501",
            "crates/orphan-rules/src/agent_orphan_orchestrator.rs",
        ),
        violation("AES502", "crates/dispatcher/src/surface_check_action.rs"),
        violation("AES701", "crates/root_cli_main_entry.rs"),
    ]
}

#[test]
fn each_layer_command_keeps_only_its_own_findings() {
    let cases = [
        ("taxonomy", "crates/shared/src/common/taxonomy_path_vo.rs"),
        (
            "contract",
            "crates/shared/src/role_rules/contract_role_protocol.rs",
        ),
        (
            "capabilities",
            "crates/quality-rules/src/capabilities_bypass_checker.rs",
        ),
        (
            "utility",
            "crates/shared/src/role_rules/utility_role_checker.rs",
        ),
        (
            "agent",
            "crates/orphan-rules/src/agent_orphan_orchestrator.rs",
        ),
        ("surfaces", "crates/dispatcher/src/surface_check_action.rs"),
    ];
    for (layer, expected_file) in cases {
        let kept = retain_violations_for_layer(one_per_layer(), layer);
        assert_eq!(
            codes(&kept),
            vec![kept[0].code.code().to_string()],
            "{layer} must keep exactly one finding"
        );
        assert_eq!(
            kept[0].file.value(),
            expected_file,
            "{layer} must keep the finding filed under its own layer"
        );
    }
}

#[test]
fn root_layer_has_no_subcommand_and_is_reported_by_scan_alone() {
    let kept = retain_violations_for_layer(one_per_layer(), "root");
    assert_eq!(codes(&kept), vec!["AES701".to_string()]);
    assert_eq!(kept[0].file.value(), "crates/root_cli_main_entry.rs");
}

#[test]
fn a_layer_command_never_reports_a_finding_from_another_layer() {
    let kept = retain_violations_for_layer(one_per_layer(), "taxonomy");
    for finding in &kept {
        assert!(
            finding.file.value().contains("taxonomy_"),
            "taxonomy must not report {}",
            finding.file.value()
        );
    }
}

/// Structure findings about a folder and doc findings about a Markdown file
/// name no layer-prefixed file, so no layer command owns them — `structure` and
/// `docs` stay the commands that report those.
#[test]
fn findings_outside_every_layer_are_dropped_by_layer_commands() {
    let mixed = vec![
        violation("AES702", "crates/feature_module/src"),
        violation("AES605", "ROADMAP.md"),
        violation(
            "AES502",
            "crates/shared/src/orphan_rules/contract_orphan_rules_vo.rs",
        ),
    ];
    for layer in [
        "taxonomy",
        "contract",
        "capabilities",
        "utility",
        "agent",
        "surfaces",
        "root",
    ] {
        let kept = retain_violations_for_layer(mixed.clone(), layer);
        let expected: Vec<String> = if layer == "contract" {
            vec!["AES502".to_string()]
        } else {
            Vec::new()
        };
        assert_eq!(codes(&kept), expected, "layer {layer}");
    }
}

#[test]
fn an_empty_scan_stays_empty_for_every_layer() {
    for layer in [
        "taxonomy",
        "contract",
        "capabilities",
        "utility",
        "agent",
        "surfaces",
    ] {
        assert!(
            retain_violations_for_layer(Vec::new(), layer).is_empty(),
            "layer {layer}"
        );
    }
}
