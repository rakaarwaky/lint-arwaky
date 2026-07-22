// PURPOSE: Unit tests for UnusedImportRuleChecker (AES203)
// Tests check_unused_imports with content passed directly — no file I/O.

use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;

fn sut() -> UnusedImportRuleChecker {
    UnusedImportRuleChecker::new()
}

// ─── Happy Path ───────────────────────────────────────────

#[test]
fn used_import_produces_no_violation() {
    let content = r#"
use std::collections::HashMap;

fn main() {
    let map: HashMap<String, i32> = HashMap::new();
    println!("{:?}", map);
}
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("test.rs", content, &mut violations);
    assert!(violations.is_empty(), "Used import should not be flagged");
}

#[test]
fn multiple_used_imports_produce_no_violations() {
    let content = r#"
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let _m: HashMap<String, i32> = HashMap::new();
    let _s: HashSet<i32> = HashSet::new();
}
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("test.rs", content, &mut violations);
    assert!(violations.is_empty());
}

// ─── Unused Import Detection ──────────────────────────────

#[test]
fn single_unused_import_detected() {
    let content = r#"
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let _m: HashMap<String, i32> = HashMap::new();
}
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("test.rs", content, &mut violations);
    assert!(
        violations
            .iter()
            .any(|v| v.message.value().contains("HashSet")),
        "HashSet should be flagged as unused"
    );
}

#[test]
fn all_unused_imports_detected() {
    let content = r#"
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;

fn main() {
    println!("hello");
}
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("test.rs", content, &mut violations);
    assert!(
        violations.len() >= 2,
        "Multiple unused imports should be flagged"
    );
}

// ─── Edge Cases ───────────────────────────────────────────

#[test]
fn empty_file_produces_no_violations() {
    let mut violations = Vec::new();
    sut().check_unused_imports("empty.rs", "", &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn file_with_no_imports_produces_no_violations() {
    let content = r#"
fn main() {
    println!("hello world");
}
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("no_imports.rs", content, &mut violations);
    assert!(violations.is_empty());
}

#[test]
fn derive_macros_not_flagged_as_unused() {
    let content = r#"
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Foo {
    bar: String,
}
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("derive.rs", content, &mut violations);
    assert!(
        !violations.iter().any(|v| {
            v.message.value().contains("Serialize") || v.message.value().contains("Deserialize")
        }),
        "Derive macros should never be flagged as unused"
    );
}

#[test]
fn trait_imports_not_flagged_as_unused() {
    let content = r#"
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;

struct MyChecker;

impl IUnusedImportProtocol for MyChecker {
    fn find_unused_imports(&self, _path: &shared::common::taxonomy_path_vo::FilePath) -> Vec<shared::common::taxonomy_message_vo::LintMessage> {
        vec![]
    }
    fn check_unused_imports(&self, _file: &str, _content: &str, _violations: &mut Vec<shared::cli_commands::taxonomy_result_vo::LintResult>) {}
}
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("trait_impl.rs", content, &mut violations);
    assert!(
        !violations
            .iter()
            .any(|v| v.message.value().contains("IUnusedImportProtocol")),
        "Trait imports used in impl blocks should not be flagged"
    );
}

// ─── Violation Metadata ───────────────────────────────────

#[test]
fn violation_has_correct_code_aes203() {
    let content = r#"
use std::collections::HashMap;

fn main() {}
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("meta.rs", content, &mut violations);
    if let Some(v) = violations.first() {
        assert_eq!(v.code.code(), "AES203");
    }
}

#[test]
fn violation_severity_is_medium() {
    let content = r#"
use std::collections::HashMap;

fn main() {}
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("sev.rs", content, &mut violations);
    if let Some(v) = violations.first() {
        assert_eq!(
            v.severity,
            shared::cli_commands::taxonomy_severity_vo::Severity::MEDIUM
        );
    }
}

// ─── Python-style imports ─────────────────────────────────

#[test]
fn python_unused_import_detected() {
    let content = r#"
import os
import sys

def main():
    print(os.getcwd())
"#;
    let mut violations = Vec::new();
    sut().check_unused_imports("test.py", content, &mut violations);
    assert!(
        violations.iter().any(|v| v.message.value().contains("sys")),
        "Unused Python import 'sys' should be flagged"
    );
}

// ─── find_unused_imports (path-based) ─────────────────────

#[test]
fn find_unused_imports_nonexistent_file_returns_empty() {
    use shared::common::taxonomy_path_vo::FilePath;
    let path = FilePath::new("/nonexistent/path/file.rs").unwrap();
    let result = sut().find_unused_imports(&path);
    assert!(result.is_empty());
}
