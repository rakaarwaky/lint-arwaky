// Unit tests for SurfaceRoleChecker — surfaces-layer role audit (AES406).
use role_rules_lint_arwaky::capabilities_surface_role_auditor::SurfaceRoleChecker;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::ISurfaceRoleProtocol;

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
fn fn_count_under_smart_limit_no_violation() {
    // `surface_something` has no tier suffix, so it is passive (limit 25).
    // 10 functions is under 25.
    let content = (0..10)
        .map(|i| format!("fn func_{}() {{}}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let f = make_file("src/surface_something.rs", Language::Rust, &content);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        v.is_empty(),
        "10 functions should be under the passive limit of 25"
    );
}

#[test]
fn fn_count_over_passive_limit_flagged() {
    // 51 functions in an unknown-suffix file → passive → limit 25 → violation.
    let content = (0..51)
        .map(|i| format!("fn func_{}() {{}}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let f = make_file("src/surface_something.rs", Language::Rust, &content);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        !v.is_empty(),
        "51 functions in a passive-tier file should flag AES406"
    );
}

#[test]
fn fn_count_over_passive_limit_metadata_flagged() {
    let meta = RustMetadata {
        function_definitions: (0..26)
            .map(|i| shared::filesystem::taxonomy_filesystem_vo::RustFnItem {
                name: format!("func_{}", i),
                has_body: true,
            })
            .collect(),
        ..Default::default()
    };
    // `_layout` is passive. 26 > 25.
    let f = make_file_with_rust_meta("src/surface_palette_layout.rs", meta);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        !v.is_empty(),
        "26 functions in a passive-tier file (via AST) should flag AES406"
    );
}

#[test]
fn smart_surface_exempt_from_passive_checks() {
    // Smart surfaces (suffix _command, _controller, _page) are exempt from
    // passive control-flow checks — but the fn-count check still applies.
    let mut content = String::new();
    for i in 0..5 {
        content.push_str(&format!("if condition_{} {{}}\n", i));
    }
    let f = make_file("src/surface_my_command.rs", Language::Rust, &content);
    let mut v = Vec::new();
    checker().check_smart_surface(&f, &mut v);
    assert!(
        v.is_empty(),
        "smart surface should be exempt from passive control flow checks"
    );
}

#[test]
fn passive_surface_control_flow_flagged_fallback() {
    // "my_view" is a passive surface suffix — excess control flow is flagged.
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
fn fn_count_python_over_passive_limit_flagged() {
    // 51 def in an unknown-suffix file → passive → limit 25 → violation.
    let content = (0..51)
        .map(|i| format!("def func_{}(): pass", i))
        .collect::<Vec<_>>()
        .join("\n");
    let f = make_file("src/surface_something.py", Language::Python, &content);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        !v.is_empty(),
        "51 functions in a passive-tier Python surface should flag AES406"
    );
}

#[test]
fn fn_count_python_under_utility_limit_no_violation() {
    // `_action` is utility (limit 25). 20 functions stays clean.
    let content = (0..20)
        .map(|i| format!("def func_{}(): pass", i))
        .collect::<Vec<_>>()
        .join("\n");
    let f = make_file("src/surface_check_action.py", Language::Python, &content);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        v.is_empty(),
        "20 functions in a utility-tier Python surface should be clean"
    );
}

#[test]
fn fn_count_typescript_metadata_flagged() {
    let meta = TypeScriptMetadata {
        function_definitions: (0..26)
            .map(|i| shared::filesystem::taxonomy_filesystem_vo::TSFnItem {
                name: format!("func_{}", i),
                has_body: true,
            })
            .collect(),
        ..Default::default()
    };
    // `_view` is passive. 26 > 25.
    let f = make_file_with_ts_meta("src/surface_dashboard_view.ts", meta);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        !v.is_empty(),
        "26 functions in a passive-tier TS surface (via AST) should flag AES406"
    );
}

#[test]
fn fn_count_typescript_smart_limit_50_no_violation_at_30() {
    // `_command` is smart (limit 50). 30 functions stays clean.
    let meta = TypeScriptMetadata {
        function_definitions: (0..30)
            .map(|i| shared::filesystem::taxonomy_filesystem_vo::TSFnItem {
                name: format!("func_{}", i),
                has_body: true,
            })
            .collect(),
        ..Default::default()
    };
    let f = make_file_with_ts_meta("src/surface_ci_command.ts", meta);
    let mut v = Vec::new();
    checker().check_fn_count_limit(&f, &mut v);
    assert!(
        v.is_empty(),
        "30 functions in a smart-tier TS surface should be clean (smart limit 50)"
    );
}

// ── Tier classification sanity ─────────────────────────────

#[test]
fn tier_classification_for_router_is_utility() {
    use shared::role_rules::taxonomy_role_rules_vo::{SurfaceTier, classify_surface_tier};
    assert_eq!(
        classify_surface_tier("surface_x_router"),
        SurfaceTier::Utility
    );
}

#[test]
fn tier_classification_for_entry_is_not_smart() {
    // `_entry` is not a surface suffix — it classifies as passive by fallback.
    // It is still legal only via AES102 (root layer); no tier limit applies here
    // because an AES102 violation would have already been reported.
    use shared::role_rules::taxonomy_role_rules_vo::{SurfaceTier, classify_surface_tier};
    assert_eq!(
        classify_surface_tier("surface_x_entry"),
        SurfaceTier::Passive
    );
}
