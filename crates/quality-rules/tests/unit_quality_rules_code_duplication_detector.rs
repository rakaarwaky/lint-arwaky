// Unit tests for code duplication normalization utilities.
use quality_rules_lint_arwaky::utility_code_duplication_detector::{
    normalize_line, normalize_window,
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
