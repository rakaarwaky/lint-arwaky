// Unit tests for code duplication normalization utilities.
use shared_quality_rules::utility_code_duplication_detector::{
    is_import_line, normalize_line, normalize_window,
};

#[test]
fn normalize_line_trims_and_filters() {
    assert_eq!(normalize_line("  hello, world!  "), "hello world");
    assert_eq!(normalize_line("fn foo() -> i32"), "fn foo  i32");
    assert_eq!(normalize_line(""), "");
}

#[test]
fn normalize_window_joins_with_pipe() {
    let window = vec!["fn foo()", "  let x = 1;", "  x + 1"];
    let result = normalize_window(&window);
    assert_eq!(result, "fn foo|let x  1|x  1");
}

// ─── is_import_line unit tests ─────────────────────────────────────────────────

#[test]
fn is_import_line_catches_rust_use() {
    assert!(is_import_line("use x;"));
    assert!(is_import_line("pub use x;"));
    assert!(is_import_line("use\tstd::io::Read;"));
    assert!(is_import_line("pub use\tstd::io::Read;"));
}

#[test]
fn is_import_line_catches_python() {
    assert!(is_import_line("import x"));
    assert!(is_import_line("import\tx"));
    assert!(is_import_line("from x import y"));
    assert!(is_import_line("from\tx import y"));
}

#[test]
fn is_import_line_catches_js_ts_import_and_reexport() {
    assert!(is_import_line("import x from 'y'"));
    assert!(is_import_line("import\tx from 'y'"));
    // Re-export: `export ... from ...` is scaffolding.
    assert!(is_import_line("export { a } from 'b'"));
    // `export const x = 1` has no `from` → real code, not an import line.
    assert!(!is_import_line("export const x = 1"));
    assert!(!is_import_line("export default function f() {}"));
}

#[test]
fn is_import_line_rejects_non_imports_and_empty() {
    assert!(!is_import_line(""));
    assert!(!is_import_line("imported()"));
    assert!(!is_import_line("fn logic() { let x = 1; }"));
    assert!(!is_import_line("let x = import x;"));
    // Tab variants of real code stay false.
    assert!(!is_import_line("\tfn logic() {}"));
}
