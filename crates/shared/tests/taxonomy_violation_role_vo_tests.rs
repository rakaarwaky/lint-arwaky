use shared_lint_arwaky::role_rules::taxonomy_violation_role_vo::{AesRoleViolation, LabeledRoleViolation};
use shared_lint_arwaky::code_analysis::taxonomy_violation_code_analysis_vo::Language;
use shared_lint_arwaky::taxonomy_message_vo::LintMessage;

fn labeled(v: AesRoleViolation, lang: Language) -> LabeledRoleViolation {
    v.with_language(lang)
}

#[test]
fn labeled_rust_matches_bare_display() {
    let v = AesRoleViolation::CoordinatesMultiple {
        reason: Some(LintMessage::new("custom".to_string())),
    };
    assert_eq!(v.to_string(), labeled(v.clone(), Language::Rust).to_string());
}

#[test]
fn labeled_python_uses_protocol_token() {
    let v = AesRoleViolation::ContractPrimitive {
        reason: Some(LintMessage::new("custom".to_string())),
    };
    let out = labeled(v, Language::Python).to_string();
    assert!(out.contains("Protocol"), "expected Protocol token in: {out}");
    assert!(!out.contains("trait"), "must not contain Rust trait in: {out}");
}

#[test]
fn missing_reason_uses_default_why() {
    let v = AesRoleViolation::SingleBottleneck { reason: None };
    let out = v.to_string();
    assert!(out.contains("single bottleneck"), "default WHY missing in: {out}");
    assert!(!out.contains("None"), "leaked None in: {out}");
}

#[test]
fn present_reason_overrides_default_why() {
    let v = AesRoleViolation::SingleBottleneck {
        reason: Some(LintMessage::new("auditor-custom".to_string())),
    };
    let out = v.to_string();
    assert!(out.contains("auditor-custom"), "custom WHY missing in: {out}");
}

#[test]
fn agent_file_size_limit_includes_max_lines() {
    let v = AesRoleViolation::AgentFileSizeLimit { max_lines: 250 };
    let out = v.to_string();
    assert!(out.contains("250"), "max_lines not in output: {out}");
}
