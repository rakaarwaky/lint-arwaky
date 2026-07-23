// PURPOSE: Unit tests for UtilityRoleChecker — TypeScript AES404 checks

use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::{ContentString, SourceContentVO};
use shared::role_rules::contract_utility_role_protocol::IUtilityRoleChecker;

fn make_ts_source(content: &str) -> SourceContentVO {
    SourceContentVO::new(
        FilePath::new("utility_helper.ts".to_string()).unwrap_or_default(),
        ContentString::new(content.to_string()),
        "ts",
    )
}

// ─── TypeScript: class/interface/enum/type in comments → NO violation ──

#[test]
fn ts_class_in_comment_should_not_flag() {
    let content = r#"
// export class CommentedClass { }
/* export interface CommentedInterface { } */

export function helper(): void {}
"#;

    let source = make_ts_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        violations.is_empty(),
        "Commented TS definitions should not be flagged"
    );
}

// ─── TypeScript: template literal → NO violation ──

#[test]
fn ts_template_literal_should_not_flag() {
    let content = r#"
const greeting = `export class TemplateClass { }`;
const msg = "hello world";

export function greet(): void {}
"#;

    let source = make_ts_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        violations.is_empty(),
        "Template literals should not be flagged"
    );
}

// ─── TypeScript: export class → violation ──

#[test]
fn ts_export_class_should_flag() {
    let content = r#"
export class BadUtility {
    public value: string;
}

export function helper(): void {}
"#;

    let source = make_ts_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        !violations.is_empty(),
        "Export class in utility should be flagged"
    );
    assert_eq!(violations[0].code.code(), "AES404");
}

// ─── TypeScript: export interface → violation ──

#[test]
fn ts_export_interface_should_flag() {
    let content = r#"
export interface BadInterface {
    value: string;
}

export function helper(): void {}
"#;

    let source = make_ts_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        !violations.is_empty(),
        "Export interface in utility should be flagged"
    );
    assert_eq!(violations[0].code.code(), "AES404");
}

// ─── TypeScript: export enum → violation ──

#[test]
fn ts_export_enum_should_flag() {
    let content = r#"
export enum BadEnum {
    A,
    B,
}

export function helper(): void {}
"#;

    let source = make_ts_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        !violations.is_empty(),
        "Export enum in utility should be flagged"
    );
    assert_eq!(violations[0].code.code(), "AES404");
}

// ─── TypeScript: export type → violation ──

#[test]
fn ts_export_type_should_flag() {
    let content = r#"
export type BadType = string | number;

export function helper(): void {}
"#;

    let source = make_ts_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        !violations.is_empty(),
        "Export type in utility should be flagged"
    );
    assert_eq!(violations[0].code.code(), "AES404");
}

// ─── TypeScript: pure functions → NO violation ──

#[test]
fn ts_pure_functions_should_not_flag() {
    let content = r#"
export function add(a: number, b: number): number {
    return a + b;
}

export function multiply(a: number, b: number): number {
    return a * b;
}
"#;

    let source = make_ts_source(content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        violations.is_empty(),
        "Pure TS functions should not be flagged"
    );
}
