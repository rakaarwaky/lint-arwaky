// PURPOSE: Unit tests for UtilityRoleChecker — Python AES404 checks

use role_rules_lint_arwaky::capabilities_utility_role_auditor::UtilityRoleChecker;
use shared::cli_commands::LintResult;
use shared::filesystem::taxonomy_filesystem_vo::{FileEntry, Language};
use std::path::PathBuf;
use shared::role_rules::IUtilityRoleChecker;

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

// ─── Python: class/function in comments → NO violation ──

#[test]
fn py_class_in_comment_should_not_flag() {
    let content = r#"
# class CommentedClass:
#     pass

x = 1
"#;

    let source = make_file("utility_helper.py", content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        violations.is_empty(),
        "Commented Python definitions should not be flagged"
    );
}

// ─── Python: triple-double-quote docstring → NO violation ──

#[test]
fn py_docstring_should_not_flag() {
    let content = r#"
"""This is a docstring with class ClassName and def foo(): inside."""
x = 1
"#;

    let source = make_file("utility_helper.py", content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(violations.is_empty(), "Docstrings should not be flagged");
}

#[test]
fn py_triple_single_docstring_should_not_flag() {
    let content = r#"
'''This is a docstring with class ClassName and def foo(): inside.'''
x = 1
"#;

    let source = make_file("utility_helper.py", content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        violations.is_empty(),
        "Triple single-quote docstrings should not be flagged"
    );
}

// ─── Python: class → violation ──

#[test]
fn py_class_should_flag() {
    let content = r#"
class BadUtility:
    value = "hello"

def helper():
    pass
"#;

    let source = make_file("utility_helper.py", content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        !violations.is_empty(),
        "Class in Python utility should be flagged"
    );
    assert_eq!(violations[0].code.code(), "AES404");
}

// ─── Python: def → violation ──

#[test]
fn py_def_should_flag() {
    let content = r#"
def bad_function():
    pass

class GoodClass:
    pass
"#;

    let source = make_file("utility_helper.py", content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        !violations.is_empty(),
        "def in Python utility should be flagged"
    );
    assert_eq!(violations[0].code.code(), "AES404");
}

// ─── Python: pure functions → NO violation (but will flag def) ──

#[test]
fn py_functions_should_flag() {
    // In Python, utility files should NOT have def at all per AES rules.
    // Even "pure functions" are flagged because they're behavior, not data.
    let content = r#"
def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b
"#;

    let source = make_file("utility_helper.py", content);
    let checker = UtilityRoleChecker::new();
    let mut violations: Vec<LintResult> = Vec::new();

    checker.check_utility_convention(&source, &mut violations);
    assert!(
        !violations.is_empty(),
        "Python functions in utility should be flagged"
    );
    assert_eq!(violations[0].code.code(), "AES404");
}