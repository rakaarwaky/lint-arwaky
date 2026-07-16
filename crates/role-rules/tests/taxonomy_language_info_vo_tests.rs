use role_rules_lint_arwaky::taxonomy_language_info_vo::LanguageInfo;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};

fn make_source(file: &str, content: &str, language: &str) -> SourceContentVO {
    let fp = FilePath::new(file.to_string()).unwrap_or_default();
    SourceContentVO::new(fp, ContentString::new(content.to_string()), language)
}

// ─── LanguageInfo::new via SourceContentVO ────────────────────────────────────

#[test]
fn detects_rust_from_source() {
    let src = make_source("agent_orchestrator.rs", "fn main() {}", "rust");
    let info = LanguageInfo::new(&src);
    assert!(info.is_rs);
    assert!(!info.is_py);
    assert!(!info.is_js);
}

#[test]
fn detects_python_from_source() {
    let src = make_source("capabilities_checker.py", "def main(): pass", "python");
    let info = LanguageInfo::new(&src);
    assert!(info.is_py);
    assert!(!info.is_rs);
    assert!(!info.is_js);
}

#[test]
fn detects_javascript_from_source() {
    let src = make_source("surface_handler.js", "function main() {}", "javascript");
    let info = LanguageInfo::new(&src);
    assert!(!info.is_rs);
    assert!(!info.is_py);
    assert!(info.is_js);
}

#[test]
fn detects_typescript_from_source() {
    let src = make_source(
        "surface_handler.ts",
        "function main(): void {}",
        "javascript",
    );
    let info = LanguageInfo::new(&src);
    assert!(!info.is_rs);
    assert!(info.is_js);
}

#[test]
fn detects_unknown_language_as_not_rs_py_js() {
    let src = make_source("some_file.go", "package main", "go");
    let info = LanguageInfo::new(&src);
    assert!(!info.is_rs);
    assert!(!info.is_py);
    assert!(!info.is_js);
}

#[test]
fn language_string_reflects_detected_lang() {
    let src = make_source("agent_test.rs", "fn main() {}", "rust");
    let info = LanguageInfo::new(&src);
    assert_eq!(info.lang.to_string(), "rust");
}

// ─── LanguageInfo::new_from_path ──────────────────────────────────────────────

#[test]
fn detects_rust_from_path() {
    let fp = FilePath::new("src/capabilities_checker.rs".to_string()).unwrap_or_default();
    let info = LanguageInfo::new_from_path(&fp);
    assert!(info.is_rs);
}

#[test]
fn detects_python_from_path() {
    let fp = FilePath::new("src/capabilities_checker.py".to_string()).unwrap_or_default();
    let info = LanguageInfo::new_from_path(&fp);
    assert!(info.is_py);
}

#[test]
fn detects_javascript_from_path() {
    let fp = FilePath::new("src/surface_handler.js".to_string()).unwrap_or_default();
    let info = LanguageInfo::new_from_path(&fp);
    assert!(info.is_js);
}

#[test]
fn detects_typescript_from_path() {
    let fp = FilePath::new("src/surface_handler.ts".to_string()).unwrap_or_default();
    let info = LanguageInfo::new_from_path(&fp);
    assert!(info.is_js);
}
