use role_rules_lint_arwaky::capabilities_taxonomy_role_auditor::TaxonomyRoleChecker;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn make_source(file: &str, content: &str, language: &str) -> SourceContentVO {
    let fp = FilePath::new(file.to_string()).unwrap_or_default();
    SourceContentVO::new(fp, ContentString::new(content.to_string()), language)
}

// ─── check_entity ───────────────────────────────────────────────────────────

#[test]
fn entity_with_primitive_fields_emits_violations() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_user_entity.rs",
        "pub struct UserEntity {\n    pub name: String,\n    pub age: i32,\n}",
        "rust",
    );
    checker.check_entity(&src, &mut violations);
    assert!(!violations.is_empty());
    assert!(violations.iter().all(|v| v.code.to_string().contains("AES401")));
}

#[test]
fn entity_skips_newtype_value_field() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_name_entity.rs",
        "pub struct NameEntity(pub(crate) value: String);",
        "rust",
    );
    checker.check_entity(&src, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn entity_skips_from_impl() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_foo_entity.rs",
        "impl From<String> for FooEntity {\n    fn from(s: String) -> Self { todo!() }\n}",
        "rust",
    );
    checker.check_entity(&src, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn non_entity_file_is_skipped() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_foo_vo.rs",
        "pub struct FooVo { pub name: String }",
        "rust",
    );
    checker.check_entity(&src, &mut violations);
    assert!(violations.is_empty());
}

// ─── check_error ────────────────────────────────────────────────────────────

#[test]
fn error_with_primitive_type_emits_violation() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_api_error.rs",
        "pub struct ApiError {\n    pub message: String,\n    pub code: i32,\n}",
        "rust",
    );
    checker.check_error(&src, &mut violations);
    assert!(!violations.is_empty());
}

#[test]
fn error_clean_no_violation() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_api_error.rs",
        "pub struct ApiError {\n    pub message: LintMessage,\n    pub code: ErrorCode,\n}",
        "rust",
    );
    checker.check_error(&src, &mut violations);
    assert!(violations.is_empty());
}

// ─── check_event ────────────────────────────────────────────────────────────

#[test]
fn event_with_primitives_emits_violations() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_scan_event.rs",
        "pub struct ScanEvent { pub path: String }",
        "rust",
    );
    checker.check_event(&src, &mut violations);
    assert!(!violations.is_empty());
}

// ─── check_constant ─────────────────────────────────────────────────────────

#[test]
fn constant_file_allows_const_and_use() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_colors_constant.rs",
        "pub const MAX_SIZE: usize = 100;\npub use shared::common::Foo;",
        "rust",
    );
    checker.check_constant(&src, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn constant_file_flags_struct_declaration() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_colors_constant.rs",
        "pub const MAX: usize = 100;\npub struct NotAConstant { pub x: i32 }",
        "rust",
    );
    checker.check_constant(&src, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES401"));
}

#[test]
fn constant_file_flags_fn_declaration() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_colors_constant.rs",
        "pub const MAX: usize = 100;\npub fn helper() -> i32 { 42 }",
        "rust",
    );
    checker.check_constant(&src, &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn non_constant_file_is_skipped() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_helper.rs",
        "pub struct Helper { pub x: i32 }",
        "rust",
    );
    checker.check_constant(&src, &mut violations);
    assert!(violations.is_empty());
}

// ─── scan_primitives via entity (Python) ────────────────────────────────────

#[test]
fn python_entity_with_primitives_emits_violations() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_user_entity.py",
        "class UserEntity:\n    name: str\n    age: int",
        "python",
    );
    checker.check_entity(&src, &mut violations);
    assert!(!violations.is_empty());
}

// ─── scan_primitives via entity (JavaScript) ────────────────────────────────

#[test]
fn js_entity_with_primitives_emits_violations() {
    let checker = TaxonomyRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source(
        "taxonomy_user_entity.ts",
        "interface UserEntity {\n  name: string;\n  age: number;\n}",
        "javascript",
    );
    checker.check_entity(&src, &mut violations);
    assert!(!violations.is_empty());
}
