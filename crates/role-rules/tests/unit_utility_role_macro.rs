// PURPOSE: Unit tests for UtilityRoleChecker — AES404 macro detection
// TDD: Test that macros are NOT flagged as struct/enum definitions

use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::{ContentString, SourceContentVO};
use shared::role_rules::contract_utility_role_protocol::IUtilityRoleChecker;

fn make_source(content: &str) -> SourceContentVO {
    SourceContentVO::new(
        FilePath::new("utility_value_object_generator.rs".to_string()).unwrap_or_default(),
        ContentString::new(content.to_string()),
        "rust",
    )
}

#[test]
fn test_utility_with_macro_defining_struct_should_not_flag_aes404() {
    // RED: This test should FAIL because current implementation
    // detects "pub struct" inside macro body as a violation
    let content = r#"
// PURPOSE: Macros for generating boilerplate impls on String/primitive wrapper value objects.

/// Generate a String-wrapped value object with the standard VO surface.
#[macro_export]
macro_rules! string_value_object {
    ($name:ident) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: String,
        }

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self {
                    value: value.into(),
                }
            }
        }
    };
}

/// Generate a primitive-wrapped value object.
#[macro_export]
macro_rules! primitive_value_object {
    ($name:ident, $inner:ty) => {
        #[derive(Default, Debug, Clone, serde::Serialize)]
        #[serde(transparent)]
        pub struct $name {
            pub value: $inner,
        }
    };
}
"#;

    let source = make_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);

    // Should NOT have any AES404 violations because this is a macro file
    assert!(
        violations.is_empty(),
        "Macro file should not be flagged as AES404 violation, but got {} violations",
        violations.len()
    );
}

#[test]
fn test_utility_with_actual_struct_should_flag_aes404() {
    // This test should PASS — actual struct definitions should be flagged
    let content = r#"
// PURPOSE: Some utility that incorrectly defines a struct

pub struct BadUtility {
    pub value: String,
}

pub fn some_function() -> String {
    "hello".to_string()
}
"#;

    let source = make_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);

    // Should have AES404 violation
    assert!(
        !violations.is_empty(),
        "Utility with actual struct should be flagged as AES404"
    );
    assert_eq!(violations[0].code.code(), "AES404");
}

#[test]
fn test_utility_with_actual_enum_should_flag_aes404() {
    // This test should PASS — actual enum definitions should be flagged
    let content = r#"
// PURPOSE: Some utility that incorrectly defines an enum

pub enum BadEnum {
    Variant1,
    Variant2,
}

pub fn some_function() -> String {
    "hello".to_string()
}
"#;

    let source = make_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);

    // Should have AES404 violation
    assert!(
        !violations.is_empty(),
        "Utility with actual enum should be flagged as AES404"
    );
    assert_eq!(violations[0].code.code(), "AES404");
}

#[test]
fn test_utility_with_commented_struct_should_not_flag_aes404() {
    // This test should PASS — commented code should not be flagged
    let content = r#"
// PURPOSE: Some utility with commented code

// pub struct CommentedStruct {
//     pub value: String,
// }

pub fn some_function() -> String {
    "hello".to_string()
}
"#;

    let source = make_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);

    // Should NOT have AES404 violations because struct is commented out
    assert!(
        violations.is_empty(),
        "Commented struct should not be flagged as AES404"
    );
}

#[test]
fn test_utility_with_only_functions_should_not_flag_aes404() {
    // This test should PASS — pure functions are correct for utility
    let content = r#"
// PURPOSE: Pure utility functions

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
"#;

    let source = make_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);

    // Should NOT have any AES404 violations
    assert!(
        violations.is_empty(),
        "Pure functions should not be flagged as AES404"
    );
}
