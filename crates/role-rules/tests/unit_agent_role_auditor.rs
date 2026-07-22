// PURPOSE: Unit tests for AgentRoleChecker (AES405) — file size limit and any-type annotation.
// Layer: Capabilities (AgentRoleChecker)

use role_rules_lint_arwaky::capabilities_agent_role_auditor::AgentRoleChecker;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::common::taxonomy_path_vo::FilePath;
use shared::role_rules::contract_agent_role_protocol::IAgentRoleChecker;
use shared::role_rules::taxonomy_violation_role_vo::AesRoleViolation;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn checker() -> AgentRoleChecker {
    AgentRoleChecker::new()
}

fn make_source(file: &str, content: &str) -> SourceContentVO {
    let fp = FilePath::new(file.to_string()).unwrap();
    let cs = ContentString::new(content.to_string());
    SourceContentVO::new(fp, cs, "rust")
}

// ─── check_file_size_limit: Happy Path ──────────────

#[test]
fn file_under_max_lines_no_violation() {
    let content: String = (0..100).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    let source = make_source("agent_foo_orchestrator.rs", &content);
    let mut violations = Vec::new();
    checker().check_file_size_limit(&source, 500, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn file_at_max_lines_no_violation() {
    let content: String = (0..499).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    let source = make_source("agent_foo_orchestrator.rs", &content);
    let mut violations = Vec::new();
    checker().check_file_size_limit(&source, 500, &mut violations);
    assert!(violations.is_empty());
}

// ─── check_file_size_limit: AES405 Violation ──────────

#[test]
fn file_over_max_lines_emits_aes405() {
    let content: String = (0..501).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    let source = make_source("agent_foo_orchestrator.rs", &content);
    let mut violations = Vec::new();
    checker().check_file_size_limit(&source, 500, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES405");
}

#[test]
fn file_over_max_lines_violation_has_correct_message() {
    let content: String = (0..600).map(|i| format!("line {}", i)).collect::<Vec<_>>().join("\n");
    let source = make_source("agent_foo_orchestrator.rs", &content);
    let mut violations = Vec::new();
    checker().check_file_size_limit(&source, 500, &mut violations);
    assert!(violations[0].message.value.contains("FILE_TOO_LARGE"));
}

// ─── check_any_type_annotation: Happy Path ──────────

#[test]
fn no_any_annotation_no_violation() {
    let content = "fn main() {\n    let x: i32 = 42;\n}";
    let source = make_source("agent_main.rs", content);
    let mut violations = Vec::new();
    checker().check_any_type_annotation(&source, &mut violations);
    assert!(violations.is_empty());
}

// ─── check_any_type_annotation: AES405 Violation ─────

#[test]
fn rust_any_annotation_flagged() {
    let content = "let x: Option<i32> = Some(1);\nlet y: Box<dyn Any> = Box::new(42);";
    let source = make_source("agent_foo.rs", content);
    let mut violations = Vec::new();
    checker().check_any_type_annotation(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES405");
}

#[test]
fn python_any_annotation_flagged() {
    let source = make_source("agent_foo.py", "x: Any = something");
    let mut violations = Vec::new();
    checker().check_any_type_annotation(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES405");
}

#[test]
fn typescript_any_annotation_flagged() {
    let source = make_source("agent_foo.ts", "let x: any = 'hello';");
    let mut violations = Vec::new();
    checker().check_any_type_annotation(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES405");
}

// ─── check_container / orchestrator / lifecycle (no-op) ──

#[test]
fn check_container_no_violation() {
    let source = make_source("agent_foo_orchestrator.rs", "pub struct Foo;");
    let mut violations = Vec::new();
    checker().check_container(&source, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn check_orchestrator_no_violation() {
    let source = make_source("agent_foo_orchestrator.rs", "pub struct Foo;");
    let mut violations = Vec::new();
    checker().check_orchestrator(&source, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn check_lifecycle_no_violation() {
    let source = make_source("agent_foo_lifecycle.rs", "pub struct Foo;");
    let mut violations = Vec::new();
    checker().check_lifecycle(&source, &mut violations);
    assert!(violations.is_empty());
}

// ─── Default trait ──────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _ = AgentRoleChecker::default();
}
