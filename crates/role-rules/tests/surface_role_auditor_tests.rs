use role_rules_lint_arwaky::capabilities_surface_role_auditor::{is_in_surfaces, is_init, SurfaceRoleChecker};
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn make_source(file: &str, content: &str, language: &str) -> SourceContentVO {
    let fp = FilePath::new(file.to_string()).unwrap_or_default();
    SourceContentVO::new(fp, ContentString::new(content.to_string()), language)
}

#[test]
fn test_is_in_surfaces() {
    let f = FilePath::new("src/surfaces/surface_handler.py")
        .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(is_in_surfaces(&f));

    let f = FilePath::new("src/capabilities/capabilities_not_checker.py")
        .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(!is_in_surfaces(&f));

    let f = FilePath::new("src/cli-commands/surface_check_command.rs")
        .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(is_in_surfaces(&f));
}

#[test]
fn test_is_init() {
    let f = FilePath::new("src/surfaces/__init__.py")
        .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(is_init(&f));

    let f = FilePath::new("src/surfaces/handler.py")
        .unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());
    assert!(!is_init(&f));
}

// ─── check_fn_count_limit ───────────────────────────────────────────────────

#[test]
fn fn_count_within_limit_no_violation() {
    let checker = SurfaceRoleChecker::new();
    let mut violations = Vec::new();
    let content = (0..10).map(|i| format!("fn helper_{}() {{}}\n", i)).collect::<String>();
    let src = make_source("surface_handler.rs", &content, "rust");
    checker.check_fn_count_limit(&src, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn fn_count_exceeding_limit_emits_violation() {
    let checker = SurfaceRoleChecker::new();
    let mut violations = Vec::new();
    let content = (0..20).map(|i| format!("fn helper_{}() {{}}\n", i)).collect::<String>();
    let src = make_source("surface_handler.rs", &content, "rust");
    checker.check_fn_count_limit(&src, &mut violations);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].code.to_string().contains("AES406"));
}

#[test]
fn python_fn_count_within_limit_no_violation() {
    let checker = SurfaceRoleChecker::new();
    let mut violations = Vec::new();
    let lines: Vec<String> = (0..5).map(|i| format!("def helper_{}():\n    pass\n", i)).collect();
    let content = lines.join("");
    let src = make_source("surface_handler.py", &content, "python");
    checker.check_fn_count_limit(&src, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn python_fn_count_exceeding_limit_emits_violation() {
    let checker = SurfaceRoleChecker::new();
    let mut violations = Vec::new();
    let lines: Vec<String> = (0..20).map(|i| format!("def helper_{}():\n    pass\n", i)).collect();
    let content = lines.join("");
    let src = make_source("surface_handler.py", &content, "python");
    checker.check_fn_count_limit(&src, &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn js_fn_count_exceeding_limit_emits_violation() {
    let checker = SurfaceRoleChecker::new();
    let mut violations = Vec::new();
    let content = (0..20).map(|i| format!("function helper_{}() {{}}\n", i)).collect::<String>();
    let src = make_source("surface_handler.js", &content, "javascript");
    checker.check_fn_count_limit(&src, &mut violations);
    assert_eq!(violations.len(), 1);
}

#[test]
fn empty_file_no_violation() {
    let checker = SurfaceRoleChecker::new();
    let mut violations = Vec::new();
    let src = make_source("surface_empty.rs", "", "rust");
    checker.check_fn_count_limit(&src, &mut violations);
    assert!(violations.is_empty());
}
