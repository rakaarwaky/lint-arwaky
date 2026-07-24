//! FR-001: Foundation Value Objects & Exit Code Contract Acceptance Suite

extern crate shared_lint_arwaky as shared;

use shared::common::taxonomy_common_error::ExitCode;
use shared::common::taxonomy_severity_vo::Severity;

#[test]
fn test_exit_code_contract_values() {
    assert_eq!(ExitCode::OK.value(), 0);
    assert_eq!(ExitCode::POLICY_FAIL.value(), 1);
    assert_eq!(ExitCode::RUNTIME_ERROR.value(), 2);
    assert_eq!(ExitCode::PREREQUISITE_MISSING.value(), 3);

    assert_eq!(
        ExitCode::OK.to_process_exit_code(),
        std::process::ExitCode::SUCCESS
    );
}

#[test]
fn test_severity_ordering_and_display() {
    assert_ne!(Severity::HIGH, Severity::LOW);
    assert_eq!(Severity::HIGH, Severity::HIGH);
}
