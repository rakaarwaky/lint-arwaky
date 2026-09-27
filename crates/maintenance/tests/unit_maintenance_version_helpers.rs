// Unit tests for MaintenanceChecker version-helper methods
use maintenance_lint_arwaky::capabilities_maintenance_checker::MaintenanceChecker;

#[test]
fn normalize_version_strips_v_prefix() {
    assert_eq!(MaintenanceChecker::normalize_version("v3.7.0"), "3.7.0");
    assert_eq!(MaintenanceChecker::normalize_version("3.7.0"), "3.7.0");
}

#[test]
fn is_newer_version_basic() {
    assert!(MaintenanceChecker::is_newer_version("3.8.0", "3.7.0"));
    assert!(!MaintenanceChecker::is_newer_version("3.7.0", "3.7.0"));
    assert!(!MaintenanceChecker::is_newer_version("3.6.0", "3.7.0"));
    assert!(MaintenanceChecker::is_newer_version("4.0.0", "3.7.0"));
    assert!(MaintenanceChecker::is_newer_version("3.7.1", "3.7.0"));
}

#[test]
fn is_newer_version_handles_different_lengths() {
    assert!(MaintenanceChecker::is_newer_version("3.7", "3.6.0"));
    assert!(!MaintenanceChecker::is_newer_version("3.7", "3.7.0"));
    assert!(MaintenanceChecker::is_newer_version("10.0.0", "9.0.0"));
}

#[test]
fn is_valid_tag_accepts_plain_versions() {
    assert!(MaintenanceChecker::is_valid_tag("v3.7.0"));
    assert!(MaintenanceChecker::is_valid_tag("3.7.0"));
    assert!(MaintenanceChecker::is_valid_tag("v3.7"));
    assert!(MaintenanceChecker::is_valid_tag("v1.0.0-alpha.1"));
}

#[test]
fn is_valid_tag_rejects_traversal_and_metacharacters() {
    // A tag that reaches a download URL and a `mv` target must never carry
    // a path separator, a parent-directory segment, or a shell metacharacter.
    assert!(!MaintenanceChecker::is_valid_tag("../../etc/passwd"));
    assert!(!MaintenanceChecker::is_valid_tag("v3.7.0/../../bin/sh"));
    assert!(!MaintenanceChecker::is_valid_tag("v3.7.0; rm -rf /"));
    assert!(!MaintenanceChecker::is_valid_tag("v3.7.0 && curl evil"));
    assert!(!MaintenanceChecker::is_valid_tag("v3.7.0$(whoami)"));
    assert!(!MaintenanceChecker::is_valid_tag("v3.7.0|nc"));
    assert!(!MaintenanceChecker::is_valid_tag(""));
    assert!(!MaintenanceChecker::is_valid_tag("v"));
    assert!(!MaintenanceChecker::is_valid_tag("v3..0"));
    assert!(!MaintenanceChecker::is_valid_tag("v3.7.0-"));
}
