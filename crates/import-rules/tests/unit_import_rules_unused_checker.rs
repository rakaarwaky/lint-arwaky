// Unit tests for UnusedImportRuleChecker — AES203 unused import detection.
use import_rules_lint_arwaky::capabilities_import_unused_checker::UnusedImportRuleChecker;
use shared::filesystem::taxonomy_filesystem_vo::{ImportEntry, ImportType, Language};
use shared::import_rules::IUnusedImportProtocol;
use std::path::PathBuf;

fn checker() -> UnusedImportRuleChecker {
    UnusedImportRuleChecker::new()
}

/// Build a simple ImportEntry for a Rust `use` statement.
fn rust_use(path: &str) -> ImportEntry {
    ImportEntry {
        source_file: PathBuf::new(),
        raw_path: path.to_string(),
        resolved_path: None,
        import_type: ImportType::Use,
        language: Language::Rust,
        is_dynamic: false,
        is_resolved: false,
        symbols: Vec::new(),
        is_reexport: false,
        is_wildcard: false,
    }
}

#[test]
fn detects_unused_rust_import() {
    let checker = checker();
    let content = r#"use std::collections::HashMap;

fn main() {
    println!("hello");
}
"#;
    let imports = vec![rust_use("std::collections::HashMap")];
    let result = checker
        .check_unused_imports("/tmp/test/src/app.rs", content, &imports)
        .unwrap();
    assert!(!result.is_empty(), "Should detect unused HashMap import");
    assert_eq!(result[0].code.code(), "AES203");
    assert!(
        result[0].message.value.contains("HashMap"),
        "Violation message should mention HashMap, got: {}",
        result[0].message
    );
}

#[test]
fn no_violation_when_import_is_used() {
    let checker = checker();
    let content = r#"use std::collections::HashMap;

fn main() {
    let _map = HashMap::new();
}
"#;
    let imports = vec![rust_use("std::collections::HashMap")];
    let result = checker
        .check_unused_imports("/tmp/test/src/main.rs", content, &imports)
        .unwrap();
    assert!(
        result.is_empty(),
        "Used import should produce no violations, got {}",
        result.len()
    );
}

#[test]
fn no_violation_for_barrel_files() {
    let checker = checker();
    let content = "use something::unused;\n";
    let imports = vec![rust_use("something::unused")];
    let result_lib = checker
        .check_unused_imports("/tmp/test/src/lib.rs", content, &imports)
        .unwrap();
    let result_mod = checker
        .check_unused_imports("/tmp/test/src/mod.rs", content, &imports)
        .unwrap();
    assert!(result_lib.is_empty(), "lib.rs should be skipped");
    assert!(result_mod.is_empty(), "mod.rs should be skipped");
}

#[test]
fn no_violation_for_empty_content() {
    let checker = checker();
    let result = checker
        .check_unused_imports("/tmp/test/src/file.rs", "", &[])
        .unwrap();
    assert!(
        result.is_empty(),
        "Empty content should produce no violations"
    );
}

#[test]
fn detects_multiple_unused_imports() {
    let checker = checker();
    let content = r#"use std::collections::HashMap;
use std::collections::BTreeMap;
use std::io::Read;

fn main() {
    println!("no imports used");
}
"#;
    let imports = vec![
        rust_use("std::collections::HashMap"),
        rust_use("std::collections::BTreeMap"),
        rust_use("std::io::Read"),
    ];
    let result = checker
        .check_unused_imports("/tmp/test/src/multi.rs", content, &imports)
        .unwrap();
    // At least HashMap and BTreeMap should be flagged (Read is a trait — may be skipped)
    assert!(
        result.len() >= 2,
        "Should detect at least 2 unused imports, got {}",
        result.len()
    );
}
