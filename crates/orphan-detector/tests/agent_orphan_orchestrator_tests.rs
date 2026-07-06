use orphan_detector_lint_arwaky::agent_orphan_orchestrator::mk_orphan_result;
use shared::cli_commands::taxonomy_severity_vo::Severity;

#[test]
fn mk_orphan_result_basic() {
    let result = mk_orphan_result("src/main.rs", "test message", Severity::HIGH, "AES501");
    assert_eq!(result.code.to_string(), "AES501");
    assert_eq!(result.message.value(), "test message");
    assert_eq!(result.line.value(), 0);
    assert_eq!(result.column.value(), 0);
}

#[test]
fn mk_orphan_result_different_severities() {
    let high = mk_orphan_result("f.rs", "high", Severity::HIGH, "AES505");
    assert_eq!(high.severity as i32, Severity::HIGH as i32);

    let med = mk_orphan_result("f.rs", "med", Severity::MEDIUM, "AES503");
    assert_eq!(med.severity as i32, Severity::MEDIUM as i32);

    let low = mk_orphan_result("f.rs", "low", Severity::LOW, "AES501");
    assert_eq!(low.severity as i32, Severity::LOW as i32);
}

#[test]
fn mk_orphan_result_code_reflects_aes_rule() {
    let r = mk_orphan_result("f.rs", "o", Severity::MEDIUM, "AES506");
    assert_eq!(r.code.to_string(), "AES506");
}
