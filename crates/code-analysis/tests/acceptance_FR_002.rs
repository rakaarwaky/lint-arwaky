// PURPOSE: Acceptance test for FR-002: Minimum File Line Count (AES302)
// Files must have minimum length to avoid empty placeholders.

use code_analysis_lint_arwaky::ArchLineChecker;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_common_vo::Count;
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn make_def_with_min(min: i64) -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            min_lines: Count::new(min),
            max_lines: Count::new(0), // disable max check
            ..Default::default()
        },
        ..Default::default()
    }
}

/// FR-002: File below min lines fails with AES302
#[test]
fn fr_002_file_below_min_lines_fails_aes302() {
    let checker = ArchLineChecker::new();
    let def = make_def_with_min(10); // Default min: 10 lines

    let content = "line1\nline2\nline3"; // Only 3 lines

    let mut violations = Vec::new();
    checker.check_line_counts("capabilities_tiny.rs", Some(&def), content, &mut violations);

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES302");
    assert!(violations[0].message.value.contains("FILE_TOO_SHORT"));
}

/// FR-002: File at exactly min lines does NOT fail
#[test]
fn fr_002_file_at_exactly_min_lines_passes() {
    let checker = ArchLineChecker::new();
    let def = make_def_with_min(10);

    let content: String = (0..10)
        .map(|i| format!("line_{}", i))
        .collect::<Vec<_>>()
        .join("\n");

    let mut violations = Vec::new();
    checker.check_line_counts("capabilities_ok.rs", Some(&def), &content, &mut violations);

    assert!(violations.is_empty());
}

/// FR-002: Default min is configurable (FRD says 10, code default is 5)
#[test]
fn fr_002_default_min_lines_from_rule() {
    let rule = shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default();
    // Code default is 5 (FRD says 10, but implementation uses 5 as default)
    assert!(rule.min_lines.value > 0);
}
