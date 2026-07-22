// PURPOSE: Unit tests for TaxonomyRoleChecker (AES401) — entity/error/event/primitive checks.
// Layer: Capabilities (TaxonomyRoleChecker)

use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::role_rules::contract_taxonomy_role_protocol::ITaxonomyRoleChecker;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn checker() -> TaxonomyRoleChecker {
    TaxonomyRoleChecker::new()
}

fn make_source(file: &str, content: &str) -> SourceContentVO {
    let fp = shared::common::taxonomy_path_vo::FilePath::new(file.to_string()).unwrap();
    let cs = ContentString::new(content.to_string());
    SourceContentVO::new(fp, cs, "rust")
}

// ─── check_entity: Happy Path ────────────────────────

#[test]
fn entity_file_with_struct_passes() {
    let content = "pub struct UserEntity {\n    name: String,\n}";
    let source = make_source("taxonomy_user_entity.rs", content);
    let mut violations = Vec::new();
    checker().check_entity(&source, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn entity_file_with_enum_passes() {
    let content = "pub enum Status {\n    Active,\n    Inactive,\n}";
    let source = make_source("taxonomy_status_entity.rs", content);
    let mut violations = Vec::new();
    checker().check_entity(&source, &mut violations);
    assert!(violations.is_empty());
}

// ─── check_entity: AES401 Violation ──────────────────

#[test]
fn entity_file_with_primitive_flagged() {
    let content = "pub type UserId = i32;";
    let source = make_source("taxonomy_user_id_entity.rs", content);
    let mut violations = Vec::new();
    checker().check_entity(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES401");
}

#[test]
fn entity_file_with_string_flagged() {
    let content = "pub type Name = String;";
    let source = make_source("taxonomy_name_entity.rs", content);
    let mut violations = Vec::new();
    checker().check_entity(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES401");
}

// ─── check_error: Happy Path ────────────────────────

#[test]
fn error_file_with_struct_passes() {
    let content = "pub struct AppError {\n    message: String,\n}";
    let source = make_source("taxonomy_app_error.rs", content);
    let mut violations = Vec::new();
    checker().check_error(&source, &mut violations);
    assert!(violations.is_empty());
}

// ─── check_error: AES401 Violation ──────────────────

#[test]
fn error_file_with_primitive_flagged() {
    let content = "pub type ErrorCode = i32;";
    let source = make_source("taxonomy_error_code_entity.rs", content);
    let mut violations = Vec::new();
    checker().check_error(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES401");
}

// ─── check_event: Happy Path ────────────────────────

#[test]
fn event_file_with_struct_passes() {
    let content = "pub struct UserCreatedEvent {\n    user_id: i32,\n}";
    let source = make_source("taxonomy_user_created_event.rs", content);
    let mut violations = Vec::new();
    checker().check_event(&source, &mut violations);
    assert!(violations.is_empty());
}

// ─── check_event: AES401 Violation ──────────────────

#[test]
fn event_file_with_primitive_flagged() {
    let content = "pub type EventId = String;";
    let source = make_source("taxonomy_event_id_entity.rs", content);
    let mut violations = Vec::new();
    checker().check_event(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES401");
}

// ─── check_constant: Happy Path ─────────────────────

#[test]
fn constant_file_with_const_passes() {
    let content = "pub const MAX_RETRIES: usize = 3;";
    let source = make_source("taxonomy_constants.rs", content);
    let mut violations = Vec::new();
    checker().check_constant(&source, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn constant_file_with_fn_passes() {
    let content = "pub fn helper() -> i32 { 42 }";
    let source = make_source("taxonomy_constants.rs", content);
    let mut violations = Vec::new();
    checker().check_constant(&source, &mut violations);
    assert!(violations.is_empty());
}

// ─── check_constant: AES401 Violation ──────────────

#[test]
fn constant_file_with_struct_flagged() {
    let content = "pub struct Constants;\nimpl Constants { pub fn max() -> i32 { 10 } }";
    let source = make_source("taxonomy_constants.rs", content);
    let mut violations = Vec::new();
    checker().check_constant(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES401");
}

#[test]
fn constant_file_with_enum_flagged() {
    let content = "pub enum Constants {\n    MaxRetries,\n}";
    let source = make_source("taxonomy_constants.rs", content);
    let mut violations = Vec::new();
    checker().check_constant(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES401");
}

// ─── check_vo (no-op) ──────────────────────────────

#[test]
fn check_vo_returns_empty() {
    let source = make_source("taxonomy_foo_vo.rs", "pub struct Foo;");
    let violations = checker().check_vo();
    assert!(violations.is_empty());
}

// ─── Default trait ──────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _ = TaxonomyRoleChecker::default();
}
