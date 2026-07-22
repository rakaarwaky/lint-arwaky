// PURPOSE: Acceptance test for AES406 — surface layer function count limit.
// Layer: Acceptance (FRD requirement validation).

use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use shared::cli_commands::taxonomy_result_vo::LintResultCode;
use shared::role_rules::contract_surface_role_protocol::ISurfaceRoleChecker;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn checker() -> SurfaceRoleChecker {
    SurfaceRoleChecker::new()
}

fn make_source(file: &str, content: &str) -> SourceContentVO {
    let fp = shared::common::taxonomy_path_vo::FilePath::new(file.to_string()).unwrap();
    let cs = ContentString::new(content.to_string());
    SourceContentVO::new(fp, cs, "rust")
}

// ─── Acceptance: Surface with 15 functions passes (boundary) ──

#[test]
fn acceptance_aes406_surface_at_function_limit_passes() {
    // FRD requirement: Surface files with <= 15 functions should pass
    let mut content = String::from("pub struct MySurface;\n");
    for i in 0..15 {
        content.push_str(&format!("    pub fn handle_{:02}(&self) {{}}\n", i));
    }
    let source = make_source("surface_boundary.rs", &content);
    let mut violations = Vec::new();
    checker().check_fn_count_limit(&source, &mut violations);
    assert!(violations.is_empty(), "AES406: Surface with 15 functions should pass");
}

// ─── Acceptance: Surface with 16+ functions flagged ──

#[test]
fn acceptance_aes406_surface_over_function_limit_flagged() {
    // FRD requirement: Surface files with > 15 functions should be flagged
    let mut content = String::from("pub struct MySurface;\n");
    for i in 0..16 {
        content.push_str(&format!("    pub fn handle_{:02}(&self) {{}}\n", i));
    }
    let source = make_source("surface_over_limit.rs", &content);
    let mut violations = Vec::new();
    checker().check_fn_count_limit(&source, &mut violations);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].code.code(), "AES406");
}

// ─── Acceptance: Surface type classification ──

#[test]
fn acceptance_aes406_smart_surface_classified() {
    // FRD requirement: Smart surfaces (commands, screens) should be classified correctly
    let content = r#"
pub struct MyCommand;
"#;
    let source = make_source("surface_my_command.rs", content);
    let mut violations = Vec::new();
    checker().check_smart_surface(&source, &mut violations);
    assert!(violations.is_empty(), "AES406: Smart surfaces should pass");
}

#[test]
fn acceptance_aes406_utility_surface_classified() {
    // FRD requirement: Utility surfaces (hooks, adapters) should be classified correctly
    let content = r#"
pub struct MyHook;
"#;
    let source = make_source("surface_my_hook.rs", content);
    let mut violations = Vec::new();
    checker().check_utility_surface(&source, &mut violations);
    assert!(violations.is_empty(), "AES406: Utility surfaces should pass");
}

#[test]
fn acceptance_aes406_passive_surface_classified() {
    // FRD requirement: Passive surfaces (views, panels) should be classified correctly
    let content = r#"
pub struct MyPanel;
"#;
    let source = make_source("surface_my_panel.rs", content);
    let mut violations = Vec::new();
    checker().check_passive_surface(&source, &mut violations);
    assert!(violations.is_empty(), "AES406: Passive surfaces should pass");
}
