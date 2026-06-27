use shared_lint_arwaky::common::taxonomy_message_vo::{ComplianceStatus, LintMessage};

// ---------------------------------------------------------------------------
// LintMessage
// ---------------------------------------------------------------------------

#[test]
fn lint_message_new_string() {
    let msg = LintMessage::new("test error message".to_string());
    assert!(msg.value().contains("error"));
}

#[test]
fn lint_message_new_display() {
    let msg = LintMessage::new("violation at line 42".to_string());
    assert_eq!(format!("{}", msg), "violation at line 42");
}

#[test]
fn lint_message_empty() {
    let msg = LintMessage::new("".to_string());
    assert_eq!(msg.value(), "");
}

#[test]
fn lint_message_clone() {
    let a = LintMessage::new("hello".to_string());
    let b = a.clone();
    assert_eq!(a.value(), b.value());
}

// ---------------------------------------------------------------------------
// ComplianceStatus
// ---------------------------------------------------------------------------

#[test]
fn compliance_status_true() {
    let s = ComplianceStatus::new(true);
    assert!(s.value());
}

#[test]
fn compliance_status_false() {
    let s = ComplianceStatus::new(false);
    assert!(!s.value());
}

#[test]
fn compliance_status_default() {
    let s = ComplianceStatus::default();
    // Default is true based on the implementation
    assert!(s.value());
}
