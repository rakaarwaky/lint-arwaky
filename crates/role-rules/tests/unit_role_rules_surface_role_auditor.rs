// Unit tests for SurfaceRoleChecker — surfaces-layer role audit (AES406).
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use shared::common::Severity;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::ISurfaceRoleChecker;

use shared::filesystem::taxonomy_filesystem_vo::{
    Language, ParseMetadata, RustMetadata, TypeScriptMetadata,
};
use std::path::PathBuf;

fn checker() -> SurfaceRoleChecker {
    SurfaceRoleChecker::new()
}

fn make_file(path: &str, lang: Language, content: &str) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: match lang {
            Language::Rust => "rs",
            Language::Python => "py",
            _ => "ts",
        }
        .to_string(),
        language: lang,
        size: content.len() as u64,
        content: content.to_string(),
        parse_ok: false,
        parse_metadata: None,
    }
}

fn make_file_with_rust_meta(path: &str, meta: RustMetadata) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: "rs".to_string(),
        language: Language::Rust,
        size: 100,
        content: String::new(),
        parse_ok: true,
        parse_metadata: Some(ParseMetadata::Rust(meta)),
    }
}

fn make_file_with_ts_meta(path: &str, meta: TypeScriptMetadata) -> FileEntry {
    FileEntry {
        path: PathBuf::from(path),
        extension: "ts".to_string(),
        language: Language::TypeScript,
        size: 100,
        content: String::new(),
        parse_ok: true,
        parse_metadata: Some(ParseMetadata::TypeScript(meta)),
    }
}

#[test]
fn construction_succeeds() {
    let _ = checker();
}

#[test]
fn fn_count_under_limit_no_violation() {
    let content = (0..10)
        .map(|i| format!("fn func_{}() {{}}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let f = make_file("src/surface_something.rs", Language::Rust, &content);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(v.is_empty(), "10 functions should be under the limit of 15");
}

#[test]
fn fn_count_over_limit_no_longer_flagged() {
    let content = (0..20)
        .map(|i| format!("fn func_{}() {{}}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let f = make_file("src/surface_something.rs", Language::Rust, &content);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        v.is_empty(),
        "fn count limit removed — no violation expected"
    );
}

#[test]
fn fn_count_over_limit_metadata_no_longer_flagged() {
    let meta = RustMetadata {
        function_definitions: (0..20)
            .map(|i| shared::filesystem::taxonomy_filesystem_vo::RustFnItem {
                name: format!("func_{}", i),
                has_body: true,
            })
            .collect(),
        ..Default::default()
    };
    let f = make_file_with_rust_meta("src/surface_something.rs", meta);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        v.is_empty(),
        "fn count limit removed — no violation expected"
    );
}

#[test]
fn smart_surface_exempt_from_passive_checks() {
    // Smart surfaces (suffix _command, _controller, etc.) should not get passive violations
    let mut content = String::new();
    for i in 0..5 {
        content.push_str(&format!("if condition_{} {{}}\n", i));
    }
    let f = make_file("src/surface_my_command.rs", Language::Rust, &content);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        v.is_empty(),
        "smart surface should be exempt from passive control flow checks"
    );
}

#[test]
fn passive_surface_control_flow_flagged_fallback() {
    // "my_view" is NOT a smart surface suffix — treated as passive
    let mut content = String::new();
    for i in 0..51 {
        content.push_str(&format!("if condition_{} {{}}\n", i));
    }
    let f = make_file("src/surface_my_view.rs", Language::Rust, &content);
    let mut v = Vec::new();
    checker().check_passive_surface(&f, &mut v);
    assert!(
        !v.is_empty(),
        "excess control flow in passive surface should be flagged"
    );
}

#[test]
fn fn_count_python_no_longer_flagged() {
    let content = (0..20)
        .map(|i| format!("def func_{}(): pass", i))
        .collect::<Vec<_>>()
        .join("\n");
    let f = make_file("src/surface_something.py", Language::Python, &content);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        v.is_empty(),
        "fn count limit removed — no violation expected"
    );
}

#[test]
fn fn_count_typescript_metadata_no_longer_flagged() {
    let meta = TypeScriptMetadata {
        function_definitions: (0..16)
            .map(|i| shared::filesystem::taxonomy_filesystem_vo::TSFnItem {
                name: format!("func_{}", i),
                has_body: true,
            })
            .collect(),
        ..Default::default()
    };
    let f = make_file_with_ts_meta("src/surface_something.ts", meta);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        v.is_empty(),
        "fn count limit removed — no violation expected"
    );
}
