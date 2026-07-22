// PURPOSE: Acceptance test for FR-001: Maximum File Line Count (AES301)
// Files must not exceed the maximum allowed line count.

use code_analysis_lint_arwaky::ArchLineChecker;
use shared::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use shared::common::taxonomy_common_vo::Count;
use shared::common::taxonomy_definition_vo::LayerDefinition;

fn make_def_with_max(max: i64) -> LayerDefinition {
    LayerDefinition {
        code_analysis: shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO {
            max_lines: Count::new(max),
            min_lines: Count::new(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// FR-001: File exceeding max lines fails with AES301
#[test]
fn fr_001_file_exceeding_max_lines_fails_aes301() {
    let checker = ArchLineChecker::new();
    let def = make_def_with_max(1000); // Default max: 1000 lines

    // Generate 1001 lines
    let content: String = (0..1001)
        .map(|i| format!("let line_{} = {};", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let mut violations = Vec::new();
    checker.check_line_counts(
        "capabilities_large.rs",
        Some(&def),
        &content,
        &mut violations,
    );

    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES301");
    assert!(violations[0].message.value.contains("FILE_TOO_LARGE"));
    assert!(violations[0].message.value.contains("1000"));
}

/// FR-001: File at exactly max lines does NOT fail
#[test]
fn fr_001_file_at_exactly_max_lines_passes() {
    let checker = ArchLineChecker::new();
    let def = make_def_with_max(1000);

    let content: String = (0..1000)
        .map(|i| format!("let line_{} = {};", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let mut violations = Vec::new();
    checker.check_line_counts(
        "capabilities_exact.rs",
        Some(&def),
        &content,
        &mut violations,
    );

    assert!(violations.is_empty());
}

/// FR-001: Default max is 1000 lines (configurable per rule)
#[test]
fn fr_001_default_max_is_1000() {
    let rule = shared::code_analysis::taxonomy_code_analysis_rule_vo::CodeAnalysisRuleVO::default();
    assert_eq!(rule.max_lines.value, 1000);
}
