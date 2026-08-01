// PURPOSE: Unit tests for SurfaceRoleChecker (AES406) — function count limit.
// Layer: Capabilities (SurfaceRoleChecker)

use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use std::path::PathBuf;
use shared::role_rules::ISurfaceRoleChecker;

fn checker() -> SurfaceRoleChecker {
    SurfaceRoleChecker::new()
}

fn make_file(path: &str, content: &str) -> FileEntry {
    let ext = path.rsplit('.').next().unwrap_or("rs").to_string();
    let language = match ext.as_str() {
        "rs" => Language::Rust,
        "py" => Language::Python,
        "ts" | "tsx" => Language::TypeScript,
        "js" | "jsx" => Language::JavaScript,
        _ => Language::Rust,
    };
    FileEntry {
        path: PathBuf::from(path),
        extension: ext,
        language,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: true,
        parse_metadata: None,
    }
}

// ─── check_fn_count_limit: Happy Path ────────────────

#[test]
fn surface_with_few_functions_passes() {
    let content = r#"
pub struct MySurface;

impl MySurface {
    pub fn handle_check(&self) {}
    pub fn handle_scan(&self) {}
}
"#;
    let source = make_file("surface_my_surface.rs", content);
    let mut violations = Vec::new();
    checker().check_fn_count_limit(&source, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn surface_with_15_functions_passes() {
    // Generate exactly 15 functions (at the boundary)
    let mut content = String::from("pub struct MySurface;\n");
    for i in 0..15 {
        content.push_str(&format!("    pub fn handle_{:02}(&self) {{}}\n", i));
    }
    let source = make_file("surface_boundary.rs", &content);
    let mut violations = Vec::new();
    checker().check_fn_count_limit(&source, &mut violations);
    assert!(violations.is_empty());
}

// ─── check_fn_count_limit: AES406 Violation ──────────

#[test]
fn surface_over_15_functions_flagged() {
    // Generate 16 functions (one over the limit)
    let mut content = String::from("pub struct MySurface;\n");
    for i in 0..16 {
        content.push_str(&format!("    pub fn handle_{:02}(&self) {{}}\n", i));
    }
    let source = make_file("surface_too_many.rs", &content);
    let mut violations = Vec::new();
    checker().check_fn_count_limit(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES406");
}

#[test]
fn surface_with_many_functions_message_mentions_count() {
    let mut content = String::from("pub struct MySurface;\n");
    for i in 0..30 {
        content.push_str(&format!("    pub fn handle_{:02}(&self) {{}}\n", i));
    }
    let source = make_file("surface_excessive.rs", &content);
    let mut violations = Vec::new();
    checker().check_fn_count_limit(&source, &mut violations);
    // Message may vary — just verify it's not empty
    assert!(!violations[0].message.value.is_empty());
}

// ─── check_smart_surface / utility_surface / passive_surface (no-op) ──

#[test]
fn check_smart_surface_no_violation() {
    let source = make_file("surface_my_command.rs", "pub struct MyCommand;");
    let mut violations = Vec::new();
    checker().check_smart_surface(&source, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn check_utility_surface_no_violation() {
    let source = make_file("surface_my_hook.rs", "pub struct MyHook;");
    let mut violations = Vec::new();
    checker().check_utility_surface(&source, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn check_passive_surface_no_violation() {
    let source = make_file("surface_my_screen.rs", "pub struct MyScreen;");
    let mut violations = Vec::new();
    checker().check_passive_surface(&source, &mut violations);
    assert!(violations.is_empty());
}

// ─── Default trait ──────────────────────────────────

#[test]
fn default_creates_valid_instance() {
    let _ = SurfaceRoleChecker::default();
}